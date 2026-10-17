// Input to the linked policy generator and the coordinated schema merge.
// Existing shared entries and their operand order remain unchanged.
export const CALLABLE_SOURCE_POLICY = {
  schemaVersion: 1,
  model: 'lexically owned callable values 1',
  statementRoles: [
    { kinds: ['const', 'let'], role: 'declare' },
    { kinds: ['assign'], role: 'assign' },
    { kinds: ['assignReference'], role: 'reference' },
    { kinds: ['return', 'expr', 'print'], role: 'read' },
    { kinds: ['block'], role: 'block' },
    { kinds: ['if'], role: 'branch' },
    { kinds: ['switch'], role: 'switch' },
    { kinds: ['while', 'doWhile'], role: 'loop' },
    { kinds: ['for'], role: 'for' },
  ],
  statementEvents: ['free name', 'assigned name', 'declaration'],
  parameterPlan: ['parameter identity', 'type identity or negative one', 'declared type flag'],
  ownershipPlan: ['actual lambda identity', 'parameter ordinal', 'actual parameter identity'],
  functionPlan: ['result type identity', 'ordered parameter type identities'],
  constraintPlan: ['ordered left type identity', 'ordered right type identity'],
  typeWhitespaceRanges: [[9, 13], [32, 32], [160, 160], [5760, 5760], [8192, 8202], [8232, 8232], [8233, 8233], [8239, 8239], [8287, 8287], [12288, 12288], [65279, 65279]],
  typeSyntaxPlan: ['form or missing-result diagnostic', 'result UTF16 start', 'result UTF16 end', 'parameter count', 'ordered parameter segment counts and UTF16 slices joined with a colon'],
  expressionProjections: {
    functionRef: { construct: 'functionValue', fields: [['path', 'declaration-path', 'fn']] },
    apply: { construct: 'app', fields: [['fn', 'expression', 'fn'], ['args', 'expressions', 'args']] },
    lambda: { construct: 'lambda', fields: [['params', 'copy', 'params'], ['ret', 'return-type', 'type'], ['body', 'expression', 'body']] },
    cellAllocate: { construct: 'cellAllocate', fields: [['value', 'expression', 'value']] },
    cellRead: { construct: 'cellRead', fields: [['cell', 'expression', 'cell']] },
    cellWrite: { construct: 'cellWrite', fields: [['cell', 'expression', 'cell'], ['value', 'expression', 'value'], ['body', 'expression', 'body']] },
  },
  projectionModes: ['declaration-path', 'return-type'],
};
