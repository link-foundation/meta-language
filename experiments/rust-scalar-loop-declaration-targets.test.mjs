// Explicit declaration-directed loop prototype; old profile assertions stay intact.
import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import {parseRust} from '../js/src/translation/rust.js';
import {parseJavaScript} from '../js/src/translation/javascript.js';
import {checkProgram} from '../js/src/translation/check.js';
import {emitRust} from '../js/src/translation/emit-rust.js';
import {emitJavaScript} from '../js/src/translation/emit-javascript.js';
import {LinkNetwork} from '../js/src/network.js';
import {readProjectData} from '../js/scripts/project-data.mjs';
import {createTypedProgramLinks,decodeTypedProgramLinks,encodeTypedProgramLinks,emitTypedProgramLinks} from '../js/src/typed-program-links.js';

import {declarationScalarLoopModel as model,declarationScalarLoopCases as inputCases} from './rust-scalar-loop-declaration-cases.mjs';
const legacy='closed primitive scalar loop 1',options={scalarLoopModel:model};
const rust=inputCases[0].source,javascript=inputCases[1].source;
const read=(source,language)=>checkProgram((language==='Rust'?parseRust:parseJavaScript)(source,options));
const importCase=entry=>createTypedProgramLinks(entry.source,entry.language,{[entry.language==='Rust'?'rustScalarLoopModel':'javaScriptScalarLoopModel']:model});
const observe=(source,name,args,expected)=>{const context=vm.createContext({scalarLoopArguments:args});vm.runInContext(source,context,{timeout:1000});assert.ok(Object.is(vm.runInContext(name+'(...scalarLoopArguments)',context,{timeout:1000}),expected),name+' independent scalar observation');};
function visit(value,callback){if(!value||typeof value!=='object')return;callback(value);for(const [key,child] of Object.entries(value))if(!['type','domain','span'].includes(key)){if(Array.isArray(child))child.forEach(value=>visit(value,callback));else visit(child,callback);}}
function relocate(program){const entry=program.declarations.get('run');entry.modulePath=['nested'];entry.fullName='nested.run';entry.sourceLoop.continuation.path=['nested','run'];visit(entry.body,node=>{if(node.k==='call'&&node.fn==='run')node.fn='nested.run';});program.declarations.delete('run');program.declarations.set('nested.run',entry);program.items=[{k:'module',name:'nested',fullName:'nested',items:[entry],span:null}];return entry;}

test('actual parameter and local capture sources use the owned declaration continuation',()=>{
 const previous=readProjectData(new URL('../parity/fixtures/rust-scalar-loops.lino',import.meta.url));
 assert.equal(previous.bindingCaptureRefusals[0].source,rust);assert.equal(previous.bindingCaptureRefusals[1].source,javascript);
 for(const entry of inputCases){
  if(entry.language==='JavaScript')for(const [args,expected] of entry.observations)observe(entry.source,entry.name,args,expected);
  const surface=(entry.language==='Rust'?parseRust:parseJavaScript)(entry.source,options),fn=surface.items[0];
  assert.equal(fn.name,entry.name);assert.deepEqual(fn.sourceLoop.continuation,{root:'crate',path:[entry.name]});
  if(entry.name==='run')assert.equal(fn.params[0].name,'run');
  visit(fn.body,node=>{if(node.k==='app')assert.deepEqual(node.fn.path,['crate',entry.name]);});
  const packet=importCase(entry),network=LinkNetwork.fromLino(packet.network.toLino()),program=decodeTypedProgramLinks(network,packet.root);
  assert.deepEqual(program.declarations.get(entry.name).sourceLoop.continuation,{root:'crate',path:[entry.name]});
  const definitions=emitJavaScript(program).definitions.join('\n\n');for(const [args,expected] of entry.observations)observe(definitions,entry.name,args,expected);
  const reverse=read(definitions,'JavaScript');for(const [args,expected] of entry.observations)observe(emitJavaScript(reverse).text,entry.name,args,expected);
  const emittedRust=emitRust(reverse).definitions.join('\n\n');
  assert.ok(emittedRust.includes('pub fn '+entry.name+'('));
  for(const [args,expected] of entry.observations)observe(emitJavaScript(read(emittedRust+'\nfn main(){}','Rust')).text,entry.name,args,expected);
 }
});
test('the frozen original profile and every original capture refusal remain exact',()=>{
 const previous=readProjectData(new URL('../parity/fixtures/rust-scalar-loops.lino',import.meta.url));
 for(const entry of previous.bindingCaptureRefusals){assert.throws(()=>createTypedProgramLinks(entry.source,entry.language,{[entry.language==='Rust'?'rustScalarLoopModel':'javaScriptScalarLoopModel']:legacy}),error=>{assert.equal(error.reason,entry.reason);assert.deepEqual(error.span,entry.span);return true;});}
 assert.throws(()=>createTypedProgramLinks(rust,'Rust'),/mutable parameter/u);
 assert.throws(()=>checkProgram(parseRust('fn ml_original()->f64{1f64}fn main(){}')),/reserved identifier/u);
 for(const entry of inputCases){assert.throws(()=>createTypedProgramLinks(entry.source,entry.language,{[entry.language==='Rust'?'rustScalarLoopModel':'javaScriptScalarLoopModel']:'unregistered scalar loop'}),/scalar loop model/u);}
});
test('surface proof requires exact absolute root and actual owned declaration segments',()=>{
 const paths=[['run'],['self','run'],['crate'],['crate','other'],['crate','nested','run'],['crate','run','extra'],['crate.run'],['crate','run.'],['crate','run',null]];
 for(const language of ['Rust','JavaScript'])for(const path of paths){const surface=(language==='Rust'?parseRust:parseJavaScript)(language==='Rust'?rust:javascript,options);visit(surface.items[0].body,node=>{if(node.k==='app')node.fn.path=path;});assert.throws(()=>checkProgram(surface),/owned tail continuation/u);}
 for(const mutate of [proof=>{proof.continuation.root='self';},proof=>{proof.continuation.root='run';},proof=>{proof.continuation.path=['other'];},proof=>{proof.continuation.path=['run',''];},proof=>{proof.continuation.extra=true;},proof=>{delete proof.continuation;}]){const surface=parseRust(rust,options);mutate(surface.items[0].sourceLoop);assert.throws(()=>checkProgram(surface));}
});
test('source-free qualified declaration edits require matching owned scope and checked targets',()=>{
 for(const language of ['Rust','JavaScript']){
  const packet=importCase(inputCases.find(entry=>entry.language===language&&entry.name==='run'));
  const program=decodeTypedProgramLinks(LinkNetwork.fromLino(packet.network.toLino()),packet.root);relocate(program);
  const linked=encodeTypedProgramLinks(program),network=LinkNetwork.fromLino(linked.network.toLino()),reloaded=decodeTypedProgramLinks(network,linked.root);
  observe(emitJavaScript(reloaded).definitions.join('\n\n'),'nested.run',[0,2],2);
  assert.deepEqual(reloaded.declarations.get('nested.run').sourceLoop.continuation,{root:'crate',path:['nested','run']});
  assert.ok(emitTypedProgramLinks(network,linked.root,'Rust').code.includes('pub mod nested'));
  const fresh=()=>decodeTypedProgramLinks(LinkNetwork.fromLino(linked.network.toLino()),linked.root);
  for(const mutate of [entry=>{entry.sourceLoop.continuation.path=['run'];},entry=>{entry.sourceLoop.continuation.root='nested';},entry=>{entry.modulePath=['other'];},entry=>{entry.fullName='other.run';},entry=>{visit(entry.body,node=>{if(node.k==='call')node.fn='run';});},entry=>{visit(entry.body,node=>{if(node.k==='call')node.fn='crate.nested.run';});},entry=>{visit(entry.body,node=>{if(node.k==='call')node.fn='nested.other';});}]){const forged=fresh();mutate(forged.declarations.get('nested.run'));assert.throws(()=>encodeTypedProgramLinks(forged));}
 }
});

test('single-slot replay requires the same owned slot and immutable snapshot proof',()=>{
 const refused=[
  ['Rust',rust.replace('(run,)=(run+1f64,);','limit=run+1f64;')],
  ['Rust',rust.replace('(run,)=(run+1f64,);','run=&run;')],
  ['Rust',rust.replace('(run,)=(run+1f64,);','run=Box::new(run);')],
  ['JavaScript',javascript.replace('[run,limit]=[ml_scalar_write0,limit];','run=limit;')],
  ['JavaScript',javascript.replace('[run,limit]=[ml_scalar_write0,limit];','missing=ml_scalar_write0;')],
  ['JavaScript',javascript.replace('const ml_scalar_write0=run+1;','const ordinary=run+1;').replace('[run,limit]=[ml_scalar_write0,limit];','run=ordinary;')],
 ];
 for(const [language,source] of refused)assert.throws(()=>read(source,language));
 assert.throws(()=>checkProgram(parseRust('fn tick(mut tick:f64)->f64{loop{return if tick<2f64{tick=tick+1f64;continue;}else{tick};}}fn main(){}',{scalarLoopModel:legacy})));
});
test('direct scalar graph edits alter transition values while retaining declaration ownership',()=>{
 const packet=importCase(inputCases[0]),program=decodeTypedProgramLinks(LinkNetwork.fromLino(packet.network.toLino()),packet.root);
 let count=0;visit(program.declarations.get('run').body,node=>{if(node.k==='lit'&&node.value==='1'){node.value='2';count++;}});assert.equal(count,1);
 const linked=encodeTypedProgramLinks(program),network=LinkNetwork.fromLino(linked.network.toLino());
 observe(javascript.replace('run+1','run+2'),'run',[0,3],4);
 const actual=emitTypedProgramLinks(network,linked.root,'JavaScript').code;observe(actual,'run',[0,3],4);
 const reverse=read(actual,'JavaScript');observe(emitJavaScript(reverse).text,'run',[0,3],4);
 const emittedRust=emitRust(reverse).definitions.join('\n\n');observe(emitJavaScript(read(emittedRust+'\nfn main(){}','Rust')).text,'run',[0,3],4);
});

test('new shared fixtures retain actual source signatures, snapshots and emitted replay',()=>{
 const fixture=readProjectData(new URL('../parity/fixtures/rust-scalar-loop-declarations.lino',import.meta.url));
 assert.equal(fixture.model,model);assert.equal(fixture.cases.length,inputCases.length);
 for(const [index,entry] of inputCases.entries()){
  const row=fixture.cases[index],surface=(entry.language==='Rust'?parseRust:parseJavaScript)(entry.source,options),program=checkProgram(surface);
  assert.equal(row.source,entry.source);assert.deepEqual(row.surface,JSON.parse(JSON.stringify(surface)));
  const javascript=emitJavaScript(program).definitions.join('\n\n');assert.equal(row.javaScriptDefinitions,javascript);
  assert.equal(row.rustDefinitions,emitRust(program).definitions.join('\n\n'));
  const reverseSurface=parseJavaScript(javascript,options);assert.deepEqual(row.javaScriptReverseSurface,JSON.parse(JSON.stringify(reverseSurface)));
  assert.equal(row.reverseRustDefinitions,emitRust(checkProgram(reverseSurface)).definitions.join('\n\n'));
 }
 const qualified=fixture.qualifiedLoopGraph,program=decodeTypedProgramLinks(LinkNetwork.fromLino(qualified.network),qualified.root);
 assert.equal(emitJavaScript(program).definitions.join('\n\n'),qualified.javaScriptDefinitions);
 assert.equal(emitRust(program).definitions.join('\n\n'),qualified.rustDefinitions);
 observe(qualified.javaScriptDefinitions,'nested.run',[0,2],2);
});

test('source calls, wrong primitive owners and unsupported loop effects remain outside the new profile',()=>{
 const refused=[
  'fn run(mut run:f32,limit:f32)->f32{loop{return if run<limit{(run,)=(run+1f32,);continue;}else{run};}}fn main(){}',
  'fn run(mut run:&f64,limit:f64)->f64{loop{return if *run<limit{(run,)=(run,);continue;}else{*run};}}fn main(){}',
  'mod f64{} fn run(mut run:f64,limit:f64)->f64{loop{return if run<limit{(run,)=(run+1f64,);continue;}else{run};}}fn main(){}',
  'fn run(mut run:f64,limit:f64)->f64{loop{return if run<limit{(run,)=(run+1f64,);continue;}else{crate::run(run,limit)};}}fn main(){}',
  'fn run(mut run:f64,limit:f64)->f64{loop{return if run<limit{println!("x");(run,)=(run+1f64,);continue;}else{run};}}fn main(){}',
  'fn run(mut run:f64,limit:f64)->f64{loop{return if run<limit{loop{continue;}}else{run};}}fn main(){}',
 ];
 for(const source of refused)assert.throws(()=>read(source,'Rust'));
 for(const source of [javascript.replace('run+1','run(run,limit)'),javascript.replace('run+1','external(run)'),javascript.replace('run+1','({value:run})'),javascript.replace('continue;','break;')])assert.throws(()=>read(source,'JavaScript'));
});
