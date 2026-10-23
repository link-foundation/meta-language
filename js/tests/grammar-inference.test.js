// Future controls, intentionally not executed during the complete draft phase.
import assert from 'node:assert/strict';
import test from 'node:test';
import { readProjectData } from '../scripts/project-data.mjs';
import { inferRegularGrammar, readRegularGrammarInferenceLinks, acceptInferredGrammarSymbols } from '../src/grammar-inference.js';
import { LinkNetwork } from '../src/network.js';
import { encodeLinkNetworkBinary, decodeLinkNetworkBinary } from '../src/link-network-binary.js';
import { createDynamicMetaLanguage } from '../src/dynamic-meta-language.js';

const fixtures = readProjectData(new URL('../../parity/fixtures/regular-grammar-inference.lino', import.meta.url));
const [major, minor] = process.versions.node.split('.').map(Number);
assert.ok(major === 22 && minor >= 15 || major === 24, 'uncovered Node ' + process.versions.node + '; ' + fixtures.versionRanges.JavaScript);

for (const fixture of fixtures.cases) test(fixture.name + ': observations, independent controls and both complete codecs', () => {
  const original = inferRegularGrammar(fixture.positives, fixture.negatives, fixture.options);
  const text = LinkNetwork.fromLino(original.network.toLino());
  const binary = decodeLinkNetworkBinary(encodeLinkNetworkBinary(original.network));
  for (const model of [original, readRegularGrammarInferenceLinks(text, original.root), readRegularGrammarInferenceLinks(binary, original.root)]) {
    for (const symbols of fixture.positives) assert.equal(acceptInferredGrammarSymbols(model, symbols), true);
    for (const symbols of fixture.negatives) assert.equal(acceptInferredGrammarSymbols(model, symbols), false);
    for (const control of fixture.controls) assert.equal(acceptInferredGrammarSymbols(model, control.symbols), control.accepted);
    assert.deepEqual(model.table, original.table);
    assert.deepEqual(model.grammar.normalized(), original.grammar.normalized());
  }
  const language = createDynamicMetaLanguage([original.source]);
  // The parser's normal concrete result retains every accepted source byte.
  const parsed = language.parse(original.source.id, fixture.positives[1].join(''));
  assert.equal(parsed.projection, 'concrete syntax');
  assert.ok(parsed.tree);
  const text = tree => tree.type === 'node' ? tree.children.map(text).join('') : tree.text;
  assert.equal(text(parsed.tree), fixture.positives[1].join(''));
  assert.ok(parsed.concrete);
  assert.ok(parsed.archive);
  const corrupted = { ...original, table: [...original.table] };
  corrupted.table[1] = original.table.length;
  assert.throws(() => acceptInferredGrammarSymbols(corrupted, ['a']), /invalid model table/u);
});

for (const fixture of fixtures.refusals) test(fixture.name + ' remains an explicit refusal', () => {
  assert.throws(() => inferRegularGrammar(fixture.positives, fixture.negatives, fixture.options), error => error.message.includes(fixture.reason));
});
