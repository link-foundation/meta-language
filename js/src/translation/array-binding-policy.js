// Authoring input to the shared Links policy. This is the closed first stage:
// homogeneous array suffixes and undefined-only primitive defaults.
export const JAVASCRIPT_ARRAY_BINDING_POLICY = {
  schemaVersion: 1,
  model: 'ordered homogeneous JavaScript array bindings 1',
  sourceLanguage: 'JavaScript',
  reservedBindings: [
    'await', 'break', 'case', 'catch', 'class', 'const', 'continue', 'debugger',
    'default', 'delete', 'do', 'else', 'enum', 'export', 'extends', 'false',
    'finally', 'for', 'function', 'if', 'import', 'in', 'instanceof', 'new',
    'null', 'return', 'super', 'switch', 'this', 'throw', 'true', 'try',
    'typeof', 'var', 'void', 'while', 'with', 'yield', 'let', 'static',
    'implements', 'interface', 'package', 'private', 'protected', 'public',
    'eval', 'arguments',
  ],
  evaluation: ['right hand side once', 'bindings from left to right', 'default only for undefined', 'rest creates one fresh outer array'],
  inferencePlan: ['actual element type', 'actual fallback type', 'result type', 'actual element type'],
  defaultElements: ['float', 'bool', 'string', 'int', 'nat', 'fixed'],
  assumptions: ['existing dense value-array model', 'standard array iteration and slice operations', 'existing reference model for shared elements'],
  refusals: ['nested rest or defaults', 'nullable or structured defaults', 'unknown iterators and property getters'],
};
