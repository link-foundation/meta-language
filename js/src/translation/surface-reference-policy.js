// Additive shared projection data. Existing reference operand names already
// belong to the original schema and remain unchanged.
export const SURFACE_REFERENCE_POLICY = {
  expressionProjections: {
    referenceAllocate: { construct: 'operand', operand: 'value', fields: [] },
    referenceRead: { construct: 'operand', operand: 'reference', fields: [] },
    'length:referenceRead': { construct: 'field', fields: [['object', 'expression', 'array'], ['field', 'constant', 'length']] },
  },
  expressionProjectionAlternatives: {
    length: [{ path: ['array', 'k'], value: 'referenceRead', projection: 'length:referenceRead' }],
  },
  projectionModes: ['operand'],
};
