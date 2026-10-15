// JavaScript-first scope: no native implementation or compile-time evaluator is claimed.
import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import {readFileSync,mkdtempSync,rmSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join} from 'node:path';
import {spawnSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {readProjectData} from '../js/scripts/project-data.mjs';
import {parseRust} from '../js/src/translation/rust.js';
import {parseJavaScript} from '../js/src/translation/javascript.js';
import {checkProgram} from '../js/src/translation/check.js';
import {emitRust} from '../js/src/translation/emit-rust.js';
import {emitJavaScript} from '../js/src/translation/emit-javascript.js';
import {createTypedProgramLinks,encodeTypedProgramLinks,decodeTypedProgramLinks,emitTypedProgramLinks} from '../js/src/typed-program-links.js';
import {LinkNetwork} from '../js/src/network.js';
import {DecoratorSet} from '../js/src/decorators.js';
import {pinnedRustConstPredicateDefinitions,rustConstFunctionFixture,readRustConstPredicateSources} from './generate-rust-const-function-fixture.mjs';
import {rustConstFunctionDecorators,rustConstFunctionRules,rustConstFunctionPolicy} from './generate-rust-const-function-rules.mjs';
import {acceptRustConstFunctionOffset,acceptRustConstFunctionNameCharacter} from '../js/src/translation/rust-const-function-rules.js';
const fixture=readProjectData(new URL('./rust-const-function-fixture.lino',import.meta.url));
const loaded=entry=>decodeTypedProgramLinks(LinkNetwork.fromLino(entry.links),entry.root);
const archive=program=>{const packet=encodeTypedProgramLinks(program);return{network:LinkNetwork.fromLino(packet.network.toLino()),root:packet.root};};
function observe(code,entry,edited=false){
 const context={};vm.runInNewContext(code,context,{timeout:1000});const fn=context[entry.name];assert.equal(typeof fn,'function');
 if(entry.name==='accept_rust_owned_local_record_value')for(const first of[false,true])for(const second of[false,true])assert.equal(fn(first,second),edited?!first&&second:!first||second);
 else if(entry.name==='accept_rust_owned_local_source_span')for(const present of[false,true])for(const record of[false,true])for(const offset of[false,true])assert.equal(fn(present,record,offset),!present||record&&offset);
 else for(const start of[-Infinity,-1,-0,0,1,Infinity,NaN])for(const end of[-Infinity,-1,-0,0,1,Infinity,NaN])assert.equal(fn(start,end),start<=end);
 const defaults=entry.name==='accept_rust_owned_local_source_span'?[true,true,true]:entry.name==='accept_source_span_interval'?[0,1]:[true,true];
 for(const bad of[undefined,null,{},new Boolean(false),'false',0n]){const args=[...defaults];args[0]=bad;assert.throws(()=>fn(...args),/requires boolean|requires number/u);}
}

test('three actual styled complete const definitions retain immutable receipts and exact default refusals',()=>{
 assert.deepEqual(rustConstFunctionFixture(),fixture);assert.equal(fixture.cases.length,3);
 const actual=pinnedRustConstPredicateDefinitions();
 for(const[at,entry]of fixture.cases.entries()){
  assert.equal(entry.definition,actual[at].definition);assert.match(entry.definition,/pub const fn /u);assert.equal(createHash('sha256').update(entry.definition).digest('hex'),entry.receipt.sha256);assert.equal(Buffer.byteLength(entry.definition),entry.receipt.bytes);
  assert.throws(()=>checkProgram(parseRust(entry.source)),error=>{assert.equal(error.kind,entry.originalRefusal.kind);assert.equal(error.reason,entry.originalRefusal.reason);assert.deepEqual(error.span,entry.originalRefusal.span);return true;});
  const program=loaded(entry),fn=program.items[0];assert.deepEqual(fn.sourceConstFunction,entry.proof);assert.deepEqual(fn.sourceSignature,entry.signature);
  const proof=fn.sourceConstFunction;assert.equal(entry.source.slice(proof.keywordSpan.start,proof.keywordSpan.end),'const');assert.equal(entry.source.slice(proof.functionKeywordSpan.start,proof.functionKeywordSpan.end),'fn');assert.equal(entry.source[proof.bodySpan.start],'{');assert.equal(entry.source[proof.bodySpan.end-1],'}');
  assert.ok(program.items.every(item=>item.sourceConstFunction));
 }
});

test('complete const source captures authenticate every original byte and refuse receipt edits',()=>{
 const captures=readProjectData(new URL('./rust-const-predicate-source-captures.lino',import.meta.url)),sources=readRustConstPredicateSources(captures);
 assert.equal(sources.length,2);
 for(const source of sources){
  const receipts=fixture.cases.filter(entry=>entry.receipt.commit===source.commit&&entry.receipt.file===source.file);
  assert.ok(receipts.length);
  for(const entry of receipts){assert.equal(source.fileSha256,entry.receipt.fileSha256);assert.equal(source.source.slice(entry.receipt.start,entry.receipt.end),entry.definition);}
 }
 for(const change of[
  value=>value.sources[0].source+='\n',value=>value.sources[0].bytes++,value=>value.sources[0].sha256='0'.repeat(64),
  value=>value.sources[0].revision='0'.repeat(40),value=>value.sources[0].blob='0'.repeat(40),value=>value.sources[0].file='different.rs',
  value=>value.sources[0].source=fixture.cases[0].definition,value=>value.sources.pop(),
 ]){const changed=structuredClone(captures);change(changed);assert.throws(()=>readRustConstPredicateSources(changed),/authenticated Rust const source capture/u);}
});

test('const fixture generation needs neither unpublished Git objects nor a Git executable',()=>{
 const directory=mkdtempSync(join(tmpdir(),'meta-language-const-source-'));
 try{
  const env={...process.env,GIT_OBJECT_DIRECTORY:directory,GIT_ALTERNATE_OBJECT_DIRECTORIES:''};
  for(const commit of['c1e88c8b','87f70e4c']){
   const hidden=spawnSync('git',['cat-file','-e',commit],{cwd:new URL('../',import.meta.url),env,encoding:'utf8'});
   assert.equal(hidden.error,undefined);assert.notEqual(hidden.status,0,'the isolated object directory contains no private source commit');
  }
  const source=`import {rustConstFunctionFixture} from ${JSON.stringify(new URL('./generate-rust-const-function-fixture.mjs',import.meta.url).href)};process.stdout.write(JSON.stringify(rustConstFunctionFixture()));`;
  const generated=spawnSync(process.execPath,['--input-type=module','-e',source],{cwd:directory,env:{...env,PATH:directory},encoding:'utf8',maxBuffer:4*1024*1024,timeout:20000});
  assert.equal(generated.error,undefined);assert.equal(generated.status,0,generated.stderr);assert.deepEqual(JSON.parse(generated.stdout),fixture);
 }finally{rmSync(directory,{recursive:true,force:true});}
});
test('actual styled Rust output replays with its qualification and complete JavaScript output executes',()=>{
 for(const entry of fixture.cases){
  assert.equal(entry.secondRustDefinition,entry.rustDefinition);assert.ok(entry.replaySource.includes(entry.rustDefinition));assert.match(entry.rustDefinition,/^pub const fn /u);
  const replay=checkProgram(parseRust(entry.replaySource,{constFunctionModel:fixture.model}));assert.equal(replay.items[0].sourceConstFunction.keyword,'const');
  observe(entry.javascript,entry);observe(entry.replayJavaScript,entry);
  const packet=archive(loaded(entry));for(const target of['Rust','JavaScript']){const code=emitTypedProgramLinks(packet.network,packet.root,target).code;if(target==='Rust')assert.ok(code.includes(entry.rustDefinition));else observe(code,entry);}
  // Complete generated runtime-shell reimport remains an independent obligation.
  assert.throws(()=>parseRust(emitRust(replay).text,{constFunctionModel:fixture.model}));
 }
});
test('the public factory owns its selected model and user provenance across source-free reload',()=>{
 for(const entry of fixture.cases){
  const packet=createTypedProgramLinks(entry.source,'Rust',{rustConstFunctionModel:fixture.model,provenance:{caller:'const predicate experiment'}}),network=LinkNetwork.fromLino(packet.network.toLino()),program=decodeTypedProgramLinks(network,packet.root);
  assert.deepEqual(program.items[0].sourceConstFunction,entry.proof);observe(emitTypedProgramLinks(network,packet.root,'JavaScript').code,entry);
  const rows=network.fieldRelations(),profile=rows.find(row=>row.label==='source profile');assert.ok(profile);
  assert.ok(rows.some(row=>row.parent.equals(profile.child)&&row.label==='rust const function model'&&network.link(row.child).metadata().definition===fixture.model));
  assert.ok(rows.some(row=>row.label==='caller'&&network.link(row.child).metadata().definition==='const predicate experiment'));
  for(const signature of['','Rust source function signature 1']){
   const selected=createTypedProgramLinks(entry.source,'Rust',{rustConstFunctionModel:fixture.model,rustSourceSignatureModel:signature}),reloaded=LinkNetwork.fromLino(selected.network.toLino()),relations=reloaded.fieldRelations(),profile=relations.find(row=>row.label==='source profile');assert.ok(profile);
   assert.ok(relations.some(row=>row.parent.equals(profile.child)&&row.label==='rust source signature model'&&reloaded.link(row.child).metadata().definition===signature));
   assert.deepEqual(decodeTypedProgramLinks(reloaded,selected.root).items[0].sourceConstFunction,entry.proof);observe(emitTypedProgramLinks(reloaded,selected.root,'JavaScript').code,entry);
  }
  assert.throws(()=>createTypedProgramLinks(entry.source,'Rust'),error=>{assert.equal(error.kind,entry.originalRefusal.kind);assert.equal(error.reason,entry.originalRefusal.reason);assert.deepEqual(error.span,entry.originalRefusal.span);return true;});
 }
 assert.throws(()=>createTypedProgramLinks('function simple(){return true;}','JavaScript',{rustConstFunctionModel:fixture.model}),/requires Rust source/u);
 assert.throws(()=>createTypedProgramLinks(fixture.cases[0].source,'Rust',{rustConstFunctionModel:'unknown'}),/const function model/u);
 assert.throws(()=>createTypedProgramLinks(fixture.cases[0].source,'Rust',{rustConstFunctionModel:fixture.model,rustSourceSignatureModel:'unknown'}),/source signature model/u);
});
test('closed primitive const source boundaries retain exact full-source diagnostics',()=>{
 assert.equal(fixture.invalid.length,27);
 for(const entry of fixture.invalid)assert.throws(()=>checkProgram(parseRust(entry.source,{constFunctionModel:fixture.model})),error=>{assert.equal(error.kind,entry.kind,entry.name);assert.equal(error.reason,entry.reason,entry.name);assert.deepEqual(error.span,entry.span,entry.name);return true;});
 assert.throws(()=>parseRust(fixture.cases[0].source,{constFunctionModel:'unknown'}),/const function model/u);
 for(const option of['localValueStateModel','typeNamespaceModel','completionModel','primitiveOwnerModel','emptyOptionSourceModel','arrayResultModel','scalarLoopModel'])assert.throws(()=>parseRust(fixture.cases[0].source,{constFunctionModel:fixture.model,[option]:'unproved'}),/composition proofs/u);
});
test('source-free actual predicate edits execute while the owned const header and interface remain intact',()=>{
 const entry=fixture.cases[0],program=loaded(entry),fn=program.items[0],proof=structuredClone(fn.sourceConstFunction),signature=structuredClone(fn.sourceSignature);assert.equal(fn.body.op,'or');fn.body.op='and';
 const packet=archive(program),reloaded=decodeTypedProgramLinks(packet.network,packet.root);assert.deepEqual(reloaded.items[0].sourceConstFunction,proof);assert.deepEqual(reloaded.items[0].sourceSignature,signature);
 const code=emitTypedProgramLinks(packet.network,packet.root,'JavaScript').code;observe(code,entry,true);
 const rust=emitRust(reloaded);assert.match(rust.definitions[0],/^pub const fn /u);assert.ok(rust.definitions[0].includes('&&'));
 const replay=checkProgram(parseRust(rust.definitions.join('\n\n')+'\nfn main(){}',{constFunctionModel:fixture.model}));observe(emitJavaScript(replay).text,entry,true);
});
test('owned const headers, primitive owners, expression fields and contextual flags fail closed',()=>{
 const entry=fixture.cases[0];
 const changes=[fn=>fn.sourceConstFunction.keyword='static',fn=>fn.sourceConstFunction.model='unproved',fn=>fn.sourceConstFunction.sourceName='different',fn=>fn.sourceConstFunction.keywordSpan.end++,fn=>fn.sourceConstFunction.functionKeywordSpan.start=fn.sourceConstFunction.keywordSpan.start,fn=>fn.sourceConstFunction.headerSpan.end++,fn=>fn.sourceConstFunction.bodySpan.end=fn.span.end+1,fn=>fn.sourceConstFunction.interpretation.excludedObservations=[],fn=>fn.sourceConstFunction.borrowEscape=false,fn=>fn.sourceSignature.parameters[0].owner.path=['CustomBool'],fn=>fn.params[0].type.kind='float',fn=>fn.body.left.arg.name='foreign',fn=>fn.body.left.arg.type.extra=false,fn=>fn.body.semantics='checked',fn=>fn.body.domain={kind:'float'},fn=>fn.generated=null,fn=>fn.recursive=true,fn=>fn.async=false,fn=>fn.params[0].optional=false,fn=>fn.modulePath=false,fn=>fn.body.span.start=0];
 for(const change of changes){const program=loaded(entry);change(program.items[0]);for(const run of[()=>encodeTypedProgramLinks(program),()=>emitRust(program),()=>emitJavaScript(program)])assert.throws(run,/const|signature|primitive|unrepresented|owned|source/u);}
});
test('source-free binary64 literal spellings preserve finite Rust values and explicit special constants',()=>{
 const fresh=()=>checkProgram(parseRust('const fn predicate(value:f64)->bool{value==1.0}fn main(){}',{constFunctionModel:fixture.model}));
 for(const spelling of['1e999','-1e999']){
  const program=fresh();program.items[0].body.right.value=spelling;
  for(const run of[()=>encodeTypedProgramLinks(program),()=>emitRust(program),()=>emitJavaScript(program)])assert.throws(run,/invalid primitive value/u);
 }
 for(const spelling of['1e308','-0','NaN','Infinity','-Infinity']){
  const program=fresh();program.items[0].body.right.value=spelling;const packet=archive(program),restored=decodeTypedProgramLinks(packet.network,packet.root),code=emitJavaScript(restored).text,context={};vm.runInNewContext(code,context,{timeout:1000});
  const value=Number(spelling);assert.equal(context.predicate(value),value===value);assert.match(emitRust(restored).definitions[0],/^pub const fn /u);
 }
});
test('const marker container validation refuses hidden fields and getters before observing them',()=>{
 const entry=fixture.cases[0];
 for(const role of['program','declarations','items','params','parameter','proof','signature','body']){
  const program=loaded(entry),fn=program.items[0],object={program,declarations:program.declarations,items:program.items,params:fn.params,parameter:fn.params[0],proof:fn.sourceConstFunction,signature:fn.sourceSignature,body:fn.body}[role];let reads=0;
  Object.defineProperty(object,'unrepresented',{enumerable:false,get(){reads++;return false;}});
  for(const action of[()=>encodeTypedProgramLinks(program),()=>emitRust(program),()=>emitJavaScript(program)])assert.throws(action,/unrepresented/u);
  assert.equal(reads,0,role);
 }
});
test('raw source-free const qualification edits refuse before any target emission',()=>{
 const entry=fixture.cases[0],network=LinkNetwork.fromLino(entry.links),rows=network.fieldRelations();const keyword=rows.find(row=>row.label==='keyword'&&network.link(row.child).metadata().definition==='const');assert.ok(keyword);
 const value=network.insertLink([],network.link(keyword.child).metadata().withDefinition('static'));keyword.link.setReferences([keyword.parent,keyword.link.references()[1],value]);const loaded=LinkNetwork.fromLino(network.toLino());
 for(const target of['Rust','JavaScript'])assert.throws(()=>emitTypedProgramLinks(loaded,entry.root,target),/qualification/u);
});
test('the new decisions and selected header decoration are generated from links data',()=>{
 assert.deepEqual(readProjectData(new URL('../js/src/translation/data/rust-const-functions.lino',import.meta.url)),rustConstFunctionPolicy);
 assert.equal(readFileSync(new URL('../js/src/translation/rust-const-function-rules.js',import.meta.url),'utf8'),rustConstFunctionRules());
 assert.equal(readFileSync(new URL('../parity/self-translation/rust-const-function-emission-decorators.lino',import.meta.url),'utf8'),rustConstFunctionDecorators());
 const decorated=DecoratorSet.fromLino(rustConstFunctionDecorators()).decorate('emitter',{format:'Rust',source:'pub fn actual(value: bool) -> bool { value }'});assert.equal(decorated.source,'pub const fn actual(value: bool) -> bool { value }');
 const kernel=checkProgram(parseJavaScript(rustConstFunctionRules())),translated=emitRust(kernel);assert.equal(translated.definitions.length,9);assert.ok(translated.definitions.every(definition=>definition.startsWith('pub fn accept_rust_const_function_')));
 for(const value of[-Infinity,-1,-0,0,.5,1,Number.MAX_SAFE_INTEGER,Number.MAX_SAFE_INTEGER+1,Infinity,NaN])assert.equal(acceptRustConstFunctionOffset(value,Object.is(value,-0)),Number.isSafeInteger(value)&&value>=0&&!Object.is(value,-0));
 for(let code=0;code<128;code++)for(const first of[false,true])assert.equal(acceptRustConstFunctionNameCharacter(code,first),(first?/^[A-Za-z_]$/u:/^[A-Za-z_0-9]$/u).test(String.fromCodePoint(code)));
});
