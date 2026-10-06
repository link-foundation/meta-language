// Compares the native GraphQL grammar (parity/grammars/native/graphql.lino) with its
// tree-sitter-graphql oracle on every case of the pinned upstream corpus
// (parity/grammars/sources/tree-sitter-graphql-*.corpus.json.gz), one case at a
// time: SAME, DIFF, REJECT or ORACLE-ERROR (the oracle recovers), with the
// time each took, and -AMBIGUOUS after a parse the executor found ambiguous. FILTER=text keeps the cases whose title holds the text;
// SHOW=DIFF,REJECT prints the source and the first differing rows.
//   node experiments/native-graphql-corpus.mjs
import { readFileSync } from 'node:fs';

import { compileGrammar } from '../src/grammar.js';
import { parseGrammarLinks } from '../src/grammar-links.js';
import { nativeOracleKinds } from '../scripts/build-language-catalog.mjs';
import { corpusCases, grammarSourceOf } from '../scripts/import-native-grammars.mjs';
import { nativeRows, oracleRecovers, oracleRows } from '../scripts/native-grammar-rows.mjs';

const grammar = 'graphql';
const language = 'GraphQL';
// LINO names another grammar file, as a variant of the scanner to try.
const text = readFileSync(process.env.LINO ?? new URL(`../../parity/grammars/native/${grammar}.lino`, import.meta.url), 'utf8');
const compiled = compileGrammar(parseGrammarLinks(text));
const options = { hidden: [], anonymous: ['unnamed_token'], extras: ['comma', 'comment'], oracleKinds: nativeOracleKinds(text) };
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
      // An ambiguous parse is marked, as the tests take none.
      kind = (JSON.stringify(oracle) === JSON.stringify(native) ? 'SAME' : 'DIFF') + (outcome.ambiguities?.length ? '-AMBIGUOUS' : '');
    }
  }
  totals[kind] = (totals[kind] ?? 0) + 1;
  console.log(`${kind} ${file} :: ${title} (${Math.round(performance.now() - started)} ms)${detail}`);
  if (show.has(kind)) console.log(`  source ${JSON.stringify(source)}`);
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
