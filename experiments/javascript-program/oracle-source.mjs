// Future complete paired producer. Never execute during the whole draft.
import{runInNewContext}from'node:vm';
import{readProjectData}from'../../js/scripts/project-data.mjs';
import{structuredDataToLino}from'../../js/src/structured-data-links.js';
import{parseJavaScript}from'../../js/src/translation/javascript.js';
import{checkProgram}from'../../js/src/translation/check.js';
import{emitRust}from'../../js/src/translation/emit-rust.js';
import{emitJavaScript}from'../../js/src/translation/emit-javascript.js';
import{encodeTypedProgramLinks,emitTypedProgramLinks}from'../../js/src/typed-program-links.js';
import{readJavaScriptModuleSyntaxError}from'./syntax-source.mjs';
export function produceJavaScriptProgramOracle(fixtureUrl,createDefinition){
const[major,minor]=process.versions.node.split('.').map(Number);
if(!(major===22&&minor>=15||major===24))throw new RangeError('uncovered Node '+process.versions.node);
const fixture=readProjectData(fixtureUrl);
if(fixture.oracleArchitectures&&!fixture.oracleArchitectures.includes(process.arch))throw new RangeError('uncovered source architecture '+process.arch);
const refusals=fixture.sourceSyntaxOracle?fixture.refusals.map(row=>{
 if(row.kind!=='syntax')throw new TypeError('every declared module-syntax oracle row needs its actual syntax assertion');
 return{...row,originalSyntax:readJavaScriptModuleSyntaxError(row.source)};
}):fixture.refusals;
const program=checkProgram(parseJavaScript(fixture.source)),packet=encodeTypedProgramLinks(program),rust=emitRust(program),js=emitJavaScript(program);
const names=fixture.cases.map(row=>js.mappings.find(mapping=>mapping.kind==='function'&&mapping.source===row.entry).target);
const functions=runInNewContext('"use strict";\n'+js.text.replace(/^export /gmu,'')+'\n['+names.join(',')+'];');
const observe=(call,row)=>{try{return{kind:'value',value:String(call())};}catch(error){
 const messageUnits=Array.from({length:error.message.length},(_,index)=>error.message.charCodeAt(index));
 return{kind:'abort',value:row.errorClass===undefined?error.message:error.message.toWellFormed(),...(row.errorClass===undefined?{}:{errorClass:error.name,messageUnits,...(row.messagePresent===undefined?{}:{messagePresent:Object.hasOwn(error,'message')})})};
}};
const cases=fixture.cases.map((row,index)=>({...row,module:row.module??'',observed:observe(()=>functions[index](),row),originalObserved:observe(()=>runInNewContext('"use strict";\n'+row.source.replace(/^export /gmu,'')+'\ncompute();',{}, {timeout:1000}),row),nativeTarget:rust.mappings.find(mapping=>mapping.kind==='function'&&mapping.source===row.entry).target}));
const header=rust.text.split('\n').find(line=>line.startsWith('#![allow('));
if(!header)throw new TypeError('complete library attributes are required');
const definition=createDefinition();
process.stdout.write(structuredDataToLino({sourceLanguage:'JavaScript',source:fixture.source,cases,refusals,...(fixture.sourceSyntaxOracle?{sourceSyntaxOracle:true}:{}),...(fixture.lexicalCases?{lexicalCases:fixture.lexicalCases,lexicalErrors:fixture.lexicalErrors}:{}),root:packet.root.asU64(),links:packet.network.toLino(),definitionRoot:definition.root.asU64(),definitionLinks:definition.network.toLino(),
 ...(fixture.unitLexicalCases?{unitLexicalCases:fixture.unitLexicalCases}:{}),
 ...(fixture.syntaxCases?{syntaxCases:fixture.syntaxCases.map(row=>({...row,originalSyntax:readJavaScriptModuleSyntaxError(row.source)}))}:{}),
 emissions:Object.fromEntries(['JavaScript','Rust','Lean','Rocq'].map(target=>[target,emitTypedProgramLinks(packet.network,packet.root,target)])),nativeLibrary:[header,...rust.preludes,...rust.definitions].join('\n\n')}));

}
