// Future independent conversion observations. Never execute during drafting.
import {readProjectData} from '../../js/scripts/project-data.mjs';
import {structuredDataToLino} from '../../js/src/structured-data-links.js';
import {convertJavaScriptStringBigIntegerUnits,convertJavaScriptNumberBigInteger} from '../../js/src/big-integer-conversion.js';
import {createBigIntegerConversionProgramLinks} from '../../js/src/big-integer-conversion-program.js';
import {emitTypedProgramLinks} from '../../js/src/typed-program-links.js';
const [major,minor]=process.versions.node.split('.').map(Number);
if(!(major===22&&minor>=15||major===24))throw new RangeError('uncovered Node '+process.versions.node);
const fixture=readProjectData(new URL('../../parity/fixtures/big-integer-conversion.lino',import.meta.url));
const units=text=>Array.from({length:text.length},(_,index)=>text.charCodeAt(index));
const observe=call=>{try{return{kind:'value',value:String(call())};}catch(error){return{kind:'error',errorClass:error.name,messageUnits:units(error.message)};}};
const cases=fixture.cases.map(row=>{
 const value=row.kind==='string'?String.fromCharCode(...row.units):Number(row.spelling);
 const view=new DataView(new ArrayBuffer(8));
 if(row.kind==='number')view.setFloat64(0,value,false);
 return {...row,...(row.kind==='number'?{numberBits:view.getBigUint64(0,false).toString(16)}:{}),
  expected:row.expected.kind==='value'?row.expected:{kind:'error',errorClass:row.expected.errorClass,messageUnits:units(row.expected.prefix+String(value)+row.expected.suffix)},
  original:observe(()=>BigInt(value)),javascript:observe(()=>row.kind==='string'?convertJavaScriptStringBigIntegerUnits(row.units):convertJavaScriptNumberBigInteger(value))};
});
const definition=createBigIntegerConversionProgramLinks();
process.stdout.write(structuredDataToLino({cases,root:definition.root.asU64(),links:definition.network.toLino(),emissions:Object.fromEntries(['JavaScript','Rust','Lean','Rocq'].map(target=>[target,emitTypedProgramLinks(definition.network,definition.root,target)]))}));
