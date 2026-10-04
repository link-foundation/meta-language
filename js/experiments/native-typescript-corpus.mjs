// Compares the native TypeScript or TSX grammar (parity/grammars/native/typescript.lino,
// tsx.lino; GRAMMAR=tsx picks TSX) with its tree-sitter-typescript oracle on
// every case of the pinned upstream corpus the grammar runs
// (parity/grammars/sources/tree-sitter-typescript-*.corpus.json.gz), one case
// at a time: SAME, DIFF, REJECT or ORACLE-ERROR (the oracle recovers), with the
// time each took. FILTER=text keeps the cases whose title holds the text;
// SHOW=DIFF,REJECT prints the first differing rows.
//   node experiments/native-typescript-corpus.mjs
import { readFileSync } from 'node:fs';

import { compileGrammar } from '../src/grammar.js';
import { parseGrammarLinks } from '../src/grammar-links.js';
import { nativeOracleKinds } from '../scripts/build-language-catalog.mjs';
import { corpusCases, grammarSourceOf } from '../scripts/import-native-grammars.mjs';
import { nativeRows, oracleRecovers, oracleRows } from '../scripts/native-grammar-rows.mjs';

const grammar = process.env.GRAMMAR ?? 'typescript';
const language = grammar === 'tsx' ? 'TSX' : 'TypeScript';
const text = readFileSync(new URL(`../../parity/grammars/native/${grammar}.lino`, import.meta.url), 'utf8');
const compiled = compileGrammar(parseGrammarLinks(text));
const options = { hidden: [], anonymous: ['unnamed_token'], extras: ['comment', 'html_comment'], oracleKinds: nativeOracleKinds(text) };
const show = new Set(process.env.SHOW ? process.env.SHOW.split(',') : []);
const totals = {};
for (const { file, title, source } of corpusCases(grammarSourceOf(`native-${grammar}`))) {
  if (process.env.FILTER && !title.includes(process.env.FILTER)) continue;
  const started = performance.now();
  let kind;
  let detail = '';
  let oracle = [];
  let native = [];
  if (oracleRecovers(source, language)) kind = 'ORACLE-ERROR';
  else {
    oracle = oracleRows(source, language);
    const outcome = compiled.parseTree(source);
    if (!outcome.ok) {
      kind = 'REJECT';
      detail = ` ${JSON.stringify(outcome.rejection).slice(0, 200)}`;
    } else {
      native = nativeRows(outcome.tree, source, options);
      kind = JSON.stringify(oracle) === JSON.stringify(native) ? 'SAME' : 'DIFF';
    }
  }
  totals[kind] = (totals[kind] ?? 0) + 1;
  console.log(`${kind} ${file} :: ${title} (${Math.round(performance.now() - started)} ms)${detail}`);
  if (show.has(kind) && kind === 'DIFF') {
    let shown = 0;
    for (let index = 0; index < Math.max(oracle.length, native.length) && shown < 6; index += 1) {
      const [o, n] = [JSON.stringify(oracle[index]), JSON.stringify(native[index])];
      if (o !== n) {
        console.log(`  ! ${o}   ${n}`);
        shown += 1;
      }
    }
  }
}
console.log(JSON.stringify(totals));
