import assert from 'node:assert/strict';
import { test } from 'node:test';
import { checkProgram } from '../src/translation/check.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { markSharedReferenceTypes } from '../src/translation/javascript-cells.js';
import {
  acceptSharedSurfaceReferenceKind,
  acceptSurfaceRawMetadataField,
  acceptSurfaceRawMetadataForm,
  selectSurfaceReferenceSemantics,
} from '../src/translation/surface-raw-metadata-rules.js';

const identity = '/** @param {boolean} value @returns {boolean} */ function identity(value) { return value; }';

test('actual grouped source keeps its marker while the ungrouped expression omits it', () => {
  const plain = parseJavaScript(identity);
  const grouped = parseJavaScript(identity.replace('return value;', 'return ((value));'));
  assert.equal(Object.hasOwn(plain.items[0].body, 'parenthesized'), false);
  assert.equal(grouped.items[0].body.parenthesized, true);
  assert.deepEqual(checkProgram(grouped).declarations.get('identity').ret, { kind: 'bool' });
});

test('raw nullable Boolean fields retain presence while the checker retains its selected program form', () => {
  const baseline = parseJavaScript(identity, { referenceSemantics: false });
  assert.equal(Object.hasOwn(baseline, 'referenceSemantics'), false);
  for (const value of [false, null, true]) {
    const raw = structuredClone(baseline);
    raw.referenceSemantics = value;
    raw.items[0].body.parenthesized = value;
    assert.deepEqual(JSON.parse(JSON.stringify(raw)), raw);
    const checked = checkProgram(raw);
    assert.equal(Object.hasOwn(checked, 'referenceSemantics'), value === true);
    if (value === true) assert.equal(checked.referenceSemantics, true);
  }
});

test('shared representation selection marks all four structural kinds and their nested types', () => {
  const scalar = { kind: 'bool' };
  const array = { kind: 'array', element: scalar };
  const record = { kind: 'record', fields: [{ name: 'values', type: array }] };
  const map = { kind: 'map', key: scalar, value: record };
  const set = { kind: 'set', element: map };
  const program = { referenceSemantics: true, items: [{ type: set }] };
  const marked = markSharedReferenceTypes(program);
  const actual = marked.items[0].type;
  assert.equal(actual.reference, true);
  assert.equal(actual.element.reference, true);
  assert.equal(actual.element.value.reference, true);
  assert.equal(actual.element.value.fields[0].type.reference, true);
  assert.equal(Object.hasOwn(actual.element.key, 'reference'), false);
  assert.equal(Object.hasOwn(set, 'reference'), false, 'the original raw record is unchanged');
  for (const value of [false, null]) {
    const unselected = { ...program, referenceSemantics: value };
    assert.equal(markSharedReferenceTypes(unselected), unselected);
  }
});

test('the shared wire policy distinguishes the two contexts and rejects unrelated value forms', () => {
  for (const context of ['expression', 'program', 'type', 'function']) {
    for (const field of ['parenthesized', 'referenceSemantics', 'reference', 'block']) {
      assert.equal(acceptSurfaceRawMetadataField(context, field), context === 'expression' && field === 'parenthesized' || context === 'program' && field === 'referenceSemantics');
    }
  }
  for (const form of ['boolean', 'null']) assert.equal(acceptSurfaceRawMetadataForm(form), true);
  for (const form of ['number', 'string', 'array', 'object', 'undefined']) assert.equal(acceptSurfaceRawMetadataForm(form), false);
  for (const kind of ['array', 'record', 'map', 'set']) assert.equal(acceptSharedSurfaceReferenceKind(kind), true);
  for (const kind of ['option', 'tuple', 'cell', 'function', 'data', 'bool']) assert.equal(acceptSharedSurfaceReferenceKind(kind), false);
  for (const required of [false, true]) for (const structural of [false, true]) for (const enabled of [false, true]) {
    assert.equal(selectSurfaceReferenceSemantics(required, structural, enabled), required || enabled && structural);
  }
});

test('raw index presence facets retain omitted null false and true values', () => {
  const source = '/** @param {number[]} values @returns {number} */ function first(values){return values[0];}';
  const baseline = parseJavaScript(source).items[0].body;
  assert.equal(baseline.k, 'index');
  for (const field of ['optional', 'provenInBounds']) {
    assert.equal(acceptSurfaceRawMetadataField('index', field), true);
    for (const value of [undefined, null, false, true]) {
      const raw = { ...baseline };
      if (value === undefined) delete raw[field];
      else raw[field] = value;
      const restored = JSON.parse(JSON.stringify(raw));
      assert.equal(Object.hasOwn(restored, field), value !== undefined);
      if (value !== undefined) assert.equal(restored[field], value);
    }
  }
  assert.equal(acceptSurfaceRawMetadataField('index', 'reference'), false);
});
