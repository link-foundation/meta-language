import assert from 'node:assert/strict';
import {fileURLToPath} from 'node:url';
import {readProjectData,checkProjectData,writeProjectData} from '../../js/scripts/project-data.mjs';
import {createTypedProgramLinks,decodeTypedProgramLinks,emitTypedProgramLinks} from '../../js/src/index.js';
import {GENERATED_JAVASCRIPT_UNIT_POLICY as policy} from './model.mjs';

export function generatedJavaScriptUnitFixture(){
 const source=readProjectData(new URL('../../parity/fixtures/rust-runtime-shell.lino',import.meta.url));
 assert.equal(source.cases.length,8);
 return {schemaVersion:1,model:policy.model,observationModel:policy.observationModel,valueEncoding:policy.valueEncoding,scope:'Experimental JavaScript-only interpretation of eight actual complete JavaScript emitter artifacts. Original Rust runtime assumptions remain explicit incoming witness evidence; the JavaScript source model does not infer them from source text or comments. Each actual emitted Rust target is independently re-read under the closed runtime-shell model and carries its own explicit successful-runtime assumptions.',cases:source.cases.map(row=>{
  const input=createTypedProgramLinks(row.fullEmittedRust,'Rust',{rustRuntimeShellModel:source.model});
  const emitted=emitTypedProgramLinks(input.network,input.root,'JavaScript');
  assert.equal(emitted.assumptions.length,4,row.id+' incoming successful-runtime assumptions');
  let defaultInterpretation;
  try{const packet=createTypedProgramLinks(emitted.code,'JavaScript'),program=decodeTypedProgramLinks(packet.network,packet.root);defaultInterpretation={kind:'ordinary source interpretation',helpers:[...program.declarations.values()].map(entry=>({name:entry.fullName,returnType:entry.ret})),effects:program.main.effects.map(effect=>effect.k)};}
  catch(error){defaultInterpretation={kind:'source refusal',message:error.message,span:error.span??null};}
  return {id:row.id,original:row.original,fullEmittedRust:row.fullEmittedRust,fullEmittedJavaScript:emitted.code,names:row.names,calls:row.calls,output:row.output,incomingRustAssumptions:emitted.assumptions,defaultInterpretation};
 })};
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
 const file=new URL('./data/fixtures.lino',import.meta.url),fixture=generatedJavaScriptUnitFixture();
 if(process.argv.includes('--check'))checkProjectData(file,fixture);else writeProjectData(file,fixture);
 console.log('Generated JavaScript Unit experiment: '+fixture.cases.length+' complete actual JavaScript artifacts');
}
