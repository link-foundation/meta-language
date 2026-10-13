// JavaScript-first bounded proof, executable values and explicit remaining source gaps.
import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {readProjectData} from '../js/scripts/project-data.mjs';
import {rustOwnedLocalStateFixture} from './generate-rust-owned-local-state-fixture.mjs';
import {parseRust} from '../js/src/translation/rust.js';
import {checkProgram} from '../js/src/translation/check.js';
import {emitRust} from '../js/src/translation/emit-rust.js';
import {emitJavaScript} from '../js/src/translation/emit-javascript.js';
import {encodeTypedProgramLinks,decodeTypedProgramLinks,emitTypedProgramLinks,TYPED_PROGRAM_LINKS_SCHEMA} from '../js/src/typed-program-links.js';
import {LinkNetwork} from '../js/src/network.js';
import {LinkMetadata} from '../js/src/primitives.js';

const fixture=readProjectData(new URL('./rust-owned-local-state-fixture.lino',import.meta.url));
function observe(code,entry,rows=entry.observations){const context={};vm.runInNewContext(code,context,{timeout:1000});for(const[text,count,flag,expected]of rows)assert.equal(context[entry.name](text,BigInt(count),flag),expected,entry.name+':'+count);}
const loaded=entry=>decodeTypedProgramLinks(LinkNetwork.fromLino(entry.links),entry.root);
const archived=program=>{const value=encodeTypedProgramLinks(program);return{network:LinkNetwork.fromLino(value.network.toLino()),root:value.root};};

test('five complete sources retain actual String moves, ordered local writes and exact i32 signatures',()=>{
 assert.deepEqual(rustOwnedLocalStateFixture(),fixture);assert.equal(fixture.cases.length,5);
 for(const entry of fixture.cases){
  observe(entry.originalJavaScript,entry);observe(entry.javascript,entry);observe(entry.replayJavaScript,entry);
  const program=loaded(entry),fn=program.items[0],proof=fn.sourceLocalState;
  assert.equal(proof.model,fixture.models.moves);assert.deepEqual(fn.params[1].type,{kind:'fixed',bits:32,signed:true});assert.deepEqual(fn.sourceSignature.parameters[1].type,{kind:'fixed',bits:32,signed:true});
  assert.deepEqual(fn.sourceSignature.parameters[0].owner,{kind:'library',path:['std','string','String'],arguments:[]});assert.equal(proof.initializers.length,1);
  const initializer=proof.bindings.find(binding=>binding.role==='tuple initializer');assert.deepEqual(initializer.type.elements,[{kind:'string'},{kind:'fixed',bits:32,signed:true}]);
  assert.ok(proof.bindings.some(binding=>binding.role==='replacement'));assert.ok(proof.reads.every(read=>read.mode==='move'));
  assert.match(entry.rustDefinitions,/count: i32/u);assert.ok(!entry.rustDefinitions.includes('count: i64'));
  const {network,root}=archived(program);for(const target of['JavaScript','Rust'])assert.ok(emitTypedProgramLinks(network,root,target).code.length);
  assert.ok(network.links().every(link=>!link.metadata().definition?.includes(entry.source)));
 }
});
test('actual canonical Rust output has its distinct standard String copy provenance',()=>{
 for(const entry of fixture.cases){
  assert.throws(()=>checkProgram(parseRust(entry.replaySource,{localValueStateModel:fixture.models.moves})),/move profile|canonical copy interpretation/u);
  const program=checkProgram(parseRust(entry.replaySource,{localValueStateModel:fixture.models.copies}));
  assert.equal(program.items[0].sourceLocalState.model,fixture.models.copies);assert.equal(program.items[0].sourceLocalState.initializers.length,0);
  assert.ok(program.items[0].sourceLocalState.reads.some(read=>read.mode==='copy'));
  assert.deepEqual(program.items[0].sourceSignature.parameters[1].type,{kind:'fixed',bits:32,signed:true});
  observe(emitJavaScript(program).text,entry);
  const again=emitRust(program).definitions.join('\n\n')+'\nfn main(){}';observe(emitJavaScript(checkProgram(parseRust(again,{localValueStateModel:fixture.models.copies}))).text,entry);
 }
});
test('all original profiles retain their refusals and every new move, owner, width and effect boundary refuses',()=>{
 for(const entry of fixture.cases)for(const options of[{}, {completionModel:'closed guarded function returns 1'}])assert.throws(()=>checkProgram(parseRust(entry.source,options)),/mutable tuple binding/u);
 assert.equal(fixture.invalid.length,18);
 for(const entry of fixture.invalid)assert.throws(()=>checkProgram(parseRust(entry.source,{localValueStateModel:fixture.models.moves})),error=>{assert.equal(error.kind,entry.kind,entry.name);assert.equal(error.reason,entry.reason,entry.name);assert.deepEqual(error.span??null,entry.span,entry.name);return true;});
 assert.throws(()=>parseRust(fixture.cases[0].source,{localValueStateModel:'unknown'}),/owned local state model/u);
 assert.throws(()=>parseRust('extern crate alloc;'+fixture.cases[0].source,{localValueStateModel:fixture.models.moves}),/external crate declarations/u);
 const context={};vm.runInNewContext(fixture.cases[0].javascript,context,{timeout:1000});assert.throws(()=>context.select('text',2147483648n,true),/i32 argument count out of range/u);assert.throws(()=>context.select('text',-2147483649n,true),/i32 argument count out of range/u);
});
test('canonical interpretation does not erase partial or conditional moves behind immutable snapshots',()=>{
 for(const body of[
  'let pair=(text,count);let first=pair.0;let second=pair.0;second',
  'let pair=(text,count);let dropped=if flag{pair.0}else{String::from("replacement")};pair.0',
  'let pair=(text,count);let dropped=if flag{text.clone()}else{String::from("replacement")};pair.0',
  'let first={let local=0i32;text};text.clone()',
  'let value=String::from("literal").clone();value',
 ])assert.throws(()=>checkProgram(parseRust(`fn bad(text:String,count:i32,flag:bool)->String{${body}}fn main(){}`,{localValueStateModel:fixture.models.copies})),/moved|standard String clone/u);
 const admitted='fn copy(text:String,count:i32,flag:bool)->String{let pair=(text.clone(),count);if flag{pair.0.clone()}else{pair.0.clone()}}fn main(){}';
 const code=emitJavaScript(checkProgram(parseRust(admitted,{localValueStateModel:fixture.models.copies}))).text;observe(code,{name:'copy',observations:[['Ω😀','0',true,'Ω😀'],['Ω😀','0',false,'Ω😀']]});
});
test('source-free ordering, ownership, width, role, occurrence and interpretation counterfeits refuse',()=>{
 const entry=fixture.cases.find(entry=>entry.name==='select');
 for(const edit of[
  fn=>fn.sourceLocalState.bindings.reverse(),fn=>fn.sourceLocalState.reads.reverse(),
  fn=>fn.sourceLocalState.initializers[0].fields.reverse(),
  fn=>fn.sourceLocalState.bindings.find(binding=>binding.role==='replacement').previous='count',
  fn=>fn.sourceLocalState.bindings.find(binding=>binding.role==='tuple field'&&binding.mutable).mutable=false,
  fn=>fn.sourceLocalState.bindings.find(binding=>binding.role==='join').role='immutable',
  fn=>fn.sourceLocalState.bindings[0].path=['else'],fn=>fn.sourceLocalState.reads[0].path=['value','items',1],
  fn=>fn.sourceLocalState.reads[0].mode='copy',fn=>fn.sourceLocalState.interpretation.integerDomain.bits=64,
  fn=>fn.params[1].type.bits=64,fn=>fn.sourceSignature.parameters[1].owner.path=['i64'],
  fn=>fn.sourceLocalState.model=fixture.models.copies,fn=>fn.sourceLocalState.sourceName='counterfeit',
  fn=>fn.sourceLocalState.reads=[],fn=>fn.generated=null,fn=>fn.async=false,
  fn=>fn.params[1].optional=false,fn=>fn.params[1].span=null,
 ]){const program=loaded(entry);edit(program.items[0]);assert.throws(()=>encodeTypedProgramLinks(program),/state|signature|unrepresented|source/i);}
 const program=loaded(entry);let node=program.items[0].body;while(node.k==='let'&&node.value.k!=='if')node=node.body;assert.equal(node.value.then.value.k,'lit');node.value.then.value.value='\ud800';assert.throws(()=>encodeTypedProgramLinks(program),/Unicode scalar/u);
 const network=LinkNetwork.fromLino(entry.links),variable=network.fieldRelations().find(row=>row.label===TYPED_PROGRAM_LINKS_SCHEMA.fields.k&&network.link(row.child).metadata().definition==='var').parent;
 const value=network.insertLink([],LinkMetadata.new().withLanguage(TYPED_PROGRAM_LINKS_SCHEMA.language).withNamed(true).withTerm(TYPED_PROGRAM_LINKS_SCHEMA.scalars.boolean).withDefinition('true'));network.insertFieldRelation(variable,'borrowEscape',value);
 const reloaded=LinkNetwork.fromLino(network.toLino());for(const target of['JavaScript','Rust'])assert.throws(()=>emitTypedProgramLinks(reloaded,entry.root,target),/state|unrepresented|unsupported/u);
});
test('a source-free String replacement edit executes with unchanged actual owner and integer width',()=>{
 const entry=fixture.cases.find(entry=>entry.name==='select'),program=loaded(entry),fn=program.items[0],before=structuredClone(fn.sourceSignature),proof=structuredClone(fn.sourceLocalState);
 let node=fn.body;while(node.k==='let'&&node.value.k!=='if')node=node.body;assert.equal(node.value.then.value.k,'lit');node.value.then.value.value='edited';
 const {network,root}=archived(program),reloaded=decodeTypedProgramLinks(network,root);assert.deepEqual(reloaded.items[0].sourceSignature,before);assert.deepEqual(reloaded.items[0].sourceLocalState,proof);
 observe(emitTypedProgramLinks(network,root,'JavaScript').code,entry,[['seed','0',true,'edited'],['seed','0',false,'7']]);
 const actualRust=emitTypedProgramLinks(network,root,'Rust').code;assert.match(actualRust,/count: i32/u);
 assert.throws(()=>checkProgram(parseRust(actualRust,{localValueStateModel:fixture.models.copies})),/function without a result: ml_main/u);
 const definitions=emitRust(reloaded).definitions.join('\n\n')+'\nfn main(){}';assert.ok(actualRust.includes(emitRust(reloaded).definitions[0]));observe(emitJavaScript(checkProgram(parseRust(definitions,{localValueStateModel:fixture.models.copies}))).text,entry,[['seed','0',true,'edited']]);
});
test('authentic complete binary formatter source remains pinned and still has its actual mutable tuple gap',()=>{
 const sources=fixture.authentic.files.map(file=>{const bytes=readFileSync(new URL('../'+file.file,import.meta.url));assert.equal(bytes.length,file.bytes);assert.equal(createHash('sha256').update(bytes).digest('hex'),file.sha256);return bytes.toString('utf8');}),source=sources.join('\n');
 assert.ok(source.includes('let (mut digits, n) = decimal(&format!("{:e}", x.abs()));'));assert.ok(source.includes('digits = other.trim_end_matches(\'0\').to_string();'));
 for(const entry of fixture.authentic.diagnostics)assert.throws(()=>checkProgram(parseRust(source,entry.options)),error=>{assert.equal(error.reason,entry.reason);assert.deepEqual(error.span,entry.span);return true;});
 const selected=fixture.authentic.diagnostics.find(entry=>entry.options.completionModel);assert.equal(selected.reason,'mutable tuple binding: mutation is outside the immutable product model');assert.deepEqual(selected.span,{start:935,end:938});
 assert.throws(()=>checkProgram(parseRust(source,{localValueStateModel:fixture.models.moves})),/direct standard String, i32 and Boolean interfaces/u);
});
