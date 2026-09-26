// Translates a corpus project with the portable core and prints the target text.
import { readFileSync } from 'node:fs';
import { parseLean } from '../src/translation/lean.js';
import { checkProgram } from '../src/translation/check.js';
import { emitRocq } from '../src/translation/emit-rocq.js';
const emitters = { rocq: emitRocq };
const program = checkProgram(parseLean(readFileSync(process.argv[2], 'utf8')));
const result = emitters[process.argv[3] ?? 'rocq'](program);
process.stdout.write(result.text);
console.error(JSON.stringify({ assumptions: result.assumptions, encodings: result.encodings.map((e) => e.id), theorems: result.theorems }, null, 1));
