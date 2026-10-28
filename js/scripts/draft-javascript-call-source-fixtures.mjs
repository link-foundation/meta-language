// Literal original source, inputs and independent expected values only.
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const root=new URL('../../',import.meta.url),number=value=>{const view=new DataView(new ArrayBuffer(8));view.setFloat64(0,value);return {$:'number',bits:view.getBigUint64(0).toString(16).padStart(16,'0')};};
const rows=[
 {name:'actual arithmetic declaration',source:'/** @param {number} value @returns {number} */ export function addOne(value){return value+1;}',entry:'addOne',functions:['addOne'],arguments:[number(40)],expected:number(41)},
 {name:'actual conditional source blocks',source:'/** @param {boolean} flag @param {number} value @returns {number} */ export function choose(flag,value){return flag?value*2:value/2;}',entry:'choose',functions:['choose'],arguments:[{$:'boolean',value:false},number(8)],expected:number(4)},
 {name:'actual nested global calls',source:'/** @param {number} value @returns {number} */ function addOne(value){return value+1;} /** @param {number} value @returns {number} */ export function twice(value){return addOne(addOne(value));}',entry:'twice',functions:['addOne','twice'],arguments:[number(40)],expected:number(42)},
 {name:'actual local binder ownership',source:'/** @param {number} value @returns {number} */ export function local(value){const doubled=value*2;return doubled+value;}',entry:'local',functions:['local'],arguments:[number(4)],expected:number(12)},
 {name:'actual text preserves isolated code units',source:'/** @param {string} value @returns {string} */ export function text(value){return value;}',entry:'text',functions:['text'],arguments:[{$:'text',units:[55296,0,56320]}],expected:{$:'text',units:[55296,0,56320]}},
 {name:'actual arbitrary integer identity',source:'/** @param {bigint} value @returns {bigint} */ export function integer(value){return value;}',entry:'integer',functions:['integer'],arguments:[{$:'integer',value:'184467440737095516160000000000001'}],expected:{$:'integer',value:'184467440737095516160000000000001'}},
 {name:'actual negative zero literal',source:'/** @returns {number} */ export function negativeZero(){return -0;}',entry:'negativeZero',functions:['negativeZero'],arguments:[],expected:number(-0)},
 {name:'actual short circuit boolean source',source:'/** @param {boolean} left @param {boolean} right @returns {boolean} */ export function conjunction(left,right){return left&&right;}',entry:'conjunction',functions:['conjunction'],arguments:[{$:'boolean',value:false},{$:'boolean',value:true}],expected:{$:'boolean',value:false}},
];
const equalitySource='/** @param {number} left @param {number} right @returns {boolean} */ export function equal(left,right){return left===right;}';
rows.push({name:'strict Number equality keeps signed zeros equal',source:equalitySource,entry:'equal',functions:['equal'],arguments:[number(-0),number(0)],expected:{$:'boolean',value:true}},
 {name:'strict Number equality never equates NaN with itself',source:equalitySource,entry:'equal',functions:['equal'],arguments:[number(NaN),number(NaN)],expected:{$:'boolean',value:false}},
 {name:'strict absence comparison keeps exact undefined',source:'/** @param {number | undefined} value @returns {boolean} */ export function absent(value){return value===undefined;}',entry:'absent',functions:['absent'],arguments:[{$:'undefined'}],expected:{$:'boolean',value:true}},
 {name:'strict text equality preserves complete code units',source:'/** @param {string} left @param {string} right @returns {boolean} */ export function equal(left,right){return left===right;}',entry:'equal',functions:['equal'],arguments:[{$:'text',units:[55296]},{$:'text',units:[55296]}],expected:{$:'boolean',value:true}});
const refusals=[
 {name:'structured arrays require ordinary allocation lowering',source:'/** @param {number[]} values @returns {number} */ export function size(values){return values.length;}',functions:['size']},
 {name:'captured function construction requires explicit lexical cells',source:'/** @param {number} value @returns {(other:number)=>number} */ export function closure(value){return other=>value+other;}',functions:['closure']},
];
writeProjectData(new URL('parity/fixtures/javascript-call-source.lino',root),{schemaVersion:1,model:'actual checked JavaScript function source call lowering 1',nodeVersionRange:'>=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',rustVersionRange:'>=1.90.0 <1.100.0',rows,refusals});
