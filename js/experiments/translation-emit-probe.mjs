// Translates a corpus project with the portable core and prints the target text.
import { readFileSync } from 'node:fs';
import { parseLean } from '../src/translation/lean.js';
import { parseRocq } from '../src/translation/rocq.js';
import { parseRust } from '../src/translation/rust.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitRocq } from '../src/translation/emit-rocq.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { emitLean } from '../src/translation/emit-lean.js';
import { emitRust } from '../src/translation/emit-rust.js';
const emitters = { rocq: emitRocq, js: emitJavaScript, lean: emitLean, rust: emitRust };
const frontends = { lean: parseLean, v: parseRocq, rs: parseRust, mjs: parseJavaScript, js: parseJavaScript };
const frontend = frontends[process.argv[2].split('.').pop()];
const program = checkProgram(frontend(readFileSync(process.argv[2], 'utf8')));
const result = emitters[process.argv[3] ?? 'rocq'](program);
process.stdout.write(result.text);
console.error(JSON.stringify({ assumptions: result.assumptions, encodings: result.encodings.map((e) => e.id), theorems: result.theorems }, null, 1));
