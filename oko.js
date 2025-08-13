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

import * as utils from "./utils.js";
import * as lexer from "./lexer.js";
import * as parser from "./parser.js";
import * as executor from "./executor.js";

const cli = {
    "license": `oko-lang: Language of vision
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
        along with this program. If not, see <https://www.gnu.org/licenses/>.`
}

function run() {
    if (Deno.args.length < 1) { utils.error(1) }
    if (cli[Deno.args[0]]) {
        console.log(cli[Deno.args[0]]);
        Deno.exit(0);
    }
    if (!Deno.args[0].endsWith(".oko")) { utils.error(2, [Deno.args[0]]) }

    const flags = {}

    Deno.args.forEach(el => {
        if (el.startsWith("--")) { 
            if ( el.slice(2) in flags ) { flags[el.slice(2)] = true }
            else { utils.warn(0, [ el ]) }
        }
    });

    let script;

    try {
        script = Deno.readTextFileSync(Deno.args[0]);
    } catch (error) {
        if (error instanceof Deno.errors.NotFound) { utils.error(3, [Deno.args[0]]) }
        else { utils.error(0) }
    }

    runScript(script);
}

function runScript(script) {
    const tokens = lexer.tokenize(script);
    const ast = parser.buildTree(tokens);
    try {
        executor.execute(ast);
    } catch (error) {
        if (error instanceof RangeError) { utils.error(27) }
        else { 
            utils.error(28, [ error ])
        }
    }
}

run();
