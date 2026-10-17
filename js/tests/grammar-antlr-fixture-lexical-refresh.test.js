import assert from 'node:assert/strict';
import test from 'node:test';
import { emitAntlr } from '../src/grammar-emitters/antlr.js';
import { importGrammarLossless, emitGrammarLossless, renderGrammarLayoutLinks } from '../src/grammar-lossless.js';
import { renderGrammarLinks } from '../src/grammar-links.js';
import { mutateGrammarStartRule } from '../src/grammar-round-trip.js';
import { renameGrammarRule } from '../src/grammar-rename.js';
import { normalizeMigrationAntlrLexerDeclarations, refreshAntlrReverseFixtureLexers } from '../scripts/antlr-reverse-fixture-lexers.mjs';

function historicalFixture() {
  const source = `// original source\ngrammar Start;\nstart: word '!' EOF;\nword: A;\nA: 'a';\n`;
  const { grammar, layout } = importGrammarLossless(source, 'antlr');
  const withoutLexers = (value) => value.split('\n').filter((line) => !line.startsWith('(lexer ')).join('\n');
  const edited = (grammar) => {
    const emitted = emitGrammarLossless(grammar, layout);
    return { links: withoutLexers(renderGrammarLinks(grammar)), source: emitted.source, lossy: emitted.report.lossy };
  };
  const exported = emitAntlr(grammar).source;
  const renamed = edited(renameGrammarRule(grammar, 'word', 'term').grammar);
  return { schemaVersion: 1, cases: [{ format: 'unchanged', source: 'immutable' }], reverse: [{
    id: 'antlr:authored', format: 'antlr', source, accepts: ['a!'], rejects: ['a', 'aa!'],
    links: withoutLexers(renderGrammarLinks(grammar)), exported,
    reimportedLinks: withoutLexers(renderGrammarLinks(grammar)), layout: renderGrammarLayoutLinks(layout),
    mutated: edited(mutateGrammarStartRule(grammar)), renamed: { from: 'word', to: 'term', ...renamed },
  }, { id: 'another format', format: 'bnf', source: 'kept without execution', links: 'immutable' }] };
}

test('ANTLR fixture refresh authenticates only added derived lexer declaration lines', () => {
  const original = historicalFixture();
  const frozen = structuredClone(original);
  const current = refreshAntlrReverseFixtureLexers(original);
  assert.deepEqual(normalizeMigrationAntlrLexerDeclarations(original, current), original);
  assert.deepEqual(original, frozen);
  assert.deepEqual(current.cases, original.cases);
  assert.deepEqual(current.reverse[1], original.reverse[1]);
  for (const path of [['links'], ['reimportedLinks'], ['mutated', 'links'], ['renamed', 'links']]) {
    const read = (value) => path.reduce((current, key) => current[key], value);
    assert.match(read(current.reverse[0]), /^\(lexer /mu);
    assert.equal(read(current.reverse[0]).split('\n').filter((line) => !line.startsWith('(lexer ')).join('\n'), read(original.reverse[0]));
  }
});

test('ANTLR fixture refresh refuses removed clauses, forged lexer lines and any protected source or sample edit', () => {
  const original = historicalFixture();
  const current = refreshAntlrReverseFixtureLexers(original);
  for (const mutate of [
    (value) => { value.reverse[0].links = original.reverse[0].links; },
    (value) => { value.reverse[0].links += '(lexer forged (seq))\n'; },
    (value) => { value.reverse[0].mutated.links = value.reverse[0].mutated.links.replace('round-trip-mutation', 'wrong'); },
    (value) => { value.reverse[0].renamed.source += '// changed\n'; },
    (value) => { value.reverse[0].layout += '(forged)\n'; },
    (value) => { value.reverse[0].accepts.push('wrong'); },
    (value) => { value.reverse[0].rejects = []; },
    (value) => { value.reverse[0].source += ' '; },
    (value) => { value.cases[0].source = 'changed'; },
    (value) => { value.reverse[1].links = 'changed'; },
  ]) {
    const changed = structuredClone(current);
    mutate(changed);
    assert.throws(() => normalizeMigrationAntlrLexerDeclarations(original, changed), /ANTLR lexical refresh/u);
  }
});
