// Explicit second-stage packet. Every existing key/value stays unchanged;
// selection never spreads future keys from a live owner policy.
import {CALLABLE_SOURCE_POLICY} from '../../js/src/translation/callable-source-policy.js';

export const ORIGINAL_LINK_RAW_SCHEMA_SECOND_EXTENSIONS = {
  expressionFields: {
    some: ['k', 'value', 'type', 'span'],
    flattenOption: ['k', 'value', 'type', 'span'],
    coalesce: ['k', 'left', 'right', 'type', 'span'],
    optionalField: ['k', 'object', 'field', 'index', 'type', 'span'],
    optionalProject: ['k', 'object', 'field', 'type', 'span'],
  },
  expressionProjections: {
    ...Object.fromEntries(['cellAllocate', 'cellRead', 'cellWrite'].map(kind => [kind, CALLABLE_SOURCE_POLICY.expressionProjections[kind]])),
    some: {construct: 'some', fields: [['value', 'expression', 'value'], ['type', 'copy', 'type']]},
    flattenOption: {construct: 'flattenOption', fields: [['value', 'expression', 'value']]},
    coalesce: {construct: 'coalesce', fields: [['left', 'expression', 'left'], ['right', 'expression', 'right']]},
    optionalField: {construct: 'optionalField', fields: [['object', 'expression', 'object'], ['field', 'copy', 'field']]},
    optionalProject: {construct: 'optionalField', fields: [['object', 'expression', 'object'], ['field', 'copy', 'field']]},
    optionalIndex: {construct: 'index', fields: [['object', 'expression', 'array'], ['index', 'expression', 'index'], ['optional', 'constant', true]]},
    optionalIndexNumber: {construct: 'operand', operand: 'arg', fields: []},
    stringPoints: {construct: 'stringPoints', fields: [['object', 'expression', 'object']]},
    collectionItems: {construct: 'method', fields: [['object', 'expression', 'object'], ['op', 'copy', 'op'], ['args', 'constant', []]]},
    method: {construct: 'method', fields: [['object', 'expression', 'object'], ['op', 'copy', 'op'], ['args', 'nullable-expressions', 'args']]},
    'index:provenInBounds': {construct: 'index', fields: [['object', 'expression', 'array'], ['index', 'expression', 'index'], ['provenInBounds', 'copy', 'provenInBounds']]},
    nullishNumber: {construct: 'numberValue', fields: [['arg', 'expression', 'arg']]},
    optionalText: {variant: ['console']},
    'optionalText:false': {construct: 'toString', fields: [['arg', 'expression', 'arg']]},
    'optionalText:true': {construct: 'show', fields: [['arg', 'expression', 'arg'], ['style', 'constant', 'js-console']]},
  },
  expressionProjectionAlternatives: {
    index: [{path: ['provenInBounds'], value: true, projection: 'index:provenInBounds'}],
  },
  typeChildPlans: {
    union: {sequence: 'alternatives', minimumLength: 2, order: 'source', preserveDuplicates: true},
  },
  declarationTypeProjections: {
    fn: {annotation: 'annotatedRet', result: 'ret', semantic: 'semanticRet'},
  },
};

export const ORIGINAL_LINK_RAW_SCHEMA_SECOND_EXTENSION_SCOPE = 'Second append-only host projection and ordered type-child source closure only; no native execution, raw producer replacement, heap equivalence or whole-language coverage claim.';
