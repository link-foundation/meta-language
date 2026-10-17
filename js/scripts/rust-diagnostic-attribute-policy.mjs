// Shared authoring data. No compiler/edition or source-owner admission is
// inferred from this finite structural token policy.
import assert from 'node:assert/strict';

export const RUST_DIAGNOSTIC_ATTRIBUTE_POLICY = {
  schemaVersion: 1,
  model: 'outer Rust diagnostic attribute token plan 1',
  language: 'Rust',
  facet: {model: 'Rust const diagnostic attributes 1', language: 'Rust', edition: '2024'},
  constKeyword: {kind: 'identifier', value: 'const'},
  emission: {canonicalPublicPrefix: 'pub const fn ', visibilityPrefix: 'pub '},
  normalization: {
    id: 'rust-diagnostic-attribute-normalization',
    statement: 'Rust diagnostic attributes remain typed source facts; this JavaScript output preserves selected runtime predicate observations and does not reproduce Rust compilation diagnostics',
  },
  fields: {
    facet: ['model', 'language', 'edition', 'span', 'raw', 'tokens', 'attachment'],
    token: ['kind', 'value', 'raw', 'span'],
    attachment: ['sourceName', 'modulePath', 'constKeywordSpan', 'functionKeywordSpan'],
    span: ['start', 'end'],
  },
  prefix: [
    {kind: 'punct', value: '#'}, {kind: 'punct', value: '['},
    {kind: 'identifier', value: 'expect'}, {kind: 'punct', value: '('},
    {kind: 'identifier', value: 'clippy'}, {kind: 'punct', value: '::'},
    {kind: 'identifier', value: 'fn_params_excessive_bools'}, {kind: 'punct', value: ','},
    {kind: 'identifier', value: 'reason'}, {kind: 'punct', value: '='},
    {kind: 'string', value: null},
  ],
  optionalFinalComma: {kind: 'punct', value: ','},
  suffix: [{kind: 'punct', value: ')'}, {kind: 'punct', value: ']'}],
  legacyMustUse: [{kind: 'punct', value: '#'}, {kind: 'punct', value: '['}, {kind: 'identifier', value: 'must_use'}, {kind: 'punct', value: ']'}],
  finalVisibility: {kind: 'identifier', value: 'pub'},
  positions: {attributeName: 2, lintNamespace: 4, lintName: 6, reasonKey: 8, reasonLiteral: 10},
  result: ['exclusive token end', 'attribute name token', 'lint namespace token', 'lint name token', 'reason key token', 'reason literal token'],
  refusal: [-1],
  prefixResult: ['exclusive token end', 'attribute count', 'diagnostic attribute count', 'final visibility token or minus one'],
  prefixBoundary: 'The complete pre-const token inventory excludes EOF, starts at the first attribute, and contains ordered legacy must_use or exact expect occurrences followed by at most one final simple pub. At least one expect occurrence is required; must_use-only syntax remains on its unchanged old path.',
  indexDomain: 'Nonnegative safe integral Number; negative zero denotes token zero.',
  columns: 'Equal-length actual Rust tokenizer kind/value columns. Rust token values are primitive Strings. Selected holes and foreign reason payloads refuse. Unselected stream contents do not become an attribute proof.',
  interpretation: 'One exact outer expect attribute, one exact Clippy lint, one mandatory single actual String reason token, and at most one final comma. Comment/trivia and decoded String contents are opaque to structural admission.',
  excludedObservations: ['compiler success', 'compile-time diagnostics', 'lint fulfillment', 'compiler or Clippy identity', 'edition applicability', 'attribute attachment', 'source token authentication', 'source-free owner/span authentication'],
  remainingAuthorities: ['unchanged must_use and old const-profile refusals', 'actual compiler/Clippy/configuration and explicit edition assertion selection', 'actual following const function and owned header', 'strict facet shape and raw token/UTF16 span provenance', 'whole source and original const proof validation'],
};

export function readRustDiagnosticAttributeRuleSource(policy = RUST_DIAGNOSTIC_ATTRIBUTE_POLICY) {
  assert.equal(policy.schemaVersion, 1);
  assert.equal(policy.prefix.filter(token => token.value === null).length, 1);
  assert.equal(policy.prefix[policy.positions.reasonLiteral].kind, 'string');
  assert.equal(policy.prefix[policy.positions.reasonLiteral].value, null);
  const literal = value => JSON.stringify(value);
  const position = offset => offset === 0 ? 'start' : 'start + ' + offset;
  const cells = (token, offset) => {
    const at = position(offset);
    return '(kinds[' + at + '] ?? "") !== ' + literal(token.kind) + ' || '
      + (token.value === null ? 'typeof values[' + at + '] !== "string"' : '(values[' + at + '] ?? "") !== ' + literal(token.value));
  };
  return [
    '// Generated from the shared diagnostic attribute token policy. Do not edit.',
    '/** @param {number} value @returns {boolean} */',
    'export function acceptRustDiagnosticAttributeOffset(value) {',
    '  return Number.isInteger(value) && value >= 0 && value <= 9007199254740991;',
    '}',
    '',
    '/** Actual token start includes #. Result is the six policy-defined token positions; [-1] refuses.',
    ' * @param {string[]} kinds @param {string[]} values @param {number} start @returns {number[]} */',
    'export function planRustDiagnosticAttribute(kinds, values, start) {',
    '  if (kinds.length !== values.length || !acceptRustDiagnosticAttributeOffset(start) || kinds.length - start < ' + (policy.prefix.length + policy.suffix.length) + ') return [-1];',
    ...policy.prefix.map((token, offset) => '  if (' + cells(token, offset) + ') return [-1];'),
    '  let close = start + ' + policy.prefix.length + ';',
    '  if ((kinds[close] ?? "") === ' + literal(policy.optionalFinalComma.kind) + ' && (values[close] ?? "") === ' + literal(policy.optionalFinalComma.value) + ') close += 1;',
    '  if (kinds.length - close < ' + policy.suffix.length + ') return [-1];',
    ...policy.suffix.map((token, offset) => '  if (' + cells(token, offset).replaceAll('start', 'close') + ') return [-1];'),
    '  return [close + ' + policy.suffix.length + ', ' + Object.values(policy.positions).map(position).join(', ') + '];',
    '}',
    '',
    '/** Metadata-only recognition of the unchanged four-token legacy spelling.',
    ' * @param {string[]} kinds @param {string[]} values @param {number} start @returns {number} */',
    'export function readRustMustUseAttributeEnd(kinds, values, start) {',
    '  if (kinds.length !== values.length || !acceptRustDiagnosticAttributeOffset(start) || kinds.length - start < ' + policy.legacyMustUse.length + ') return -1;',
    ...policy.legacyMustUse.map((token, offset) => '  if (' + cells(token, offset) + ') return -1;'),
    '  return start + ' + policy.legacyMustUse.length + ';',
    '}',
    '',
    '/** Complete pre-const token inventory, excluding EOF. The result has four policy-defined cells.',
    ' * @param {string[]} kinds @param {string[]} values @returns {number[]} */',
    'export function planRustDiagnosticAttributePrefix(kinds, values) {',
    '  if (kinds.length !== values.length) return [-1];',
    '  let index = 0;',
    '  let attributes = 0;',
    '  let diagnostics = 0;',
    '  while (index < kinds.length) {',
    '    if (index === kinds.length - 1 && (kinds[index] ?? "") === ' + literal(policy.finalVisibility.kind) + ' && (values[index] ?? "") === ' + literal(policy.finalVisibility.value) + ') {',
    '      return diagnostics > 0 ? [kinds.length, attributes, diagnostics, index] : [-1];',
    '    }',
    '    const legacyEnd = readRustMustUseAttributeEnd(kinds, values, index);',
    '    if (legacyEnd >= 0) {',
    '      index = legacyEnd;',
    '    } else {',
    '      const plan = planRustDiagnosticAttribute(kinds, values, index);',
    '      const next = plan[0] ?? -1;',
    '      if (next <= index) return [-1];',
    '      index = next;',
    '      diagnostics += 1;',
    '    }',
    '    attributes += 1;',
    '  }',
    '  return diagnostics > 0 ? [index, attributes, diagnostics, -1] : [-1];',
    '}',
    '',
  ].join('\n');
}
