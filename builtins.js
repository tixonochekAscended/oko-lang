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

import { error } from "./utils.js"
import { valTypes } from "./executor.js"

function stringify(value) {
    if (value.type === valTypes.Nil) {
        return "Nil";
    } else if (value.type === valTypes.Array) {
        const elementsFormatted = value.value.map(x => stringify(x)).join(", ");
        return `[ ${elementsFormatted} ]`;
    } else {
        return value.value;
    }
}

function numberize(value) {
    if (value.type == valTypes.Nil) {
        return {
            type: valTypes.Number,
            value: 0
        };
    } else if (value.type == valTypes.String) {
        const posRes = Number(value.value);
        return Number.isFinite(posRes) ? {
            type: valTypes.Number,
            value: posRes
        } : {
            type: valTypes.Nil,
            value: null
        };
    } else if (value.type == valTypes.Number) { 
        return value;
    } else { error(0); }
}

export const modules = {
    prog: {
        exit: {
            requiredArgs: 1,
            callback: (errCode) => {
                if (errCode.type !== valTypes.Number) {
                    error(25, [ "exit" ]);
                }

                Deno.exit(errCode.value);
            }
        },

        throw: {
            requiredArgs: 1,
            callback: (msg) => {
                if (msg.type !== valTypes.String) {
                    error(25, [ "throw" ]);
                }

                error(26, [ msg.value ]);
            }
        }
    },

    time: {
        now: {
            requiredArgs: 0,
            callback: () => {
                return {
                    type: valTypes.Number,
                    value: Date.now()
                }
            }
        },

        sleep: {
            requiredArgs: 1,
            callback: (ms) => {
                if (ms.type !== valTypes.Number) {
                    error(25, ["sleep"]);
                }

                const end = Date.now() + ms.value;
                while (Date.now() < end) { /* ... */ }
        
                return;
            }
        }
    },

    tu: {
        getNil: {
            requiredArgs: 0,
            callback: () => {}
        },

        toNumber: {
            requiredArgs: 1,
            callback: (value) => {
                if (value.type === valTypes.Array) {
                    error(25, [ "toNumber" ]);
                }

                return numberize(value);
            }
        },

        toString: {
            requiredArgs: 1,
            callback: (value) => {
                return {
                    type: valTypes.String,
                    value: stringify(value)
                }
            }
        },

        typeOf: {
            requiredArgs: 1,
            callback: (value) => {
                return {
                    type: valTypes.String,
                    value: value.type
                }
            }
        }
    },

    io: {
        input: {
            requiredArgs: 0,
            callback: () => {
                const input = prompt("");

                return {
                    type: input ? valTypes.String : valTypes.Nil,
                    value: input ? input : null
                }
            }
        },

        //print: {
        //    requiredArgs: Infinity,
        //    callback: (printables) => {
        //        printables = printables.map(x => stringify(x)).join(" ");
        //
        //        Deno.stdout.write(new TextEncoder().encode(printables));
        //    }
        //},

        println: {
            requiredArgs: Infinity,
            callback: (printables) => {
                printables = printables.map(x => stringify(x)).join(" ");

                console.log(printables);
            }
        },

        readTextFile: {
            requiredArgs: 1,
            callback: (path) => {
                if (path.type !== valTypes.String) {
                    error(25, [ "readTextFile" ]);
                }

                try {
                    return {
                        type: valTypes.String,
                        value: Deno.readTextFileSync(path.value)
                    }
                } catch (_error) { return; }
            }
        },

        writeTextFile: {
            requiredArgs: 2,
            callback: (path, content) => {
                if (path.type !== valTypes.String || content.type !== valTypes.String) {
                    error(25, [ "writeTextFile" ]);
                }

                try {
                    Deno.writeTextFileSync(path.value, content.value);
                    
                    return {
                        type: valTypes.Number,
                        value: 1
                    }
                } catch (_error) { return; }
            }
        }
    },

    math: {
        sqrt: {
            requiredArgs: 1,
            callback: (num) => {
                if (num.type !== valTypes.Number) {
                    error(25, [ "sqrt" ]);
                }

                const result = Math.sqrt(num.value);

                return {
                    type: Number.isFinite(result) ? valTypes.Number : valTypes.Nil,
                    value: Number.isFinite(result) ? result : null
                }
            }
        },

        abs: {
            requiredArgs: 1,
            callback: (num) => {
                if (num.type !== valTypes.Number) {
                    error(25, [ "abs" ]);
                }

                return {
                    type: valTypes.Number,
                    value: Math.abs(num.value)
                }
            }
        },

        round: {
            requiredArgs: 1,
            callback: (num) => {
                if (num.type !== valTypes.Number) {
                    error(25, [ "round" ]);
                }

                const result = Math.floor(num.value);

                return {
                    type: Number.isFinite(result) ? valTypes.Number : valTypes.Nil,
                    value: Number.isFinite(result) ? result : null
                }
            }
        },

        ceil: {
            requiredArgs: 1,
            callback: (num) => {
                if (num.type !== valTypes.Number) {
                    error(25, [ "ceil" ]);
                }

                const result = Math.ceil(num.value);

                return {
                    type: Number.isFinite(result) ? valTypes.Number : valTypes.Nil,
                    value: Number.isFinite(result) ? result : null
                }
            }
        },

        floor: {
            requiredArgs: 1,
            callback: (num) => {
                if (num.type !== valTypes.Number) {
                    error(25, [ "floor" ]);
                }

                const result = Math.floor(num.value);

                return {
                    type: Number.isFinite(result) ? valTypes.Number : valTypes.Nil,
                    value: Number.isFinite(result) ? result : null
                }
            }
        },

        sin: {
            requiredArgs: 1,
            callback: (num) => {
                if (num.type !== valTypes.Number) {
                    error(25, [ "sin" ]);
                }

                const result = Math.sin(num.value);

                return {
                    type: Number.isFinite(result) ? valTypes.Number : valTypes.Nil,
                    value: Number.isFinite(result) ? result : null
                }
            }
        },

        cos: {
            requiredArgs: 1,
            callback: (num) => {
                if (num.type !== valTypes.Number) {
                    error(25, [ "cos" ]);
                }

                const result = Math.cos(num.value);

                return {
                    type: Number.isFinite(result) ? valTypes.Number : valTypes.Nil,
                    value: Number.isFinite(result) ? result : null
                }
            }
        },

        tan: {
            requiredArgs: 1,
            callback: (num) => {
                if (num.type !== valTypes.Number) {
                    error(25, [ "tan" ]);
                }

                const result = Math.tan(num.value);

                return {
                    type: Number.isFinite(result) ? valTypes.Number : valTypes.Nil,
                    value: Number.isFinite(result) ? result : null
                }
            }
        },

        log: {
            requiredArgs: 2,
            callback: (num, base) => {
                if (num.type !== valTypes.Number || base.type !== valTypes.Number) {
                    error(25, [ "log" ]);
                }

                const result = Math.log(num.value) / Math.log(base.value);

                return {
                    type: Number.isFinite(result) ? valTypes.Number : valTypes.Nil,
                    value: Number.isFinite(result) ? result : null
                }
            }
        },

        random: {
            requiredArgs: 0,
            callback: () => {
                return {
                    type: valTypes.Number,
                    value: Math.random()
                }
            }
        }
    },

    stru: {
        len: {
            requiredArgs: 1,
            callback: (str) => {
                if (str.type !== valTypes.String) {
                    error(25, [ "len" ]);
                }

                return {
                    type: valTypes.Number,
                    value: str.value.length
                }
            }
        },

        at: {
            requiredArgs: 2,
            callback: (str, index) => {
                if (str.type !== valTypes.String || index.type !== valTypes.Number) {
                    error(25, [ "at" ]);
                }

                return {
                    type: valTypes.String,
                    value: str.value.at(index.value)
                }
            }
        },

        sub: {
            requiredArgs: 3,
            callback: (str, start, end) => {
                if (str.type !== valTypes.String || start.type !== valTypes.Number || end.type !== valTypes.Number) {
                    error(25, [ "sub" ]);
                }

                return {
                    type: valTypes.String,
                    value: str.value.substring(start.value, end.value)
                }
            }
        },

        split: {
            requiredArgs: 2,
            callback: (str, delim) => {
                if (str.type !== valTypes.String || delim.type !== valTypes.String) {
                    error(25, [ "split" ]);
                }

                return {
                    type: valTypes.Array,
                    value: str.value.split(delim.value).map(x => ({ type: valTypes.String, value: x }))
                }
            }
        },

        lower: {
            requiredArgs: 1,
            callback: (str) => {
                if (str.type !== valTypes.String) {
                    error(25, [ "lower" ]);
                }

                return {
                    type: valTypes.String,
                    value: str.value.toLowerCase()
                }
            }
        },

        upper: {
            requiredArgs: 1,
            callback: (str) => {
                if (str.type !== valTypes.String) {
                    error(25, [ "upper" ]);
                }

                return {
                    type: valTypes.String,
                    value: str.value.toUpperCase()
                }
            }
        },

        trim: {
            requiredArgs: 1,
            callback: (str) => {
                if (str.type !== valTypes.String) {
                    error(25, [ "trim" ]);
                }

                return {
                    type: valTypes.String,
                    value: str.value.trim()
                }
            }
        },

        replace: {
            requiredArgs: 3,
            callback: (str, find, replace) => {
                if (str.type !== valTypes.String || find.type !== valTypes.String || replace.type !== valTypes.String) {
                    error(25, [ "replace" ]);
                }

                return {
                    type: valTypes.String,
                    value: str.value.replaceAll(find.value, replace.value)
                }
            }
        }
    },

    arru: {
        len: {
            requiredArgs: 1,
            callback: (arr) => {
                if (arr.type !== valTypes.Array) {
                    error(25, [ "len" ]);
                }

                return {
                    type: valTypes.Number,
                    value: arr.value.length
                }
            }
        },

        at: {
            requiredArgs: 2,
            callback: (arr, index) => {
                if (arr.type !== valTypes.Array || index.type !== valTypes.Number) {
                    error(25, [ "at" ]);
                }

                return {
                    type: arr.value.at(index.value).type,
                    value: arr.value.at(index.value).value
                }
            }
        },

        push: {
            requiredArgs: 2,
            callback: (arr, val) => {
                if (arr.type !== valTypes.Array) {
                    error(25, [ "push" ]);
                }

                arr.value.push(val);
            }
        },

        remove: {
            requiredArgs: 2,
            callback: (arr, index) => {
                if (arr.type !== valTypes.Array || index.type !== valTypes.Number) {
                    error(25, [ "remove" ]);
                }

                const removedElement = arr.value.splice(index, 1);

                if (removedElement.length !== 0) {
                    return {
                        type: removedElement[0].type,
                        value: removedElement[0].value
                    }
                }
            }
        },

        join: {
            requiredArgs: 2,
            callback: (arr, separator) => {
                if (arr.type !== valTypes.Array || separator.type !== valTypes.String) {
                    error(25, [ "join" ]);
                }

                return {
                    type: valTypes.String,
                    value: arr.value.map(x => {
                        return (x.type !== valTypes.Nil) ? x.value : "Nil"
                    }).join(separator.value)
                }
            }
        }
    }
}