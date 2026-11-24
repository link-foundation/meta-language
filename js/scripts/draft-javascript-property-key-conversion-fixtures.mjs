// Literal inputs/keys and retained primitive controls only; no coercion runs.
import {readProjectData,writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const root=new URL('../../',import.meta.url),primitive=readProjectData(new URL('parity/fixtures/javascript-primitive-conversion.lino',root));
const text=value=>({$:'text',units:Array.from({length:value.length},(_,index)=>value.charCodeAt(index))});
const literalKeys=new Map([['undefined','undefined'],['null','null'],['false','false'],['negative zero','0'],['NaN','NaN'],
 ['arbitrary BigInt','340282366920938463463374607431768211455'],['isolated surrogate','\uD800']]);
const selected=primitive.cases.filter(row=>row.name.startsWith('direct primitive retains ')||row.hint==='string'||row.name==='missing methods exhaust the ordinary conversion'||row.name==='noncallable exotic value requires TypeError immediately'||row.name==='abrupt getter completion propagates its exact value');
const cases=selected.map(row=>{
 const {expected,...input}=row;let result;
 if(expected.$==='primitiveCompleted'){
  const direct=row.name.slice('direct primitive retains '.length),key=literalKeys.has(direct)?text(literalKeys.get(direct)):expected.value;
  if(key.$!=='text'&&key.$!=='symbol')throw new TypeError('a key control needs its independent literal key');
  result={$:'propertyKeyCompleted',key};
 }else if(expected.$==='primitiveThrow')result={$:'propertyKeyThrow',value:expected.value};
 else if(expected.$==='primitiveIntrinsicError')result={$:'propertyKeyIntrinsicError',errorClass:expected.errorClass,reason:expected.reason};
 else throw new TypeError('a key control needs its complete expected source outcome');
 return {...input,name:'ToPropertyKey '+row.name,hint:'string',primitiveExpected:expected,expected:result};
});
const base=primitive.cases.find(row=>row.name==='direct primitive retains undefined');if(!base)throw new TypeError('complete primitive literal input is required');
function number(value){const bytes=new ArrayBuffer(8),view=new DataView(bytes);view.setFloat64(0,value);return {$:'number',bits:view.getBigUint64(0).toString(16).padStart(16,'0')};}
for(const [name,value,key]of [['positive infinity',Infinity,'Infinity'],['negative infinity',-Infinity,'-Infinity'],['large decimal',1e20,'100000000000000000000'],['large exponent',1e21,'1e+21'],['small decimal',1e-6,'0.000001'],['small exponent',1e-7,'1e-7'],['negative integer',-123,'-123']]){
 const input=number(value);cases.push({...base,name:'ToPropertyKey '+name,hint:'string',input,primitiveExpected:{$:'primitiveCompleted',value:input},expected:{$:'propertyKeyCompleted',key:text(key)}});
}
const state={$:'primitiveConversion',receiver:{$:'object',identity:'1',callable:false},exoticKey:base.exoticKey,hint:'number',phase:'exoticGet'};
const invalid=[{name:'property key cannot resume a number-hint conversion',state,reason:'a property key continuation retains its actual string hint'},
 {name:'property key cannot complete a number-hint call',state:{...state,phase:'firstCall'},operation:'resume',completion:{$:'normalCompletion',value:{$:'undefined'}},reason:'a property key continuation retains its actual string hint'}];
writeProjectData(new URL('parity/fixtures/javascript-property-key-conversion.lino',root),{schemaVersion:1,model:'ordinary JavaScript resumable ToPropertyKey 1',versions:primitive.versions,cases,invalid,invalidHeap:primitive.invalidHeap,
 boundary:'Complete ordinary primitive conversion and key formatting protocol; actual call execution, intrinsic authority, Error allocation, source frontend integration and exotics remain required.'});
