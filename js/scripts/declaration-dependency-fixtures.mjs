// Complete module inputs and independent expected observations. These are
// ordinary original fixtures, never substitutes for an unsupported body.
export const recursivePairSource = `/** @param {number} value @returns {boolean} */
export function isEven(value) { if (value <= 0) return true; return isOdd(value - 1); }
/** @param {number} value @returns {boolean} */
function isOdd(value) { if (value <= 0) return false; return isEven(value - 1); }
/** @param {number} value @returns {boolean} */
export function choose(value) { return isEven(value); }
`;
export const recursivePairCalls = ['isEven(-1)', 'isEven(0)', 'isEven(1)', 'isEven(6)', 'isOdd(0)', 'isOdd(5)', 'choose(7)'];
export const recursivePairExpected = 'true|true|false|true|false|true|false';

export function readDeclarationDependencyModuleFixtures() {
  const originalOdd = '/** @param {number} value @returns {boolean} */\nfunction isOdd';
  if (recursivePairSource.split(originalOdd).length !== 2) throw new Error('one complete original odd declaration boundary');
  const mixed = recursivePairSource.replace(originalOdd, 'function unavailable() { return new Date(); }\n' + originalOdd)
    + 'export function independent() { return 17; }\nexport function blocked() { return unavailable(); }\n';
  const failed = second => 'export function first(value) { if(value <= 0) return true; return second(value - 1); }\n'
    + 'function second(value) { ' + second + ' }\n'
    + 'export function caller(value) { return first(value); }\n'
    + 'export function independent() { return 23; }\n';
  return [
    { id: 'complete-pair', language: 'JavaScript', source: recursivePairSource,
      functionStatuses: ['translated', 'translated', 'translated'], signatures: ['isEven', 'choose'],
      calls: recursivePairCalls, rustCalls: ['is_even(-1f64)', 'is_even(0f64)', 'is_even(1f64)', 'is_even(6f64)', 'is_odd(0f64)', 'is_odd(5f64)', 'choose(7f64)'],
      expectedLines: 'true\ntrue\nfalse\ntrue\nfalse\ntrue\nfalse\n', semanticCodecs: true },
    { id: 'noncontiguous-pair-with-unavailable-owner', language: 'JavaScript', source: mixed,
      functionStatuses: ['translated', 'carried', 'translated', 'translated', 'translated', 'carried'], signatures: ['isEven', 'choose', 'independent'],
      calls: recursivePairCalls, rustCalls: ['is_even(-1f64)', 'is_even(0f64)', 'is_even(1f64)', 'is_even(6f64)', 'is_odd(0f64)', 'is_odd(5f64)', 'choose(7f64)'],
      expectedLines: 'true\ntrue\nfalse\ntrue\nfalse\ntrue\nfalse\n', semanticCodecs: false },
    { id: 'failed-unsupported-cycle', language: 'JavaScript', source: failed('if(value <= 0) return new Date(); return first(value - 1);'),
      functionStatuses: ['carried', 'carried', 'carried', 'translated'], signatures: ['independent'],
      calls: ['independent()'], rustCalls: ['independent()'], expectedLines: '23\n', semanticCodecs: false },
    { id: 'failed-incompatible-cycle', language: 'JavaScript', source: failed('if(value <= 0) return "wrong"; return first(value - 1);'),
      functionStatuses: ['carried', 'carried', 'carried', 'translated'], signatures: ['independent'],
      calls: ['independent()'], rustCalls: ['independent()'], expectedLines: '23\n', semanticCodecs: false },
    { id: 'failed-conservative-candidate-preserves-independent-proof', language: 'JavaScript',
      source: 'export function first(second) { return second; }\nfunction second() { return first(new Date()); }\n',
      functionStatuses: ['translated', 'carried'], signatures: ['first'], calls: ['first(11)'], rustCalls: ['first(11f64)'], expectedLines: '11\n', semanticCodecs: false },
    { id: 'three-member-number-cycle', language: 'JavaScript', source:
      'export function alpha(value) { if(value <= 0) return 0; return beta(value - 1); }\n'
      + 'function unavailable() { return new Date(); }\n'
      + 'function beta(value) { if(value <= 0) return 1; return gamma(value - 1); }\n'
      + 'function gamma(value) { if(value <= 0) return 2; return alpha(value - 1); }\n',
      functionStatuses: ['translated', 'carried', 'translated', 'translated'], signatures: ['alpha'],
      calls: ['alpha(0)', 'alpha(1)', 'alpha(2)', 'alpha(3)', 'beta(3)', 'gamma(4)'],
      rustCalls: ['alpha(0f64)', 'alpha(1f64)', 'alpha(2f64)', 'alpha(3f64)', 'beta(3f64)', 'gamma(4f64)'],
      expectedLines: '0\n1\n2\n0\n1\n0\n', semanticCodecs: false },
    ...['between', 'after'].map(position => ({ id: position + '-var-rebinding-denies-recursive-owner', language: 'JavaScript',
      source: 'function first(value) { if(value <= 0) return true; return second(value - 1); }\n'
        + (position === 'between' ? 'var first = 0;\n' : '')
        + 'function second(value) { return first(value); }\n'
        + (position === 'after' ? 'var first = 0;\n' : '')
        + 'function independent() { return 31; }\n',
      functionStatuses: ['carried', 'carried', 'translated'], signatures: [],
      calls: ['independent()'], rustCalls: ['independent()'], expectedLines: '31\n', semanticCodecs: false,
      sourceMode: 'Script', sourceOwnerValue: 0, sourceOwnerCallError: 'TypeError' })),
  ];
}

export const ordinaryModuleOwnerSource = 'function first(value) { return value; }\nvar first = 0;\nfunction second(value) { return first(value); }\n';

// Additional complete originals; the preceding eight fixtures stay unchanged.
export function readOrdinaryDeclarationModuleFixtures() {
  const hazards=[];
  for(const kind of ['var','assignment','eval']) for(const position of ['between','after']) {
    const script=kind==='var',prefix=script?'':'export ';
    const effect=kind==='var'?'var first = 0;\n':kind==='assignment'?'first = 0;\n':'eval("first = 0");\n';
    const first=prefix+'function first(value) { return value + 1; }\n';
    const second=prefix+'function second(value) { return first(value); }\n';
    hazards.push({id:'acyclic-'+kind+'-'+position,language:'JavaScript',sourceMode:script?'Script':'Module',
      source:first+(position==='between'?effect:'')+second+(position==='after'?effect:'')+prefix+'function independent() { return 31; }\n',
      functionStatuses:['translated','carried','translated'],signatures:[],
      sourceOwnerValue:0,sourceOwnerCallError:'TypeError',
      calls:['independent()'],rustCalls:['independent()'],expectedLines:'31\n'});
  }
  return [...hazards,
    {id:'safe-ordinary-named-declarations',language:'JavaScript',sourceMode:'Module',
      source:'export function first(value) { return value + 1; }\nexport function second(value) { return first(value); }\n',
      functionStatuses:['translated','translated'],signatures:['first','second'],
      calls:['second(3)'],rustCalls:['second(3f64)'],expectedLines:'4\n'},
    {id:'safe-certified-literal-and-declarations',language:'JavaScript',sourceMode:'Module',
      source:'const OFFSET = 2;\nexport function first(value) { return value + OFFSET; }\nexport function second(value) { return first(value); }\n',
      functionStatuses:['translated','translated'],signatures:['first','second'],
      calls:['second(3)'],rustCalls:['second(3f64)'],expectedLines:'5\n'},
    {id:'retained-count-limit-and-empty-class-oracle',language:'JavaScript',sourceMode:'Module',
      source:'/** @returns {number} */\nexport function count() { return 2; }\nexport const limit = 3;\nexport class Unsupported {}\n',
      functionStatuses:['translated','translated','carried'],signatures:['count','limit'],
      calls:['count()','limit'],rustCalls:['count()','limit'],expectedLines:'2\n3\n'},
  ];
}
