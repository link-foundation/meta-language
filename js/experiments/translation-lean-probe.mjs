import { parseLean } from '../src/translation/lean.js';
import { checkProgram } from '../src/translation/check.js';
import { readFileSync } from 'node:fs';
const source = readFileSync(process.argv[2], 'utf8');
const surface = parseLean(source);
const program = checkProgram(surface);
const replacer = (key, value) => (key === 'declarations' ? undefined : typeof value === 'bigint' ? String(value) : value);
console.log(JSON.stringify(program, replacer, 1).slice(0, Number(process.argv[3] ?? 4000)));
for (const entry of program.declarations.values()) if (entry.k === 'fn') console.log(entry.fullName, 'recursive', entry.recursive, 'decreasing', entry.decreasing);
