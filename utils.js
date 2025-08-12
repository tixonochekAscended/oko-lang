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

import { red, yellow, reset } from "https://deno.land/std/fmt/colors.ts";

const errorCodes = {
    0: "An unknown error occured. This could be caused by many things, including a JS error which shouldn't generally occur.",
    1: "Not enough arguments have been provided to the oko-lang interpreter.",
    2: "The file you provided, \"~\", does not have the extension \".oko\", which is necessary for the file to be interpreted and ran.",
    3: "The file you provided, \"~\", does not exist, can't be found or the oko-lang interpreter doesn't have the necessary permissions to read it.",
    4: "At line ~ found an unexpected character: \"~\"\n\t~\n\t~",
    5: "At line ~ found an unparsable number literal: \"~\"\n\t~\n\t~",
    6: "At line ~ found an invalid operator: \"~\"\n\t~\n\t~",
    7: "At line ~ expected a token of type \"~\" with value \"~\", but got \"~\" with value \"~\"\n\t~\n\t~",
    8: "At line ~ expected a token of type \"~\", but got \"~\"\n\t~\n\t~",
    9: "At line ~ expected a token of value \"~\", but got \"~\"\n\t~\n\t~",
    10: "Didn't expect EOF (end of file) just yet. Instead, expected a token~~.",
    11: "At line ~ there is a syntax error inside of the statement.\n\t~\n\t~",
    12: "At line ~ there is a syntax error inside of the expression.\n\t~\n\t~",
    13: "At line ~, either \"elif\" or \"else\" was used without it being preceeded by an if statement.\n\t~\n\t~",
    14: "At line ~, an identifier that wasn't pointing to anything was attempted to be used.",
    15: "Attempted to mutate a variable's value via \"~\", but a type mismatch occured.",
    16: "Attempted to mutate a value of a variable that doesn't exist via the \"~\" operator.",
    17: "Attempted to use the \"~\" operator with types of values that the it doesn't support.",
    18: "Attempted to use the \"return\" keyword outside of a function.",
    19: "Attempted to call a function \"~\", which doesn't actually exist.",
    20: "Attempted to call a function \"~\" with the wrong number of arguments.",
    21: "Attempted to pass a function as a first-class citizen, which is not possible.",
    22: "Invalid usage of the module access \"::\" operator, which can only be used to call functions located in different modules.",
    23: "Attempted to access a module that doesn't exist or hasn't been imported yet - \"~\".",
    24: "Attempted to import a module which doesn't exist - \"~\".",
    25: "Attempted to use a function \"~\" with the arguments of types it doesn't support.",
    26: "~",
    27: "Maximum call stack size exceeded. This means that the recursion depth limit has been exceeded."
};

const warnCodes = {
    0: "The argument \"~\" is not a valid flag."
};

const placeholderMaps = {};
for (const [code, message] of Object.entries(errorCodes)) {
    placeholderMaps[`err-${code}`] = [];
    let match;
    const regex = /~/g;
    while ((match = regex.exec(message)) !== null) {
        placeholderMaps[`err-${code}`].push(match.index);
    }
}
for (const [code, message] of Object.entries(warnCodes)) {
    placeholderMaps[`warn-${code}`] = [];
    let match;
    const regex = /~/g;
    while ((match = regex.exec(message)) !== null) {
        placeholderMaps[`warn-${code}`].push(match.index);
    }
}

export function error(code, optionalArgs = []) {
    if (errorCodes[code]) {
        let errorText = errorCodes[code];
        let parts = errorText.split('~');

        if (parts.length - 1 === optionalArgs.length) {
            errorText = parts.reduce((acc, part, i) => {
                if (i === 0) return part;
                return acc + optionalArgs[i - 1] + part;
            }, '');
        } else {
            console.warn(yellow("[warn-?] ") + reset(`Error code ${code} expects ${parts.length - 1} arguments, but got ${optionalArgs.length}.`));
        }

        console.log(red(`[err-${code}] `) + errorText);
        Deno.exit(1);
    }

    console.log(red("[err-?] ") + "Unknown error code.");
    Deno.exit(1);
}

export function warn(code, optionalArgs = []) {
    if (warnCodes[code]) {
        let warnText = warnCodes[code];
        let parts = warnText.split('~');

        if (parts.length - 1 === optionalArgs.length) {
            warnText = parts.reduce((acc, part, i) => {
                if (i === 0) return part;
                return acc + optionalArgs[i - 1] + part;
            }, '');
        } else {
            console.warn(yellow("[warn-?] ") + reset(`Warning code ${code} expects ${parts.length - 1} arguments, but got ${optionalArgs.length}.`));
        }

        console.log(yellow(`[warn-${code}] `) + reset(warnText));
        return;
    }

    console.log(red("[err-?] ") + "Unknown warn code.");
    Deno.exit(1);
}