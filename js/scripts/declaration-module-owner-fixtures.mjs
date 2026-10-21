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
    {
      id: 'actual-deferred-class-collection-owner', language: 'JavaScript',
      source: 'class Map { constructor(){this.size=17;} }\nfunction selected(){return new Map().size;}\nfunction independent(value){return value+1;}\n',
      functionStatuses: ['carried','translated'], signatures: [],
      calls: ['independent(7)'], rustCalls: ['independent(7f64)'], expectedLines: '8\n', semanticCodecs: false,
      sourceMode: 'Module', sourceProbe: 'selected()===17 && new Map().constructor===Map', sourceProbeExpected: true,
    },
  ]);
}

// Separate additive profile; all original fixture rows and source inputs stay.
export function readDeclarationDeferredModuleFixtures() {
  const first = 'export function first(value){return value+1;}\n';
  const last = 'export function second(value){return first(value);}\nexport function independent(value){return value+1;}\n';
  const opaque = 'class Opaque { constructor(value){this.value=value;} }\n';
  const examples = [
    ['ordinary-arrow-value', 'const halve=(value)=>value/2;\n', 2, 'halve'],
    ['ordinary-function-value', 'const halve=function(value){return value/2;};\n', 2, 'halve'],
    ['ordinary-named-function-value', 'const halve=function inner(value){return value/2;};\n', 2, 'halve'],
    ['deferred-constructor-assignment', 'class Box { constructor(value){this.value=value;} }\n', 1, 'Box'],
    ['deferred-ordinary-method-body', 'class Deferred { constructor(){throw new Error("not called");} method(value){return value;} }\n', 1, 'Deferred'],
  ].map(([id,bindingSource,bindingKind,bindingName])=>({id,language:'JavaScript',sourceMode:'Module',bindingSource,bindingKind,bindingName,
    source:first+bindingSource+last,signatures:['first','second','independent'],functionStatuses:['translated','translated','translated'],
    calls:['first(7)','second(7)','independent(7)'],rustCalls:['first(7f64)','second(7f64)','independent(7f64)'],expectedLines:'8\n8\n8\n'}));
  const refused = [
    ['heritage-evaluation', 'class Deferred extends (first=0,Object) {}\n'],
    ['static-block-evaluation', 'class Deferred { static { first=0; } }\n'],
    ['static-field-evaluation', 'class Deferred { static value=(first=0); }\n'],
    ['computed-method-evaluation', 'class Deferred { [(first=0,"method")](){} }\n'],
    ['instance-field-profile-boundary', 'class Deferred { value=1; }\n'],
    ['static-method-profile-boundary', 'class Deferred { static method(){} }\n'],
    ['binding-list-profile-boundary', 'const halve=(value)=>value/2,other=(value)=>value;\n'],
    ['destructured-binding-profile-boundary', 'const {halve}={halve:0};\n'],
    ['mutable-binding-profile-boundary', 'let halve=(value)=>value/2;\n'],
    ['var-binding-profile-boundary', 'var halve=(value)=>value/2;\n'],
    ['module-assignment', 'first=0;\n'],
    ['module-eval', 'eval("first=0");\n'],
    ['eager-function-call-initializer', 'const halve=(()=>{first=0;return 1;})();\n'],
  ].map(([id,bindingSource])=>({id,language:'JavaScript',sourceMode:'Module',bindingSource,bindingKind:0,bindingName:'',
    source:first+opaque+bindingSource+last,signatures:[],functionStatuses:['translated','carried','translated'],
    calls:['independent(7)'],rustCalls:['independent(7f64)'],expectedLines:'8\n'}));
  return examples.concat(refused);
}
