import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {existsSync,mkdirSync,readFileSync,writeFileSync} from 'node:fs';
import {dirname,join} from 'node:path';
import {fileURLToPath} from 'node:url';
import test from 'node:test';
import {readProjectData,writeProjectData,serializeProjectData} from '../../js/scripts/project-data.mjs';
import {readStructuredDataLino} from '../../js/src/index.js';
import {RUST_RAW_BINDING_RULE_MODEL_COMMIT,verifyRustRawBindingRuleModel,rustRawBindingRuleNativeMode,planRustRawBindingRuleNative,runRustRawBindingRuleNative} from './native-observations.mjs';

// Explicit local native requests fail before registering any tests.
const nativeEnabled=rustRawBindingRuleNativeMode(process.env),plan=planRustRawBindingRuleNative(),entry=plan.entries[0];
const policy=readProjectData(new URL('../rust-raw-binding-identifiers/data/policy.lino',import.meta.url));
const fakeEnvironment={CI:'true',RUST_RAW_BINDING_RULE_NATIVE:'1',GITHUB_SHA:'injected-process-contract-only'};
const stream=entry.observations.map((row,index)=>index+':bool:'+row.value+'\n').join('');
function fakeProcess(override){
 const calls=[];
 const spawn=(command,args,options)=>{
  const phase=args.includes('--crate-type=rlib')?'library':args.includes('--extern')?'consumer':'execute',call={command,args,options,phase};calls.push(call);
  const replacement=override?.(call);if(replacement)return replacement;
  if(phase!=='execute')writeFileSync(args[args.indexOf('-o')+1],'injected artifact; never executable Rust');
  return{status:0,signal:null,stdout:phase==='execute'?stream:'',stderr:''};
 };
 return{spawn,calls};
}
function injected(fake,options={}){return runRustRawBindingRuleNative(plan,{environment:fakeEnvironment,spawn:fake.spawn,...options});}
function fails(action,pattern){assert.throws(action,error=>{assert.ok(error instanceof AggregateError);assert.equal(error.errors.length,1);assert.match(error.message,pattern);return true;});}
const rows=name=>entry.observations.filter(row=>row.selector==='acceptRustRawBindingIdentifier'+name);

test('one plan retains the entire unchanged six-rule artifact, source CST and all frozen model receipts',()=>{
 assert.equal(plan.modelCommit,RUST_RAW_BINDING_RULE_MODEL_COMMIT);assert.equal(plan.modelCommit,'258d3baa11fab25b8701c3fe1243a9176d214bee');assert.equal(plan.originalCount,1);assert.equal(plan.entries.length,1);
 const original=readFileSync(new URL('../rust-raw-binding-identifiers/data/rules.rs',import.meta.url));assert.deepEqual(Buffer.from(entry.source),original);assert.equal(entry.sourceBytes,3588);
 assert.equal(entry.sourceSha256,'5ab7da7314104264d8f46a0b678eb8f967f19f581fabdcf33e7594b3f28d42f1');assert.equal(createHash('sha256').update(original).digest('hex'),entry.sourceSha256);assert.equal(entry.replaySha256,entry.sourceSha256);
 assert.deepEqual(entry.archiveFormats,['complete source CST Links Notation text','binary links']);assert.equal(plan.modelSources.length,8);assert.throws(()=>{entry.source='pub fn changed(){}';},TypeError);
 assert.throws(()=>verifyRustRawBindingRuleModel(()=>Buffer.from('counterfeit artifact')),/frozen raw interpretation or selector artifact changed/u);
});

test('all six actual exports retain checked primitive argument order and Boolean results',()=>{
 assert.deepEqual(entry.declarations.map(row=>row.name),[
  'acceptRustRawBindingIdentifierModel','acceptRustRawBindingIdentifierName','acceptRustRawBindingIdentifierType',
  'acceptRustRawBindingIdentifierClosure','acceptRustRawBindingIdentifierInvocationModel','acceptRustRawBindingIdentifierInput',
 ]);
 assert.deepEqual(entry.declarations.map(row=>row.parameters.map(parameter=>parameter.kind)),[['string'],['string','bool','bool'],['string','bool','float'],['bool','bool'],['string'],['string','bool','bool']]);
 assert.ok(entry.declarations.every(row=>row.result==='bool'));assert.equal(entry.invocationCount,644);assert.equal(entry.observations.length,644);
 assert.deepEqual(entry.declarations.map(row=>rows(row.name.replace('acceptRustRawBindingIdentifier','')).length),[6,276,320,4,6,32]);
 for(const declaration of entry.declarations)assert.equal((entry.source.match(new RegExp('fn '+declaration.native+'\\(','gu'))??[]).length,1);
});

test('every forbidden spelling and ordinary keyword receives all explicit raw and character-premise Boolean combinations',()=>{
 const names=rows('Name');assert.match(entry.observationScope,/Conditional primitive selector decisions/u);assert.ok(names.every(row=>row.label.startsWith('conditional character premise ')));assert.equal(new Set(names.map(row=>row.arguments[0].value)).size,69);
 for(const name of new Set([...policy.forbiddenNames,...policy.ordinaryKeywords])){
  const actual=names.filter(row=>row.arguments[0].value===name);assert.equal(actual.length,4);
  assert.deepEqual(actual.map(row=>row.arguments.slice(1).map(argument=>argument.value)),[[false,false],[false,true],[true,false],[true,true]]);
 }
 for(const name of policy.forbiddenNames)assert.ok(names.filter(row=>row.arguments[0].value===name).every(row=>row.value===false));
 // These rows test the pure Boolean premise parameter, not a new Unicode-name model.
 assert.ok(names.some(row=>row.arguments[0].value==='😀'&&row.arguments[1].value&&row.arguments[2].value&&row.value));
 assert.ok(names.some(row=>row.arguments[0].value==='😀'&&!row.arguments[2].value&&!row.value));
});

test('all signed-type, closure and invocation flags and exact binary64 boundaries are retained as typed arguments',()=>{
 assert.equal(rows('Type').length,320);for(const row of rows('Type'))assert.deepEqual(Object.keys(row.arguments[2]),['kind','bits'],'nonfinite and signed zero cells are exact bits, never JSON Number values');const numbers=new Set(rows('Type').map(row=>row.arguments[2].bits));assert.equal(numbers.size,20);
 for(const bits of['0000000000000000','8000000000000000','7ff8000000000000','7ff0000000000000','fff0000000000000','0000000000000001'])assert.ok(numbers.has(bits),bits);
 assert.deepEqual(rows('Closure').map(row=>row.arguments.map(argument=>argument.value)),[[false,false],[false,true],[true,false],[true,true]]);
 assert.deepEqual(rows('Closure').map(row=>row.value),[true,false,false,false]);
 for(const name of['bool','fixed','float','int','nat','string','','fixed signed 32'])assert.equal(rows('Input').filter(row=>row.arguments[0].value===name).length,4);
 assert.equal(rows('Model').filter(row=>row.value).length,1);assert.equal(rows('InvocationModel').filter(row=>row.value).length,1);
 assert.match(entry.consumerSource,/f64::from_bits\(0x8000000000000000u64\)/u);assert.match(entry.consumerSource,/f64::from_bits\(0x7ff8000000000000u64\)/u);
 assert.equal((entry.consumerSource.match(/f64::from_bits\(0x[0-9a-f]{16}u64\)/gu)??[]).length,320);
});

test('native execution needs its own exact CI flag and checked in-memory plan before even injected processes',()=>{
 const fake=fakeProcess();assert.equal(rustRawBindingRuleNativeMode({}),false);assert.equal(rustRawBindingRuleNativeMode({CI:'true'}),false);
 for(const environment of[{RUST_RAW_BINDING_RULE_NATIVE:'1'},{CI:'false',RUST_RAW_BINDING_RULE_NATIVE:'1'}])assert.throws(()=>runRustRawBindingRuleNative(plan,{environment,spawn:fake.spawn}),/requires CI=true/u);
 for(const environment of[{}, {CI:'true',RUST_RAW_BINDING_IDENTIFIER_NATIVE:'1'}])assert.throws(()=>runRustRawBindingRuleNative(plan,{environment,spawn:fake.spawn}),/explicit CI-only switch/u);
 assert.throws(()=>runRustRawBindingRuleNative(structuredClone(plan),{environment:fakeEnvironment,spawn:fake.spawn}),/exact checked in-memory/u);assert.equal(fake.calls.length,0);
});

test('one unchanged library and one separate consumer perform exactly one process execution in the injected contract',()=>{
 const fake=fakeProcess(({phase,args,options})=>{
  assert.equal(options.shell,false);assert.equal(options.encoding,'utf8');assert.equal(options.env,fakeEnvironment);assert.equal(options.maxBuffer,1048576);
  if(phase==='library'){assert.deepEqual(readFileSync(args[args.indexOf('--crate-name')+2]),Buffer.from(entry.source));assert.ok(args.includes('--crate-type=rlib'));assert.equal(options.timeout,30000);}
  if(phase==='consumer'){assert.equal(readFileSync(args[1],'utf8'),entry.consumerSource);assert.ok(args.includes('oracle_source='+join(options.cwd,'liboracle_source.rlib')));}
  if(phase==='execute'){assert.deepEqual(args,[]);assert.equal(options.timeout,10000);}
 });
 const receipt=injected(fake);assert.equal(receipt.processMode,'injected process contract');assert.deepEqual(receipt.processCounts,{libraryCompilations:1,consumerCompilations:1,executions:1});assert.equal(fake.calls.length,3);
 assert.equal(receipt.observations.length,1);assert.equal(receipt.observations[0].observations.length,644);assert.deepEqual(receipt.observations[0].observations.map(row=>row.value),entry.observations.map(row=>row.value));
 assert.equal((entry.consumerSource.match(/let observed:bool=oracle_source::/gu)??[]).length,644);assert.doesNotMatch(entry.consumerSource,/include!|mod original|fn accept_rust/u);
 assert.deepEqual(readStructuredDataLino(serializeProjectData(receipt,'observations.lino')),receipt);
 for(const call of fake.calls)assert.equal(existsSync(call.options.cwd),false);
});

test('missing compilers and failed library, consumer or execution statuses never become observations',()=>{
 const missing=fakeProcess(()=>({status:null,stdout:'',stderr:'',error:Object.assign(new Error('compiler unavailable'),{code:'ENOENT'})}));fails(()=>injected(missing,{compiler:'/missing/compiler'}),/compiler unavailable/u);assert.equal(missing.calls.length,1);
 for(const phase of['library','consumer','execute']){const fake=fakeProcess(call=>call.phase===phase?{status:1,stdout:'',stderr:'actual subprocess failure'}:null);fails(()=>injected(fake),/actual subprocess failure/u);assert.equal(fake.calls.at(-1).phase,phase);}
});

test('timeouts and signals remain process failures with actual diagnostics',()=>{
 for(const phase of['library','consumer','execute']){const fake=fakeProcess(call=>call.phase===phase?{status:null,stdout:'',stderr:'',error:Object.assign(new Error('timeout'),{code:'ETIMEDOUT'})}:null);fails(()=>injected(fake),/timed out after/u);}
 const signal=fakeProcess(call=>call.phase==='execute'?{status:null,signal:'SIGTERM',stdout:'',stderr:''}:null);fails(()=>injected(signal),/terminated by SIGTERM/u);
});

test('wrong Boolean values, tags, order, count and noncanonical native output fail closed',()=>{
 const lines=stream.trimEnd().split('\n'),outputs=[stream.replace('0:bool:true','0:bool:false'),stream.replace(':bool:',':i32:'),stream.replace('true','1'),stream.replace('true','TRUE'),stream.replace('0:','00:'),stream.slice(0,-1),lines.slice(1).join('\n')+'\n',stream+lines[0]+'\n',[lines[1],lines[0],...lines.slice(2)].join('\n')+'\n',{},'unexpected output\n'];
 for(const stdout of outputs){const fake=fakeProcess(call=>call.phase==='execute'?{status:0,stdout,stderr:''}:null);fails(()=>injected(fake),/Original complete Rust raw binding rule observations failed/u);assert.equal(fake.calls.length,3);}
});

test('successful compiler status requires real artifacts and any source or consumer byte edit invalidates the receipt',()=>{
 for(const phase of['library','consumer']){const fake=fakeProcess(call=>call.phase===phase?{status:0,stdout:'',stderr:''}:null);fails(()=>injected(fake),/compiler did not create/u);assert.equal(fake.calls.some(call=>call.phase==='execute'),false);}
 for(const phase of['library','consumer','execute']){
  const fake=fakeProcess(call=>{if(call.phase===phase){const file=join(call.options.cwd,phase==='consumer'?'consumer.rs':'original.rs');writeFileSync(file,readFileSync(file,'utf8')+'\n// modified source bytes\n');}});fails(()=>injected(fake),/bytes changed|external consumer changed/u);if(phase!=='execute')assert.equal(fake.calls.some(call=>call.phase==='execute'),false);
 }
});

// No current CI wiring. Only a later reviewed batch may enable this real subprocess group.
if(nativeEnabled)test('CI executes the complete unchanged six-rule artifact for all 644 declared selector calls',()=>{
 const receipt=runRustRawBindingRuleNative(plan);assert.equal(receipt.processMode,'native subprocesses');assert.equal(receipt.originalCount,1);assert.deepEqual(receipt.processCounts,{libraryCompilations:1,consumerCompilations:1,executions:1});assert.equal(receipt.observations.length,1);
 const row=receipt.observations[0];assert.equal(row.invocationCount,644);assert.equal(row.observations.length,644);assert.equal(row.sourceSha256,entry.sourceSha256);assert.deepEqual(row.declarations,entry.declarations);
 const file=fileURLToPath(new URL('../../issue-195-artifacts/rust-raw-binding-rules-native-observations.lino',import.meta.url));mkdirSync(dirname(file),{recursive:true});writeProjectData(file,receipt);
 process.stdout.write('Original complete Rust raw binding rules: one unchanged artifact, 644 actual selector calls; Links Notation receipts: '+file+'\n');
});
