/*
oko-lang: Language of vision
Copyright (C) 2025 tixonochek
        This program is free software: you can redistribute it and/or modify
        it under the terms of the GNU General Public License as published by
        the Free Software Foundation, either version 3 of the License, or
        (at your option) any later version.
    
        This program is distributed in the hope that it will be useful,
        but WITHOUT ANY WARRANTY; without even the implied warranty of
        MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
        GNU General Public License for more details.
    
        You should have received a copy of the GNU General Public License
        along with this program. If not, see <https://www.gnu.org/licenses/>.
*/

import { nodeTypes } from "./parser.js";
import { error } from "./utils.js";
import * as builtIns from "./builtins.js";

export const valTypes = {
    Number: "Number",
    String: "String",
    Array: "Array",
    Nil: "Nil"
}

const loadedMods = {};

export const opFunctions = {
    "+": (left, right) => {
        if (left.type == valTypes.Nil || right.type == valTypes.Nil) {
            return { type: valTypes.Nil, value: null }
        }

        if (left.type == valTypes.Number && right.type == valTypes.Number) {
            return {
                type: valTypes.Number,
                value: left.value + right.value
            }
        }

        if (left.type == valTypes.String && right.type == valTypes.String) {
            return {
                type: valTypes.String,
                value: left.value + right.value
            }
        }

        if (left.type == valTypes.String && right.type == valTypes.Number) {
            return {
                type: valTypes.String,
                value: left.value + right.value
            }
        }

        if (left.type == valTypes.Number && right.type == valTypes.String) {
            return {
                type: valTypes.String,
                value: left.value + right.value
            }
        }

        error(17, [ "+" ]);
    },

    "-": (left, right) => {
        if (left.type == valTypes.Nil || right.type == valTypes.Nil) {
            return { type: valTypes.Nil, value: null }
        }

        if (left.type == valTypes.Number && right.type == valTypes.Number) {
            return {
                type: valTypes.Number,
                value: left.value - right.value
            }
        }

        error(17, [ "-" ]);
    },

    "*": (left, right) => {
        if (left.type == valTypes.Nil || right.type == valTypes.Nil) {
            return { type: valTypes.Nil, value: null }
        }

        if (left.type == valTypes.Number && right.type == valTypes.Number) {
            return {
                type: valTypes.Number,
                value: left.value * right.value
            }
        }

        if (left.type == valTypes.Number && right.type == valTypes.String) {
            return {
                type: valTypes.String,
                value: right.value.repeat(left.value)
            }
        }

        if (left.type == valTypes.String && right.type == valTypes.Number) {
            return {
                type: valTypes.String,
                value: left.value.repeat(right.value)
            }
        }

        error(17, [ "*" ]);
    },

    "/": (left, right) => {
        if (left.type == valTypes.Nil || right.type == valTypes.Nil) {
            return { type: valTypes.Nil, value: null }
        }

        if (left.type == valTypes.Number && right.type == valTypes.Number) {
            return {
                type: valTypes.Number,
                value: left.value / right.value
            }
        }

        error(17, [ "/" ]);
    },

    "^": (left, right) => {
        if (left.type == valTypes.Nil || right.type == valTypes.Nil) {
            return { type: valTypes.Nil, value: null }
        }

        if (left.type == valTypes.Number && right.type == valTypes.Number) {
            return {
                type: valTypes.Number,
                value: Math.pow(left.value, right.value)
            }
        }

        error(17, [ "^" ]);
    },

    "%": (left, right) => {
        if (left.type == valTypes.Nil || right.type == valTypes.Nil) {
            return { type: valTypes.Nil, value: null }
        }

        if (left.type == valTypes.Number && right.type == valTypes.Number) {
            return {
                type: valTypes.Number,
                value: left.value % right.value
            }
        }

        error(17, [ "%" ]);
    },

    ">": (left, right) => {
        if (left.type == valTypes.Nil || right.type == valTypes.Nil) {
            return { type: valTypes.Nil, value: null }
        }

        if (left.type == valTypes.Number && right.type == valTypes.Number) {
            return {
                type: valTypes.Number,
                value: left.value > right.value ? 1 : 0
            }
        }

        error(17, [ ">" ]);
    },

    "<": (left, right) => {
        if (left.type == valTypes.Nil || right.type == valTypes.Nil) {
            return { type: valTypes.Nil, value: null }
        }

        if (left.type == valTypes.Number && right.type == valTypes.Number) {
            return {
                type: valTypes.Number,
                value: left.value < right.value ? 1 : 0
            }
        }

        error(17, [ "<" ]);
    },

    "<=": (left, right) => {
        if (left.type == valTypes.Nil || right.type == valTypes.Nil) {
            return { type: valTypes.Nil, value: null }
        }

        if (left.type == valTypes.Number && right.type == valTypes.Number) {
            return {
                type: valTypes.Number,
                value: left.value <= right.value ? 1 : 0
            }
        }

        error(17, [ "<=" ]);
    },

    ">=": (left, right) => {
        if (left.type == valTypes.Nil || right.type == valTypes.Nil) {
            return { type: valTypes.Nil, value: null }
        }

        if (left.type == valTypes.Number && right.type == valTypes.Number) {
            return {
                type: valTypes.Number,
                value: left.value <= right.value ? 1 : 0
            }
        }

        error(17, [ ">=" ]);
    },

    "==": (left, right) => {
        if (left.type == valTypes.Array || right.type == valTypes.Array) {
            error(17, [ "==" ]);
        }

        return {
            type: valTypes.Number,
            value: ((left.value === right.value) && (left.type === right.type)) ? 1 : 0
        }
    },

    "&&": (left, right) => {
        if (left.type == valTypes.Nil || right.type == valTypes.Nil) {
            return { type: valTypes.Nil, value: null }
        }

        if (left.type == valTypes.Number && right.type == valTypes.Number) {
            return {
                type: valTypes.Number,
                value: left.value && right.value ? 1 : 0
            }
        }

        error(17, [ "&&" ]);
    },

    "||": (left, right) => {
        if (left.type == valTypes.Nil || right.type == valTypes.Nil) {
            return { type: valTypes.Nil, value: null }
        }

        if (left.type == valTypes.Number && right.type == valTypes.Number) {
            return {
                type: valTypes.Number,
                value: left.value || right.value ? 1 : 0
            }
        }

        error(17, [ "||" ]);
    }
}

export const opFunctionsUnary = {
    "!": (operand) => {
        if (operand.type == valTypes.Nil) {
            return {
                type: valTypes.Number,
                value: 0
            }
        }

        if (operand.type == valTypes.Number) {
            return {
                type: valTypes.Number,
                value: operand.value == 0 ? 1 : 0
            }
        }

        error(17, [ "!" ]);
    }
}

const nodeFunctions = {
    "VariableAssign": (ctx, node) => {
        const varName = node.name;
        const varValue = evalExpr(ctx, node.value);
        const assignOp = node.operator;

        if (varValue.body) { error(21); }

        switch (assignOp) {
            case ":=":
                ctx.scopes[ctx.curScope][varName] = {
                    type: varValue.type,
                    value: varValue.value
                };
                break;

            case "=": {
                const posVar = scopeLookup(ctx, varName);

                if (!posVar) {
                    error(16, [ "=" ]);
                }

                if (posVar.type !== varValue.type) {
                    error(15, [ "=" ]);
                }

                ctx.scopes[posVar.originScope][varName] = {
                    type: varValue.type,
                    value: varValue.value
                };
                break;
            }

            case "/=":
            case "*=":
            case "-=":
            case "+=": {
                const posVar = scopeLookup(ctx, varName);
                const fstChar = assignOp[0];

                if (!posVar) {
                    error(16, [ fstChar + "=" ]);
                }

                if (posVar.type !== varValue.type) {
                    error(15, [ fstChar + "=" ]);
                }

                ctx.scopes[posVar.originScope][varName] = opFunctions[fstChar](posVar, varValue);
                break;
            }
        }

        return ctx;
    },
    
    "FunctionDeclaration": (ctx, node) => {
        const funcName = node.name;
        const funcArgs = node.arguments;
        const funcBody = node.body;

        ctx.scopes[ctx.curScope][funcName] = {
            arguments: funcArgs,
            body: funcBody
        };

        return ctx;
    },

    "ReturnStat": (ctx, node) => {
        if (ctx.curScope <= 0) {
            error(18);
        }

        ctx.stop = true;
        ctx.returnVal = evalExpr(ctx, node.value);

        return ctx;
    },

    "ExprStat": (ctx, node) => {
        evalExpr(ctx, node.expression);
    },

    "IfStat": (ctx, node) => {
        const mainCond = evalExpr(ctx, node.condition);
        const mainBody = node.body;
        let shouldElse = true;

        if (!(mainCond.value === null || mainCond.value === 0 || mainCond.value === "" || mainCond.value.length === 0)) {
            executeFunction(ctx, mainBody);
            shouldElse = false;
            return;
        }

        node.elifs.some(elif => {
            const elifCond = evalExpr(ctx, elif.condition);
            const elifBody = elif.body;

            if (!(elifCond.value === null || elifCond.value === 0 || elifCond.value === "" || elifCond.value.length === 0)) {
                executeFunction(ctx, elifBody);
                shouldElse = false;
                return true;
            }

            return false;
        });

        if (node.else && shouldElse) {
            executeFunction(ctx, node.else);
            return;
        }
    },

    "ImportStat": (ctx, node) => {
        const modName = node.module;

        if (!builtIns.modules[modName]) {
            error(24, [ modName ])
        }

        loadedMods[modName] = builtIns.modules[modName];
    },

    "WhileStat": (ctx, node) => {
        while (evalExpr(ctx, node.condition).value !== 0 && evalExpr(ctx, node.condition).value !== "" && evalExpr(ctx, node.condition).value.length !== 0 && evalExpr(ctx, node.condition).value !== null) {
            executeFunction(ctx, node.body);
        }
    },

    "FeachStat": (ctx, node) => {
        const arr = evalExpr(ctx, node.array);
        const elName = node.element;
        const body = node.body;

        for (const jsElement of arr.value) {
            ctx.scopes[ctx.curScope][elName] = {
                type: jsElement.type,
                value: jsElement.value
            }
            executeFunction(ctx, body);
        }
        ctx.scopes[ctx.curScope][elName] = undefined;
    }
}

function evalExpr(ctx, expr) {    
    switch (expr.type) {
        case nodeTypes.BinaryExpr: {
            if (!opFunctions[expr.operator]) { error(0); }

            return opFunctions[expr.operator](evalExpr(ctx, expr.left), evalExpr(ctx, expr.right));
        }

        case nodeTypes.UnaryExpr: {
            if (!opFunctionsUnary[expr.operator]) { error(0); }

            return opFunctionsUnary[expr.operator](evalExpr(ctx, expr.operand));
        }

        case nodeTypes.NumLiteral: {
            return {
                type: valTypes.Number,
                value: expr.value
            }
        }

        case nodeTypes.StrLiteral: {
            return {
                type: valTypes.String,
                value: expr.value
            }
        }

        case nodeTypes.Identifier: {
            const posVar = scopeLookup(ctx, expr.name);

            if (!posVar) { error(14, [ expr.lineIndex ]); }

            return posVar;
        }

        case nodeTypes.ArrayLiteral: {
            return {
                type: valTypes.Array,
                value: expr.elements.map(el => evalExpr(ctx, el))
            }
        }

        case nodeTypes.FunctionCall: {
            const funcName = expr.name.name;
            const funcArgs = expr.args.map(el => evalExpr(ctx, el));
            if (expr.fromMod) {
                const posMod = loadedMods[expr.fromMod];

                if (!posMod) {
                    error(23, [ expr.fromMod ]);
                }

                const posBuiltInFunc = posMod[funcName];

                if (!posBuiltInFunc) {
                    error(19, [ funcName ]);
                }

                if (posBuiltInFunc.requiredArgs !== Infinity) {
                    if (funcArgs.length !== posBuiltInFunc.requiredArgs) {
                        error(20, [ funcName ]);
                    }
                }

                let posRet = null;
                if (posBuiltInFunc.requiredArgs === Infinity) {
                    posRet = posBuiltInFunc.callback(funcArgs);
                } else {
                    posRet = posBuiltInFunc.callback(...funcArgs);
                }

                if (posRet) { return posRet; }
                else { return { type: valTypes.Nil, value: null } }
            }

            const posFunc = scopeLookup(ctx, funcName);

            if (!posFunc) {
                error(19, [ funcName ]);
            }

            if (funcArgs.length !== posFunc.arguments.length) {
                error(20, [ funcName ]);
            }

            const argArr = [];
            posFunc.arguments.forEach(el => {
                argArr[el] = funcArgs.shift();
            });

            ctx.scopes.push(argArr);
            ctx.curScope++;
            const posRet = executeFunction(ctx, posFunc.body);
            ctx.scopes.length--;
            ctx.curScope--;

            if (posRet) { return posRet; }
            else { return { type: valTypes.Nil, value: null } }
        }

        case (nodeTypes.ModAccess): {
            if (expr.member.type !== nodeTypes.FunctionCall) {
                error(22);
            }

            return evalExpr(ctx, {
                type: "FunctionCall",
                name: expr.member.name,
                args: expr.member.args,
                fromMod: expr.mod.name
            })
        }

        default: {
            error(0);
        }
    }
}

function scopeLookup(ctx, name) {
    while (ctx.curScope >= 0) {
        if (ctx.scopes[ctx.curScope][name]) {
            return {originScope: ctx.curScope, ...ctx.scopes[ctx.curScope][name]};
        }

        ctx.curScope--;
    }

    return null;
}

function executeFunction(ctx, body) {
    let j = 0;

    function peek(index = 0) {
        return (body[j + index]) ? body[j + index] : null;
    }

    while (peek() !== null) {
        const node = peek();

        if (!nodeFunctions[node.type]) { error(0); }
        
        const result = nodeFunctions[node.type]({...ctx}, node);
        ctx = result ? result : ctx;
        if (ctx.stop) { break; }

        j++;
    }

    if (ctx.returnVal) { return ctx.returnVal; }
}

export function execute(body) {
    // deno-lint-ignore prefer-const
    let programContext = {
        scopes: [ {} ],
        curScope: 0
    }

    executeFunction(programContext, body.body);
}