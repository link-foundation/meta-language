// Prints the oracle's and the native grammar's rows of one Solidity text side
// by side, from START to END offsets: SOURCE=text node experiments/native-solidity-rows.mjs
import { readFileSync } from 'node:fs';

import { compileGrammar } from '../src/grammar.js';
import { parseGrammarLinks } from '../src/grammar-links.js';
import { nativeOracleKinds } from '../scripts/build-language-catalog.mjs';
import { nativeRows, oracleRows } from '../scripts/native-grammar-rows.mjs';

const text = readFileSync(new URL('../../parity/grammars/native/solidity.lino', import.meta.url), 'utf8');
const compiled = compileGrammar(parseGrammarLinks(text));
const options = { hidden: [], anonymous: ['unnamed_token'], extras: ['comment'], oracleKinds: nativeOracleKinds(text) };
const source = process.env.SOURCE;
const oracle = oracleRows(source, 'Solidity');
const outcome = compiled.parseTree(source);
const native = outcome.ok ? nativeRows(outcome.tree, source, options) : [];
if (!outcome.ok) console.log(JSON.stringify(outcome.rejection).slice(0, 400));
const [start, end] = [Number(process.env.START ?? 0), Number(process.env.END ?? Infinity)];
const within = (row) => row[4] >= start && row[5] <= end;
console.log('oracle'); oracle.filter(within).forEach((row) => console.log(`  ${JSON.stringify(row)}`));
console.log('native'); native.filter(within).forEach((row) => console.log(`  ${JSON.stringify(row)}`));
