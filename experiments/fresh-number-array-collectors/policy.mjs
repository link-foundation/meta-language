// Authoring data for one finite source interpretation, not general Array.push.
export const freshNumberArrayCollectorPolicy = {
  model: 'closed fresh Number array collector 1',
  language: 'JavaScript',
  interpretation: 'ordered primitive Number values under successful allocation',
  roles: ['function', 'input', 'collector', 'index'],
  nameCodeMinimum: 1000,
  maximumTokens: 256,
  maximumSourceUnits: 16384,
  sourceGrammarBasis: 'https://tc39.es/ecma262/multipage/ecmascript-language-lexical-grammar.html#sec-keywords-and-reserved-words',
  // This complete grammar is a non-async, non-generator Script without a
  // strict directive. Contextual await/yield and ordinary globals are not
  // classified as unconditionally reserved by this table.
  unconditionallyReservedBindings: ['break', 'case', 'catch', 'class', 'const',
    'continue', 'debugger', 'default', 'delete', 'do', 'else', 'enum', 'export',
    'extends', 'false', 'finally', 'for', 'function', 'if', 'import', 'in',
    'instanceof', 'new', 'null', 'return', 'super', 'switch', 'this', 'throw',
    'true', 'try', 'typeof', 'var', 'void', 'while', 'with'],
  forbiddenGlobalFunctionNames: ['undefined', 'NaN', 'Infinity'],
  globalFunctionDeclarationBasis: [
    'https://tc39.es/ecma262/multipage/executable-code-and-execution-contexts.html#sec-candeclareglobalfunction',
    'https://tc39.es/ecma262/multipage/global-object.html#sec-value-properties-of-the-global-object',
  ],
  sourceContext: { sourceGoal: 'Script', initialGlobalRealm: 'fresh standard realm',
    initialGlobalValueProperties: 'ECMAScript default property descriptors',
    initialGlobalExtensibility: true, ambientDeclarationChanges: false },
  fixedTokens: ['function', '(', ')', '{', '}', 'const', '=', '[', ']', ';',
    'for', 'let', '0', '1', '<', '.', 'length', '+=', 'push', 'return', '??', '-', 'eof'],
  // Negative entries refer to one actual lexical binder. All four binders
  // must differ; the complete grammar contains no other binding or call.
  prefix: ['function', -1, '(', -2, ')', '{', 'const', -3, '=', '[', ']', ';',
    'for', '(', 'let', -4, '=', '0', ';', -4, '<', -2, '.', 'length', ';',
    -4, '+=', '1', ')', '{', -3, '.', 'push', '('],
  arguments: [
    { id: 'input-or-negative-one', syntax: [-2, '[', -4, ']', '??', '-', '1'] },
    { id: 'input-or-zero', syntax: [-2, '[', -4, ']', '??', '0'] },
    { id: 'previous-collector-length', syntax: [-3, '.', 'length'] },
    { id: 'current-index', syntax: [-4] },
    { id: 'constant-one', syntax: ['1'] },
  ],
  suffix: [')', ';', '}', 'return', -3, ';', '}', 'eof'],
  declarations: {
    function: 1, input: 3, collector: 7, index: 15,
    collectorKeyword: 6, collectorEnd: 11, pushStart: 30, argumentStart: 34,
  },
  assumptions: [
    { id: 'standard-array-intrinsics', role: 'complete source realm',
      meaning: 'The ordinary Array prototype, indexed properties and iterator are standard and unchanged; no proxies, accessors, frozen arrays or inherited indexed setters participate.' },
    { id: 'readonly-number-array-input', role: 'actual input parameter',
      meaning: 'The input is a dense ordinary array of primitive Numbers, read without external mutation or getters for the complete synchronous call.' },
    { id: 'successful-allocation', role: 'actual collector allocation and value-state copies',
      meaning: 'All allocations succeed. Allocation counts, capacity, intermediate object identities, garbage collection and resource-exhaustion behavior are outside this value observation.' },
    { id: 'bounded-array-length', role: 'actual source loop',
      meaning: 'The counter starts at zero and grows by one below the unchanged input length. Exactly one element is appended per iteration, so collector length never exceeds the input array length or 4294967295.' },
  ],
  lowering: {
    role: 'certified collector', bindingForm: 'let', operation: 'append value state',
    argumentEvaluation: 'once before append',
    appendTemplate: '$collector = [...$collector, $argument];',
  },
  exclusions: ['array aliases', 'escaped collectors', 'captured variables', 'input mutation',
    'used push result', 'multiple arguments', 'nested or reentrant calls', 'property receivers',
    'identity observations', 'prototype changes', 'getters', 'imports', 'unknown source tokens'],
};
