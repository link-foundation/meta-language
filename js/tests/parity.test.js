import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { test } from 'node:test';

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
  const manifest = JSON.parse(
    await readFile(new URL('../../parity/language-features.json', import.meta.url), 'utf8'),
  );
  const required = manifest.features.filter((feature) => feature.required);

  assert.ok(required.length > 0);

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
