// Compares a native .lino grammar with the language's tree-sitter oracle on
// the cases of upstream tree-sitter corpus files (test/corpus/*.txt), one
// file and one case at a time. Each case is SAME (equal rows), DIFF (unequal
// rows), REJECT (the native grammar rejects what the oracle accepts) or
// ORACLE-ERROR (the oracle recovers, so the case is a rejection or a
// divergence candidate).
//   node experiments/native-corpus-compare.mjs LANGUAGE GRAMMAR.lino CORPUS.txt...
// Options come from the environment: EXTRAS=comment HIDDEN= ANONYMOUS=,
// SHOW=DIFF,REJECT prints the first differing rows of those classes.
import { readFileSync } from 'node:fs';

import { compileGrammar } from '../src/grammar.js';
import { parseGrammarLinks } from '../src/grammar-links.js';
import { nativeOracleKinds } from '../scripts/build-language-catalog.mjs';
import { nativeRows, oracleRecovers, oracleRows } from '../scripts/native-grammar-rows.mjs';

const [language, grammarPath, ...corpora] = process.argv.slice(2);
const list = (name) => (process.env[name] ? process.env[name].split(',') : []);
const show = new Set(list('SHOW'));
const text = readFileSync(grammarPath, 'utf8');
const compiled = compileGrammar(parseGrammarLinks(text));
const options = { hidden: list('HIDDEN'), anonymous: list('ANONYMOUS'), extras: list('EXTRAS'), oracleKinds: nativeOracleKinds(text) };

// The cases of a tree-sitter corpus file: a `===` header line, its title, a
// closing `===` line, the source, a `---` line and the expected tree.
export function corpusCases(corpus) {
  const lines = corpus.split('\n');
  const cases = [];
  for (let index = 0; index < lines.length; index += 1) {
    if (!/^={3,}/.test(lines[index]) || !/^={3,}/.test(lines[index + 2] ?? '')) continue;
    const title = lines[index + 1];
    const start = index + 3;
    let end = start;
    while (end < lines.length && !/^-{3,}\s*$/.test(lines[end])) end += 1;
    cases.push({ title, source: lines.slice(start, end).join('\n').replace(/\n+$/, '\n') });
    index = end;
  }
  return cases;
}

const totals = {};
for (const corpus of corpora) {
  for (const { title, source } of corpusCases(readFileSync(corpus, 'utf8'))) {
    let kind;
    let detail = '';
    const started = performance.now();
    if (oracleRecovers(source, language)) {
      kind = 'ORACLE-ERROR';
    } else {
      const oracle = oracleRows(source, language);
      let outcome;
      try {
        outcome = compiled.parseTree(source);
      } catch (error) {
        outcome = { ok: false, thrown: error.message };
      }
      if (!outcome.ok) {
        kind = 'REJECT';
        detail = outcome.thrown ?? JSON.stringify(outcome.rejection ?? {}).slice(0, 200);
      } else {
        const native = nativeRows(outcome.tree, source, options);
        const at = oracle.findIndex((row, index) => JSON.stringify(row) !== JSON.stringify(native[index]));
        kind = at === -1 && oracle.length === native.length ? 'SAME' : 'DIFF';
        if (kind === 'DIFF') {
          const first = at === -1 ? Math.min(oracle.length, native.length) : at;
          for (let index = Math.max(0, first - 1); index < first + 3; index += 1) {
            detail += `\n    ${index} oracle ${JSON.stringify(oracle[index])}\n    ${index} native ${JSON.stringify(native[index])}`;
          }
        }
      }
    }
    totals[kind] = (totals[kind] ?? 0) + 1;
    const ms = Math.round(performance.now() - started);
    console.log(`${kind} ${corpus.split('/').pop()}: ${title} (${ms} ms)`);
    if (show.has(kind)) console.log(`  ${JSON.stringify(source)}${detail ? ` ${detail}` : ''}`);
  }
}
console.log(JSON.stringify(totals));
