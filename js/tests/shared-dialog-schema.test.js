import assert from 'node:assert/strict';
import { readFile, readdir } from 'node:fs/promises';
import { test } from 'node:test';

// The shared-dialog source-description schema is defined once, for every
// repository that captures or consumes shared AI dialogs, under
// docs/schemas/shared-dialog/. These tests give the JavaScript implementation
// the same guarantees the Rust suite enforces for the JSON interchange form:
// every example matches shared-dialog.schema.json, and the demo_memory mapping
// preserves provider, source URL, turn role, and turn content.

const schemaDir = new URL('../../docs/schemas/shared-dialog/', import.meta.url);
const examplesDir = new URL('examples/', schemaDir);

async function readJson(url) {
  return JSON.parse(await readFile(url, 'utf8'));
}

function stringArray(schema, path) {
  let node = schema;
  for (const key of path) {
    assert.ok(node && typeof node === 'object', `schema missing ${path.join('/')}`);
    node = node[key];
  }
  assert.ok(Array.isArray(node), `schema ${path.join('/')} must be an array`);
  return node;
}

function loadRules(schema) {
  return {
    sourceRequired: stringArray(schema, ['required']),
    captureStatus: new Set(stringArray(schema, ['$defs', 'captureStatus', 'enum'])),
    captureMethod: new Set(stringArray(schema, ['$defs', 'captureMethod', 'enum'])),
    role: new Set(stringArray(schema, ['$defs', 'role', 'enum'])),
    visibility: new Set(stringArray(schema, ['$defs', 'visibility', 'enum'])),
    turnRequired: stringArray(schema, ['$defs', 'sharedDialogTurn', 'required']),
    diagnosticRequired: stringArray(schema, ['$defs', 'sharedDialogCaptureDiagnostic', 'required']),
  };
}

function checkSource(rules, label, source) {
  assert.ok(source && typeof source === 'object', `${label}: source must be an object`);
  for (const field of rules.sourceRequired) {
    assert.ok(field in source, `${label}: missing required field \`${field}\``);
  }
  assert.ok(
    rules.captureStatus.has(source.capture_status),
    `${label}: capture_status \`${source.capture_status}\` is not a known status value`,
  );
  assert.ok(
    rules.captureMethod.has(source.capture_method),
    `${label}: capture_method \`${source.capture_method}\` is not a known method`,
  );

  if (source.capture_status === 'captured') {
    assert.ok(Array.isArray(source.turns) && source.turns.length > 0, `${label}: captured source needs turns`);
    for (const turn of source.turns) checkTurn(rules, label, turn);
  } else {
    assert.ok(
      Array.isArray(source.diagnostics) && source.diagnostics.length > 0,
      `${label}: non-captured source needs diagnostics`,
    );
    for (const diagnostic of source.diagnostics) checkDiagnostic(rules, label, diagnostic);
  }
}

function checkTurn(rules, label, turn) {
  for (const field of rules.turnRequired) {
    assert.ok(field in turn, `${label}: turn missing required field \`${field}\``);
  }
  assert.ok(rules.role.has(turn.role), `${label}: turn role \`${turn.role}\` is not a known role`);
  assert.ok(Number.isInteger(turn.order), `${label}: turn order must be an integer`);
  if (turn.visibility !== undefined) {
    assert.ok(
      rules.visibility.has(turn.visibility),
      `${label}: turn visibility \`${turn.visibility}\` is not a known visibility`,
    );
  }
}

function checkDiagnostic(rules, label, diagnostic) {
  for (const field of rules.diagnosticRequired) {
    assert.ok(field in diagnostic, `${label}: diagnostic missing required field \`${field}\``);
  }
}

test('JSON examples match the shared-dialog schema', async () => {
  const schema = await readJson(new URL('shared-dialog.schema.json', schemaDir));
  const rules = loadRules(schema);

  const entries = (await readdir(examplesDir)).filter((name) => name.endsWith('.json')).sort();
  assert.ok(entries.length > 0, 'expected JSON examples to validate');

  let captured = 0;
  let diagnostics = 0;
  for (const name of entries) {
    const value = await readJson(new URL(name, examplesDir));
    // The mapping example wraps the instance under `source`.
    const source = value.source ?? value;
    checkSource(rules, name, source);
    if (source.capture_status === 'captured') captured += 1;
    else diagnostics += 1;
  }
  assert.ok(captured >= 3, 'expected at least three captured examples');
  assert.ok(diagnostics >= 1, 'expected at least one diagnostic example');
});

test('schema covers every required capture_status value', async () => {
  const schema = await readJson(new URL('shared-dialog.schema.json', schemaDir));
  const rules = loadRules(schema);
  for (const required of [
    'captured',
    'unsupported_provider_format',
    'provider_challenge',
    'login_required',
    'expired_or_deleted',
    'no_transcript_found',
  ]) {
    assert.ok(
      rules.captureStatus.has(required),
      `schema is missing required capture_status value \`${required}\``,
    );
  }
});

test('demo_memory mapping preserves provider, source URL, role, and content', async () => {
  const mapping = await readJson(new URL('demo-memory-mapping.json', examplesDir));
  const { source, events } = mapping;
  const turns = source.turns;

  assert.equal(events.length, turns.length, 'mapping must emit one event per turn');

  for (let i = 0; i < turns.length; i += 1) {
    assert.equal(events[i].provider, source.provider, 'event lost provider');
    assert.equal(events[i].source_url, source.source_url, 'event lost source_url');
    assert.equal(events[i].role, turns[i].role, 'event lost turn role');
    assert.equal(events[i].content, turns[i].content, 'event lost turn content');
  }
});
