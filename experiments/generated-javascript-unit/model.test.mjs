import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import {readProjectData} from '../../js/scripts/project-data.mjs';
import {createTypedProgramLinks,decodeTypedProgramLinks,encodeTypedProgramLinks,emitTypedProgramLinks,LinkNetwork} from '../../js/src/index.js';
import {checkProgram} from '../../js/src/translation/check.js';
import {parseRust} from '../../js/src/translation/rust.js';
import {RUST_RUNTIME_SHELL_POLICY as runtimePolicy} from '../../js/src/translation/rust-runtime-shell.js';
import {generatedJavaScriptUnitFixture} from './generate-fixtures.mjs';
import {GENERATED_JAVASCRIPT_UNIT_POLICY as policy,createGeneratedJavaScriptUnitLinks,checkGeneratedJavaScriptUnit,emitGeneratedJavaScriptUnit,emitGeneratedJavaScriptUnitLinks} from './model.mjs';
const fixture=readProjectData(new URL('./data/fixtures.lino',import.meta.url));
const options={rustRuntimeShellModel:runtimePolicy.model};
const archive=packet=>({network:LinkNetwork.fromLino(packet.network.toLino()),root:packet.root});
const load=source=>archive(createGeneratedJavaScriptUnitLinks(source));
const read=packet=>decodeTypedProgramLinks(packet.network,packet.root);
const source=fixture.cases.find(row=>row.id==='ordered-repeated').fullEmittedJavaScript;
const example=()=>read(load(source));
const plain="'use strict'; function leaf(){return null;} function main(){const ignored=leaf();} main();";
function execute(code){const output=[],context=vm.createContext({console:{log:(...args)=>output.push(args.join(' '))}});vm.runInContext(code,context,{timeout:1000});return {output:output.join('\n'),context};}
function observe(code,names){const actual=execute(code),calls=[];for(const name of names){assert.equal(actual.context[name](),null,name+' returns actual singleton null');actual.context[name]=()=>{calls.push(name);return null;};}vm.runInContext('main();',actual.context,{timeout:1000});return {output:actual.output,calls};}
function operations(node,result=[]){if(node?.kind==='source operation'){result.push(node);node.parts.forEach(child=>operations(child,result));}return result;}
function refused(program,message){assert.throws(()=>checkGeneratedJavaScriptUnit(program),undefined,message+' checker');for(const target of ['Rust','JavaScript'])assert.throws(()=>emitGeneratedJavaScriptUnit(program,target),undefined,message+' '+target);}
function savedRefused(program,message){refused(program,message);let packet;try{packet=archive(encodeTypedProgramLinks(program));}catch{return;}for(const target of ['Rust','JavaScript'])assert.throws(()=>emitGeneratedJavaScriptUnitLinks(packet.network,packet.root,target),undefined,message+' actual saved '+target);}
function serialize(program){checkGeneratedJavaScriptUnit(program);return archive(encodeTypedProgramLinks(program));}

// These are experiment assertions, not issue-195 observations or support claims.
test('eight actual complete generated JavaScript modules perform three complete JS to Rust to JS code cycles',()=>{
 assert.deepEqual(generatedJavaScriptUnitFixture(),fixture);assert.equal(fixture.cases.length,8);
 for(const row of fixture.cases){let actualJavaScript=row.fullEmittedJavaScript;const observationWitness=[row.incomingRustAssumptions];
  assert.equal(execute(row.original).output,row.output);assert.equal(observationWitness[0].length,4);
  for(let cycle=0;cycle<3;cycle++){
   const packet=load(actualJavaScript),program=read(packet),proof=program.sourceGeneratedJavaScriptUnit;
   assert.ok(checkGeneratedJavaScriptUnit(program));assert.deepEqual([...program.declarations.keys()],row.names);assert.deepEqual(program.main.effects.map(effect=>effect.value.fn),row.calls);
   assert.ok(program.items.every(entry=>entry.ret.kind==='unit'&&entry.body.k==='unit'));assert.equal(program.declarations.has('main'),false);
   const same=emitGeneratedJavaScriptUnitLinks(packet.network,packet.root,'JavaScript');assert.equal(same.text,actualJavaScript);assert.equal(same.assumptions.length,0);assert.equal(same.encodings[0].id,'closed-generated-javascript-unit');
   assert.deepEqual(observe(same.text,row.names),{calls:row.calls,output:row.output});
   assert.equal(proof.span.end,actualJavaScript.length);assert.equal(proof.tokens.at(-1).span.end,actualJavaScript.length);assert.equal(proof.tokens.filter(token=>token.entryTarget===program.main.sourceGeneratedJavaScriptEntry).length,2);
   const actualRust=emitGeneratedJavaScriptUnitLinks(packet.network,packet.root,'Rust');assert.deepEqual(actualRust.assumptions.map(row=>row.id),runtimePolicy.assumptions.map(row=>row.id));assert.ok(actualRust.assumptions.every(row=>row.details[0].startsWith('target artifact source operation ')));
   for(const fragment of ['fn ml_main()','std::env::args().any','std::thread::Builder::new()','.stack_size(1 << 28)','.spawn(move ||','.expect("spawn the program thread")','worker.join().is_err()','std::process::exit(101);'])assert.ok(actualRust.text.includes(fragment),row.id+' '+fragment);
   const wholeRust=archive(createTypedProgramLinks(actualRust.text,'Rust',options)),rustProgram=read(wholeRust);assert.deepEqual([...rustProgram.declarations.keys()],row.names);assert.deepEqual(rustProgram.main.effects.map(effect=>effect.value.fn),row.calls);
   const next=emitTypedProgramLinks(wholeRust.network,wholeRust.root,'JavaScript');assert.deepEqual(observe(next.code,row.names),{calls:row.calls,output:row.output});assert.equal(next.assumptions.length,4);
   observationWitness.push(actualRust.assumptions,next.assumptions);actualJavaScript=next.code;
  }
  assert.equal(observationWitness.length,7);assert.ok(observationWitness.flat().every(row=>runtimePolicy.assumptions.some(expected=>row.id===expected.id)));
 }
});

test('default interpretations and refusals remain exactly those of the actual emitted artifacts',()=>{
 for(const row of fixture.cases){const oracle=row.defaultInterpretation;if(oracle.kind==='source refusal')assert.throws(()=>createTypedProgramLinks(row.fullEmittedJavaScript,'JavaScript'),error=>error.message===oracle.message&&JSON.stringify(error.span??null)===JSON.stringify(oracle.span));else{const p=read(createTypedProgramLinks(row.fullEmittedJavaScript,'JavaScript'));assert.deepEqual([...p.declarations.values()].map(entry=>({name:entry.fullName,returnType:entry.ret})),oracle.helpers);assert.deepEqual(p.main.effects.map(effect=>effect.k),oracle.effects);}}
 assert.throws(()=>createGeneratedJavaScriptUnitLinks(plain,{model:''}));assert.throws(()=>createGeneratedJavaScriptUnitLinks(plain,{model:'unknown'}));
});

test('every actual token spelling and trivia code unit is retained without source payload or substitute module',()=>{
 const actual='// header \ud83c\udf0a\n'+plain.replace("'use strict'",'"use strict"').replace('return null','return/*owned*/\t null')+'\n// final comment';
 const p=read(load(actual)),proof=p.sourceGeneratedJavaScriptUnit;assert.equal(emitGeneratedJavaScriptUnit(p,'JavaScript').text,actual);
 let previous=0;for(const [index,token]of proof.tokens.entries()){const gap=proof.gaps[index];assert.equal(gap.span.start,previous);assert.equal(gap.span.end,token.span.start);assert.deepEqual(gap.units,Array.from({length:token.span.start-previous},(_,offset)=>actual.charCodeAt(previous+offset)));assert.equal(actual.slice(token.span.start,token.span.end),token.raw);previous=token.span.end;}
 assert.equal(previous,actual.length);assert.ok(!Object.values(proof).includes(actual));assert.ok(operations(proof.operations).some(node=>node.op==='owned null Unit return'));assert.ok(operations(proof.operations).some(node=>node.op==='owned generated entry invocation'));assert.equal(proof.valueEncoding,policy.valueEncoding);
 const rootOnly=example();delete rootOnly.sourceGeneratedJavaScriptUnit;savedRefused(rootOnly,'independent actual entry context cannot lose the whole module proof');
 const mainOnly=example();delete mainOnly.main.sourceGeneratedJavaScriptEntry;savedRefused(mainOnly,'complete proof cannot lose actual entry context');
});

test('coherent owned helper rename, call retarget and local rename survive Links and whole actual code replay',()=>{
 const p=example(),proof=p.sourceGeneratedJavaScriptUnit,alpha=p.declarations.get('alpha');alpha.name='gamma';alpha.fullName='gamma';p.declarations=new Map(p.items.map(entry=>[entry.fullName,entry]));for(const effect of p.main.effects)if(effect.value.fn==='alpha')effect.value.fn='gamma';p.main.effects[0].name='firstResult';
 const call=operations(proof.operations).find(node=>node.op==='owned generated Unit invocation');call.parts[0].declaration=p.declarations.get('beta');p.main.effects[0].value.fn='beta';
 const packet=serialize(p),q=read(packet),emitted=emitGeneratedJavaScriptUnitLinks(packet.network,packet.root,'JavaScript');assert.deepEqual(observe(emitted.text,['gamma','beta']),{calls:['beta','beta','gamma'],output:''});assert.ok(emitted.text.includes('const firstResult = beta();'));assert.ok(proof.tokens.some(token=>token.raw==='alpha'&&token.declaration===alpha));
 const rust=emitGeneratedJavaScriptUnit(q,'Rust'),actual=archive(createTypedProgramLinks(rust.text,'Rust',options)),back=emitTypedProgramLinks(actual.network,actual.root,'JavaScript');assert.deepEqual(observe(back.code,['gamma','beta']),{calls:['beta','beta','gamma'],output:''});assert.doesNotThrow(()=>load(back.code));
});

test('actual final invocation and null Unit domain exclude ordinary functions, nullable options and extra active source',()=>{
 const changes=[
  ['strict spelling',plain.replace("'use strict'","'use\\x20strict'")],['no directive',plain.replace("'use strict';",'')],['nullable branch',plain.replace('return null','return true?null:null')],['undefined helper',plain.replace('return null','return undefined')],['bare return',plain.replace('return null','return')],['empty helper',plain.replace('return null;','')],['observed result',plain.replace('const ignored=leaf();','const ignored=leaf();console.log(ignored);')],['final helper',plain.replace('} main();','} leaf();')],['no final entry',plain.replace(' main();','')],['double entry',plain+'main();'],['extra effect',plain+'console.log(1);'],['extra helper',plain+'function hidden(){return null;}'],['parameter',plain.replace('leaf()','leaf(input)')],['actual argument',plain.replace('ignored=leaf()','ignored=leaf(null)')],['async',plain.replace('function leaf','async function leaf')],['export','export '+plain],['import','import "other";'+plain],['borrow-like operator',plain.replace('ignored=leaf()','ignored=&leaf()')],['wrong Unit literal',plain.replace('return null','return 0')],['returned closure',plain.replace('return null','return ()=>null')],['wrong generated binding',plain.replace('ignored=','ml_other=')],['reserved helper',plain.replaceAll('leaf','ml_helper')],['recursive helper',plain.replace('return null','return leaf()')],['entry return',plain.replace('const ignored=leaf();','return leaf();')],['mutable entry',plain.replace('const ignored','let ignored')],['duplicate helper',plain.replace('function main','function leaf(){return null;} function main')],['duplicate local',plain.replace('const ignored=leaf();','const ignored=leaf();const ignored=leaf();')],['shadow helper',plain.replace('const ignored','const leaf')],['later TDZ',plain.replace('const ignored=leaf();','const ignored=leaf();const leaf=leaf();')],['shadow entry',plain.replace('const ignored','const main')]
 ];
 for(const [id,actual]of changes){assert.notEqual(actual,plain,id);assert.throws(()=>load(actual),undefined,id);}
 assert.throws(()=>execute(plain.replace('const ignored','const leaf')),/before initialization/u);assert.throws(()=>execute(plain.replace('const ignored=leaf();','const ignored=leaf();const leaf=leaf();')),/before initialization/u);
});

test('all four actual JavaScript line terminators govern return ASI and line-comment active source',()=>{
 for(const code of policy.lineTerminatorCodeUnits){const newline=String.fromCharCode(code),asi=plain.replace('return null','return/*'+newline+'*/null');assert.equal(execute(asi).context.leaf(),undefined);assert.throws(()=>load(asi),undefined,'actual return ASI '+code);
  const active=plain.replace("'use strict';","'use strict'; // hidden"+newline+"console.log('ACTIVE');\n");assert.equal(execute(active).output,'ACTIVE');assert.throws(()=>load(active),undefined,'actual active source after line comment '+code);
 }
 const accepted=plain.replace('return null','return/* no line break */null');assert.equal(execute(accepted).context.leaf(),null);assert.equal(emitGeneratedJavaScriptUnit(read(load(accepted)),'JavaScript').text,accepted);
});

test('source-free core edits, false Unit contexts and detached declaration, binder, entry and operation identities refuse',()=>{
 const changes=[
  p=>{p.main.effects[0].value.fn='beta';},p=>{p.sourceGeneratedJavaScriptUnit.tokens.find(token=>token.declaration===p.items[0]).declaration=p.items[1];},p=>{p.items[0].ret={kind:'bool'};},p=>{p.items[0].body={k:'bool',value:true,type:{kind:'bool'}};},p=>{p.items[0].params.push({name:'input',type:{kind:'unit'}});},p=>{p.main.effects[0].name='beta';},p=>{p.main.effects[0].name='main';},p=>{p.main.effects[1].name=p.main.effects[0].name;},p=>{p.main.effects.push({...p.main.effects[0]});},p=>{p.imports=[{path:['other']}];},p=>{p.externals=[{k:'fn',name:'foreign'}];},p=>{p.referenceSemantics=true;},p=>{p.outputThreaded=true;},p=>{p.items[0].generated=true;},p=>{p.items[0].sourcePureUnit={model:'closed pure Rust unit calls 1'};},
  p=>{p.sourceGeneratedJavaScriptUnit.model='unknown';},p=>{p.sourceGeneratedJavaScriptUnit.valueEncoding='nullable null';},p=>{p.sourceGeneratedJavaScriptUnit.observationModel='unconditional Rust success';},p=>{p.main.sourceGeneratedJavaScriptEntry.valueEncoding='undefined Unit';},p=>{p.main.sourceGeneratedJavaScriptEntry.name='ordinary';},p=>{p.main.sourceGeneratedJavaScriptEntry.span={start:0,end:1};},p=>{p.sourceGeneratedJavaScriptUnit.extra=1;},p=>{p.sourceGeneratedJavaScriptUnit=null;},p=>{p.main.sourceGeneratedJavaScriptEntry=null;},
  p=>{const t=p.sourceGeneratedJavaScriptUnit.tokens.find(token=>token.declaration);t.declaration=structuredClone(t.declaration);},p=>{const t=p.sourceGeneratedJavaScriptUnit.tokens.find(token=>token.binding);t.binding=structuredClone(t.binding);},p=>{const t=p.sourceGeneratedJavaScriptUnit.tokens.find(token=>token.entryTarget);t.entryTarget=structuredClone(t.entryTarget);},p=>{p.sourceGeneratedJavaScriptUnit.operations.parts[0].parts[0]=structuredClone(p.sourceGeneratedJavaScriptUnit.operations.parts[0].parts[0]);},p=>{p.sourceGeneratedJavaScriptUnit.tokens.push(p.sourceGeneratedJavaScriptUnit.tokens[0]);},p=>{p.sourceGeneratedJavaScriptUnit.operations.parts.pop();}
 ];
 for(const [index,change]of changes.entries()){const p=example();change(p);savedRefused(p,'owned Unit semantic/context/identity edit '+index);}
});

test('complete CST rejects missing, invented and lexically impossible spans, trivia and scalar spellings',()=>{
 const changes=[
  p=>{p.sourceGeneratedJavaScriptUnit.tokens[0].span.start=-0;},p=>{p.sourceGeneratedJavaScriptUnit.tokens[0].span.end++;},p=>{p.sourceGeneratedJavaScriptUnit.tokens[1].span.start=0.5;},p=>{p.sourceGeneratedJavaScriptUnit.tokens[0].span.end=Number.MAX_SAFE_INTEGER+1;},p=>{p.sourceGeneratedJavaScriptUnit.tokens.at(-1).span.end++;},p=>{p.sourceGeneratedJavaScriptUnit.operations.span.start=-0;},p=>{p.sourceGeneratedJavaScriptUnit.span.start=1;},p=>{p.sourceGeneratedJavaScriptUnit.tokens.find(token=>token.declaration).raw='a b c';},p=>{p.sourceGeneratedJavaScriptUnit.tokens.find(token=>token.value==='null').raw='true';},p=>{p.sourceGeneratedJavaScriptUnit.tokens.find(token=>token.kind==='string').value='not strict';},
  p=>{p.sourceGeneratedJavaScriptUnit.gaps.pop();},p=>{p.sourceGeneratedJavaScriptUnit.gaps[0].span.end--;},p=>{p.sourceGeneratedJavaScriptUnit.gaps[0].units.pop();},p=>{p.sourceGeneratedJavaScriptUnit.gaps[0].units[0]=-0;},p=>{p.sourceGeneratedJavaScriptUnit.gaps[0].units[0]=65536;},p=>{p.sourceGeneratedJavaScriptUnit.gaps[0].units[0]='47';},p=>{p.sourceGeneratedJavaScriptUnit.gaps[0].units[0]=1.5;},p=>{p.sourceGeneratedJavaScriptUnit.gaps[0].units[0]=33;},p=>{p.sourceGeneratedJavaScriptUnit.gaps[0].units.fill(32);p.sourceGeneratedJavaScriptUnit.gaps[0].units[0]=47;p.sourceGeneratedJavaScriptUnit.gaps[0].units[1]=47;},p=>{p.sourceGeneratedJavaScriptUnit.gaps[0].units.fill(32);p.sourceGeneratedJavaScriptUnit.gaps[0].units[0]=47;p.sourceGeneratedJavaScriptUnit.gaps[0].units[1]=42;}
 ];
 for(const [index,change]of changes.entries()){const p=example();change(p);savedRefused(p,'complete actual CST edit '+index);}
 const p=example();p.sourceGeneratedJavaScriptUnit.span={end:p.sourceGeneratedJavaScriptUnit.span.end,start:0};for(const token of p.sourceGeneratedJavaScriptUnit.tokens)token.span={end:token.span.end,start:token.span.start};assert.equal(emitGeneratedJavaScriptUnit(read(serialize(p)),'JavaScript').text,source);
});

test('actual saved Links mutations remain rejected by the experimental reload and whole-artifact emitter',()=>{
 for(const [before,after]of [[policy.model,'unknown'],[policy.valueEncoding,'nullable Unit'],['owned null Unit return','invented operation'],['null','true'],['main','other']]){
  const packet=load(source),value=packet.network.links().find(link=>link.metadata().term==='string value'&&link.metadata().definition===before);assert.ok(value,before);value.setMetadata(value.metadata().withDefinition(after));const saved=archive(packet);for(const target of ['Rust','JavaScript'])assert.throws(()=>emitGeneratedJavaScriptUnitLinks(saved.network,saved.root,target),undefined,before+' saved mutation');
 }
});

test('host accessors, hidden fields, structural cycles, prototypes and sparse arrays refuse before callbacks',()=>{
 const targets=[p=>p,p=>p.sourceGeneratedJavaScriptUnit,p=>p.sourceGeneratedJavaScriptUnit.tokens[0],p=>p.sourceGeneratedJavaScriptUnit.tokens[0].span,p=>p.sourceGeneratedJavaScriptUnit.gaps[0],p=>p.sourceGeneratedJavaScriptUnit.operations,p=>p.main.sourceGeneratedJavaScriptEntry,p=>p.items[0],p=>p.items[0].body,p=>p.main.effects[0]];let invoked=0;
 for(const [index,target]of targets.entries())for(const change of [node=>Object.defineProperty(node,'host',{get(){invoked++;return 0;},enumerable:true}),node=>Object.defineProperty(node,'hidden',{value:1}),node=>{node[Symbol('host')]=1;},node=>Object.setPrototypeOf(node,{inherited:1})]){const p=example();change(target(p));refused(p,'owned host record '+index);}
 for(const change of [p=>{p.sourceGeneratedJavaScriptUnit.tokens.extra=1;},p=>{delete p.sourceGeneratedJavaScriptUnit.tokens[0];},p=>{p.declarations.extra=1;},p=>Object.setPrototypeOf(p.sourceGeneratedJavaScriptUnit.tokens,{}),p=>{p.sourceGeneratedJavaScriptUnit.operations.parts.push(p.sourceGeneratedJavaScriptUnit);},p=>{p.sourceGeneratedJavaScriptUnit.gaps[0].units[0]=undefined;},p=>{p.sourceGeneratedJavaScriptUnit.gaps[0].units[0]=()=>{invoked++;};}]){const p=example();change(p);refused(p,'owned host container');}assert.equal(invoked,0);
});
