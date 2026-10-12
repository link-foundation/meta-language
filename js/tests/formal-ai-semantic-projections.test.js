import { readProjectData } from '../scripts/project-data.mjs';
import assert from 'node:assert/strict';

import { test } from 'node:test';
import * as api from '../src/index.js';
import { formalAiSemanticProjections } from '../scripts/formal-ai-semantic-projections.mjs';
import { consumerHelpersModule, JAVASCRIPT_WORKLOAD_TESTS, JAVASCRIPT_PROBE } from '../scripts/issue-195-formal-ai-consumer.mjs';

const fixture = readProjectData(new URL('../../parity/fixtures/formal-ai-semantic-projections.json', import.meta.url));

test('all six projection directions execute numeric examples from independent fixtures', async () => {
  const observations = await formalAiSemanticProjections(api, fixture.cases);
  assert.equal(observations.length, 6);
  assert.deepEqual(observations.flatMap(({ id, failures }) => failures.map((failure) => `${id}: ${failure}`)), []);
  assert.equal(new Set(observations.map(({ from, target }) => `${from}->${target}`)).size, 6);
  assert.ok(observations.every(({ calls }) => calls.length === 3));
});

test('a translator that carries every source cannot pass semantic projection', async () => {
  const observations = await formalAiSemanticProjections({ selfTranslate() {
    return { code: 'export function calculate() { return 0; }', items: [{ status: 'carried' }] };
  } }, fixture.cases);
  assert.ok(observations.every(({ failures }) => failures.length > 0));
});

test('source restoration cannot conceal a non-executable Rust target', async () => {
  const counterfeit = { selfTranslate(source, from, target) {
    if (target === 'Rust') return { code: '// | export function calculate(a,b) { return a+b; }\npub fn calculate(a: f64, b: f64) -> f64 { 0.0 }', items: [{ status: 'translated' }] };
    return api.selfTranslate(source, from, target);
  } };
  const observations = await formalAiSemanticProjections(counterfeit, fixture.cases.filter(({ target }) => target === 'Rust'));
  assert.ok(observations.every(({ failures }) => failures.some((failure) => failure.includes('semantic call differs'))));
});

test('the clean consumer invokes projections through its installed package and retains observations', () => {
  assert.match(consumerHelpersModule(), /import \* as api from 'meta-language'/u);
  assert.match(JAVASCRIPT_WORKLOAD_TESTS, /semanticProjections\(inputs.semanticProjections\)/u);
  assert.match(JAVASCRIPT_PROBE, /semanticProjectionObservations/u);
});
