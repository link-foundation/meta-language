// Produce every input from the real public emitter, preserving old oracles.
import {createHash} from 'node:crypto';
import {fileURLToPath} from 'node:url';
import {readProjectData,writeProjectData,checkProjectData} from '../../js/scripts/project-data.mjs';
import {createTypedProgramLinks,emitTypedProgramLinks} from '../../js/src/index.js';
import {parseRust} from '../../js/src/translation/rust.js';
import {checkProgram} from '../../js/src/translation/check.js';
import {RUST_RUNTIME_SHELL_POLICY} from '../../js/src/translation/rust-runtime-shell.js';
import {createGeneratedOwnedRust,GENERATED_OWNED_RUST_POLICY as policy} from './model.mjs';
export function generatedOwnedRustFixture(){
 const original=readProjectData(new URL('../owned-tuple-call-fixture.lino',import.meta.url));
 return{version:1,model:policy.model,scope:'Private JavaScript-only whole generated Rust interpretation. Complete emitted source, actual core identities and every token/trivia unit remain inspectable. Runtime and owned String success conditions are separate; no native compile, production selection, generic-codec enforcement or ledger evidence is claimed.',cases:original.cases.map(entry=>{
  const input=createTypedProgramLinks(entry.source,'Rust',{rustOwnedLocalStateModel:original.model}),source=emitTypedProgramLinks(input.network,input.root,'Rust').code;
  const {proof}=createGeneratedOwnedRust(source),refusals=[['default',{}],[original.model,{localValueStateModel:original.model}],['canonical owned text local copies 1',{localValueStateModel:'canonical owned text local copies 1'}],[RUST_RUNTIME_SHELL_POLICY.model,{runtimeShellModel:RUST_RUNTIME_SHELL_POLICY.model}]].map(([model,options])=>{try{checkProgram(parseRust(source,options));}catch(error){return{model,kind:error.kind,reason:error.reason,span:error.span??null};}throw new Error('the original source boundary unexpectedly changed '+model);});
  return{name:entry.name,fullEmittedRust:source,bytes:Buffer.byteLength(source),sha256:createHash('sha256').update(source).digest('hex'),observations:entry.observations,tokens:proof.tokens.length,clones:proof.clones.length,runtimeAssumptions:proof.assumptions.length,stringValueAssumptions:proof.valueAssumptions.length,refusals};
 })};
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
 const file=new URL('./data/fixtures.lino',import.meta.url),data=generatedOwnedRustFixture();
 if(process.argv.includes('--check'))checkProjectData(file,data);else writeProjectData(file,data);
 console.log('Generated whole owned Rust experiment: '+data.cases.length+' complete actual emitted artifacts');
}
