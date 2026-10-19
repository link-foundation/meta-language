import assert from 'node:assert/strict';
import { test } from 'node:test';
import { readProjectData } from '../scripts/project-data.mjs';
import { readFileSync } from 'node:fs';

import {
  API_OPERATIONS,
  ApiOperation,
  ApiStyle,
  ApiStyleFixtureKind,
  runApiStyleFixture,
} from '../src/index.js';

test('JavaScript API operation registry mirrors Rust operation families and style cells', () => {
  for (const entry of API_OPERATIONS) {
    for (const style of ApiStyle.ALL) {
      const cell = entry.style(style);
      assert.ok(cell, `${entry.name()} is missing ${style} style cell`);
      assert.ok(cell.fixture.value.length > 0);
    }
  }

  for (const operation of Object.values(ApiOperation)) {
    assert.ok(
      API_OPERATIONS.some((candidate) => candidate.operation === operation),
      `missing API operation registry entry for ${operation}`,
    );
  }
});

test('executable JavaScript API-style fixtures cover every applicable registry cell', () => {
  for (const entry of API_OPERATIONS) {
    for (const cell of entry.styles()) {
      if (cell.fixture.kind === ApiStyleFixtureKind.Executable) {
        runApiStyleFixture(cell.fixture.value);
      }
    }
  }
});

test('issue-163 parity manifest keeps Rust and JavaScript feature rows in sync', async () => {
  const manifest = readProjectData(new URL('../../parity/language-features.json', import.meta.url));
  const required = manifest.features.filter((feature) => feature.required);

  assert.ok(required.length > 0);
  assert.ok(
    required.some((feature) => feature.id === 'truth-value-semantics'),
    'manifest must track truth-value semantics parity',
  );

  // Keep the native evidence scope gate visible in the JavaScript-first checks.
  const nativeAdapter = 'experiments/grammar-outcome-network/native-transport/inverse/static-borrows.rs';
  const nativeOwner = readFileSync(new URL('../../rust/tests/unit/grammar_outcome_static_borrows.rs', import.meta.url), 'utf8');
  const nativeModules = readFileSync(new URL('../../rust/tests/unit/mod.rs', import.meta.url), 'utf8');
  const nativeStorageRegistered = nativeOwner.includes('#[path = "../../../' + nativeAdapter + '"]\nmod storage;')
    && nativeModules.split('\n').includes('mod grammar_outcome_static_borrows;');
  const rustEvidence = (feature, file) => file.startsWith('rust/') || file.startsWith('docs/') || file.startsWith('parity/')
    || file === nativeAdapter && nativeStorageRegistered
    || file.startsWith('experiments/') && file.endsWith('.lino')
      && file.split('/').every(part => part !== '' && part !== '.' && part !== '..')
      && feature.javascript.evidence.includes(file);
  for (const feature of manifest.features) for (const file of feature.rust.evidence) {
    assert.ok(rustEvidence(feature, file), feature.id + ' Rust evidence must cite native or shared source: ' + file);
  }
  const translation = manifest.features.find(feature => feature.id === 'four-language-portable-translation');
  const driver = 'experiments/original-link-complete-table-projection/raw-borrowed-emission.mjs';
  const fixture = 'experiments/original-link-complete-table-projection/data/raw-borrowed-parameter-receipt.lino';
  assert.ok(translation.javascript.evidence.includes(driver), 'the JavaScript producer stays in its source evidence scope');
  assert.ok(translation.javascript.evidence.includes(driver.replace('.mjs', '.test.mjs')));
  assert.ok(translation.rust.evidence.includes(fixture), 'the shared source receipt remains Rust evidence without claiming native execution');
  const receipt = readProjectData(new URL('../../' + fixture, import.meta.url));
  assert.equal(receipt.nativeCompilation, false);
  assert.equal(receipt.nativeExecution, false);
  assert.ok(rustEvidence(translation, fixture));
  assert.equal(rustEvidence(translation, driver), false, 'citing a driver in JavaScript does not turn it into native evidence');
  assert.equal(nativeStorageRegistered, true, 'the exact native storage source belongs to the actual registered Rust unit module');
  assert.equal(rustEvidence(translation, nativeAdapter), true, 'a compiled native Rust module remains native source inside experiments');
  for (const file of [nativeAdapter.replace('static-borrows.rs', 'unregistered.rs'), nativeAdapter.replace('.rs', '.mjs')]) {
    assert.equal(rustEvidence({...translation, javascript: {evidence: [file]}}, file), false, 'unregistered native sources and JavaScript drivers remain outside native evidence');
  }
  assert.equal(rustEvidence({...translation, javascript: {evidence: []}}, fixture), false, 'experiment data must be shared by both runtime cells');
  for (const file of ['experiments/../outside.lino', 'experiments//outside.lino', 'outside/receipt.lino']) {
    assert.equal(rustEvidence({...translation, javascript: {evidence: [file]}}, file), false);
  }

  for (const feature of required) {
    assert.equal(feature.rust.status, 'implemented', `${feature.id} missing Rust implementation`);
    assert.equal(
      feature.javascript.status,
      'implemented',
      `${feature.id} missing JavaScript implementation`,
    );
    assert.ok(feature.rust.evidence.length > 0, `${feature.id} missing Rust evidence`);
    assert.ok(feature.javascript.evidence.length > 0, `${feature.id} missing JS evidence`);
  }
});
