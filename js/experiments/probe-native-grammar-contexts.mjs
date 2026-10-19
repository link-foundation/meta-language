// Own minimized sources only; CI executes the separately pinned upstream corpus.
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { compileGrammar, parseGrammarLinks } from '../src/index.js';
import { nativeOracleKinds } from '../scripts/build-language-catalog.mjs';
import { NATIVE_BATCH_FIXTURES } from '../scripts/native-grammar-batch-fixtures.mjs';
import { nativeRows, oracleRows, oracleRecovers, nativeCorpusFailure } from '../scripts/native-grammar-rows.mjs';

const inventory = JSON.parse(readFileSync(new URL('../../parity/language-grammar-inventory.json', import.meta.url), 'utf8'));
const selected = process.argv[2]?.split(',') ?? ['groovy'];
const failures = [];
for (const fixtures of NATIVE_BATCH_FIXTURES.filter(({ id }) => selected.includes(id))) {
  const listing = readFileSync(new URL(`../../parity/grammars/native/${fixtures.id}.lino`, import.meta.url), 'utf8');
  const parser = compileGrammar(parseGrammarLinks(listing));
  const metadata = { ...inventory.nativeGrammars[`native-${fixtures.id}`], oracleKinds: nativeOracleKinds(listing) };
  for (const [sources, accepted] of [[fixtures.matches, true], [fixtures.rejections, false]]) {
    for (const source of sources) {
      try {
        assert.equal(oracleRecovers(source, fixtures.language), !accepted, 'oracle acceptance');
        const outcome = parser.parseTree(source);
        assert.equal(outcome.ok, accepted, 'native acceptance');
        if (accepted) {
          assert.deepEqual(outcome.ambiguities, []);
          assert.deepEqual(nativeRows(outcome.tree, source, metadata), oracleRows(source, fixtures.language));
        }
      } catch (error) { failures.push(nativeCorpusFailure(`${fixtures.id}: ${JSON.stringify(source)}`, error)); }
    }
  }
  console.log(fixtures.id, fixtures.matches.length, fixtures.rejections.length);
}
console.log(JSON.stringify(failures, null, 2));
process.exitCode = failures.length ? 1 : 0;
