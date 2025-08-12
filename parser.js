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

import { brightRed } from "https://deno.land/std/fmt/colors.ts";
import { error } from "./utils.js";

const variableAssignOps = [
    ":=",
    "=",
    "+=",
    "-=",
    "*=",
    "/=",
];

export const nodeTypes = {
    Program: "Program",
    ImportStat: "ImportStat",
    VariableAssign: "VariableAssign",
    BinaryExpr: "BinaryExpr",
    NumLiteral: "NumLiteral",
    StrLiteral: "StrLiteral",
    Identifier: "Identifier",
    FunctionCall: "FunctionCall",
    ModAccess: "ModAccess",
    UnaryExpr: "UnaryExpr",
    ArrayLiteral: "ArrayLiteral",
    ReturnStat: "ReturnStat",
    FunctionDeclaration: "FunctionDeclaration",
    ExprStat: "ExprStat",
    IfStat: "IfStat",
    WhileStat: "WhileStat",
    FeachStat: "FeachStat",
};

const opPrecedence = {
    "||": 1,
    "&&": 2,

    "!=": 3,
    "==": 3,
    ">": 3,
    "<": 3,
    ">=": 3,
    "<=": 3,

    "+": 5,
    "-": 5,
    "*": 6,
    "/": 6,
    "%": 6,

    "^": 7,

    "::": 8,
};

const unaryOps = [
    "!",
];

export function buildTree(tokens) {
    let j = 0;

    function peek(index = 0) {
        return (tokens[j + index]) ? tokens[j + index] : null;
    }

    function consume(type, value) {
        const token = peek();
        const ofType = type ? ` (of type ${type})` : "";
        const ofValue = value ? ` (of value ${value})` : "";

        if (token === null) {
            error(10, [
                ofType,
                ofValue
            ]);
            return null;
        }

        if (type && value) {
            if (token?.type !== type || token?.value !== value) {
                const tkn = peek();

                error(7, [
                    tkn.lineIndex,
                    type,
                    value,
                    tkn.type,
                    tkn.value,
                    brightRed(tkn.lineText),
                    tkn.lineText.replace(/./g, "^"),
                ]);
                return null;
            }
        } else if (type) {
            if (token?.type !== type) {
                const tkn = peek();

                error(8, [
                    tkn.lineIndex,
                    type,
                    tkn.type,
                    brightRed(tkn.lineText),
                    tkn.lineText.replace(/./g, "^"),
                ]);
                return null;
            }
        } else if (value) {
            if (token?.value !== value) {
                const tkn = peek();

                error(9, [
                    tkn.lineIndex,
                    value,
                    tkn.value,
                    brightRed(tkn.lineText),
                    tkn.lineText.replace(/./g, "^"),
                ]);
                return null;
            }
        } else error(0);

        j++;

        return token;
    }

    function advance() {
        const tkn = peek();
        j++;

        return tkn;
    }

    function parseBlock() {
        const body = [];
        consume("Bracket", "{");

        while (peek()?.value !== "}") {
            const statement = parseStatement();
            if (statement) {
                body.push(statement);
            } else {
                error(11, [
                    peek(-1).lineIndex,
                    brightRed(peek(-1).lineText),
                    peek(-1).lineText.replace(/./g, "^")
                ]);
                break;
            }
        }

        consume("Bracket", "}");
        return body;
    }

    // A reusable function to parse an expression inside parentheses (a condition)
    function parseCondition() {
        consume("Bracket", "(");
        const expr = parseExpr();
        consume("Bracket", ")");
        return expr;
    }

    function parseIfStatement() {
        const elifs = [];
        let elseBody = null;

        // 'if' keyword has already been consumed
        const ifCondition = parseCondition();
        const ifBody = parseBlock();

        // Loop to handle any 'elif' clauses
        while (peek()?.value === "elif") {
            advance(); // Consume 'elif'
            const elifCondition = parseCondition();
            const elifBody = parseBlock();
            elifs.push({
                condition: elifCondition,
                body: elifBody,
            });
        }

        // Check for an 'else' clause
        if (peek()?.value === "else") {
            advance(); // Consume 'else'
            elseBody = parseBlock();
        }

        return {
            type: nodeTypes.IfStat,
            condition: ifCondition,
            body: ifBody,
            elifs: elifs,
            else: elseBody,
        };
    }

    function parsePrimaryExpr() {
        const token = peek();
        if (!token) return null;

        if (token.type === "Bracket" && token.value === "[") {
            return parseArrayLiteral();
        }

        if (token.type === "Operator" && unaryOps.includes(token.value)) {
            const op = advance().value;
            const operand = parseExpr(9);

            return {
                type: nodeTypes.UnaryExpr,
                operator: op,
                operand: operand,
            };
        }

        if (token.type === "Number") {
            const atkn = advance();

            return {
                type: nodeTypes.NumLiteral,
                value: Number(atkn.value),
                lineIndex: atkn.lineIndex,
            };
        }

        if (token.type == "String") {
            const atkn = advance();

            return {
                type: nodeTypes.StrLiteral,
                value: atkn.value,
                lineIndex: atkn.lineIndex,
            };
        }

        if (token.type === "Identifier") {
            const atkn = advance();

            const identifier = {
                type: nodeTypes.Identifier,
                name: atkn.value,
                lineIndex: atkn.lineIndex,
            };

            if (peek()?.value === "(") {
                return parseFunctionCall(identifier);
            }

            if (peek()?.value === "::") {
                advance();
                return parseModAccess(identifier);
            }

            return identifier;
        }

        if (token.type === "Bracket" && token.value === "(") {
            advance();
            const expr = parseExpr();
            consume("Bracket", ")");
            return expr;
        }

        error(12, [
            token.lineIndex,
            brightRed(token.lineText),
            token.lineText.replace(/./g, "^")
        ]);
        advance();
        return null;
    }

    function parseExpr(precedence = 0) {
        let left = parsePrimaryExpr();
        if (!left) return null;

        while (precedence <= opPrecedence[peek()?.value]) {
            const operatorToken = advance();
            const operatorValue = operatorToken.value;

            if (opPrecedence[operatorValue] === undefined) {
                j--;
                break;
            }

            const right = parseExpr(opPrecedence[operatorValue]);
            if (!right) return null;

            left = {
                type: nodeTypes.BinaryExpr,
                operator: operatorValue,
                left: left,
                right: right,
            };
        }

        return left;
    }

    // This function parses the arguments inside the parentheses of a function definition
    function parseFuncArgs() {
        const args = [];
        consume("Bracket", "(");

        if (peek()?.value !== ")") {
            do {
                const arg = consume("Identifier");
                if (arg) {
                    args.push(arg.value);
                }
            } while (peek()?.value === "," && advance());
        }

        consume("Bracket", ")");
        return args;
    }

    function parseWhileStat() {
        const condition = parseCondition();
        const body = parseBlock();

        return {
            type: nodeTypes.WhileStat,
            condition: condition,
            body: body,
        };
    }

    function parseFeachStat() {
        consume("Bracket", "(");
        const elementName = consume("Identifier").value;
        consume("Bracket", ")");

        consume("Bracket", "(");
        const array = parseExpr();
        consume("Bracket", ")");

        const body = parseBlock();

        return {
            type: nodeTypes.FeachStat,
            element: elementName,
            array: array,
            body: body,
        };
    }

    function parseReturnStatement() {
        let expr = null;

        if (peek()?.value !== ";") {
            expr = parseExpr();
        }

        consume("Semicolon");

        return {
            type: nodeTypes.ReturnStat,
            value: expr,
        };
    }

    function parseFunctionDeclaration() {
        const name = consume("Identifier");
        const args = parseFuncArgs();
        const body = parseBlock();

        return {
            type: nodeTypes.FunctionDeclaration,
            name: name.value,
            arguments: args,
            body: body,
        };
    }

    function parseCallArgs() {
        const args = [];
        consume("Bracket", "(");

        if (peek()?.value !== ")") {
            do {
                args.push(parseExpr());
            } while (peek()?.value === "," && advance());
        }

        consume("Bracket", ")");
        return args;
    }

    function parseFunctionCall(nameNode) {
        const args = parseCallArgs();
        return {
            type: nodeTypes.FunctionCall,
            name: nameNode,
            args: args,
        };
    }

    function parseModAccess(nameNode) {
        const member = parsePrimaryExpr();

        return {
            type: nodeTypes.ModAccess,
            mod: nameNode,
            member: member,
        };
    }

    function parseImport() {
        const modName = consume("Identifier")?.value;
        consume("Semicolon");

        return {
            type: nodeTypes.ImportStat,
            module: modName,
        };
    }

    function parseVariableAssign() {
        const varName = consume("Identifier");
        const operator = consume("Operator");
        const expr = parseExpr();
        consume("Semicolon");

        return {
            type: nodeTypes.VariableAssign,
            name: varName.value,
            operator: operator.value,
            value: expr,
        };
    }

    function parseArrayLiteral() {
        consume("Bracket", "["); // Consume the opening '['

        const elements = [];

        // Check if the array is not empty
        if (peek()?.value !== "]") {
            do {
                const expr = parseExpr();
                elements.push(expr);
            } while (peek()?.value === "," && advance());
        }

        consume("Bracket", "]"); // Consume the closing ']'

        return {
            type: nodeTypes.ArrayLiteral,
            elements: elements,
        };
    }

    function parseStatement() {
        const token = peek();
        if (!token) return null;

        if (token.type == "Keyword" && token.value == "if") {
            advance();
            return parseIfStatement();
        }

        if (token.type == "Keyword" && token.value == "while") {
            advance();
            return parseWhileStat();
        }

        if (token.type == "Keyword" && token.value == "for") {
            advance();
            return parseFeachStat();
        }

        if (
            token.type == "Keyword" &&
            (token.value === "elif" || token.value === "else")
        ) {
            error(13, [
                token.lineIndex,
                brightRed(token.lineText),
                token.lineText.replace(/./g, "^")
            ]);
            advance();
            return null;
        }

        if (token.type == "Keyword" && token.value == "fun") {
            advance();
            return parseFunctionDeclaration();
        }

        if (token.type == "Keyword" && token.value == "return") {
            advance();
            return parseReturnStatement();
        }

        if (token.type == "Keyword" && token.value == "import") {
            advance();
            return parseImport();
        }

        if (
            token.type == "Identifier" && peek(1)?.type === "Operator" &&
            variableAssignOps.includes(peek(1)?.value)
        ) {
            return parseVariableAssign();
        }

        const expr = parseExpr();
        if (expr) {
            consume("Semicolon");
            return {
                type: nodeTypes.ExprStat,
                expression: expr,
            };
        }

        error(11, [
            token.lineIndex,
            brightRed(token.lineText),
            token.lineText.replace(/./g, "^")
        ]);
        advance();
        return null;
    }

    const nodes = [];
    while (peek() !== null) {
        const statement = parseStatement();

        if (statement) {
            nodes.push(statement);
        }
    }

    return {
        type: nodeTypes.Program,
        body: nodes,
    };
}
