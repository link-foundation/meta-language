// Deferred controls; source authoring does not execute these cases.
import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import {parseRust} from '../src/translation/rust.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {emitRust} from '../src/translation/emit-rust.js';
import {emitLean} from '../src/translation/emit-lean.js';
import {encodeTypedProgramLinks,decodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {readCallableDataFacts,readCallableDataRecursiveProfile} from '../src/translation/callable-data.js';
import {readCallableDataDerivation,readCallableDataProfileError,readCallableDataObservationError,readCallableDataTraversalError,readCallableDataRecursivePlan,renderCallableDataLeanDefault} from '../src/translation/callable-data-derivation-rules.js';
import {callableDataOriginals,callableDataObservationRefusals,callableDataRecursiveRefusals,callableDataSourceVersionRange} from './fixtures/callable-data-originals.mjs';
function observe(source){const lines=[];vm.runInNewContext(source,{console:{log:(...values)=>lines.push(values.map(String).join(' '))},process:{argv:[]}},{timeout:1000});return lines.length?lines.join('\n')+'\n':'';}
for(const row of callableDataOriginals)for(const sourceVersion of ['1.90.0','1.99.0'])test(row.name+' / '+sourceVersion,()=>{
 assert.equal(callableDataSourceVersionRange,'>=1.90.0 <1.100.0');
 const program=checkProgram(parseRust(row.source,{sourceVersion})),packet=encodeTypedProgramLinks(program);
 assert.equal(observe(emitJavaScript(program).text),row.stdout);
 assert.ok(emitRust(program).definitions.some(text=>text.includes('#[derive(Clone)]')));
 assert.match(emitLean(program).text,/deriving Inhabited|instance : Inhabited/u);
 for(const network of [packet.network,LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]){
  const restored=decodeTypedProgramLinks(network,packet.root);assert.deepEqual(restored.items,program.items);
  for(const target of ['JavaScript','Rust','Lean','Rocq'])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),emitTypedProgramLinks(packet.network,packet.root,target));
  assert.equal(observe(emitTypedProgramLinks(network,packet.root,'JavaScript').code),row.stdout);
 }
});
for(const row of callableDataObservationRefusals)for(const sourceVersion of ['1.90.0','1.99.0'])test(row.name+' / '+sourceVersion,()=>assert.throws(()=>checkProgram(parseRust(row.source,{sourceVersion})),error=>error.kind==='unsupported'));
for(const row of callableDataRecursiveRefusals)for(const sourceVersion of ['1.90.0','1.99.0'])for(const [target,emit]of [['Lean',emitLean],['Rust',emitRust]])test(row.name+' / '+sourceVersion+' / '+target,()=>assert.throws(()=>emit(checkProgram(parseRust(row.source,{sourceVersion}))),error=>error.kind==='unsupported'));
const scalar={kind:'fixed',bits:8,signed:false},pointer={kind:'function',params:[scalar],ret:scalar},data={kind:'data',name:'Callback'},name=value=>({k:'name',path:[value]});
function observationSurface(effect){return {language:'Rust',items:[{k:'data',name:'Callback',ctors:[{name:'Value',fields:[{name:'read',type:pointer}]}]},{k:'fn',name:'read',params:[{name:'value',type:scalar}],ret:scalar,body:name('value')}],main:{effects:[{k:'let',name:'value',value:{k:'app',head:{k:'name',path:['Callback','Value']},args:[{k:'functionValue',path:['read']}]}},effect]}};}
test('direct checked propositions deep equality display and proof quantifiers refuse callable aggregate observation',()=>{
 for(const effect of [{k:'assert',prop:{p:'eq',left:name('value'),right:name('value')}},{k:'print',expr:name('value')},{k:'assert',prop:{p:'forall',binders:[{name:'item',type:data}],body:{p:'bool',expr:{k:'bool',value:true}}}}])assert.throws(()=>checkProgram(observationSurface(effect)),error=>error.kind==='unsupported'&&error.reason.includes('callable values and aggregates'));
});
test('self recursion uses a least finite constructor witness and rejects negative or endless graphs',()=>{
 const kinds=['fixed','fixed','function','reference','constructor','constructor','data','reference'],ends=[0,0,2,2,4,4,6,6],edges=[0,1,2,3,4,5],references=[-1,-1,-1,0,-1,-1,-1,0];
 assert.deepEqual(readCallableDataRecursivePlan(kinds,ends,edges,references,[6],7),['profile','true','true','true','1']);
 assert.equal(renderCallableDataLeanDefault('Chain','Chain.End',0),'instance : Inhabited Chain := ⟨Chain.End⟩');
 assert.match(readCallableDataRecursivePlan(['fixed','fixed','function','reference','constructor','data','reference'],[0,0,2,2,4,5,5],[0,1,2,3,4],[-1,-1,-1,0,-1,-1,0],[5],6)[1],/no finite constructive witness/u);
 assert.match(readCallableDataRecursivePlan(['reference','fixed','function','constructor','constructor','data','reference'],[0,0,2,3,3,5,5],[0,1,2,3,4],[0,-1,-1,-1,-1,-1,0],[5],6)[1],/strict positivity/u);
 const chain={kind:'data',name:'Chain'},declarations=new Map([['Chain',{k:'data',ctors:[{name:'Link',fields:[{type:pointer},{type:chain}]},{name:'End',fields:[]}]}]]);
 assert.deepEqual(readCallableDataRecursiveProfile(chain,declarations),{callable:true,recursive:true,constructive:true,witness:1});
 // The old acyclic-only operation retains its exact boundary independently.
 assert.notEqual(readCallableDataProfileError(true,false,true),'');
});
test('derivation keeps old traits while callable data gets no debug or equality instances',()=>{
 assert.equal(readCallableDataDerivation('Rust',true,false,false,false),'Clone');assert.equal(readCallableDataDerivation('Lean',true,false,false,false),'Inhabited');
 assert.equal(readCallableDataDerivation('Rust',false,false,false,false),'Clone, Debug, PartialEq, Eq');assert.equal(readCallableDataDerivation('Rust',false,true,false,false),'Clone, Debug, PartialEq');
 assert.equal(readCallableDataDerivation('Lean',false,false,false,false),'Repr, DecidableEq, Inhabited');assert.equal(readCallableDataDerivation('Lean',false,false,false,true),'Repr, Inhabited');assert.equal(readCallableDataDerivation('Lean',false,false,true,false),'Inhabited');
 assert.equal(readCallableDataObservationError('comparison',false),'');assert.equal(readCallableDataObservationError('call',true),'');assert.notEqual(readCallableDataObservationError('comparison',true),'');
 assert.equal(readCallableDataTraversalError(256,65536),'');assert.notEqual(readCallableDataTraversalError(257,1),'');assert.notEqual(readCallableDataTraversalError(0,65537),'');
 for(const invalid of [-1,0.5,NaN,Infinity,-Infinity]){assert.notEqual(readCallableDataTraversalError(invalid,1),'');assert.notEqual(readCallableDataTraversalError(0,invalid),'');}
 const declarations=new Map([['Callback',{k:'data',ctors:[{fields:[{type:pointer}]}]}]]);assert.deepEqual(readCallableDataFacts(data,declarations),{callable:true,constructive:true,acyclic:true});
 declarations.set('Callback',{k:'data',ctors:[{fields:[{type:{kind:'function',params:[data],ret:scalar}}]}]});const cycle=readCallableDataFacts(data,declarations);assert.equal(cycle.callable,true);assert.equal(cycle.acyclic,false);assert.notEqual(readCallableDataProfileError(cycle.callable,cycle.acyclic,cycle.constructive),'');
 declarations.set('Empty',{k:'data',ctors:[]});declarations.set('Callback',{k:'data',ctors:[{fields:[{type:{kind:'function',params:[],ret:{kind:'data',name:'Empty'}}}]}]});const empty=readCallableDataFacts(data,declarations);assert.equal(empty.constructive,false);assert.notEqual(readCallableDataProfileError(empty.callable,empty.acyclic,empty.constructive),'');
 declarations.set('Callback',{k:'data',ctors:[{fields:[{type:{kind:'function',params:[{kind:'data',name:'Empty'}],ret:scalar}}]}]});assert.equal(readCallableDataFacts(data,declarations).constructive,false);
});
test('each target independently refuses actual checked negative recursive defaultless and mutual callable layouts',()=>{
 const scalar={kind:'bool'},data=name=>({kind:'data',name}),callback={kind:'function',params:[scalar],ret:scalar};
 const cases=[
  [{k:'data',name:'Negative',ctors:[{name:'End',fields:[]},{name:'More',fields:[{name:'read',type:{kind:'function',params:[data('Negative')],ret:scalar}}]}]}],
  [{k:'data',name:'Endless',ctors:[{name:'More',fields:[{name:'read',type:callback},{name:'tail',type:data('Endless')}]}]}],
  [{k:'data',name:'First',ctors:[{name:'End',fields:[]},{name:'More',fields:[{name:'read',type:callback},{name:'tail',type:data('Second')}]}]},{k:'data',name:'Second',ctors:[{name:'More',fields:[{name:'tail',type:data('First')}]}]}],
 ];
 for(const items of cases){const checked=checkProgram({language:'Rust',items});for(const emit of [emitLean,emitRust])assert.throws(()=>emit(checked),error=>error.kind==='unsupported');}
});
