// Declaration author only: no parsing, product execution or observations.
import { writeProjectData } from './project-data.mjs';
if (process.argv.slice(2).join(' ') !== '--draft') throw new TypeError('use --draft');
const values = [
  ['original projection identifiers', 'const ml_reference_array1=3,ml_reference_position1=4,ml_reference_projection1=5;const values=[6];values.push(7);return (values[0]??0)+ml_reference_array1+ml_reference_position1+ml_reference_projection1;', '18'],
  ['old local refusal input', 'let ml_x=17;return ml_x;', '17'],
  ['private namespace gaps', 'const ml_source1_temporary_result1=2,ml_source3_temporary_result1=7;let result=0;for(let index=0;index<3;index+=1){result+=index;}return result+ml_source1_temporary_result1+ml_source3_temporary_result1;', '12'],
  ['contiguous private namespaces', 'const ml_source1_a1=2,ml_source2_a1=3,ml_source3_a1=4;const values=[5];values.push(6);return (values[0]??0)+ml_source1_a1+ml_source2_a1+ml_source3_a1;', '14'],
  ['target sanitizer collision', 'const mlReferenceArray1=2,ml_reference_array1=3,source_ml_reference_array1=4;const values=[5];values.push(6);return (values[0]??0)+mlReferenceArray1+ml_reference_array1+source_ml_reference_array1;', '14'],
  ['mutable captured local', 'let ml_cell_value=1;const increment=()=>{ml_cell_value+=1;return ml_cell_value;};const ml_temporary_result1=4;return increment()+increment()+ml_temporary_result1;', '9'],
  ['nested lexical shadowing', 'let ml_value=3;{let ml_value=7;ml_value+=1;}return ml_value;', '3'],
  ['private loop binder', 'let ml_total=0;for(let ml_index=0;ml_index<4;ml_index+=1){ml_total+=ml_index;}return ml_total;', '6'],
  ['escaped private identifier', String.raw`const m\u006c_a1=5;return ml_a1;`, '5'],
  ['comment namespace facts', '/* ml_source1_temporary_result1 */ const ml_value=3;let result=0;for(let index=0;index<2;index+=1){result+=ml_value;}return result;', '6'],
  ['private array defaults', 'const [ml_first=0,ml_second=0]=[3,4];return ml_first+ml_second;', '7'],
  ['private array rest', 'const [ml_first=0,...ml_tail]=[1,2,3];return ml_first+ml_tail.length;', '3'],
  ['private mutable array binding', 'let [ml_value=0]=[3];ml_value+=1;return ml_value;', '4'],
  ['private object binding', 'const {length:ml_length}=[1,2];return ml_length;', '2'],
  ['prefix decimal spelling', 'const ml_source01_a1=3,ml_source9007199254740991_a1=4,ml_source9007199254740992_a1=5;return ml_source01_a1+ml_source9007199254740991_a1+ml_source9007199254740992_a1;', '12'],
  ['source and escaped target spelling', 'const ml_math=3,source_ml_math=4;return ml_math+source_ml_math+Math.floor(1.9);', '8'],
];
const cases = values.map(([name, body, value], index) => ({ name, source: 'export function compute(){'+body+'}', entry: 'sourceIdentifierCase'+index, value, abort:null }));
cases.push({ name:'private parameter and capture', source:'/** @param {number} ml_value @returns {number} */ function readPrivateParameter(ml_value){const captured=()=>ml_value+1;return captured();}\nexport function compute(){return readPrivateParameter(4);}', entry:'sourceIdentifierParameter', value:'5', abort:null });
cases.push({ name:'private function and mapped collision', source:'/** @param {number} value @returns {number} */ function ml_array(value){return value+1;}\n/** @param {number} value @returns {number} */ function source_ml_array(value){return value+2;}\nexport function compute(){return ml_array(3)+source_ml_array(3);}', entry:'sourceIdentifierDeclaration', value:'9', abort:null });
cases.push({ name:'mutable parameter cell name', source:'/** @param {number} value @returns {number} */ function readPrivateCellParameter(value){const ml_cell_value=4;const increment=()=>{value+=1;return value;};return increment()+increment()+ml_cell_value;}\nexport function compute(){return readPrivateCellParameter(1);}', entry:'sourceIdentifierCellParameter', value:'9', abort:null });
cases.push({ name:'private required record parameter', source:'function readPrivateRecordParameter({length:ml_length}){return ml_length+1;}\nexport function compute(){return readPrivateRecordParameter([3,4]);}', entry:'sourceIdentifierRecordParameter', value:'3', abort:null });
writeProjectData(new URL('../../parity/fixtures/source-identifiers.lino', import.meta.url), {
  schemaVersion:1, model:'capture free source generated and target identifier namespaces 1', sourceLanguage:'JavaScript', cases, refusals:[],
  source:cases.map(row=>row.source.replace('function compute(', 'function '+row.entry+'(')).join('\n'),
  versions:{JavaScript:'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0; ECMAScript modules',Rust:'>=1.90.0 <1.100.0; edition 2024'},
  sourceBasis:'https://tc39.es/ecma262/2025/multipage/ecmascript-language-lexical-grammar.html#sec-names-and-keywords ; ml_ is an ordinary identifier prefix, not an ECMAScript reserved word. Compiler namespaces require capture-free allocation.',
  historicalAssertions:[
    {source:'function f(n) { let ml_x = n; return ml_x; }',message:"reserved identifier: ml_x uses the translator's reserved ml_ prefix at 20..24",correction:'retain the complete input; compare f(17) with independent Node and emitted runtimes'},
    {source:'function probe(ml_source_name){return ml_source_name;}',message:'reserved identifier: ml_source_name',correction:'retain the complete input; compare probe(19) with independent Node and emitted runtimes'},
  ],
  boundary:'Future independent original Node and emitted JavaScript/native Rust values; exact paired definitions/programs, both codecs and all four emissions. No observations. Existing closed pattern and scalar-loop profiles remain separate.'
});
