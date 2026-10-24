// Future paired producer. Never execute during the whole requirements draft.
import{runInNewContext}from'node:vm';
import{readProjectData}from'../../js/scripts/project-data.mjs';
import{structuredDataToLino}from'../../js/src/structured-data-links.js';
import{parseRust}from'../../js/src/translation/rust.js';
import{checkProgram}from'../../js/src/translation/check.js';
import{emitRust}from'../../js/src/translation/emit-rust.js';
import{emitJavaScript}from'../../js/src/translation/emit-javascript.js';
import{encodeTypedProgramLinks,emitTypedProgramLinks}from'../../js/src/typed-program-links.js';

export function produceRustNominalOracle(fixtureUrl,createDefinition){
const[major,minor]=process.versions.node.split('.').map(Number);
if(!(major===22&&minor>=15||major===24))throw new RangeError('uncovered Node '+process.versions.node);
const fixture=readProjectData(fixtureUrl);
const program=checkProgram(parseRust(fixture.source)),packet=encodeTypedProgramLinks(program),rust=emitRust(program),js=emitJavaScript(program);
const names=fixture.cases.map(row=>js.mappings.find(mapping=>mapping.kind==='function'&&mapping.source===(row.module?row.module+'.result':'result')).target);
const functions=runInNewContext(js.text.replace(/^export /gmu,'')+'\n['+names.join(',')+'];');
const cases=fixture.cases.map((row,index)=>{
 let result;try{result={kind:'value',value:String(functions[index]())};}catch(error){result={kind:'abort',value:error.message};}
 return{...row,observed:result,nativeTarget:rust.mappings.find(mapping=>mapping.kind==='function'&&mapping.source===(row.module?row.module+'.result':'result')).target};
});
const header=rust.text.split('\n').find(line=>line.startsWith('#![allow('));
if(!header)throw new TypeError('complete library attributes are required');
const definition=createDefinition();
process.stdout.write(structuredDataToLino({source:fixture.source,refusals:fixture.refusals,cases,...(fixture.layout?{layout:fixture.layout}:{}),root:packet.root.asU64(),links:packet.network.toLino(),definitionRoot:definition.root.asU64(),definitionLinks:definition.network.toLino(),
 emissions:Object.fromEntries(['JavaScript','Rust','Lean','Rocq'].map(target=>[target,emitTypedProgramLinks(packet.network,packet.root,target)])),
 nativeLibrary:[header,...rust.preludes,...rust.definitions].join('\n\n')}));

}
