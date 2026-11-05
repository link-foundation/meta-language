// Deferred product/original controls. Do not execute during draft-only work.
import {readFileSync} from 'node:fs';
import vm from 'node:vm';
import {readStructuredDataLino,structuredDataToLino} from '../../js/src/structured-data-links.js';
import {checkProgram} from '../../js/src/translation/check.js';
import {encodeTypedProgramLinks,emitTypedProgramLinks} from '../../js/src/typed-program-links.js';
import {emitJavaScript} from '../../js/src/translation/emit-javascript.js';
const fixture=readStructuredDataLino(readFileSync(new URL('../../parity/fixtures/ordered-record-values.lino',import.meta.url),'utf8'));
const program=checkProgram(fixture.surface),packet=encodeTypedProgramLinks(program);
const context=vm.createContext({});vm.runInContext(emitJavaScript(program).text,context,{timeout:10000});
const cases=fixture.cases.map(row=>{
 let observed;
 try {context.input=row.input;const value=vm.runInContext('make(input)',context,{timeout:10000});observed={kind:'value',keys:Object.keys(value),values:Object.values(value)};}
 catch(error){observed={kind:'error',className:error.name,message:error.message};}
 let original=null;
 if(row.error===null){const object={};for(const index of row.input)Object.defineProperty(object,fixture.names[index],{value:fixture.values[index],enumerable:true,writable:true,configurable:true});original={kind:'value',keys:Object.keys(object),values:Object.values(object)};}
 return {name:row.name,observed,original};
});
const emissions=Object.fromEntries(['JavaScript','Rust','Lean','Rocq'].map(target=>[target,emitTypedProgramLinks(packet.network,packet.root,target)]));
process.stdout.write(structuredDataToLino({cases,emissions,links:packet.network.toLino()}));
