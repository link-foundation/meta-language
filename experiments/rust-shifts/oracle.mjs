// Future paired CI producer; never execute during the whole requirements draft.
import{runInNewContext}from'node:vm';
import{readProjectData}from'../../js/scripts/project-data.mjs';
import{structuredDataToLino}from'../../js/src/structured-data-links.js';
import{parseRust}from'../../js/src/translation/rust.js';
import{checkProgram}from'../../js/src/translation/check.js';
import{emitRust}from'../../js/src/translation/emit-rust.js';
import{emitJavaScript}from'../../js/src/translation/emit-javascript.js';
import{encodeTypedProgramLinks,emitTypedProgramLinks}from'../../js/src/typed-program-links.js';
const[major,minor]=process.versions.node.split('.').map(Number);
if(!(major===22&&minor>=15||major===24))throw new RangeError('uncovered Node '+process.versions.node);
const fixture=readProjectData(new URL('../../parity/fixtures/rust-shifts.lino',import.meta.url));
const source=fixture.source+fixture.precedenceSource+fixture.contextualSource+'\nfn main(){}\n';
const program=checkProgram(parseRust(source)),packet=encodeTypedProgramLinks(program),rust=emitRust(program),js=emitJavaScript(program);
const names=fixture.functions.map(row=>js.mappings.find(mapping=>mapping.kind==='function'&&mapping.source===row.name).target);
const functions=runInNewContext(js.text.replace(/^export /gmu,'')+'\n['+names.join(',')+'];');
const cases=[];
for(const[index,row]of fixture.functions.entries()){
 const values=fixture.types.find(type=>type.name===row.type).values,counts=fixture.countTypes.find(type=>type.name===row.countType).values;
 for(const left of values)for(const count of counts){
  let result;
  try{result={kind:'value',value:String(functions[index](BigInt(left),BigInt(count)))};}
  catch(error){result={kind:'abort',value:error.message};}
  cases.push([index,left,count,result.kind,result.value]);
 }
}
const header=rust.text.split('\n').find(line=>line.startsWith('#![allow('));
if(!header)throw new TypeError('complete library attributes are required');
process.stdout.write(structuredDataToLino({source,cases,functions:fixture.functions.map(row=>({...row,nativeTarget:rust.mappings.find(mapping=>mapping.kind==='function'&&mapping.source===row.name).target})),root:packet.root.asU64(),links:packet.network.toLino(),
 emissions:Object.fromEntries(['JavaScript','Rust','Lean','Rocq'].map(target=>[target,emitTypedProgramLinks(packet.network,packet.root,target)])),
 nativeLibrary:[header,...rust.preludes,...rust.definitions].join('\n\n')}));
