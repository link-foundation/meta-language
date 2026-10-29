// Write independent literal source inputs only; never parse or execute them.
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
writeProjectData(new URL('../../parity/fixtures/source-library-constant-functions.lino',import.meta.url),{
 schemaVersion:1,model:'actual source module function bindings 1',
 versions:{node:'>=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',rust:'>=1.90.0 <1.100.0; edition 2024'},
 policies:[
  {keyword:'function',kind:'identifier',name:'apply',callable:false,binding:'function',exported:true},
  {keyword:'const',kind:'identifier',name:'apply',callable:true,binding:'const',exported:true},
  {keyword:'const',kind:'identifier',name:'amount',callable:false,binding:'',exported:false},
  {keyword:'let',kind:'identifier',name:'apply',callable:true,binding:'',exported:false},
  {keyword:'function',kind:'string',name:'apply',callable:true,binding:'',exported:false},
  {keyword:'function',kind:'identifier',name:'',callable:true,binding:'',exported:false}
 ],
 facts:[
  {name:'constant arrow exports retain actual const bindings',source:'export const apply=value=>value;',exports:['apply'],bindings:[{name:'apply',kind:'const',initializer:'arrow'}]},
  {name:'ordinary functions retain their existing declaration form',source:'export function apply(value){return value;}',exports:['apply'],bindings:[{name:'apply',kind:'function'}]},
  {name:'anonymous function expressions retain their actual outer binding',source:'export const apply=function(value){return value;};',exports:['apply'],bindings:[{name:'apply',kind:'const',initializer:'function expression'}]},
  {name:'named function expressions are not module declarations',source:'export const apply=function apply(value){return value;};',exports:['apply'],bindings:[{name:'apply',kind:'const',initializer:'function expression'}]},
  {name:'functions inside default parameter groups remain nested',source:'export function apply(read=function nested(){return 1;}){return read();}',exports:['apply'],bindings:[{name:'apply',kind:'function'}]},
  {name:'source-like literal text does not declare functions',source:'const words=["function phantom(){}","const fake=()=>1"];export function apply(){return 1;}',exports:['apply'],bindings:[{name:'apply',kind:'function'}]},
  {name:'scalar constant exports retain their complete separate obligation',source:'export const amount=3;',error:true},
  {name:'duplicate actual module declarations refuse before renaming',source:'export const apply=value=>value;function apply(value){return value;}',error:true}
 ],
 modules:[
  {name:'standalone exported constant arrow',entry:'entry.js',aliases:false,arguments:[4],expected:5,files:[{path:'entry.js',source:'/** @param {number} value @returns {number} */ export const run=value=>value+1;'}]},
  {name:'standalone constant anonymous function',entry:'entry.js',aliases:false,arguments:[4],expected:4,files:[{path:'entry.js',source:'/** @param {number} value @returns {number} */ export const run=function(value){return value;};'}]},
  {name:'standalone same-name function expression',entry:'entry.js',aliases:false,arguments:[4],expected:4,files:[{path:'entry.js',source:'/** @param {number} value @returns {number} */ export const run=function run(value){return value;};'}]},
  {name:'constant arrow import alias preserves the actual provider',entry:'entry.js',aliases:true,arguments:[4],expected:8,files:[
   {path:'provider.js',source:'/** @param {number} value @returns {number} */ export const double=value=>value*2;'},
   {path:'entry.js',source:'import {double as twice} from "./provider.js";/** @param {number} value @returns {number} */ export function run(value){return twice(value);}'}]},
  {name:'duplicate private constant helpers retain independent owners',entry:'entry.js',aliases:true,arguments:[4],expected:11,files:[
   {path:'left.js',source:'/** @param {number} value @returns {number} */ const increment=value=>value+1;/** @param {number} value @returns {number} */ export const left=value=>increment(value);'},
   {path:'right.js',source:'/** @param {number} value @returns {number} */ const increment=value=>value+2;/** @param {number} value @returns {number} */ export const right=value=>increment(value);'},
   {path:'entry.js',source:'import {left} from "./left.js";import {right} from "./right.js";/** @param {number} value @returns {number} */ export function run(value){return left(value)+right(value);}'}]},
  {name:'function-expression aliases retain their inner-scope obligation',entry:'entry.js',aliases:true,arguments:[4],expected:4,error:'separate inner binding scopes',files:[
   {path:'provider.js',source:'/** @param {number} value @returns {number} */ export const identity=function identity(value){return value;};'},
   {path:'entry.js',source:'import {identity as original} from "./provider.js";/** @param {number} value @returns {number} */ export function run(value){return original(value);}'}]}
 ],
 boundary:'Unexecuted literal controls. Actual module bindings and all source bytes remain explicit. Function-only libraries, alias scopes, complete target/native policy behavior and semantic translations retain their independent controls. General constant data/effects, function-expression alias scopes and full JavaScript modules remain required.'});
