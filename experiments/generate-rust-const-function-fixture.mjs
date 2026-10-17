// Pinned complete styled definitions, never a rewritten or qualifier-stripped oracle.
import {createHash} from 'node:crypto';
import {fileURLToPath} from 'node:url';
import {readProjectData,writeProjectData,checkProjectData} from '../js/scripts/project-data.mjs';
import {tokenize} from '../js/src/translation/lexer.js';
import {parseRust} from '../js/src/translation/rust.js';
import {checkProgram} from '../js/src/translation/check.js';
import {emitRust} from '../js/src/translation/emit-rust.js';
import {emitJavaScript} from '../js/src/translation/emit-javascript.js';
import {encodeTypedProgramLinks} from '../js/src/typed-program-links.js';
const policy=readProjectData(new URL('../js/src/translation/data/rust-const-functions.lino',import.meta.url));
const sha=text=>createHash('sha256').update(text).digest('hex');
const sources=[
 {commit:'c1e88c8b',revision:'c1e88c8be7d3f8e63cebc49a46bb4d48823ba329',file:'rust/src/translation/rust/owned_local_state_rules.rs',fileSha256:'b36f7104270c568db448ae3b488349228f3ef162c2bcf0620901557691b892ba',names:['accept_rust_owned_local_record_value','accept_rust_owned_local_source_span']},
 {commit:'87f70e4c',revision:'87f70e4cce215ce7722e01c79c5c8e6e4a6f22cf',file:'rust/src/typed_program_links/source_span_rules.rs',fileSha256:'ad1a00d53521e702b316c70b48307b6d01c4a5f72007ded81c60d9469119a232',names:['accept_source_span_interval']},
];
export function readRustConstPredicateSources(captures=readProjectData(new URL('./rust-const-predicate-source-captures.lino',import.meta.url))){
 const fail=()=>{throw new Error('the authenticated Rust const source capture differs from its original immutable receipt');};
 if(captures.schemaVersion!==1||!Array.isArray(captures.sources)||captures.sources.length!==sources.length)fail();
 return sources.map(owner=>{
  const capture=captures.sources.find(value=>value.commit===owner.commit&&value.file===owner.file);
  if(!capture||capture.revision!==owner.revision||capture.sha256!==owner.fileSha256||typeof capture.source!=='string')fail();
  const bytes=Buffer.from(capture.source,'utf8');
  if(bytes.length!==capture.bytes||sha(bytes)!==owner.fileSha256||createHash('sha1').update(Buffer.from('blob '+bytes.length+'\0')).update(bytes).digest('hex')!==capture.blob)fail();
  return{...owner,source:capture.source};
 });
}
export function pinnedRustConstPredicateDefinitions(){
 return readRustConstPredicateSources().flatMap(owner=>{
  const source=owner.source,tokens=tokenize(source,'Rust').tokens;
  return owner.names.map(name=>{
   const index=tokens.findIndex((token,index)=>token.value==='const'&&tokens[index+1]?.value==='fn'&&tokens[index+2]?.value===name);if(index<0)throw new Error('the actual styled const definition disappeared '+name);
   const open=tokens.findIndex((token,at)=>at>index&&token.value==='{');let depth=0,end;
   for(let at=open;at<tokens.length;at++){if(tokens[at].value==='{')depth++;if(tokens[at].value==='}'&&--depth===0){end=tokens[at].end;break;}}
   if(end===undefined)throw new Error('the actual const definition is unclosed');
   let start=source.lastIndexOf('\n',tokens[index-1]?.value==='pub'?tokens[index-1].start:tokens[index].start)+1;
   while(start>0){const previous=source.lastIndexOf('\n',start-2)+1,line=source.slice(previous,start-1);if(!/^\s*(?:\/\/\/|#\[must_use\])/u.test(line))break;start=previous;}
   const definition=source.slice(start,end);return{name,definition,receipt:{commit:owner.commit,file:owner.file,fileSha256:sha(source),start,end,bytes:Buffer.byteLength(definition),sha256:sha(definition)}};
  });
 });
}
const invalidBodies=[
 ['keyword function name','const fn true(value:bool)->bool{value}'],
 ['keyword parameter name','const fn predicate(true:bool)->bool{true}'],
 ['overflowing float literal','const fn predicate(value:f64)->bool{value==1e999}'],
 ['duplicate ordinary main','const fn predicate(value:bool)->bool{value}fn main(){}'],
 ['discarded first main effect','const fn predicate(value:bool)->bool{value}fn main(){println!("lost");}'],
 ['duplicate const main','const fn main(value:bool)->bool{value}'],
 ['crate attribute','#![allow(dead_code)] const fn bad(value:bool)->bool{value}'],
 ['conditional attribute','#[cfg(any())] const fn bad(value:bool)->bool{value}'],
 ['derive attribute','#[derive(Clone)] const fn bad(value:bool)->bool{value}'],
 ['restricted visibility','pub(in missing) const fn bad(value:bool)->bool{value}'],
 ['ordinary function','fn no_const(value:bool)->bool{value}'],
 ['function call','const fn bad(value:bool)->bool{foreign(value)}'],
 ['local binding','const fn bad(value:bool)->bool{let local=value;local}'],
 ['conditional','const fn bad(value:bool)->bool{if value{true}else{false}}'],
 ['numeric result','const fn bad(value:f64)->f64{value}'],
 ['integer interface','const fn bad(value:i32)->bool{value==0i32}'],
 ['borrowed interface','const fn bad(value:&bool)->bool{*value}'],
 ['mutable parameter','const fn bad(mut value:bool)->bool{value}'],
 ['lifetime binder',"const fn bad<'a>(value:bool)->bool{value}"],
 ['numeric arithmetic','const fn bad(value:f64)->bool{value+1.0>0.0}'],
 ['panic','const fn bad(value:bool)->bool{panic!("not admitted")}'],
 ['string interface','const fn bad(value:String)->bool{true}'],
 ['imported owner','use core::primitive::f64 as Number;const fn bad(value:Number)->bool{value<0.0}'],
 ['datatype owner','enum f64{Value}const fn bad(value:f64)->bool{true}'],
 ['constant invocation','const VALUE:bool=predicate(true);const fn predicate(value:bool)->bool{value}'],
 ['module context','mod nested{const fn bad(value:bool)->bool{value}}'],
 ['main effect','const fn bad(value:bool)->bool{value}fn main(){println!("x");}'],
];
export function rustConstFunctionFixture(){
 const cases=pinnedRustConstPredicateDefinitions().map(entry=>{
  const source=entry.definition+'\nfn main(){}',options={constFunctionModel:policy.model};let originalRefusal;
  try{checkProgram(parseRust(source));}catch(error){originalRefusal={kind:error.kind,reason:error.reason,span:error.span??null};}
  if(!originalRefusal)throw new Error('the original const qualification refusal changed');
  const program=checkProgram(parseRust(source,options)),archive=encodeTypedProgramLinks(program),rust=emitRust(program),replaySource=rust.definitions.join('\n\n')+'\nfn main(){}';
  const replay=checkProgram(parseRust(replaySource,options)),again=emitRust(replay);
  if(!rust.definitions.every(definition=>definition.startsWith('pub const fn ')))throw new Error('actual Rust output erased const');
  return{...entry,source,originalRefusal,proof:program.items[0].sourceConstFunction,signature:program.items[0].sourceSignature,links:archive.network.toLino(),root:Number(archive.root.value),rustDefinition:rust.definitions[0],replaySource,javascript:emitJavaScript(program).text,replayJavaScript:emitJavaScript(replay).text,secondRustDefinition:again.definitions[0]};
 });
 const invalid=invalidBodies.map(([name,body])=>{
  const source=body.endsWith('fn main(){println!("x");}')?body:body+'fn main(){}';
  try{checkProgram(parseRust(source,{constFunctionModel:policy.model}));}catch(error){return{name,source,kind:error.kind,reason:error.reason,span:error.span??null};}
  throw new Error('an unproved const context was admitted '+name);
 });
 return{schemaVersion:1,model:policy.model,scope:policy.interpretation,cases,invalid};
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
 const file=new URL('./rust-const-function-fixture.lino',import.meta.url),value=rustConstFunctionFixture();
 if(process.argv.includes('--check')){if(!checkProjectData(file,value))throw new Error('const function draft fixture is stale');}else writeProjectData(file,value);
}
