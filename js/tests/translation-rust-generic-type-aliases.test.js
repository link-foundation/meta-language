// Deferred controls. Do not execute during the complete-source drafting phase.
import assert from 'node:assert/strict';
import test from 'node:test';
import {runInNewContext} from 'node:vm';
import {parameterPlans,substitutionPlans,expansionPlans,declarationPlans,sourcePrograms,vectorScalarDomains} from './fixtures/rust-generic-type-alias-draft.mjs';
import {readRustGenericTypeAliasParameterPlan,readRustGenericTypeAliasSubstitutionPlan,readRustGenericTypeAliasExpansionError,readRustGenericTypeAliasDeclarationPlan,acceptRustGenericTypeAliasConcreteTemplate,acceptRustGenericTypeAliasNamedTemplate,readRustGenericTypeAliasStandardSumOwner,readRustGenericTypeAliasVectorOwner,acceptRustGenericTypeAliasVectorArguments,readRustGenericTypeAliasVectorElementError} from '../src/translation/rust-generic-type-alias-rules.js';
import {acceptRustScalarArrayIndexElement} from '../src/translation/rust-array-index-rules.js';
import {parseRust} from '../src/translation/rust.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {selfTranslationSignatures} from '../src/self-translation.js';
import {encodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {readProjectData} from '../src/project-data-files.js';

for(const [index,row]of parameterPlans.entries())test('generic alias actual parameter columns '+index,()=>assert.deepEqual(readRustGenericTypeAliasParameterPlan(row.kinds,row.values,row.raws,row.position),row.expected));
for(const [index,row]of substitutionPlans.entries())test('generic alias actual binding substitution '+index,()=>assert.deepEqual(readRustGenericTypeAliasSubstitutionPlan(row.parameters,row.argumentsProven,row.occurrences),row.expected));
for(const [index,row]of expansionPlans.entries())test('generic alias declaration recursion '+index,()=>assert.equal(readRustGenericTypeAliasExpansionError(row.declaration,row.declarations,row.expansions),row.expected));
for(const [index,row]of declarationPlans.entries())test('generic alias module declaration directory '+index,()=>assert.deepEqual(readRustGenericTypeAliasDeclarationPlan(row.kinds,row.values,row.raws),row.expected));
test('generic concrete leaves keep actual primitive ownership and width',()=>{
 assert.equal(acceptRustGenericTypeAliasConcreteTemplate('fixed',0,'primitive','i64','64',true,0,true),true);
 assert.equal(acceptRustGenericTypeAliasConcreteTemplate('fixed',0,'declaration','i64','64',true,0,true),false);
 assert.equal(acceptRustGenericTypeAliasConcreteTemplate('fixed',0,'primitive','u64','64',true,0,true),false);
 assert.equal(acceptRustGenericTypeAliasConcreteTemplate('fixed',1,'primitive','i64','64',true,0,true),false);
 assert.equal(acceptRustGenericTypeAliasConcreteTemplate('fixed',0,'primitive','i64','64',true,0,false),false);
});
test('symbolic standard sums retain canonical owners arity and actual named leaves',()=>{
 assert.equal(readRustGenericTypeAliasStandardSumOwner('std::option::Option','',1),'std::option::Option');
 assert.equal(readRustGenericTypeAliasStandardSumOwner('std::result::Result','',2),'std::result::Result');
 assert.equal(readRustGenericTypeAliasStandardSumOwner('std::option::Option','Some',1),'');
 assert.equal(readRustGenericTypeAliasStandardSumOwner('std::option::Option','',2),'');
 assert.equal(readRustGenericTypeAliasStandardSumOwner('local::Option','',1),'');
 assert.equal(acceptRustGenericTypeAliasNamedTemplate('named',0,'declaration','errors::Error',0,true,'errors.Error'),true);
 assert.equal(acceptRustGenericTypeAliasNamedTemplate('named',0,'declaration','caller::Error',0,true,'errors.Error'),false);
 assert.equal(acceptRustGenericTypeAliasNamedTemplate('named',0,'declaration','errors::Error',0,true,''),false);
});
test('generic vector templates retain actual owner arity scalar provenance and String ownership',()=>{
 assert.equal(readRustGenericTypeAliasVectorOwner('std::vec::Vec',false),'std::vec::Vec');
 assert.equal(readRustGenericTypeAliasVectorOwner('Vec',false),'std::vec::Vec');
 assert.equal(readRustGenericTypeAliasVectorOwner('std::vec::Vec',true),'');
 assert.equal(readRustGenericTypeAliasVectorOwner('alloc::vec::Vec',false),'');
 assert.equal(acceptRustGenericTypeAliasVectorArguments('std::vec::Vec',1),true);
 assert.equal(acceptRustGenericTypeAliasVectorArguments('std::vec::Vec',2),false);
 assert.equal(readRustGenericTypeAliasVectorElementError('fixed',0,'primitive','i64','64',true,0,true,''),'');
 assert.notEqual(readRustGenericTypeAliasVectorElementError('fixed',0,'declaration','i64','64',true,0,true,''),'');
 assert.notEqual(readRustGenericTypeAliasVectorElementError('fixed',0,'primitive','u64','64',true,0,true,''),'');
 assert.equal(readRustGenericTypeAliasVectorElementError('string',0,'library','std::string::String','',false,0,true,'owned string'),'');
 assert.equal(acceptRustScalarArrayIndexElement('string'),false);
 assert.notEqual(readRustGenericTypeAliasVectorElementError('string',1,'primitive','str','',false,0,true,'string slice'),'');
 assert.notEqual(readRustGenericTypeAliasVectorElementError('array',0,'library','std::vec::Vec','',false,1,true,''),'');
});
for(const row of vectorScalarDomains)test('generic vector retains actual '+row.name+' element domain',()=>{
 assert.equal(readRustGenericTypeAliasVectorElementError(row.kind,0,row.kind==='unit'?'unit':'primitive',row.kind==='unit'?'':row.name,row.bits,row.signed,0,true,''),'');
 assert.equal(acceptRustScalarArrayIndexElement(row.kind),true);
});

for(const row of sourcePrograms)test(row.name,()=>{
 const source=row.source+(row.source.includes('fn main()')?'':'fn main(){}');
 if(row.refusal){
  assert.throws(()=>checkProgram(parseRust(source,{retainSourceTypeOwners:true})));
  assert.throws(()=>selfTranslationSignatures(row.source,'Rust',{semanticTranslation:true}));
  return;
 }
 const program=checkProgram(parseRust(source,{retainSourceTypeOwners:true})),packet=encodeTypedProgramLinks(program);
 if(row.vectorSignature){
  const position=program.declarations.get(row.vectorSignature.function).sourceSignature.parameters[0];
  assert.equal(position.type.kind,'array');assert.equal(position.type.element.kind,row.vectorSignature.elementKind);
  assert.deepEqual(position.owner,{kind:'library',path:['std','vec','Vec'],arguments:[{kind:'library',path:row.vectorSignature.elementOwner.split('::'),arguments:[]}]});
 }
 for(const target of ['JavaScript','Rust','Lean','Rocq']){
  const expected=emitTypedProgramLinks(packet.network,packet.root,target);
  for(const network of [LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),expected);
 }
 const signatures=selfTranslationSignatures(row.source,'Rust',{semanticTranslation:true});
 if(row.expectedExports){assert.deepEqual(signatures.map(signature=>signature.name).sort(),row.expectedExports);return;}
 assert.deepEqual(signatures.map(signature=>signature.name),['result']);
 const emitted=emitJavaScript(program),mapping=emitted.mappings.find(value=>value.kind==='function'&&value.source==='result');
 assert.ok(mapping,'actual result function mapping');
 const result=runInNewContext(emitted.text.replace(/^export /gmu,'')+'\n'+mapping.target+'()');
 assert.equal(String(result),row.expected);
});

for(const source of [
 'type Erased<T>=i64;',
 'type Wrapped<T>=Box<T>;',
 'type Callable<T>=fn(T)->T;',
 'type Defaulted<T=i64>=T;',
 'type Reference<\'a,T>=&\'a T;',
 'type Repeated<T,T>=T;',
 'type Identity<T>=T;pub fn read()->Identity<i64,bool>{7}',
 'type Pair<T>=(T,T);pub fn read()->Pair<String>{(String::new(),String::new())}'
])test('unsupported generic alias interpretation retains refusal: '+source,()=>assert.throws(()=>checkProgram(parseRust(source+'fn main(){}'))));

test('an unchanged generic refusal input retains its preceding assertion as source history',()=>{
 const row=sourcePrograms.find(value=>value.predecessorAssertion);
 assert.equal(row.source,'type Identity<T>=T;fn main(){}');
 assert.equal(row.predecessorAssertion,'generic, constrained or malformed type aliases require their complete parameter interpretation');
 assert.deepEqual(selfTranslationSignatures(row.source,'Rust',{semanticTranslation:true}),[]);
});
test('the original symbolic vector declaration retains its preceding refusal and now exports no value',()=>{
 const row=sourcePrograms.find(value=>value.source==='type Vector<T>=Vec<T>;fn main(){}');
 assert.deepEqual(row.previousExpectation,{kind:'unsupported',reason:'Rust array element: only direct immutable scalar and owned string elements are admitted'});
 assert.deepEqual(selfTranslationSignatures(row.source,'Rust',{semanticTranslation:true}),[]);
});

test('the fixture author retains the exact corrected generic original and previous oracle',()=>{
 const fixture=readProjectData(new URL('../../parity/fixtures/rust-type-aliases.lino',import.meta.url));
 assert.equal(fixture.genericOriginalControls.length,1);
 for(const row of fixture.genericOriginalControls){
  assert.equal(row.source,'type Identity<T>=T;fn main(){}');
  assert.deepEqual(row.previousExpectation,{kind:'unsupported',reason:'Rust type alias: generic, constrained or malformed type aliases require their complete parameter interpretation'});
  assert.ok(row.correctionReason.length>0);
  assert.deepEqual(selfTranslationSignatures(row.source,'Rust',{semanticTranslation:true}),row.expectedExports);
  const program=checkProgram(parseRust(row.source)),packet=encodeTypedProgramLinks(program);
  for(const target of ['JavaScript','Rust','Lean','Rocq']){
   const expected=emitTypedProgramLinks(packet.network,packet.root,target);
   for(const network of [LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),expected);
  }
 }
});
