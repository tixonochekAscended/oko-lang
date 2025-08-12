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

import { error } from "./utils.js";
import { brightRed } from "https://deno.land/std/fmt/colors.ts";

const numerical = "-.0123456789";
const identifiable =
    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_";
const operable = "=><+-/*%^!:&|";
const bracketable = "()[]{}";

const keywords = [
    "return",
    "funct",
    "while",
    "if",
    "else",
    "import",
    "feach",
];

const operators = [
    ":=",
    "=",
    "+=",
    "-=",
    "*=",
    "/=",
    "+",
    "-",
    "*",
    "/",
    "%",
    "^",
    "!",
    ">",
    "<",
    ">=",
    "<=",
    "&&",
    "||",
    "==",
    "::",
];

const escapeSequences = {
    "\\n": "\n",
    "\\e": "\x1B",
    "\\t": "\t",
    '\\"': '"'
};

function doEscapeSequences(str) {
    for (const key in escapeSequences) {
        str = str.replaceAll(key, escapeSequences[key]);
    }
    return str;
}

const currentlyLexingToTokenType = {
    "ident": "Identifier",
    "number": "Number",
    "op": "Operator",
    "string": "String",
};

export function tokenize(script) {
    const tokens = [];

    let j = 0;
    let currentlyLexing = "nothing";
    let buffer = "";
    let commentSkip = false;

    let lineIndex = 1;
    let lineText = "";

    function doCounters(char) {
        j++;
        if (char !== "\n") {
            lineText += char;
        }
    }

    function pushToken(type, value) {
        if (type === "Operator") {
            if (!operators.includes(buffer)) {
                error(6, [
                    lineIndex,
                    buffer,
                    brightRed(lineText),
                    lineText.replace(/./g, "^")
                ]);
            }
        } else if (type === "String") {
            value = doEscapeSequences(value);
        } else if (type === "Number") {
            if (Number.isNaN(Number(value))) {
                error(5, [
                    lineIndex,
                    value,
                    brightRed(lineText),
                    lineText.replace(/./g, "^")
                ]);
            }
        } else if (type === "Identifier") {
            if (keywords.includes(value)) {
                type = "Keyword";
            }
        }

        tokens.push({
            type: type,
            value: value,
            lineIndex: lineIndex,
            lineText: lineText,
        });
    }
    while (j < script.length) {
        const char = script[j];
        const prevChar = script[j - 1];
        const nextChar = script[j + 1];

        if (currentlyLexing === "nothing" && char === '"') {
            currentlyLexing = "string";
            buffer = "";
            doCounters(char);
            continue;
        }

        if (currentlyLexing === "string" && char === '"') {
            if (prevChar === "\\") {
                buffer += char;
                doCounters(char);
                continue;
            }

            if (!commentSkip) {
                pushToken("String", buffer);
            }
            currentlyLexing = "nothing";
            buffer = "";
            doCounters(char);
            continue;
        }

        if (currentlyLexing === "nothing" && char === ";") {
            doCounters(char);
            if (!commentSkip) {
                pushToken("Semicolon", char);
            }
            continue;
        }

        if (currentlyLexing === "nothing" && bracketable.includes(char)) {
            doCounters(char);
            if (!commentSkip) {
                pushToken("Bracket", char);
            }
            continue;
        }

        if (currentlyLexing === "nothing" && char === ",") {
            doCounters(char);
            if (!commentSkip) {
                pushToken("Comma", char);
            }
            continue;
        }

        if (currentlyLexing === "nothing" && char === "\n") {
            if (commentSkip) {
                commentSkip = false;
            }

            lineIndex += 1;
            lineText = "";

            if (tokens?.at(-1)?.type !== "Newline") {
                doCounters(char);
                continue;
            }
            doCounters(char);
            continue;
        }

        if (currentlyLexing === "nothing" && numerical.includes(char)) {
            if (char == "-") {
                if (numerical.includes(nextChar)) {
                    currentlyLexing = "number";
                    buffer = "";
                    continue;
                }
            } else {
                currentlyLexing = "number";
                buffer = "";
                continue;
            }
        }

        if (currentlyLexing === "number" && !numerical.includes(char)) {
            if (!commentSkip) {
                pushToken("Number", buffer);
            }
            currentlyLexing = "nothing";
            buffer = "";
            continue;
        }

        if (currentlyLexing === "nothing" && operable.includes(char)) {
            currentlyLexing = "op";
            buffer = "";
            continue;
        }

        if (currentlyLexing === "op" && !operable.includes(char)) {
            if (buffer === "//") {
                commentSkip = true;
                currentlyLexing = "nothing";
                buffer = "";
                doCounters(char);
                continue;
            }

            if (!commentSkip) {
                pushToken("Operator", buffer);
            }
            currentlyLexing = "nothing";
            buffer = "";
            continue;
        }

        if (currentlyLexing === "nothing" && identifiable.includes(char)) {
            currentlyLexing = "ident";
            buffer = "";
            continue;
        }

        if (currentlyLexing === "ident" && !identifiable.includes(char)) {
            if (!commentSkip) {
                pushToken("Identifier", buffer);
            }
            currentlyLexing = "nothing";
            buffer = "";
            continue;
        }

        if (currentlyLexing !== "nothing") {
            buffer += char;
            doCounters(char);
            continue;
        }

        if (char !== " " && !commentSkip) {
            doCounters(char);

            error(4, [
                lineIndex,
                char,
                brightRed(lineText),
                lineText.replace(/./g, "^")
            ]);
        }

        doCounters(char);
    }

    if (buffer.length !== 0) {
        if (!currentlyLexingToTokenType[currentlyLexing]) error(0);

        pushToken(currentlyLexingToTokenType[currentlyLexing], buffer);
    }

    return tokens;
}
