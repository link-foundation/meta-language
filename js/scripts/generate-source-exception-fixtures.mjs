// Declaration author. No constructor, conversion or fixture source runs here.
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const rows=[
 ['generic computed message','throw new Error("value "+String(7));','Error','value 7',true],
 ['TypeError literal','throw new TypeError("bad input");','TypeError','bad input',true],
 ['TypeError call without new','throw TypeError("bad input");','TypeError','bad input',true],
 ['Error call without new absent message','throw Error();','Error','',false],
 ['RangeError Number message','throw new RangeError(7);','RangeError','7',true],
 ['SyntaxError null message','throw new SyntaxError(null);','SyntaxError','null',true],
 ['ReferenceError undefined message','throw new ReferenceError(undefined);','ReferenceError','',false],
 ['URIError absent message','throw new URIError();','URIError','',false],
 ['EvalError boolean message','throw new EvalError(true);','EvalError','true',true],
 ['generic absent message','throw new Error();','Error','',false],
 ['generic explicit empty message','throw new Error("");','Error','',true],
 ['generic undefined message','throw new Error(undefined);','Error','',false],
 ['BigInt error message','throw new TypeError(9007199254740993n);','TypeError','9007199254740993',true],
 ['Unicode error message','throw new TypeError("bad 🌍");','TypeError','bad 🌍',true],
 ['error argument capture precedes extras','let value=1;throw new TypeError(value,value=7);','TypeError','1',true],
 ['error ignored extra arguments evaluate','let value=1;throw new TypeError("bad",undefined,value=7);','TypeError','bad',true],
];
const cases=rows.map(([name,body,errorClass,abort,messagePresent],index)=>({name,source:'export function compute(){'+body+'}',entry:'sourceExceptionCase'+index,value:null,abort,errorClass,messagePresent,messageUnits:Array.from({length:abort.length},(_,at)=>abort.charCodeAt(at))}));
cases.push({name:'computed captured message retains closure names',source:'export function compute(){const value=7;const fail=()=>{throw new TypeError("captured "+String(value));};return fail();}',entry:'sourceExceptionCapture',value:null,abort:'captured 7',errorClass:'TypeError',messagePresent:true,messageUnits:[99,97,112,116,117,114,101,100,32,55]});
cases.push({name:'optional message retains absence property',source:'/** @param {string | undefined} value @returns {number} */ function failOptionalMessage(value){throw new TypeError(value);}\nexport function compute(){return failOptionalMessage(undefined);}',entry:'sourceExceptionOptional',value:null,abort:'',errorClass:'TypeError',messagePresent:false,messageUnits:[]});
cases.push({name:'optional null message has own property',source:'/** @param {string | null} value @returns {number} */ function failOptionalNullMessage(value){throw new TypeError(value);}\nexport function compute(){return failOptionalNullMessage(null);}',entry:'sourceExceptionOptionalNull',value:null,abort:'null',errorClass:'TypeError',messagePresent:true,messageUnits:[110,117,108,108]});
cases.push({name:'later exception argument error wins',source:'/** @returns {number} */ function failLaterExceptionArgument(){throw new RangeError("later argument");}\nexport function compute(){throw new TypeError("first",failLaterExceptionArgument());}',entry:'sourceExceptionLaterArgument',value:null,abort:'later argument',errorClass:'RangeError',messagePresent:true,messageUnits:[108,97,116,101,114,32,97,114,103,117,109,101,110,116]});
for(const [index,errorClass] of ['Error','TypeError','RangeError','SyntaxError','ReferenceError','URIError','EvalError'].entries())cases.push({
 name:errorClass+' retains absent message through nested calls and private-name collisions',
 source:'/** @returns {number} */ function ml_io'+(index+1)+'(){throw '+errorClass+'();}\n/** @param {number} ml_out @returns {number} */ function ml_source'+(index+1)+'_forward(ml_out){const ml_o1=ml_out+1;return ml_io'+(index+1)+'()+ml_o1;}\nexport function compute(){return ml_source'+(index+1)+'_forward(7);}',
 entry:'sourceExceptionNested'+index,value:null,abort:'',errorClass,messagePresent:false,messageUnits:[],
});
cases.push({name:'implicit BigInt conversion retains TypeError through nested calls',source:'/** @returns {number} */ function sourceExceptionImplicit(){return +1n;}\nexport function compute(){return sourceExceptionImplicit();}',entry:'sourceExceptionImplicitForward',value:null,abort:'Cannot convert a BigInt value to a number',errorClass:'TypeError',messagePresent:true,messageUnits:Array.from('Cannot convert a BigInt value to a number',character=>character.charCodeAt(0))});
writeProjectData(new URL('../../parity/fixtures/source-exceptions.lino',import.meta.url),{
 schemaVersion:1,model:'JavaScript exception class and message intrinsic operations 1',sourceLanguage:'JavaScript',cases,refusals:[],
 source:cases.map(row=>row.source.replace('function compute(','function '+row.entry+'(')).join('\n'),
 versions:{JavaScript:'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0; ECMAScript 2025',Rust:'>=1.90.0 <1.100.0; edition 2024'},
 sourceBasis:'https://tc39.es/ecma262/2025/multipage/fundamental-objects.html#sec-error-message ; https://tc39.es/ecma262/2025/multipage/fundamental-objects.html#sec-nativeerror',
 boundary:'Future independent original Node, both runtimes, exact class/message units/property-presence, nested forwarding, generated-name collisions, structured formal entry and complete program/definition codecs/emissions. No observations. Cause objects, arbitrary thrown values, catch/finally, source arithmetic errors and uncaught process/stack behavior remain obligations.',
});
