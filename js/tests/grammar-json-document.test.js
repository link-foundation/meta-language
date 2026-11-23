import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import test from 'node:test';
import { fileURLToPath } from 'node:url';
import { readProjectData } from '../scripts/project-data.mjs';
import { recordIssue195Observations } from './support/issue-195-observations.js';
import {
  compileGrammar, GrammarBuilder, emitGbnf, emitJsonSchema, emitLark, importGbnf,
  importJsonSchema, importLark, recoverJson, validateJson,
} from '../src/index.js';

const fixtureFile = 'parity/fixtures/json-schema-and-recovery.lino';
const shared = readProjectData(new URL(`../../${fixtureFile}`, import.meta.url));

const answerSchema = {
  type: 'object', additionalProperties: false, required: ['status', 'evidence'],
  properties: { status: { enum: ['met', 'not_met', 'unknown'] }, evidence: { type: 'string' } },
};
const range = ({ byteRange }) => [byteRange.start, byteRange.end];

function schemaParsers(schema) {
  const grammar = importJsonSchema(schema);
  return [
    grammar,
    importGbnf(emitGbnf(grammar).source),
    importLark(emitLark(grammar).source),
  ].map(item => compileGrammar(item));
}

test('the shipped JSON decoder grammar and self-translated pattern policy match their sources', () => {
  for (const script of ['build-json-decoder-grammar.mjs', 'build-json-schema-pattern-policy.mjs', 'build-json-schema-integer-policy.mjs']) {
    execFileSync(process.execPath, [fileURLToPath(new URL(`../scripts/${script}`, import.meta.url)), '--check']);
  }
});

test('JSON Schema emits decoders that constrain required fields, enums and escaped property names', () => {
  for (const parser of schemaParsers(answerSchema)) {
    for (const [source, accepted] of [
      ['{"status":"met","evidence":"café"}', true],
      ['{"evidence":"x","status":"unknown"}', true],
      ['{"st\\u0061tus":"m\\u0065t", "evidence":"x"}', true],
      ['{"status":"partially","evidence":"x"}', false],
      ['{"status":"met"}', false],
      ['{"status":"met","evidence":"x","extra":1}', false],
      ['{"status":"met","evidence":"x","status":"met"}', false],
      ['{"status":"met","evidence":"unterminated}', false],
    ]) assert.equal(parser.parseTree(source).ok, accepted, source);
  }
  const grammar = importJsonSchema(answerSchema);
  assert.deepEqual(grammar.schemaDiagnostics, []);
  assert.match(emitLark(grammar).source, /^start: /mu);
  assert.doesNotMatch(emitLark(grammar).source, /^json_format_email: /mu);
  assert.deepEqual(emitJsonSchema(grammar).schema, answerSchema);
});

test('JSON Schema lowers tuple limits, local references, formats and finite integer ranges', () => {
  const schema = {
    $defs: { code: { type: 'string', pattern: '^[A-Z]{2}$' } },
    type: 'array', minItems: 4, maxItems: 5,
    prefixItems: [
      { $ref: '#/$defs/code' },
      { type: 'integer', exclusiveMinimum: -3, maximum: 13 },
      { type: 'string', format: 'date' },
      { anyOf: [{ type: 'null' }, { type: 'string', format: 'uuid' }] },
    ],
    items: { type: 'string', format: 'email' },
  };
  for (const parser of schemaParsers(schema)) {
    for (const [source, accepted] of [
      ['["AX",-2,"2024-02-29",null]', true],
      ['["AX",13,"2000-02-29","123e4567-e89b-12d3-a456-426614174000","first.last@example.org"]', true],
      ['["Ax",0,"2024-02-29",null]', false],
      ['["AX",-3,"2024-02-29",null]', false],
      ['["AX",14,"2024-02-29",null]', false],
      ['["AX",0,"2100-02-29",null]', false],
      ['["AX",0,"2024-02-29","short"]', false],
      ['["AX",0,"2024-02-29",null,"bad@@example.org"]', false],
      ['["AX",0,"2024-02-29",null,"a@example.org","b@example.org"]', false],
      ['["AX",0,"2024-02-29"]', false],
    ]) assert.equal(parser.parseTree(source).ok, accepted, source);
  }
  const residual = importJsonSchema({ oneOf: [{ type: 'number' }, { const: 1 }], unsupportedKeyword: true });
  assert.deepEqual(residual.schemaDiagnostics.map(issue => issue.pointer), ['/unsupportedKeyword', '/oneOf', '/oneOf']);
  assert.match(residual.schemaDiagnostics[2].message, /Exclusive branch selection/u);
});

test('JSON Schema reverse export infers semantic constants from sample productions', () => {
  const { choice, literal, ref, seq } = GrammarBuilder;
  const grammar = new GrammarBuilder('document')
    .rule('document', seq(literal('{"status":'), ref('status'), literal('}')))
    .rule('status', choice(literal('"met"'), literal('"unknown"')))
    .build();
  const emitted = emitJsonSchema(grammar);
  assert.deepEqual(emitted.schema, { enum: [{ status: 'met' }, { status: 'unknown' }] });
  assert.equal(emitted.report.lossy.length, 1);
  assert.equal(validateJson('{"status":"met"}', emitted.schema).valid, true);
  assert.equal(validateJson('{"status":"not_met"}', emitted.schema).valid, false);
  assert.equal(emitJsonSchema(new GrammarBuilder('word').rule('word', literal('not JSON')).build()).schema, null);
  const imported = importJsonSchema(answerSchema);
  imported.rules.delete('start');
  assert.equal(emitJsonSchema(imported).schema, null);
});

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
  assert.equal(validateJson('"K@example.org"', { format: 'email' }).valid, false);
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
    ['true false null', 'extractFirstValue', true],
    ['{"a":true} suffix', 'stripTrailingGarbage', { a: true }],
    ['{"a":"x\u0004y"}', 'repairEscapes', { a: 'x\u0004y' }],
    ['{"a":"x\\qy"}', 'repairEscapes', { a: 'x\\qy' }],
    ['{"a":"x\\Q\\u01Z\u0001y"}', 'repairEscapes', { a: 'x\\Q\\u01Z\u0001y' }],
  ]) {
    const result = recoverJson(source, { allow: [action] });
    assert.deepEqual(result.value, expected, JSON.stringify(source));
    assert.equal(result.complete, true);
    assert.ok(result.edits.length);
    assert.ok(result.edits.every(edit => edit.kind === action));
    assertEditAnchors(source, result);
  }
  for (const [source, expected] of [['[1,', [1]], ['{"a":1,', { a: 1 }]]) {
    const result = recoverJson(source, { allow: ['dropTrailingCommas', 'closeOpenContainers'] });
    assert.deepEqual(result.value, expected);
    assert.equal(result.complete, false);
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

test('shared JSON Schema fixtures validate decoders and locate document errors', context => {
  for (const [schema, cases, diagnostics, validation] of shared.grammars) {
    if (diagnostics !== undefined) {
      assert.deepEqual(importJsonSchema(schema).schemaDiagnostics.map(issue => issue.pointer), diagnostics);
      if (diagnostics.length === 0) {
        for (const [source, accepted] of cases) assert.equal(validateJson(source, schema).valid, accepted, source);
      }
    }
    for (const parser of schemaParsers(schema)) {
      for (const [source, accepted] of cases) assert.equal(parser.parseTree(source).ok, accepted, source);
    }
    for (const [source, accepted] of validation ?? []) assert.equal(validateJson(source, schema).valid, accepted, source);
  }
  assert.deepEqual(emitJsonSchema(importJsonSchema(shared.schema)).schema, shared.schema);
  const grammar = new GrammarBuilder('document').rule('document',
    GrammarBuilder.choice(GrammarBuilder.literal('{"status":"met"}'), GrammarBuilder.literal('{"status":"unknown"}'))).build();
  assert.deepEqual(emitJsonSchema(grammar).schema, { enum: [{ status: 'met' }, { status: 'unknown' }] });
  for (const [source, errors] of [
    [shared.wrong, [['enum', '/status', [10, 21]], ['additionalProperties', '/extra', [41, 48]]]],
    [shared.truncated, [['syntax', '/evidence', [Buffer.byteLength(shared.truncated), Buffer.byteLength(shared.truncated)]], ['enum', '/status', [10, 21]]]],
  ]) {
    const report = validateJson(source, shared.schema);
    assert.equal(report.valid, false);
    assert.equal(report.network.reconstructText(), source);
    assert.deepEqual(report.errors.map(issue => [issue.keyword, issue.pointer, range(issue.span)]), errors);
    for (const issue of report.errors) {
      assert.ok(report.network.link(issue.linkId));
      assert.ok(issue.expected.length);
    }
  }
  const unsupported = importJsonSchema({ type: 'string', unregistered: true });
  assert.deepEqual(unsupported.schemaDiagnostics.map(issue => issue.pointer), ['/unregistered']);
  for (const schema of shared.ungeneratableSchemas) {
    assert.throws(() => importJsonSchema(schema), /no generatable document/u);
  }
  for (const [source, schema, expected] of shared.numericValidation) {
    const report = validateJson(source, schema);
    assert.deepEqual(report.schemaIssues, []);
    assert.equal(report.valid, expected.length === 0, source);
    assert.deepEqual(report.errors.map(issue => {
      assert.ok(report.network.link(issue.linkId));
      return [issue.keyword, issue.pointer, ...range(issue.span)];
    }), expected, source);
  }
  recordIssue195Observations({
    requirementId: 'I195-I240-JSON-SCHEMA', suffix: 'behavior',
    fixtureId: 'i195-i240-json-schema', fixtureFile,
    assertions: ['decoderGrammarsConstrainSchema', 'schemaDiagnosticsRetainSyntaxLinks', 'grammarAndSchemaRoundTrip', 'decoderDisjointExclusiveBranchesHaveNoResidual', 'decoderPatternsRespectDecodedStrings', 'decoderFormatsRespectDecodedStrings', 'decoderPatternsCombineEscapedSurrogatePairs', 'decoderObjectConstantsAcceptMemberPermutations', 'decoderExclusiveIntegerAlternativesRejectOverlap', 'decoderRequiredStringTagsDistinguishAlternatives', 'decoderFormatsCombineWithRegularPatterns', 'decoderChecksFiniteReferencedAlternatives', 'validationUsesExactNumericSpellings', 'decoderAppliesSiblingTypeAssertions', 'decoderSelectsExclusiveRegularStrings', 'decoderIntersectsSiblingRegularStrings', 'decoderResolvesLinkedAlternatives', 'decoderCombinesLinkedNumericBounds', 'decoderCombinesLinkedCollectionAssertions', 'decoderAppliesSiblingCollectionAssertionsToAlternatives', 'decoderBoundsClosedArrayTailsByTheirOwnPrefix', 'decoderAppliesLinkedAssertionsWithinInstanceTypes', 'decoderCombinesCompatibleObjectPropertyLayouts', 'decoderCombinesCompatibleArrayItemLayouts', 'decoderIntersectsScalarPropertyAndItemAssertions', 'decoderFiltersFinitePropertyAndItemCandidates', 'decoderIntersectsNestedCollectionLayouts', 'decoderDistinguishesExclusiveArrayLengthIntervals', 'decoderDistinguishesRequiredArrayPrefixValues', 'decoderExclusiveDecimalAlternativesRespectEndpoints', 'decoderDistinguishesIntegerValuesWithinNumberAlternatives', 'decoderChecksExclusiveAlternativesWithinInstanceTypes', 'decoderCountsFiniteNumericBranchesInExclusiveAlternatives', 'decoderChecksBooleanAndNullAlternativeValues'],
    testName: context.name,
  });
  recordIssue195Observations({
    requirementId: 'I195-I240-JSON-SCHEMA-FINITE-COLLECTIONS', suffix: 'behavior',
    fixtureId: 'i195-i240-json-schema-finite-collections', fixtureFile,
    assertions: ['decoderCountsOverlappingFiniteCollections'],
    testName: context.name,
  });
});

test('shared JSON recovery fixtures preserve opt-in edits and provenance', context => {
  for (const [source, allow, value, complete, insertions] of shared.recovery) {
    const untouched = recoverJson(source);
    assert.equal(untouched.text, source);
    assert.deepEqual(untouched.edits, []);
    const report = recoverJson(source, { allow });
    assert.deepEqual(report.value, value, source);
    assert.equal(report.complete, complete, source);
    assert.deepEqual(report.edits.map(edit => edit.inserted), insertions, source);
    assert.ok(report.edits.every(edit => allow.includes(edit.kind)));
    assertEditAnchors(source, report);
  }
  const missing = recoverJson(shared.missing, { schema: shared.schema, allow: ['closeOpenContainers'] });
  assert.deepEqual(missing.value, { status: 'met' });
  assert.equal(missing.complete, false);
  assert.deepEqual(missing.issues.map(issue => [issue.keyword, issue.pointer]), [['required', '/evidence']]);
  const repeated = recoverJson(shared.repeated);
  assert.equal(repeated.text, shared.repeated);
  assert.equal(repeated.complete, true);
  assert.deepEqual(repeated.edits, []);
  const [issue] = repeated.issues;
  assert.equal(issue.keyword, 'repetition');
  assert.equal(issue.pattern, 'abcde');
  assert.equal(issue.pointer, '/content');
  assert.equal(Buffer.from(shared.repeated).subarray(issue.span.start, issue.span.end).toString(), 'abcde'.repeat(20));
  assert.ok(repeated.sourceNetwork.link(issue.linkId));
  recordIssue195Observations({
    requirementId: 'I195-I241-JSON-RECOVERY', suffix: 'behavior',
    fixtureId: 'i195-i241-json-recovery', fixtureFile,
    assertions: ['permittedRepairsRetainEditAnchors', 'closingRetainsTruncatedStatusAndMissingFields', 'repetitionsRetainSourceSpans'],
    testName: context.name,
  });
});
