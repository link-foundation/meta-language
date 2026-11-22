import assert from 'node:assert/strict';
import test from 'node:test';
import { recoverJson } from '../src/json-recovery.js';
import { validateJson } from '../src/json-schema-validation.js';

const answerSchema = {
  type: 'object', additionalProperties: false, required: ['status', 'evidence'],
  properties: { status: { enum: ['met', 'not_met', 'unknown'] }, evidence: { type: 'string' } },
};
const range = ({ byteRange }) => [byteRange.start, byteRange.end];

function assertEditAnchors(source, result) {
  assert.equal(result.sourceNetwork.reconstructText(), source);
  assert.equal(result.network.reconstructText(), result.text);
  for (const edit of result.edits) {
    const anchor = result.sourceNetwork.link(edit.linkId).metadata();
    assert.ok(anchor.flags.hasRecoveryIssue());
    assert.equal(Buffer.from(source).subarray(edit.span.start, edit.span.end).toString(), edit.removed);
  }
}

test('strict JSON validation locates both schema violations and malformed values on the CST', () => {
  const wrong = '{"status":"partially","evidence":"café","extra":1}';
  const report = validateJson(wrong, answerSchema);
  assert.equal(report.valid, false);
  assert.deepEqual(report.errors.map(error => [error.keyword, error.pointer, range(error.span)]), [
    ['enum', '/status', [10, 21]], ['additionalProperties', '/extra', [41, 48]],
  ]);
  assert.deepEqual(report.errors[0].expected, ['met', 'not_met', 'unknown']);
  for (const issue of report.errors) assert.ok(report.network.link(issue.linkId));
  const truncated = '{"status":"partially","evidence":"café';
  const partial = validateJson(truncated, answerSchema);
  assert.equal(partial.network.reconstructText(), truncated);
  assert.deepEqual(partial.errors.map(error => [error.keyword, error.pointer]), [['syntax', '/evidence'], ['enum', '/status']]);
  assert.deepEqual(range(partial.errors[0].span), [Buffer.byteLength(truncated), Buffer.byteLength(truncated)]);
  assert.ok(partial.errors[0].expected.length);
  assert.equal(validateJson('{"status":"met","evidence":"known"}', answerSchema).valid, true);
  for (const text of ['01', '1.', 'true false', '/* annotation */ null', '1\v', '"a\u0001b"']) {
    assert.equal(validateJson(text, true).valid, false, JSON.stringify(text));
  }
});

test('JSON validation checks local references, alternatives, tuple bounds, formats and unknown keywords', () => {
  const schema = {
    $defs: { 'label/name': { pattern: '^[A-Z]+$' } }, type: 'array',
    prefixItems: [{ $ref: '#/$defs/label~1name' }, { oneOf: [{ type: 'integer', minimum: 1 }, { const: 3 }] }],
    items: { anyOf: [{ format: 'date', type: 'string' }, { type: 'null' }] }, minItems: 3, maxItems: 4,
  };
  assert.equal(validateJson('["YES",2,"2024-02-29",null]', schema).valid, true);
  assert.deepEqual(validateJson('["Yes",3,"2023-02-29",null,true]', schema).errors.map(error => [error.keyword, error.pointer]), [
    ['maxItems', ''], ['pattern', '/0'], ['oneOf', '/1'], ['anyOf', '/2'], ['anyOf', '/4'],
  ]);
  const unknown = validateJson('null', { unknownKeyword: 1, $ref: 'https://example.org/schema' });
  assert.equal(unknown.valid, false);
  assert.deepEqual(unknown.schemaIssues.map(issue => issue.pointer), ['/unknownKeyword', '/$ref']);
  assert.equal(validateJson('"2024-02-29T13:50:00+03:00"', { format: 'date-time' }).valid, true);
  assert.equal(validateJson('"123e4567-e89b-12d3-a456-426614174000"', { format: 'uuid' }).valid, true);
  assert.equal(validateJson('"first.last@example.org"', { format: 'email' }).valid, true);
});

test('JSON recovery only edits explicitly enabled locations and preserves truncated status', () => {
  const truncation = '{"status":"met","evidence":"café';
  const unchanged = recoverJson(truncation);
  assert.equal(unchanged.text, truncation);
  assert.deepEqual(unchanged.edits, []);
  assert.equal(unchanged.complete, false);
  const recovered = recoverJson(truncation, { schema: answerSchema, allow: ['closeOpenContainers'] });
  assert.deepEqual(recovered.value, { status: 'met', evidence: 'café' });
  assert.equal(recovered.complete, false);
  assert.deepEqual(recovered.edits.map(edit => edit.inserted), ['"', '}']);
  assertEditAnchors(truncation, recovered);
  const missing = recoverJson('{"status":"met"', { schema: answerSchema, allow: ['closeOpenContainers'] });
  assert.equal(missing.complete, false);
  assert.deepEqual(missing.value, { status: 'met' });
  assert.deepEqual(missing.issues.map(error => [error.keyword, error.pointer]), [['required', '/evidence']]);
  for (const [source, action, expected] of [
    ['{"a":1, }', 'dropTrailingCommas', { a: 1 }],
    ['[1,2, ]', 'dropTrailingCommas', [1, 2]],
    ['```json\n{"status":"met"}\n```', 'stripCodeFence', { status: 'met' }],
    ['true false', 'extractFirstValue', true],
    ['{"a":true} suffix', 'stripTrailingGarbage', { a: true }],
    ['{"a":"x\u0004y"}', 'repairEscapes', { a: 'x\u0004y' }],
    ['{"a":"x\\qy"}', 'repairEscapes', { a: 'x\\qy' }],
  ]) {
    const result = recoverJson(source, { allow: [action] });
    assert.deepEqual(result.value, expected, JSON.stringify(source));
    assert.equal(result.complete, true);
    assert.ok(result.edits.length);
    assert.ok(result.edits.every(edit => edit.kind === action));
    assertEditAnchors(source, result);
  }
  assert.throws(() => recoverJson('null', { allow: ['inventValues'] }), RangeError);
});

test('degenerate substring repetition is reported without editing valid JSON', () => {
  const source = JSON.stringify({ content: `🌍${'abcde'.repeat(20)}!` });
  const report = recoverJson(source);
  assert.equal(report.complete, true);
  assert.equal(report.text, source);
  assert.deepEqual(report.edits, []);
  const [issue] = report.issues;
  assert.equal(issue.keyword, 'repetition');
  assert.equal(issue.pointer, '/content');
  assert.equal(issue.pattern, 'abcde');
  assert.equal(Buffer.from(source).subarray(issue.span.start, issue.span.end).toString(), 'abcde'.repeat(20));
});
