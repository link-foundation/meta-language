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
import {acceptRustOwnedLocalTextInput,renderRustOwnedLocalTextInput} from '../js/src/translation/rust-owned-local-state-rules.js';

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
  for(const constructor of proof.literalConstructors){assert.deepEqual(constructor.sourceOwner,['String']);assert.equal(constructor.constructorOperation,'from');assert.match(entry.source.slice(constructor.callSpan.start,constructor.callSpan.end),/^String::from\("[^"]*"\)$/u);assert.match(entry.source.slice(constructor.span.start,constructor.span.end),/^"[^"]*"$/u);}
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
test('borrowed str literals cannot counterfeit owned String source values or constructor sites',()=>{
 assert.equal(fixture.borrowedStringRefusals.length,7);
 for(const entry of fixture.borrowedStringRefusals)assert.throws(()=>checkProgram(parseRust(entry.source,{localValueStateModel:entry.model})),error=>{assert.equal(error.kind,entry.kind,entry.name);assert.equal(error.reason,entry.reason,entry.name);assert.deepEqual(error.span??null,entry.span,entry.name);return true;});
 const entry=fixture.cases.find(entry=>entry.name==='select');
 for(const edit of[
  fn=>fn.sourceLocalState.literalConstructors=[],
  fn=>fn.sourceLocalState.literalConstructors[0].constructorOperation='borrow',
  fn=>fn.sourceLocalState.literalConstructors[0].sourceOwner=['CustomString'],
  fn=>fn.sourceLocalState.literalConstructors[0].owner.path=['String'],
  fn=>fn.sourceLocalState.literalConstructors[0].inputWrappers=[],
  fn=>fn.sourceLocalState.literalConstructors[0].inputOwner.path=['String'],
  fn=>fn.sourceLocalState.literalConstructors[0].path=['value','items',0],
  fn=>fn.sourceLocalState.literalConstructors[0].span.start++,
  fn=>fn.sourceLocalState.literalConstructors[0].callSpan.start=fn.sourceLocalState.literalConstructors[0].span.end,
  fn=>fn.sourceLocalState.literalConstructors[0].callSpan={start:0,end:1000000},
  fn=>fn.sourceLocalState.literalConstructors[0].callSpan.end=fn.span.end+1,
  fn=>fn.sourceLocalState.literalConstructors[0].callSpan={...fn.sourceLocalState.literalConstructors[0].span},
  fn=>fn.span=null,
  fn=>fn.span.end=fn.span.start,
 ]){const program=loaded(entry);edit(program.items[0]);assert.throws(()=>encodeTypedProgramLinks(program),/constructor|source-owner interval/u);}
 const archive=archived(loaded(entry)),rows=archive.network.fieldRelations(),call=rows.find(row=>row.label==='call span');
 assert.ok(call);const end=rows.find(row=>row.parent.equals(call.child)&&row.label==='end');assert.ok(end);
 const outside=archive.network.insertLink([],archive.network.link(end.child).metadata().withDefinition('1000000'));end.link.setReferences([end.parent,end.link.references()[1],outside]);
 const reloadedNetwork=LinkNetwork.fromLino(archive.network.toLino());for(const target of['JavaScript','Rust'])assert.throws(()=>emitTypedProgramLinks(reloadedNetwork,archive.root,target),/constructor/u);
});
test('original immutable source names retain distinct checked binder ownership after LiNo reload',()=>{
 const source='fn replacement(text:String,count:i32,flag:bool)->String{let(mut digits,n)=(text,count);let constant=String::from("constant");digits=constant;digits}fn main(){}';
 const program=checkProgram(parseRust(source,{localValueStateModel:fixture.models.moves}));
 const before=archived(program),reloaded=decodeTypedProgramLinks(before.network,before.root);
 observe(emitJavaScript(reloaded).text,{name:'replacement',observations:[['Ω😀','0',false,'constant']]});
 for(const name of['digits','text','renamed']){
  const changed=decodeTypedProgramLinks(before.network,before.root);changed.items[0].sourceLocalState.bindings.find(binding=>binding.name==='constant').sourceName=name;
  assert.throws(()=>encodeTypedProgramLinks(changed),/distinct actual source binder/u);
 }
 for(const name of['digits','text']){
  const network=LinkNetwork.fromLino(before.network.toLino()),rows=network.fieldRelations();
  const relation=rows.find(row=>row.label==='source name'&&network.link(row.child).metadata().definition==='constant'&&rows.some(other=>other.parent.equals(row.parent)&&other.label==='role'&&network.link(other.child).metadata().definition==='immutable'));
  assert.ok(relation);const child=network.insertLink([],network.link(relation.child).metadata().withDefinition(name));relation.link.setReferences([relation.parent,relation.link.references()[1],child]);
  const reloadedNetwork=LinkNetwork.fromLino(network.toLino());for(const target of['JavaScript','Rust'])assert.throws(()=>emitTypedProgramLinks(reloadedNetwork,before.root,target),/distinct actual source binder/u);
 }
 const entry=fixture.cases.find(entry=>entry.name==='select');
 const changed=loaded(entry),fact=changed.items[0].sourceLocalState.bindings.find(binding=>binding.role==='tuple field');fact.sourceName='text';
 assert.throws(()=>encodeTypedProgramLinks(changed),/distinct actual source binder/u);
});
test('selected text inputs are Unicode scalars before ordered state evaluation and helper names stay hygienic',()=>{
 const {validText:valid,invalidText:invalid}=fixture.inputValues;
 const context={};vm.runInNewContext(renderRustOwnedLocalTextInput(),context,{timeout:1000});
 for(const text of[...valid,...invalid]){
  const independentlyValid=Array.from(text).every(character=>{const point=character.codePointAt(0);return point<0xd800||point>0xdfff;});
  assert.equal(acceptRustOwnedLocalTextInput(text),independentlyValid);assert.equal(context.ml_ownedTextInput(text),independentlyValid);
 }
 for(const text of[null,undefined,0,{},new String('owned')])assert.equal(acceptRustOwnedLocalTextInput(text),false);
 for(const entry of fixture.cases){
  const archive=archived(loaded(entry));
  for(const javascript of[entry.javascript,entry.replayJavaScript,emitTypedProgramLinks(archive.network,archive.root,'JavaScript').code]){
   const runtime={};vm.runInNewContext(javascript,runtime,{timeout:1000});
   for(const text of invalid)assert.throws(()=>runtime[entry.name](text,0n,true),/requires Unicode scalar text/u);
   assert.throws(()=>runtime[entry.name](0,0n,true),/requires Unicode scalar text/u);
   assert.throws(()=>runtime[entry.name]('text',0,true),/requires bigint/u);
   assert.throws(()=>runtime[entry.name]('text',0n,1),/requires boolean/u);
  }
 }
 const source='fn hygiene_input(ml_ownedTextInput:String,count:i32,flag:bool)->String{let(mut digits,n)=(ml_ownedTextInput,count);digits=String::from("hygiene");digits}fn main(){}';
 const emission=emitJavaScript(checkProgram(parseRust(source,{localValueStateModel:fixture.models.moves}))),name=emission.mappings.find(mapping=>mapping.source==='hygiene_input').target;
 assert.match(emission.text,/function hygiene_input\(ml_ownedTextInput_2,/u);assert.ok(emission.assumptions.some(assumption=>assumption.id==='owned-local-text-input-realm'));observe(emission.text,{name,observations:[['😀','0',true,'hygiene']]});
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
