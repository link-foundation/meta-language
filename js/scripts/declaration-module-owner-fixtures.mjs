// Complete original programs shared by JavaScript checks and native CI.
// Authored fixture construction only; translation never rewrites these bodies.
export const declarationSpecialGlobalInventory = [
  'Map', 'Set', 'console', 'Math', 'Number', 'Array', 'Object', 'String',
  'Error', 'RangeError', 'TypeError', 'NaN', 'Infinity', 'undefined', 'isFinite', 'isNaN',
];

export function readDeclarationSpecialGlobalSources() {
  return [
    ['Map', 'function selected(){return new Map([[1,2]]).size;}'],
    ['Set', 'function selected(){return new Set([1,2]).size;}'],
    ['console', 'function selected(){console.log(1);return 2;}'],
    ['Math', 'function selected(){return Math.floor(2.5);}'],
    ['Number', 'function selected(){return Number.isFinite(2.5);}'],
    ['Array', 'function selected(){return Array.isArray([1]);}'],
    ['Object', 'function selected(){return Object.freeze([1,true]);}'],
    ['String', 'function selected(){return String(2.5);}'],
    ['Error', '/** @returns {number} */function selected(){throw new Error("stop");}'],
    ['RangeError', '/** @returns {number} */function selected(){throw new RangeError("stop");}'],
    ['TypeError', '/** @returns {number} */function selected(){throw new TypeError("stop");}'],
    ['NaN', 'function selected(){return NaN;}'],
    ['Infinity', 'function selected(){return Infinity;}'],
    ['undefined', 'function selected(){return undefined;}'],
    ['isFinite', 'function selected(){return isFinite(2.5);}'],
    ['isNaN', 'function selected(){return isNaN(2.5);}'],
  ];
}

export function readDeclarationModuleOwnerFixtures() {
  const independent = '\nfunction independent(value){return value+1;}\n';
  const examples = readDeclarationSpecialGlobalSources().map(([name, selected]) => {
    const source = `class ${name} { /* inert actual outer owner */ }\n` + selected + independent;
    const numeric = ['NaN', 'Infinity', 'undefined'].includes(name);
    const error = ['Error', 'RangeError', 'TypeError'].includes(name);
    const collection = name === 'Map' || name === 'Set';
    const sourceProbe = collection ? `selected().constructor===${name}`
      : numeric ? `selected()===${name}`
      : error ? `(()=>{try{selected();return false;}catch(error){return error.constructor===${name};}})()` : 'selected()';
    // Collection .size is absent on the actual empty class. The other
    // callable/properties of an empty class are not the standard intrinsics.
    const sourceProbeValue = collection ? `new ${name}().constructor===${name} && selected()===undefined` : sourceProbe;
    return {
      id: 'actual-empty-class-special-owner-' + name, language: 'JavaScript', source,
      functionStatuses: ['carried', 'translated'], signatures: [],
      calls: ['independent(7)'], rustCalls: ['independent(7f64)'], expectedLines: '8\n',
      semanticCodecs: false, sourceMode: 'Module',
      sourceProbe: sourceProbeValue,
      ...(numeric || error || collection ? { sourceProbeExpected: true } : { sourceProbeError: 'TypeError' }),
    };
  });
  return examples.concat([
    {
      id: 'unknown-var-constructor-owner', language: 'JavaScript',
      source: 'var Map = class {};\nfunction selected(){return new Map();}' + independent,
      functionStatuses: ['carried', 'translated'], signatures: [],
      calls: ['independent(7)'], rustCalls: ['independent(7f64)'], expectedLines: '8\n', semanticCodecs: false,
      sourceMode: 'Script', sourceProbe: 'selected().constructor===Map', sourceProbeExpected: true,
    },
    {
      id: 'unknown-var-scope-keeps-actual-parameter', language: 'JavaScript',
      source: 'var Map = class {};\n/** @param {number} Map @returns {number} */\nfunction selected(Map){return Map+1;}' + independent,
      functionStatuses: ['translated', 'translated'], signatures: [],
      calls: ['selected(7)', 'independent(7)'], rustCalls: ['selected(7f64)', 'independent(7f64)'], expectedLines: '8\n8\n', semanticCodecs: false,
      sourceMode: 'Script', sourceProbe: 'selected(7)===8', sourceProbeExpected: true,
    },
    {
      id: 'empty-class-keeps-actual-parameter', language: 'JavaScript',
      source: 'class String {}\n/** @param {number} String @returns {number} */\nfunction selected(String){return String+1;}' + independent,
      functionStatuses: ['translated', 'translated'], signatures: [],
      calls: ['selected(7)', 'independent(7)'], rustCalls: ['selected(7f64)', 'independent(7f64)'], expectedLines: '8\n8\n', semanticCodecs: false,
      sourceMode: 'Module', sourceProbe: 'selected(7)===8', sourceProbeExpected: true,
    },
  ]);
}
