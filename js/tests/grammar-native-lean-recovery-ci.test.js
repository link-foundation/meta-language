import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { test } from 'node:test';

import { compileGrammar, LinkNetwork, parseGrammarLinks, renderSyntaxTree } from '../src/index.js';
import { stepBudget } from '../src/grammar-runtime/executor.js';
import { documentGrammarRoots, renderCstLines } from './support/cst-lines.js';
import { applyEdit } from './support/generative.js';

const read = (path) => JSON.parse(readFileSync(new URL(`../../${path}`, import.meta.url), 'utf8'));
const fixture = read('parity/fixtures/native-grammars/lean.json');
const generative = read('parity/fixtures/issue-195-generative/lean.json');
const recovery = read('parity/fixtures/native-recovery.json').cases;
const grammar = compileGrammar(parseGrammarLinks(readFileSync(new URL('../../parity/grammars/native/lean.lino', import.meta.url), 'utf8')));

function preservesSignature(id, suite, source, bytes, hash) {
  assert.equal(Buffer.byteLength(source), bytes, id);
  assert.equal(stepBudget({ errorRecovery: true }, bytes).limit, 200_000 + 2000 * bytes, `${id}: original resource bound`);
  const recorded = recovery.find((row) => row.language === 'Lean' && row.suite === suite && row.case === id);
  assert.equal(recorded.native.cstSha256, hash, `${id}: retain the original recovery signature`);
  const network = LinkNetwork.parse(source, 'Lean');
  assert.equal(network.verifyFullMatch().isClean(), false, id);
  const text = renderCstLines(documentGrammarRoots(network, 'Lean'), 'Lean').text;
  assert.equal(createHash('sha256').update(text).digest('hex'), hash, id);
}

// These are the five exact inputs that regressed in CI 38227158183. Their
// signatures precede scanner-cache changes; they must never be regenerated to
// accept a larger ERROR suffix after a recovery round exhausts its budget.
for (const [label, prefix, id, bytes, hash] of [
  ['Macro Rules', '\nmacro_rules', 'corpus/syntax.txt/1', 99, '914d57e5e7b29b2fa3c4194fed719c8f8cdcf53071d75fd552a61ed9b405bdcd'],
  ['Scoped / Local', '\nscoped elab', 'corpus/syntax.txt/6', 574, '9dc6050a5d80a6ecf5a7981a70541c2a94302ed1ea5a1fc6f2db3538452861e9'],
]) test(`Lean ${label} retains its original recovered tree within the original step budget`, () => {
  const { source, recovered } = fixture.rejections.find((entry) => entry.source.startsWith(prefix));
  const result = grammar.parseTree(source, { errorRecovery: true, stepLimit: 200_000 + 2000 * bytes });
  assert.equal(result.ok, false);
  assert.equal(result.rejection.reason, 'recovered');
  assert.equal(renderSyntaxTree(result.tree), recovered);
  preservesSignature(id, 'conformance', source, bytes, hash);
});

const property = generative.cases.find((entry) => entry.id === 'property/2');
test('Lean generative property/2 retains the original recovery signature', () => {
  preservesSignature(property.id, 'generative', property.source, 701, 'c8cab9aa8f0b357120975c6498681859959fc20f624b79d99ff9e194b0c5087a');
});

const edit = generative.cases.find((entry) => entry.id === 'edit/7');
let edited = edit.source;
for (const [index, step] of edit.steps.entries()) {
  edited = applyEdit(edited, step);
  const expected = {
    0: [482, 'e677a75d9f5801be54e00a45cb2da3a0c6b167281d1ebf539eb627b90729e4f4'],
    3: [173, '1f30b872b536d61843035d8c77d185114612d0e61a019e9c6ae1afd9817500e8'],
  }[index];
  if (!expected) continue;
  const source = edited;
  test(`Lean edit/7 step ${index} retains the original recovery signature`, () => {
    preservesSignature(`edit/7 step ${index}`, 'generative', source, ...expected);
  });
}
