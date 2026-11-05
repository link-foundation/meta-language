// Automatic source declarations from existing exact conversion inputs.
import {readProjectData,writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const inputs=readProjectData(new URL('../../parity/fixtures/big-integer-conversion.lino',import.meta.url));
const units=text=>Array.from({length:text.length},(_,index)=>text.charCodeAt(index));
const cases=inputs.cases.map((row,index)=>{
 const text=row.kind==='string'?String.fromCharCode(...row.units):row.spelling;
 const input=row.kind==='string'?JSON.stringify(text):row.spelling;
 const error=row.expected.kind==='error';
 const message=error?row.expected.prefix+text+row.expected.suffix:null;
 return{name:row.name,source:'export function compute(){return BigInt('+input+');}',entry:'bigIntegerSourceCase'+index,
  value:error?null:row.expected.value,abort:message===null?null:message.toWellFormed(),
  ...(error?{errorClass:row.expected.errorClass,messagePresent:true,messageUnits:units(message)}:{})};
});
for(const [name,body,value] of [
 ['Boolean true','return BigInt(true);','1'],['Boolean false','return BigInt(false);','0'],
 ['BigInt identity','return BigInt(18446744073709551615n);','18446744073709551615'],
 ['BigInt arithmetic after conversion','return BigInt("12")*BigInt("7");','84'],
 ['first argument capture precedes extra effects','let value=7;const big=BigInt(value,value=9);return big*10n+BigInt(value);','79'],
 ['ignored object argument','return BigInt("7",{value:1});','7'],
 ['round trip to Number','return Number(BigInt("9007199254740993"));','9007199254740992'],
])cases.push({name,source:'export function compute(){'+body+'}',entry:'bigIntegerSourceExtra'+cases.length,value,abort:null});
for(const [name,body,message] of [
 ['missing argument','return BigInt();','Cannot convert undefined to a BigInt'],
 ['undefined argument','return BigInt(undefined);','Cannot convert undefined to a BigInt'],
 ['null argument','return BigInt(null);','Cannot convert null to a BigInt'],
])cases.push({name,source:'export function compute(){'+body+'}',entry:'bigIntegerSourceError'+cases.length,value:null,abort:message,errorClass:'TypeError',messagePresent:true,messageUnits:units(message)});
cases.push({name:'extra argument abort precedes conversion',source:'/** @returns {number} */ function failBigIntegerExtraArgument(){throw new TypeError("BigInt extra");}\nexport function compute(){return BigInt("invalid",failBigIntegerExtraArgument());}',entry:'bigIntegerSourceExtraAbort',value:null,abort:'BigInt extra',errorClass:'TypeError',messagePresent:true,messageUnits:units('BigInt extra')});
cases.push({name:'optional primitive present value',source:'/** @param {string | null} value @returns {bigint} */ function parseOptionalBigIntegerText(value){return BigInt(value);}\nexport function compute(){return parseOptionalBigIntegerText("42");}',entry:'bigIntegerSourceOptional',value:'42',abort:null});
cases.push({name:'optional primitive undefined error',source:'/** @param {string | undefined} value @returns {bigint} */ function parseOptionalUndefinedBigIntegerText(value){return BigInt(value);}\nexport function compute(){return parseOptionalUndefinedBigIntegerText(undefined);}',entry:'bigIntegerSourceOptionalError',value:null,abort:'Cannot convert undefined to a BigInt',errorClass:'TypeError',messagePresent:true,messageUnits:units('Cannot convert undefined to a BigInt')});
writeProjectData(new URL('../../parity/fixtures/big-integer-source.lino',import.meta.url),{
 schemaVersion:1,model:'exact JavaScript source BigInt intrinsic lowering 1',sourceLanguage:'JavaScript',cases,refusals:[],
 source:cases.map(row=>row.source.replace('function compute(','function '+row.entry+'(')).join('\n'),
 versions:inputs.versions,sourceBasis:inputs.sourceBasis,
 boundary:'Declarations only. Future independent original Node/emitted JavaScript/native Rust values, exact error class/units/presence and full definition/program codec/emission controls. No observations. Object/Symbol primitive conversion, iterator spreads, source surrogate transport and formal class payloads remain required.',
});
