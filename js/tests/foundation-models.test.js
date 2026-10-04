// Foundation neutrality: integer and overflow models, universes, effects,
// proof systems and logics are distinct data with explicit correspondences,
// the translations' encodings are justified by those correspondences, no
// production source builds in a consumer's own foundation, and the check
// rejects a hard-coded universal logic (requirement I195-VISION-FOUNDATION-NEUTRAL).
// The Rust twin is rust/tests/unit/foundation_models.rs.
import assert from 'node:assert/strict';
import { mkdtempSync, mkdirSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import path from 'node:path';
import test from 'node:test';
import { fileURLToPath } from 'node:url';

import {
  CONSUMER_REGISTRIES,
  FOUNDATION_MODEL_PATHS,
  FOUNDATION_REGISTER,
  consumerFoundationReferences,
} from '../scripts/build-foundation-models.mjs';
import {
  FOUNDATION_MODELS,
  checkFoundationModels,
  foundationCorrespondences,
  foundationJustifications,
  foundationModel,
  languageFoundationModels,
} from '../src/foundation-models.js';
import { translateProgram } from '../src/program-translation.js';
import { recordIssue195Observations } from './support/issue-195-observations.js';

const REQUIREMENT = 'I195-VISION-FOUNDATION-NEUTRAL';
const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const LANGUAGES = ['JavaScript', 'Rust', 'Lean', 'Rocq'];
const stages = JSON.parse(readFileSync(path.join(root, 'parity/fixtures/translation-stages.json'), 'utf8'));

function observe(assertions, testName) {
  recordIssue195Observations({
    requirementId: REQUIREMENT,
    suffix: 'behavior',
    fixtureId: `planned:repository-directive:${REQUIREMENT.toLowerCase()}`,
    fixtureFile: FOUNDATION_REGISTER,
    assertions,
    testName,
  });
}

const kinds = (register) => checkFoundationModels(register).map(({ kind }) => kind);
const copy = () => structuredClone(FOUNDATION_MODELS);

test('foundations are data: every family has distinct models and every language records its own', () => {
  const register = readFileSync(path.join(root, FOUNDATION_REGISTER), 'utf8');
  for (const file of FOUNDATION_MODEL_PATHS) assert.equal(readFileSync(path.join(root, file), 'utf8'), register, file);
  assert.deepEqual(checkFoundationModels(FOUNDATION_MODELS), []);
  const families = FOUNDATION_MODELS.families.map(({ id }) => id);
  assert.deepEqual(families, ['integer-model', 'overflow-model', 'universe-model', 'effect-model', 'proof-system', 'logic-model']);
  for (const family of families) {
    assert.ok(FOUNDATION_MODELS.models.filter((model) => model.family === family).length >= 2, `${family} has several models`);
  }
  for (const language of LANGUAGES) {
    for (const family of families) assert.ok(languageFoundationModels(language, family).length > 0, `${language} records its ${family}`);
  }
  // Each language keeps its own proof authority: no two languages share a proof checker.
  assert.deepEqual(LANGUAGES.map((language) => languageFoundationModels(language, 'proof-system').map(({ id }) => id)), [
    ['proof-system.no-proof-checker'],
    ['proof-system.no-proof-checker'],
    ['proof-system.lean-kernel'],
    ['proof-system.rocq-kernel'],
  ]);
  assert.equal(foundationModel('overflow-model.abort').family, 'overflow-model');
  assert.equal(foundationCorrespondences('proof-system.rocq-kernel', 'proof-system.lean-kernel')[0].kind, 'restatement');
  observe(['foundationsRepresentedAsData'], 'foundations are data: every family has distinct models and every language records its own');
});

test('distinct models stay distinct, and every translation encoding is justified by a recorded correspondence', () => {
  // Homonyms: the same spelling names different models in different languages.
  const homonyms = FOUNDATION_MODELS.distinctions.map(({ models }) => models.join(' / '));
  assert.ok(homonyms.includes('effect-model.abort / effect-model.panic-with-default-value'), 'panic! differs between Rust and Lean');
  assert.ok(homonyms.includes('logic-model.constructive-propositions / logic-model.classical-propositions'), 'Prop differs between Rocq and Lean');
  assert.ok(homonyms.includes('proof-system.bounded-property-check / proof-system.lean-kernel'), 'a bounded check is not a proof');

  const merged = copy();
  merged.models.find(({ id }) => id === 'overflow-model.saturating').properties = { ...merged.models.find(({ id }) => id === 'overflow-model.wrapping').properties };
  assert.deepEqual(kinds(merged), ['indistinct-models']);
  const exact = copy();
  exact.correspondences[0].kind = 'exact';
  assert.deepEqual(kinds(exact), ['correspondence-kind']);
  const unconditioned = copy();
  unconditioned.correspondences[0].condition = '';
  assert.deepEqual(kinds(unconditioned), ['unconditioned-correspondence']);

  const rustOverflow = 'fn add(a: u8, b: u8) -> u8 { a + b }\nfn main() { println!("{}", add(200, 50)); }\n';
  const leanTheorem = stages.programs.find(({ name }) => name === 'check/lean-theorem-cases').source;
  const cases = [
    ...['JavaScript', 'Lean', 'Rocq'].map((target) => ['Rust', target, rustOverflow]),
    ...['JavaScript', 'Rust'].map((target) => ['Lean', target, leanTheorem]),
  ];
  const covered = new Set();
  for (const [source, target, program] of cases) {
    const { semantics } = translateProgram(program, source, target);
    const sourceModels = new Set(languageFoundationModels(source).map(({ id }) => id));
    const targetModels = new Set(languageFoundationModels(target).map(({ id }) => id));
    const ids = [...semantics.encodings, ...semantics.assumptions].map(({ id }) => id.split(':')[0]);
    for (const id of ids.filter((candidate) => ['abort-threading', 'machine-integer', 'numbers', 'theorem-properties'].includes(candidate))) {
      const justified = foundationJustifications(id).some((entry) =>
        entry.models ? entry.models.some((model) => sourceModels.has(model)) : sourceModels.has(entry.from) && targetModels.has(entry.to),
      );
      assert.ok(justified, `${source} -> ${target} encoding ${id} is justified by a correspondence between their models`);
      covered.add(id);
    }
  }
  assert.deepEqual([...covered].sort(), ['abort-threading', 'machine-integer', 'numbers', 'theorem-properties']);
  observe(['distinctModelsKeptDistinct'], 'distinct models stay distinct, and every translation encoding is justified by a recorded correspondence');
});

test('the check rejects a hard-coded universal logic and a consumer foundation built into production code', () => {
  const marked = copy();
  marked.models.find(({ id }) => id === 'logic-model.belnap-four-valued').universal = true;
  assert.deepEqual(kinds(marked), ['unknown-field']);
  // A hub every other logic embeds into is a universal logic, however it is named.
  const hub = copy();
  for (const model of hub.models.filter(({ family, id }) => family === 'logic-model' && id !== 'logic-model.belnap-four-valued')) {
    hub.correspondences = hub.correspondences.filter(({ from, to }) => !(from === model.id && to === 'logic-model.belnap-four-valued'));
    hub.correspondences.push({ from: model.id, to: 'logic-model.belnap-four-valued', kind: 'embedding', condition: 'Every value is read as a four-valued truth value.' });
  }
  assert.deepEqual(kinds(hub), ['universal-model']);
  // One proof checker imposed on every language is a universal foundation.
  const imposed = copy();
  for (const language of imposed.languages) {
    language.models = [...language.models.filter((id) => !id.startsWith('proof-system.')), 'proof-system.lean-kernel'];
  }
  assert.deepEqual(kinds(imposed), ['universal-model']);
  const unrecorded = copy();
  unrecorded.languages[0].models = unrecorded.languages[0].models.filter((id) => !id.startsWith('proof-system.'));
  assert.deepEqual(kinds(unrecorded), ['language-family']);
  observe(['hardCodedFoundationRejected'], 'the check rejects a hard-coded universal logic and a consumer foundation built into production code');
});

test('no production source names a consumer\'s own foundation outside the consumer registries', () => {
  assert.deepEqual(consumerFoundationReferences(root), []);
  const directory = mkdtempSync(path.join(tmpdir(), 'meta-language-foundations-'));
  try {
    mkdirSync(path.join(directory, 'js/src'), { recursive: true });
    mkdirSync(path.join(directory, 'rust/src'), { recursive: true });
    writeFileSync(path.join(directory, 'js/src/logic.js'), '// the RML truth tables are the logic of every proof\nexport const logic = 1;\n');
    writeFileSync(path.join(directory, 'rust/src/parity.rs'), '// upstream: relative-meta-logic\n');
    assert.deepEqual(consumerFoundationReferences(directory).map(({ path: file, line }) => `${file}:${line}`), ['js/src/logic.js:1']);
    assert.ok(CONSUMER_REGISTRIES.includes('rust/src/parity.rs'));
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
  observe(['noRelativeMetaLogicSpecificSyntax'], 'no production source names a consumer\'s own foundation outside the consumer registries');
});
