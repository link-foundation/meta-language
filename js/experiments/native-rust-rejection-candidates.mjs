// Checks candidate rejections and added matches for the native Rust grammar:
// a rejection must be one the tree-sitter-rust oracle recovers from, that the
// native grammar rejects and repairs under errorRecovery; a match one the
// oracle accepts with the native grammar's rows. Also checks the inventory
// source and recovery source of Rust.
import { readFileSync } from 'node:fs';
import { compileGrammar, parseGrammarLinks } from '../src/index.js';
import { nativeOracleKinds } from '../scripts/build-language-catalog.mjs';
import { nativeRows, oracleRecovers, oracleRows } from '../scripts/native-grammar-rows.mjs';

const text = readFileSync(new URL('../../parity/grammars/native/rust.lino', import.meta.url), 'utf8');
const parser = compileGrammar(parseGrammarLinks(text));
const options = { hidden: [], anonymous: ['unnamed_token'], extras: ['line_comment', 'block_comment'], oracleKinds: nativeOracleKinds(text) };
const inventory = JSON.parse(readFileSync(new URL('../../parity/language-grammar-inventory.json', import.meta.url), 'utf8'));
const rust = inventory.languages.find(({ name }) => name === 'Rust');
const candidates = JSON.parse(process.env.REJECTIONS ?? '[]');
const matches = [...JSON.parse(process.env.MATCHES ?? '[]'), rust.source];
for (const source of candidates) {
  const started = performance.now();
  const oracle = oracleRecovers(source, 'Rust');
  const plain = parser.parseTree(source);
  const repaired = parser.parseTree(source, { errorRecovery: true });
  console.log(`${oracle && !plain.ok && repaired.rejection?.reason === 'recovered' ? 'OK  ' : 'BAD '} reject ${JSON.stringify(source)} oracleRecovers=${oracle} nativeOk=${plain.ok} repaired=${repaired.rejection?.reason} (${Math.round(performance.now() - started)} ms)`);
}
for (const source of matches) {
  const oracle = oracleRecovers(source, 'Rust');
  const outcome = parser.parseTree(source);
  const same = outcome.ok && JSON.stringify(nativeRows(outcome.tree, source, options)) === JSON.stringify(oracleRows(source, 'Rust'));
  console.log(`${!oracle && same ? 'OK  ' : 'BAD '} match ${JSON.stringify(source)} oracleRecovers=${oracle} nativeOk=${outcome.ok}`);
}
console.log(`recovery source: oracleRecovers=${oracleRecovers(rust.recoverySource, 'Rust')} native=${parser.parseTree(rust.recoverySource, { errorRecovery: true }).rejection?.reason}`);
