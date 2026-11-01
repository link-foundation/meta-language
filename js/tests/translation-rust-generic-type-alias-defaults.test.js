// Deferred source controls. No execution until the complete source draft.
import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import {parseRust} from '../src/translation/rust.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {createTypedProgramLinks,decodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {readRustGenericTypeAliasParameterDirectory,readRustGenericTypeAliasDefaultSubstitutionPlan,readRustGenericTypeAliasDefaultArgumentPlan,acceptRustGenericTypeAliasConcreteIdentifier,readRustGenericTypeAliasDefaultOwnerPhase} from '../src/translation/rust-generic-type-alias-rules.js';
import {genericAliasDefaultOriginals,genericAliasDefaultRefusals,genericAliasDefaultVersionRange,defaultParameterDirectories,genericAliasDefaultPreviousBoundaries} from './fixtures/rust-generic-type-alias-defaults-draft.mjs';
import {acceptRustGenericTypeAliasStandardSumLeaf,acceptRustGenericTypeAliasOwnedStandardSumLeaf,readRustGenericTypeAliasConstructorPlan,readRustGenericTypeAliasOwnedConstructorPlan} from '../src/translation/rust-generic-type-alias-rules.js';
import {readRustGenericTypeAliasSubstitutionPlan,readRustGenericTypeAliasHeaderSubstitutionPlan,readRustGenericTypeAliasHeaderConstructorPlan} from '../src/translation/rust-generic-type-alias-rules.js';
import {acceptRustGenericTypeAliasConcreteTemplate,acceptRustGenericTypeAliasConcreteCallableTemplate,acceptRustGenericTypeAliasHeaderCallableTemplate} from '../src/translation/rust-generic-type-alias-rules.js';
for(const [index,row]of defaultParameterDirectories.entries())test('actual default parameter token directory '+index,()=>assert.deepEqual(readRustGenericTypeAliasParameterDirectory(row.kinds,row.values,row.raws,row.position),row.expected));
test('default substitution selects only earlier actual binding identities',()=>{
 assert.deepEqual(readRustGenericTypeAliasDefaultSubstitutionPlan(['owner/T','owner/U'],1,1,['owner/T']),['substitution','0']);
 assert.deepEqual(readRustGenericTypeAliasDefaultSubstitutionPlan(['owner/T'],0,0,[]),['substitution']);
 assert.equal(readRustGenericTypeAliasDefaultSubstitutionPlan(['owner/T','owner/U'],0,0,['owner/U'])[0],'error');
 assert.equal(readRustGenericTypeAliasDefaultSubstitutionPlan(['owner/T'],0,0,['caller/T'])[0],'error');
 assert.deepEqual(readRustGenericTypeAliasDefaultArgumentPlan([false,true],1,true),['arguments','1']);
 assert.deepEqual(readRustGenericTypeAliasDefaultArgumentPlan([true,true],0,false),['arguments','0','1']);
 assert.equal(readRustGenericTypeAliasDefaultArgumentPlan([false,true],0,false)[0],'error');
 assert.equal(readRustGenericTypeAliasDefaultArgumentPlan([true],2,true)[0],'error');
 assert.equal(acceptRustGenericTypeAliasConcreteIdentifier(['T'],'T',true),true);
 assert.equal(acceptRustGenericTypeAliasConcreteIdentifier(['T'],'T',false),false);
 assert.equal(readRustGenericTypeAliasDefaultOwnerPhase(true,true),'defer');
 assert.equal(readRustGenericTypeAliasDefaultOwnerPhase(true,false),'prepare');
 assert.equal(readRustGenericTypeAliasDefaultOwnerPhase(false,true),'prepare');
 assert.equal(readRustGenericTypeAliasDefaultOwnerPhase(false,false),'prepare');
});
test('owned sum profile is independent of the retained Copy-only alias gates',()=>{
 assert.equal(acceptRustGenericTypeAliasStandardSumLeaf('named',0,'std::option::Option',1,true,true,false,0),false);
 assert.equal(acceptRustGenericTypeAliasOwnedStandardSumLeaf('named',0,'std::option::Option',1,true,true,0,'owned'),true);
 for(const profile of ['','copy','borrowed','custom Drop'])assert.equal(acceptRustGenericTypeAliasOwnedStandardSumLeaf('named',0,'std::option::Option',1,true,true,0,profile),false);
 assert.deepEqual(readRustGenericTypeAliasConstructorPlan('named',0,'library','std::option::Option',1,'','',true,'std::option::Option',1,true,false,'Some'),[]);
 assert.deepEqual(readRustGenericTypeAliasOwnedConstructorPlan('named',0,'library','std::option::Option',1,true,'std::option::Option',1,true,'owned','Some'),['sum constructor','1']);
 assert.deepEqual(readRustGenericTypeAliasOwnedConstructorPlan('named',0,'library','std::option::Option',1,true,'std::option::Option',1,true,'copy','Some'),[]);
 assert.deepEqual(readRustGenericTypeAliasOwnedConstructorPlan('named',1,'library','std::option::Option',1,true,'std::option::Option',1,true,'owned','Some'),[]);
 assert.deepEqual(readRustGenericTypeAliasOwnedConstructorPlan('named',0,'library','custom::Option',1,true,'std::option::Option',1,true,'owned','Some'),[]);
 assert.deepEqual(readRustGenericTypeAliasOwnedConstructorPlan('named',0,'library','std::option::Option',1,true,'std::option::Option',1,true,'owned','Err'),[]);
});
test('private alias plans preserve provisional origins without weakening admitted gates',()=>{
 assert.equal(readRustGenericTypeAliasSubstitutionPlan(['actual/T'],[false],['actual/T'])[0],'error');
 assert.deepEqual(readRustGenericTypeAliasHeaderSubstitutionPlan(['actual/T'],true,[true],[false],[true],['actual/T']),['substitution','0']);
 assert.equal(readRustGenericTypeAliasHeaderSubstitutionPlan(['actual/T'],false,[true],[false],[true],['actual/T'])[0],'error');
 assert.equal(readRustGenericTypeAliasHeaderSubstitutionPlan(['actual/T'],true,[false],[false],[true],['actual/T'])[0],'error');
 assert.equal(readRustGenericTypeAliasHeaderSubstitutionPlan(['actual/T'],true,[true],[false],[false],['actual/T'])[0],'error');
 assert.deepEqual(readRustGenericTypeAliasHeaderConstructorPlan(true,true,'named',0,'library','std::option::Option',1,'std::option::Option',1,0,'Some'),['sum constructor','1']);
 for(const [header,provisional]of [[false,true],[true,false],[false,false]])assert.deepEqual(readRustGenericTypeAliasHeaderConstructorPlan(header,provisional,'named',0,'library','std::option::Option',1,'std::option::Option',1,0,'Some'),[]);
 assert.deepEqual(readRustGenericTypeAliasHeaderConstructorPlan(true,true,'named',0,'library','custom::Option',1,'std::option::Option',1,0,'Some'),[]);
 assert.deepEqual(readRustGenericTypeAliasHeaderConstructorPlan(true,true,'named',0,'library','std::option::Option',1,'std::option::Option',1,1,'Some'),[]);
 assert.deepEqual(readRustGenericTypeAliasOwnedConstructorPlan('named',0,'library','std::option::Option',1,false,'std::option::Option',1,false,'owned','Some'),[]);
});
test('owned extension retains exact preceding unsupported boundary assertion',()=>{
 const history=genericAliasDefaultPreviousBoundaries[0];
 assert.equal(history.source,history.previousAssertion.source);
 assert.deepEqual(history.previousAssertion.expectedErrorKinds,['unsupported','type','syntax']);
 assert.equal(genericAliasDefaultOriginals.filter(row=>row.source===history.source).length,1);
 assert.equal(genericAliasDefaultRefusals.filter(row=>row.source===history.source).length,0);
});
test('concrete callable leaf admission preserves the original scalar leaf domain',()=>{
 assert.equal(acceptRustGenericTypeAliasConcreteTemplate('function',0,'function','','',false,2,true),false);
 assert.equal(acceptRustGenericTypeAliasConcreteCallableTemplate(true,'function',0,'function',0,2,1,[true],true,true,true,true,false),true);
 assert.equal(acceptRustGenericTypeAliasConcreteCallableTemplate(true,'function',0,'function',0,2,1,[false],true,true,true,true,false),false);
 assert.equal(acceptRustGenericTypeAliasConcreteCallableTemplate(true,'function',1,'function',0,2,1,[true],true,true,true,true,false),false);
 assert.equal(acceptRustGenericTypeAliasConcreteCallableTemplate(true,'function',0,'function',0,2,1,[true],true,true,true,true,true),false);
 assert.equal(acceptRustGenericTypeAliasConcreteCallableTemplate(true,'function',0,'function',1,2,1,[true],true,true,true,true,false),false);
 assert.equal(acceptRustGenericTypeAliasConcreteCallableTemplate(true,'function',0,'function',0,2,1,[true],true,false,true,true,false),false);
 assert.equal(acceptRustGenericTypeAliasHeaderCallableTemplate(true,'function',0,'function',0,2,1,[false],false,true,true,true,false),true);
 assert.equal(acceptRustGenericTypeAliasHeaderCallableTemplate(true,'function',0,'function',0,2,1,[false],false,true,false,true,false),false);
 assert.equal(acceptRustGenericTypeAliasHeaderCallableTemplate(true,'function',0,'function',0,2,1,[false],false,true,true,false,false),false);
});
function observe(code){const lines=[];vm.runInNewContext(code,{console:{log:(...values)=>lines.push(values.map(String).join(' '))},process:{argv:[]}}, {timeout:1000});return lines.length?lines.join('\n')+'\n':'';}
for(const row of genericAliasDefaultOriginals)for(const sourceVersion of ['1.90.0','1.99.0'])test(row.name+' / '+sourceVersion,()=>{
 assert.equal(genericAliasDefaultVersionRange,'>=1.90.0 <1.100.0');
 const checked=checkProgram(parseRust(row.source,{sourceVersion}));assert.equal(observe(emitJavaScript(checked).text),row.stdout);
 const packet=createTypedProgramLinks(row.source,'Rust',{rustSourceVersion:sourceVersion});const text=packet.network.toLino(),binary=encodeLinkNetworkBinary(packet.network);
 for(const graph of [packet.network,LinkNetwork.fromLino(text),decodeLinkNetworkBinary(binary)]){
  const restored=decodeTypedProgramLinks(graph,packet.root);assert.deepEqual(restored.items,checked.items);assert.deepEqual(restored.main,checked.main);
  for(const target of ['JavaScript','Rust','Lean','Rocq'])assert.deepEqual(emitTypedProgramLinks(graph,packet.root,target),emitTypedProgramLinks(packet.network,packet.root,target));
  assert.equal(observe(emitTypedProgramLinks(graph,packet.root,'JavaScript').code),row.stdout);
 }
});
for(const row of genericAliasDefaultRefusals)test(row.name,()=>{
 for(const sourceVersion of ['1.90.0','1.99.0'])assert.throws(()=>checkProgram(parseRust(row.source,{sourceVersion})),error=>['unsupported','type','syntax'].includes(error.kind));
});
