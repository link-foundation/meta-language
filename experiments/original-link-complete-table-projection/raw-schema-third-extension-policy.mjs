// Exact third-stage append. All published predecessor data remains immutable.
// The alternatives compare their literal Boolean values without converting
// them to strings. The empty root deliberately supplies no fallback construct.
export const ORIGINAL_LINK_RAW_SCHEMA_THIRD_EXTENSIONS = {
  expressionProjections: {
    strictEqual: {},
    'strictEqual:false': {
      construct: 'binary',
      fields: [['op', 'constant', 'eq'], ['left', 'expression', 'left'], ['right', 'expression', 'right']],
    },
    'strictEqual:true': {
      construct: 'binary',
      fields: [['op', 'constant', 'ne'], ['left', 'expression', 'left'], ['right', 'expression', 'right']],
    },
  },
  expressionProjectionAlternatives: {
    strictEqual: [
      { path: ['negate'], value: false, projection: 'strictEqual:false' },
      { path: ['negate'], value: true, projection: 'strictEqual:true' },
    ],
  },
};

export const ORIGINAL_LINK_RAW_SCHEMA_THIRD_EXTENSION_SCOPE = 'Third append-only strict equality projection source closure only; no native execution, raw producer replacement or whole-language coverage claim.';
