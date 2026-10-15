import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import {readProjectData} from '../scripts/project-data.mjs';
import {rustRuntimeShellFixture,RUST_RUNTIME_SHELL_MODEL} from '../scripts/generate-rust-runtime-shell-fixtures.mjs';
import {createTypedProgramLinks,decodeTypedProgramLinks,encodeTypedProgramLinks,emitTypedProgramLinks,LinkNetwork} from '../src/index.js';
import {emitRust} from '../src/translation/emit-rust.js';
import {parseRust} from '../src/translation/rust.js';
import {recheckLinkedTypedProgram} from '../src/typed-program-checked-projection.js';
import {RUST_RUNTIME_SHELL_POLICY as policy} from '../src/translation/rust-runtime-shell.js';
const fixture=readProjectData(new URL('../../parity/fixtures/rust-runtime-shell.lino',import.meta.url));
const options={rustRuntimeShellModel:RUST_RUNTIME_SHELL_MODEL};
const archive=packet=>({network:LinkNetwork.fromLino(packet.network.toLino()),root:packet.root});
const load=entry=>archive(createTypedProgramLinks(entry.fullEmittedRust,'Rust',options));
function example(){const packet=load(fixture.cases.find(entry=>entry.id==='ordered-repeated'));return decodeTypedProgramLinks(packet.network,packet.root);}
function execute(code){const output=[],context=vm.createContext({console:{log:(...args)=>output.push(args.join(' '))}});vm.runInContext(code,context,{timeout:1000});return {output:output.join('\n'),context};}
function operations(node,result=[]){if(node?.kind==='source operation'){result.push(node);node.parts.forEach(child=>operations(child,result));}return result;}
function observe(packet){const program=decodeTypedProgramLinks(packet.network,packet.root),emitted=emitTypedProgramLinks(packet.network,packet.root,'JavaScript'),target=execute(emitted.code),calls=[];for(const name of program.declarations.keys())target.context[name]=()=>{calls.push(name);return null;};vm.runInContext('main();',target.context,{timeout:1000});return {calls,output:target.output,emitted};}
function assertRefused(program,message){assert.throws(()=>encodeTypedProgramLinks(program),undefined,message);assert.throws(()=>recheckLinkedTypedProgram(program,{wholeProgram:true}),undefined,message+' source-free checker');}

test('eight complete actual emitter artifacts save, reload and execute all ordered Unit calls across three whole-artifact replays',()=>{
 assert.deepEqual(rustRuntimeShellFixture(),fixture);assert.equal(fixture.cases.length,8);
 for(const entry of fixture.cases){
  assert.equal(execute(entry.original).output,entry.output,entry.id+' original full input');
  const original=createTypedProgramLinks(entry.original,'JavaScript');assert.equal(emitRust(decodeTypedProgramLinks(original.network,original.root)).text,entry.fullEmittedRust);
  let packet=load(entry);
  for(let cycle=0;cycle<3;cycle++){
   const program=decodeTypedProgramLinks(packet.network,packet.root),proof=program.sourceRuntimeShell,observed=observe(packet);
   assert.deepEqual([...program.declarations.keys()],entry.names);assert.deepEqual(program.main.effects.map(effect=>effect.value.fn),entry.calls);assert.deepEqual(observed.calls,entry.calls);assert.equal(observed.output,entry.output);
   assert.equal(proof.span.start,0);assert.equal(proof.tokens.at(-1).kind,'eof');assert.equal(proof.tokens.at(-1).span.end,proof.span.end);assert.equal(proof.model,fixture.model);assert.equal(proof.observationModel,fixture.observationModel);
   const nodes=operations(proof.operations);for(const requirement of policy.assumptions){const saved=proof.assumptions.find(row=>row.id===requirement.id);assert.equal(saved.meaning,requirement.meaning);assert.ok(nodes.includes(saved.operationNode));assert.equal(saved.operationNode.op,requirement.operation);}
   assert.equal(observed.emitted.assumptions.length,4);assert.deepEqual(observed.emitted.assumptions.map(row=>row.id),policy.assumptions.map(row=>row.id));assert.ok(observed.emitted.assumptions.every(row=>row.details.length===1));
   const actualWholeArtifact=emitTypedProgramLinks(packet.network,packet.root,'Rust');
   for(const fragment of ['std::env::args().any','std::thread::Builder::new()','.stack_size(1 << 28)','.spawn(move ||','.expect("spawn the program thread")','worker.join().is_err()','std::process::exit(101);'])assert.ok(actualWholeArtifact.code.includes(fragment),fragment);
   assert.equal(actualWholeArtifact.assumptions.length,4);packet=archive(createTypedProgramLinks(actualWholeArtifact.code,'Rust',options));
  }
 }
});

test('every wrapper operation, namespace owner, literal and original concrete interval is inspectable without an opaque source payload',()=>{
 const entry=fixture.cases.find(row=>row.id==='ordered-repeated'),program=example(),proof=program.sourceRuntimeShell,nodes=operations(proof.operations);
 assert.equal(proof.span.end,entry.fullEmittedRust.length);assert.equal(proof.tokens.at(-1).span.end,entry.fullEmittedRust.length);
 for(const label of ['inert runtime lint attribute','owned Unit worker definition','complete standard runtime entry','standard argument owner','standard argument iteration','standard argument existence test','closed argument predicate','standard argument flag comparison','standard thread builder owner','standard thread builder construction','bounded stack size request','unsigned stack size shift','closed moved Unit worker','owned worker invocation','standard Unit thread spawn','standard thread success requirement','standard worker join','standard worker failure predicate','retained worker failure branch','standard exit owner','standard worker failure exit'])assert.ok(nodes.some(node=>node.op===label),label);
 for(const token of proof.tokens){assert.ok(token.span.start>=0&&token.span.end<=proof.span.end);if(token.kind==='identifier')assert.equal(entry.fullEmittedRust.slice(token.span.start,token.span.end),token.alias??token.value);}
 assert.equal(nodes.filter(node=>node.op==='owned public Unit helper').length,2);assert.equal(nodes.filter(node=>node.op==='owned Unit invocation').length,3);
 const declarations=proof.tokens.filter(token=>Object.hasOwn(token,'declaration'));assert.equal(declarations.length,5);for(const token of declarations)assert.equal(program.declarations.get(token.declaration.fullName),token.declaration);
 assert.ok(!Object.values(proof).includes(entry.fullEmittedRust));assert.equal(Object.hasOwn(program,'sourcePureUnitCalls'),false);assert.equal(proof.assumptions.some(row=>/failure behavior/u.test(row.meaning)),true);
});

test('coherent source-free helper, lexical-binding and resource edits recheck the whole shell and produce the edited complete artifact',()=>{
 const program=example(),proof=program.sourceRuntimeShell,alpha=program.declarations.get('alpha');
 program.declarations.delete('alpha');alpha.name='gamma';alpha.fullName='gamma';program.declarations.set('gamma',alpha);program.declarations=new Map(program.items.map(item=>[item.fullName,item]));for(const effect of program.main.effects)if(effect.value.fn==='alpha')effect.value.fn='gamma';
 proof.tokens.find(token=>token.kind==='identifier'&&token.value==='worker').value='runner';operations(proof.operations).find(node=>node.op==='unsigned stack size shift').parts.at(-1).value='20';
 const packet=archive(encodeTypedProgramLinks(program));assert.deepEqual(observe(packet).calls,['gamma','beta','gamma']);const actual=emitTypedProgramLinks(packet.network,packet.root,'Rust');
 for(const fragment of ['pub fn gamma(','crate::gamma()','let runner =','runner.join()','.stack_size(1 << 20)'])assert.ok(actual.code.includes(fragment),fragment);assert.ok(!actual.code.includes('1 << 28'));
 const replay=archive(createTypedProgramLinks(actual.code,'Rust',options));assert.deepEqual(observe(replay).calls,['gamma','beta','gamma']);assert.equal(observe(replay).emitted.assumptions.length,4);
});

test('independent complete-shell interpretation preserves every previous default and Unit refusal',()=>{
 const source=fixture.cases.find(row=>row.id==='actual-invocation').fullEmittedRust;
 assert.throws(()=>createTypedProgramLinks(source,'Rust'));for(const model of ['closed pure Rust unit calls 1','closed public pure Rust unit calls 1'])assert.throws(()=>createTypedProgramLinks(source,'Rust',{rustPureUnitCallModel:model}),/attributes require a separate compilation context proof/u);
 for(const extra of [{rustPureUnitCallModel:'closed public pure Rust unit calls 1'},{rustTypeNamespaceModel:'resolved Rust type namespace 1'},{rustOwnedLocalStateModel:'closed owned text local state 1'}])assert.throws(()=>createTypedProgramLinks(source,'Rust',{...options,...extra}),/independent source profile/u);
 assert.throws(()=>createTypedProgramLinks('function empty(){}','JavaScript',options),/requires Rust source/u);assert.throws(()=>parseRust(source,{runtimeShellModel:'unknown'}),/runtime shell model/u);
 const packet=load(fixture.cases[2]);assert.throws(()=>emitTypedProgramLinks(packet.network,packet.root,'Rust',{rustSourceHeaderEmissionModel:'Rust source function headers 1'}),/independent runtime composition proof/u);assert.throws(()=>emitTypedProgramLinks(packet.network,packet.root,'JavaScript',{javaScriptScopedBindingEmissionModel:'source scoped JavaScript bindings 1'}),/independent runtime composition proof/u);
});

test('actual namespace changes, borrowed operations, captured context and unrepresented whole-source operations are refused',()=>{
 const source=fixture.cases.find(row=>row.id==='actual-invocation').fullEmittedRust;
 const replacements=[['std::env::args','other::env::args'],['pub fn empty','pub fn std'],['crate::empty()','crate::missing()'],['-> ()','-> &()'],['crate::empty()','&crate::empty()'],['move ||','move |borrow: &()|'],['move || { ml_main() }','move || { check }'],['move ||','||'],['crate::empty();','crate::empty(); std::process::exit(0);'],['invalid_nan_comparisons','unsafe_code'],['exit(101)','exit(0)'],['argument ==','argument !='],['1 << 28','1 < < 28'],['1 << 28','1 << 15'],['1 << 28','1 << 64'],['crate::empty()','crate::empty(())'],['ml_main() }','return ml_main() }'],['#![allow','#![no_std]\n#![allow'],['pub fn empty','fn empty'],['fn ml_main','pub fn ml_main'],['fn main()','pub fn main()']];
 const mutations=replacements.map(([before,after])=>[before+' -> '+after,source.replace(before,after)]);mutations.push(['extra whole function',source+'\nfn hidden(){}'],['extra import','use std::thread;\n'+source]);
 for(const [id,changed]of mutations){assert.notEqual(changed,source,id+' actual mutation');assert.throws(()=>createTypedProgramLinks(changed,'Rust',options),undefined,id);}
});

test('source-free semantic, provenance, namespace, borrow and complete-context corruption cannot bypass the real core or shell checker',()=>{
 const changes=[
  p=>{p.main.effects[0].value.fn='beta';},p=>{p.sourceRuntimeShell.tokens.find(token=>token.declaration?.name==='alpha'&&token.span.start>p.items[1].span.end).declaration=p.declarations.get('beta');},
  p=>{p.items[0].ret={kind:'bool'};},p=>{p.items[0].body={k:'bool',value:true,type:{kind:'bool'}};},p=>{p.items[0].params.push({name:'borrow',type:{kind:'unit'}});},p=>{p.main.effects.push({...p.main.effects[0]});},p=>{p.imports=[{path:['std']}];},p=>{p.externals=[{k:'fn',name:'foreign'}];},p=>{p.referenceSemantics=true;},p=>{p.outputThreaded=true;},p=>{p.items[0].generated=true;},p=>{p.items[0].sourcePureUnit={model:'closed public pure Rust unit calls 1'};},
  p=>{p.sourceRuntimeShell.model='closed pure Rust unit runtime shell 2';},p=>{p.sourceRuntimeShell.observationModel='unconditional equivalence';},p=>{p.sourceRuntimeShell.tokens.find(token=>token.value==='std').value='bad';},p=>{p.sourceRuntimeShell.tokens.find(token=>token.value==='move').value='copy';},p=>{p.sourceRuntimeShell.tokens.pop();},p=>{p.sourceRuntimeShell.tokens.push(p.sourceRuntimeShell.tokens[0]);},p=>{p.sourceRuntimeShell.operations.parts.pop();},p=>{p.sourceRuntimeShell.assumptions.pop();},p=>{p.sourceRuntimeShell.assumptions[0].meaning='unconditional success';},p=>{p.sourceRuntimeShell.assumptions[0].operationNode=structuredClone(p.sourceRuntimeShell.assumptions[0].operationNode);},p=>{p.sourceRuntimeShell.tokens.find(token=>token.binding).binding=structuredClone(p.sourceRuntimeShell.tokens.find(token=>token.binding).binding);},p=>{p.sourceRuntimeShell.tokens.find(token=>token.declaration).declaration=structuredClone(p.items[0]);},
  p=>{p.sourceRuntimeShell.tokens[0].span.start=-0;},p=>{p.sourceRuntimeShell.tokens[0].span.start=0.5;},p=>{p.sourceRuntimeShell.tokens[0].span.end=Number.MAX_SAFE_INTEGER+1;},p=>{p.sourceRuntimeShell.tokens[1].span.start=p.sourceRuntimeShell.tokens[0].span.start;},p=>{p.sourceRuntimeShell.tokens.at(-1).span.end++;},p=>{p.sourceRuntimeShell.span.start=1;},p=>{p.sourceRuntimeShell.operations.span.end--;},p=>{p.main.span.end--;}
 ];
 for(const [index,change]of changes.entries()){const p=example();change(p);assertRefused(p,'owned semantic corruption '+index);}
});

test('owned containers reject accessors, hidden fields, prototypes and sparse arrays before invoking host callbacks',()=>{
 const targets=[p=>p,p=>p.sourceRuntimeShell,p=>p.sourceRuntimeShell.tokens[0],p=>p.sourceRuntimeShell.tokens[0].span,p=>p.sourceRuntimeShell.operations,p=>p.sourceRuntimeShell.assumptions[0],p=>p.items[0],p=>p.items[0].body,p=>p.main.effects[0]];let invoked=0;
 for(const [index,target]of targets.entries())for(const mutate of [node=>Object.defineProperty(node,'host',{get(){invoked++;return 0;},enumerable:true}),node=>Object.defineProperty(node,'hidden',{value:1}),node=>{node[Symbol('host')]=1;},node=>Object.setPrototypeOf(node,{inherited:1})]){const p=example();mutate(target(p));assertRefused(p,'owned record '+index);}
 for(const mutate of [p=>{p.sourceRuntimeShell.tokens.extra=1;},p=>{delete p.sourceRuntimeShell.tokens[0];},p=>{p.declarations.extra=1;},p=>Object.setPrototypeOf(p.sourceRuntimeShell.tokens,{}),p=>{p.sourceRuntimeShell.assumptions[0].operationNode=p.sourceRuntimeShell;}]){const p=example();mutate(p);assertRefused(p,'owned ordered container');}assert.equal(invoked,0);
});

test('source span record field order is label based and complete artifact declaration and lexical references preserve their actual identities',()=>{
 const p=example();p.sourceRuntimeShell.span={end:p.sourceRuntimeShell.span.end,start:0};for(const token of p.sourceRuntimeShell.tokens)token.span={end:token.span.end,start:token.span.start};const packet=archive(encodeTypedProgramLinks(p));assert.deepEqual(observe(packet).calls,['alpha','beta','alpha']);
 const decoded=decodeTypedProgramLinks(packet.network,packet.root);assert.equal(decoded.sourceRuntimeShell.tokens.filter(token=>token.declaration===decoded.declarations.get('alpha')).length,3);assert.equal(decoded.sourceRuntimeShell.tokens.filter(token=>token.binding).length,3);
});

test('actual serialized Links resource edits and malformed saved model, namespace, borrowed syntax and context remain observable on reload',()=>{
 const entry=fixture.cases.find(row=>row.id==='ordered-repeated');
 const changed=load(entry),shift=changed.network.links().find(link=>link.metadata().term==='string value'&&link.metadata().definition==='28');assert.ok(shift);shift.setMetadata(shift.metadata().withDefinition('20'));const saved=archive(changed);assert.deepEqual(observe(saved).calls,entry.calls);assert.ok(emitTypedProgramLinks(saved.network,saved.root,'Rust').code.includes('1 << 20'));
 for(const [before,after]of [[fixture.model,'unknown model'],['std','bad'],['move','borrow'],['standard argument iteration','unowned operation'],['101','0']]){
  const packet=load(entry),value=packet.network.links().find(link=>link.metadata().term==='string value'&&link.metadata().definition===before);assert.ok(value,before);value.setMetadata(value.metadata().withDefinition(after));const raw=archive(packet);for(const target of ['Rust','JavaScript'])assert.throws(()=>emitTypedProgramLinks(raw.network,raw.root,target),undefined,before+' raw linked corruption');
 }
});


test('actual source lexical separators cannot be erased by forged saved spans while comment-separated words remain supported',()=>{
 const p=example(),tokens=p.sourceRuntimeShell.tokens,index=tokens.findIndex(token=>token.value==='pub');tokens[index+1].span={start:tokens[index].span.end,end:tokens[index].span.end+2};assertRefused(p,'forged pub/fn lexical adjacency');
 const actual=fixture.cases.find(row=>row.id==='unused-helper').fullEmittedRust.replace('pub fn','pub/**/fn');const packet=archive(createTypedProgramLinks(actual,'Rust',options));const emitted=emitTypedProgramLinks(packet.network,packet.root,'Rust');assert.ok(emitted.code.includes('pub fn empty'));assert.doesNotThrow(()=>createTypedProgramLinks(emitted.code,'Rust',options));
});


test('a surviving actual worker context cannot lose its complete runtime operations or conditional assumptions through root-only removal',()=>{
 for(const change of [p=>{delete p.sourceRuntimeShell;},p=>{delete p.main.sourceRuntimeWorker;},p=>{p.main.sourceRuntimeWorker.model='unknown';},p=>{p.main.sourceRuntimeWorker.observationModel='unconditional';},p=>{p.main.sourceRuntimeWorker.span={start:0,end:1};},p=>{p.sourceRuntimeShell.tokens.find(token=>token.declaration).alias='!bad!';}]){const p=example();change(p);assertRefused(p,'owned runtime context removal or mismatch');}
 const p=example();assert.equal(p.main.sourceRuntimeWorker.span,p.main.span);assert.equal(p.main.sourceRuntimeWorker.model,fixture.model);assert.equal(p.main.sourceRuntimeWorker.observationModel,fixture.observationModel);
});


test('source-free rechecking preserves negative-zero refusal on every derived operation interval',()=>{
 const entry=fixture.cases[0],packet=load(entry),actual=emitTypedProgramLinks(packet.network,packet.root,'Rust'),again=createTypedProgramLinks(actual.code,'Rust',options),p=decodeTypedProgramLinks(again.network,again.root);assert.equal(p.sourceRuntimeShell.operations.span.start,0);p.sourceRuntimeShell.operations.span.start=-0;assertRefused(p,'derived operation source negative zero');
});


test('fixed ASCII runtime string facts require enough original extent for their actual scalar contents and delimiters',()=>{
 const p=example(),string=p.sourceRuntimeShell.tokens.find(token=>token.kind==='string');string.span.start=string.span.end-1;assertRefused(p,'impossible runtime string source extent');
});
