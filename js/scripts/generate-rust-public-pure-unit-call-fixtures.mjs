import assert from 'node:assert/strict';
import {fileURLToPath} from 'node:url';
import {checkProjectData,writeProjectData} from './project-data.mjs';
import {createTypedProgramLinks,decodeTypedProgramLinks} from '../src/typed-program-links.js';
import {emitRust} from '../src/translation/emit-rust.js';
export const RUST_PUBLIC_PURE_UNIT_CALL_MODEL='closed public pure Rust unit calls 1';
const specifications=[
 ['public-implicit-empty','pub fn empty(){}fn main(){}',['empty'],[]],
 ['public-implicit-unit','pub fn empty(){()}fn main(){empty();}',['empty'],['empty']],
 ['public-explicit-empty','pub fn empty()->(){}fn main(){crate::empty();}',['empty'],['empty']],
 ['public-explicit-unit','pub fn empty()->(){()}fn main(){empty();}',['empty'],['empty']],
 ['public-ordered-repeated','pub fn alpha()->(){()}pub fn beta(){()}fn main(){alpha();crate::beta();alpha();}',['alpha','beta'],['alpha','beta','alpha']],
 ['public-main-first','fn main(){last();}pub fn last()->(){()}',['last'],['last']],
 ['public-layout','pub\nfn empty ( ) -> ( ) { ( ) }\nfn main ( ) { crate :: empty ( ) ; }',['empty'],['empty']],
 ['public-comments','pub /* owned visibility */ fn empty(/* zero arguments */){/* no effects */()}fn main(){empty();}',['empty'],['empty']],
 ['public-temporary-capture','pub fn unit_call_result(){}fn main(){unit_call_result();unit_call_result();}',['unit_call_result'],['unit_call_result','unit_call_result']],
];
const originalJavaScript=[
 ['actual-empty','function empty(){}',['empty']],
 ['actual-bare-return','function empty(){return;}',['empty']],
 ['actual-two-helpers','function alpha(){}function beta(){return;}',['alpha','beta']],
 ['actual-underscore','function _empty(){}',['_empty']],
 ['actual-comments','// pure helper\nfunction empty(/* no arguments */){/* no effects */}',['empty']],
];
const javaScriptRefusedSources=[['actual-void-annotation','/** @returns {void} */function empty(){}']];
const refusedSources=[
 ['private-helper','fn empty(){}fn main(){}'],
 ['mixed-private-first','fn private(){}pub fn empty(){}fn main(){}'],
 ['mixed-public-first','pub fn empty(){}fn private(){}fn main(){}'],
 ['restricted-crate','pub(crate)fn empty(){}fn main(){}'],
 ['restricted-super','pub(super)fn empty(){}fn main(){}'],
 ['restricted-self','pub(self)fn empty(){}fn main(){}'],
 ['restricted-path','pub(in crate)fn empty(){}fn main(){}'],
 ['public-main','pub fn empty(){}pub fn main(){}'],
 ['unsafe-helper','pub unsafe fn empty(){}fn main(){}'],
 ['async-helper','pub async fn empty(){}fn main(){}'],
 ['extern-helper','pub extern "C" fn empty(){}fn main(){}'],
 ['public-module','pub mod unit{pub fn empty(){}}fn main(){}'],
 ['import','use crate::empty as other;pub fn empty(){}fn main(){other();}'],
 ['attribute','#[inline]pub fn empty(){}fn main(){}'],
 ['inner-attribute','#![allow(dead_code)]pub fn empty(){}fn main(){}'],
 ['parameters','pub fn empty(value:())->(){}fn main(){}'],
 ['borrowed-result','pub fn empty()->&(){}fn main(){}'],
 ['call-chain','pub fn empty(){other();}pub fn other(){}fn main(){}'],
 ['body-print','pub fn empty(){println!("effect");}fn main(){}'],
 ['main-print','pub fn empty(){}fn main(){println!("effect");}'],
 ['std-wrapper','pub fn empty(){}fn main(){std::thread::Builder::new().spawn(move||empty());}'],
 ['exit-wrapper','pub fn empty(){}fn main(){std::process::exit(101);}'],
];
function archive(id,source,names,calls){
 const packet=createTypedProgramLinks(source,'Rust',{rustPureUnitCallModel:RUST_PUBLIC_PURE_UNIT_CALL_MODEL,provenance:{fixture:id}}),program=decodeTypedProgramLinks(packet.network,packet.root);
 assert.deepEqual([...program.declarations.keys()],names,id+' owned declarations');assert.deepEqual(program.main.effects.map(effect=>effect.value.fn),calls,id+' actual call order');
 for(const name of names){const fn=program.declarations.get(name),proof=fn.sourcePureUnit;assert.deepEqual(fn.params,[]);assert.deepEqual(fn.ret,{kind:'unit'});assert.deepEqual(fn.body,{k:'unit',type:{kind:'unit'}});assert.equal(source.slice(proof.visibility.span.start,proof.visibility.span.end),'pub');assert.equal(proof.visibility.form,'public');}
 return {id,source,names,calls,output:'',links:packet.network.toLino(),root:packet.root.asU64()};
}
export function rustPublicPureUnitCallFixture(){
 const cases=specifications.map(([id,source,names,calls])=>archive(id,source,names,calls));
 const generated=originalJavaScript.map(([id,original,names])=>{
  const input=createTypedProgramLinks(original,'JavaScript'),checked=decodeTypedProgramLinks(input.network,input.root),emitted=emitRust(checked);
  // This established API intentionally returns the complete definition set.
  // Its runtime/main wrapper is retained separately as an expected refusal.
  const definitions=emitted.definitions.join('\n\n'),source=definitions+'\nfn main(){}';
  const result=archive(id,source,names,[]);
  return {...result,original,definitions,fullEmittedRust:emitted.text};
 });
 const invalid=refusedSources.map(([id,source])=>{
  try{createTypedProgramLinks(source,'Rust',{rustPureUnitCallModel:RUST_PUBLIC_PURE_UNIT_CALL_MODEL});}catch(error){return{id,source,kind:error.kind,reason:error.reason,span:error.span??null};}
  throw Error('an out-of-scope public Unit source was admitted: '+id);
 });
 const javaScriptInvalid=javaScriptRefusedSources.map(([id,source])=>{
  try{createTypedProgramLinks(source,'JavaScript');}catch(error){return{id,source,kind:error.kind,reason:error.reason,span:error.span??null};}
  throw Error('an explicitly recorded JavaScript input gap changed: '+id);
 });
 return {schemaVersion:1,model:RUST_PUBLIC_PURE_UNIT_CALL_MODEL,scope:'Every owned helper is an actual flat bare-public zero-argument pure Unit declaration; main remains private and contains only owned Unit calls. Actual definition-set replay uses emitRust.definitions with canonical empty main, never the full runtime wrapper or general public ABI.',cases,generated,invalid,javaScriptInvalid};
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
 const file=new URL('../../parity/fixtures/rust-public-pure-unit-calls.lino',import.meta.url),fixture=rustPublicPureUnitCallFixture();
 if(process.argv.includes('--check'))checkProjectData(file,fixture);else writeProjectData(file,fixture);
 console.log('Rust public pure Unit calls: '+fixture.cases.length+' original sources, '+fixture.generated.length+' actual emitted definition sets and '+fixture.invalid.length+' exact source refusals');
}
