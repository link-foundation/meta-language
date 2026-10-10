import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { test } from 'node:test';

import { LinkNetwork, compileGrammar, parseGrammarLinks } from '../src/index.js';
import { nativeRows, oracleRecovers, oracleRows } from '../scripts/native-grammar-rows.mjs';

const read = (file) => readFileSync(new URL(`../../${file}`, import.meta.url), 'utf8');
const lock = JSON.parse(read('js/src/vendor/grammars/grammar-lock.json'));

for (const [id, language, source] of [
  ['lean', 'Lean', 'def answer : Nat := 42\n'],
  ['rocq', 'Rocq', 'Definition answer : nat := 42.\n'],
]) {
  test(`native ${language} retains its independent development oracle`, () => {
    assert.equal(lock.grammars[id].oracle, true);
    const fixture = JSON.parse(read(`parity/fixtures/native-grammars/${id}.json`));
    const parser = compileGrammar(parseGrammarLinks(read(`parity/grammars/native/${id}.lino`)));
    assert.equal(oracleRecovers(source, language), false);
    const outcome = parser.parseTree(source);
    assert.equal(outcome.ok, true);
    assert.deepEqual(nativeRows(outcome.tree, source, fixture), oracleRows(source, language));
    const network = LinkNetwork.parse(source, language);
    assert.equal(network.reconstructText(), source);
    assert.ok(network.parseGrammars().some(({ id: grammar }) => grammar === `native-${id}`));
  });
}
