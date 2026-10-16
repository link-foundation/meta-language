// One coordinated append plan; none of the raw producer's consumed maps change.
import {CALLABLE_SOURCE_POLICY} from '../../js/src/translation/callable-source-policy.js';
import {SURFACE_REFERENCE_POLICY} from '../../js/src/translation/surface-reference-policy.js';

export const ORIGINAL_LINK_RAW_SCHEMA_EXTENSIONS = {
  expressionOperands: {record: ['entries']},
  expressionFields: {
    record: ['k', 'type', 'span', 'entries'],
    recordField: ['k', 'type', 'span', 'object', 'field', 'index'],
  },
  expressionProjections: {
    ...SURFACE_REFERENCE_POLICY.expressionProjections,
    ...Object.fromEntries(['functionRef', 'apply', 'lambda'].map(kind => [kind, CALLABLE_SOURCE_POLICY.expressionProjections[kind]])),
    record: {construct: 'record', fields: [['entries', 'record-entries', 'entries']]},
    recordField: {construct: 'field', fields: [['object', 'expression', 'object'], ['field', 'copy', 'field']]},
  },
  // Entries retain their labels and original order. Only each actual value is
  // an expression, so its existing lexical binding references remain actual.
  expressionOperandEntries: {record: {entries: 'value'}},
  expressionProjectionAlternatives: SURFACE_REFERENCE_POLICY.expressionProjectionAlternatives,
};

export const ORIGINAL_LINK_RAW_SCHEMA_EXTENSION_SCOPE = 'Append-only host projection and ordered entry traversal source closure only; no native execution, raw producer replacement, heap equivalence or whole-language coverage claim.';
