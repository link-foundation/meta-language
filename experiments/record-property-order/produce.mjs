// Future CI producer; never execute during the complete drafting phase.
import {readProjectData} from '../../js/scripts/project-data.mjs';
import {structuredDataToLino} from '../../js/src/structured-data-links.js';
import {createJavaScriptRecordOrderProgramLinks} from '../../js/src/javascript-record-order-program.js';
import {emitTypedProgramLinks} from '../../js/src/typed-program-links.js';
import {createJavaScriptRecordPropertyOrder,mergeJavaScriptRecordPropertyOrder,rebaseJavaScriptRecordPropertyOrder,readJavaScriptRecordPropertyKeys} from '../../js/src/translation/javascript-record-order-rules.js';
const fixture=readProjectData(new URL('../../parity/fixtures/javascript-record-order.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);
if(!(major===22&&minor>=15||major===24))throw new RangeError('uncovered Node '+process.versions.node);
const invoke=row=>row.operation==='create'?createJavaScriptRecordPropertyOrder(row.names,row.input):row.operation==='merge'?mergeJavaScriptRecordPropertyOrder(row.names,row.input):row.operation==='rebase'?rebaseJavaScriptRecordPropertyOrder(row.names,row.sourceNames,row.input):readJavaScriptRecordPropertyKeys(row.names,row.input);
const units=text=>Array.from({length:text.length},(_,index)=>text.charCodeAt(index));
const original=row=>{
 const names=(row.operation==='rebase'?row.sourceNames:row.names).map(name=>String.fromCharCode(...name));
 const insertion=row.operation==='merge'?row.input.flat():row.input;
 const object={};
 for(const index of insertion)Object.defineProperty(object,names[index],{value:1,writable:true,enumerable:true,configurable:true});
 const keys=Object.keys(object);
 return row.operation==='keys'?keys.map(units):keys.map(key=>row.names.map(name=>String.fromCharCode(...name)).indexOf(key));
};
const observe=call=>{try{return{kind:'value',value:call()};}catch(error){return{kind:'error',className:error.name,message:error.message};}};
const cases=fixture.cases.map(row=>({...row,observed:observe(()=>invoke(row)),...(row.error===null?{originalObserved:observe(()=>original(row))}:{})}));
const packet=createJavaScriptRecordOrderProgramLinks();
process.stdout.write(structuredDataToLino({model:fixture.model,cases,root:packet.root.asU64(),links:packet.network.toLino(),emissions:Object.fromEntries(['JavaScript','Rust','Lean','Rocq'].map(target=>[target,emitTypedProgramLinks(packet.network,packet.root,target)]))}));
