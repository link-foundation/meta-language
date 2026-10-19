import assert from 'node:assert/strict';
import {fileURLToPath} from 'node:url';
import {checkProjectData,writeProjectData} from './project-data.mjs';
import {createTypedProgramLinks,decodeTypedProgramLinks} from '../src/typed-program-links.js';
import {emitRust} from '../src/translation/emit-rust.js';
export const RUST_RUNTIME_SHELL_MODEL='closed pure Rust unit runtime shell 1';
const specifications=[
 ['empty-program','',[],[]],
 ['unused-helper','function empty(){}',['empty'],[]],
 ['actual-invocation','function empty(){}empty();',['empty'],['empty']],
 ['bare-return','function empty(){return;}empty();',['empty'],['empty']],
 ['ordered-repeated','function alpha(){}function beta(){}alpha();beta();alpha();',['alpha','beta'],['alpha','beta','alpha']],
 ['forward-source-call','last();function last(){}',['last'],['last']],
 ['named-result','function empty(){}const result=empty();',['empty'],['empty']],
 ['underscore-helper','function _empty(){}_empty();',['_empty'],['_empty']],
];
export function rustRuntimeShellFixture(){
 return {schemaVersion:1,model:RUST_RUNTIME_SHELL_MODEL,observationModel:'successful closed pure Unit worker observation 1',scope:'Complete actual emitter artifacts, under explicit successful standard argument enumeration, bounded thread creation, worker join and no external termination assumptions. All concrete runtime operations and failure branches remain owned linked syntax; no failure-path equivalence is asserted.',cases:specifications.map(([id,original,names,calls])=>{
  const input=createTypedProgramLinks(original,'JavaScript'),emitted=emitRust(decodeTypedProgramLinks(input.network,input.root));
  const packet=createTypedProgramLinks(emitted.text,'Rust',{rustRuntimeShellModel:RUST_RUNTIME_SHELL_MODEL}),checked=decodeTypedProgramLinks(packet.network,packet.root);
  assert.deepEqual([...checked.declarations.keys()],names,id+' actual declarations');assert.deepEqual(checked.main.effects.map(effect=>effect.value.fn),calls,id+' actual worker calls');
  return {id,original,fullEmittedRust:emitted.text,names,calls,output:''};
 })};
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
 const file=new URL('../../parity/fixtures/rust-runtime-shell.lino',import.meta.url),fixture=rustRuntimeShellFixture();
 if(process.argv.includes('--check'))checkProjectData(file,fixture);else writeProjectData(file,fixture);
 console.log('Closed Rust runtime shell: '+fixture.cases.length+' complete actual emitter artifacts');
}
