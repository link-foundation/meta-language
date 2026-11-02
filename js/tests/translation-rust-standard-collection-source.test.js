// Deferred source controls. No source parse, check, behavior or generation runs
// while drafting this batch; these tests are ordinary future test inputs.
import test from 'node:test';
import assert from 'node:assert/strict';
import {initializeRustStandardCollectionSource,readRustStandardCollectionSourceImport,readRustStandardCollectionSourceStorage,finishRustStandardCollectionSource,rustStandardCollectionSourceModel,refuseRustStandardCollectionSourcePrivateName} from '../src/translation/rust-standard-collection-source.js';
import {readRustStandardCollectionSourceScalarDomain,readRustStandardCollectionSourceOrigin} from '../src/translation/rust-standard-collection-types.js';
import {readRustStandardCollectionSourceConstructor,readRustStandardCollectionSourceMethod} from '../src/translation/rust-standard-collection-calls.js';
import {projectRustStandardCollectionLibraryType,encodeRustStandardCollectionSourceConstant,constructRustStandardCollectionSourceVariant,matchRustStandardCollectionSourceVariant,readRustStandardCollectionSourcePositionError} from '../src/translation/rust-standard-collection-signatures.js';
const primitive=(domain,bits,signed)=>({proven:true,wrappers:[],type:domain==='bool'?{kind:'bool'}:{kind:'fixed',bits,signed},sourceOwner:{kind:'primitive',path:[domain],arguments:[]}});
test('legacy source mode introduces no collection import type or method admission',()=>{
 const parser={origins:{}};initializeRustStandardCollectionSource(parser,'',null,64);assert.equal(parser.standardCollectionSource,null);assert.equal(parser.origins.standardCollectionSourceStorage,null);assert.equal(readRustStandardCollectionSourceImport(parser,['std','collections','HashMap'],[]),false);assert.equal(readRustStandardCollectionSourceConstructor(parser,['HashMap','new'],[],[],{}),null);assert.equal(readRustStandardCollectionSourceMethod(parser,{}, {value:'insert'},[],{}),null);const program={items:[]};assert.equal(finishRustStandardCollectionSource(parser,program),program);
});
test('opt-in admission requires actual complete checked library bodies and explicit active target coverage',()=>{
 for(const library of [null,{}, {sourceProgram:{items:[]},functions:[{role:'packet',bodyAvailable:true}]}])assert.throws(()=>initializeRustStandardCollectionSource({origins:{},sourceVersionMinor:90},rustStandardCollectionSourceModel,library,64),/complete actual|projected library/);
 assert.throws(()=>initializeRustStandardCollectionSource({origins:{},sourceVersionMinor:89},rustStandardCollectionSourceModel,{},64),/uncovered outside Rust/);
 assert.throws(()=>initializeRustStandardCollectionSource({origins:{}},rustStandardCollectionSourceModel,{},32),/64-bit/);assert.throws(()=>initializeRustStandardCollectionSource({origins:{}},'invented model',{},64),/exact closed/);
});
test('source scalar domains retain actual primitive width sign and standard owned String origins',()=>{
 assert.equal(readRustStandardCollectionSourceScalarDomain(primitive('u8',8,false),64),'u8');assert.equal(readRustStandardCollectionSourceScalarDomain(primitive('i128',128,true),64),'i128');assert.equal(readRustStandardCollectionSourceScalarDomain(primitive('bool'),64),'bool');assert.equal(readRustStandardCollectionSourceScalarDomain(primitive('usize',64,false),64),'usize');assert.equal(readRustStandardCollectionSourceScalarDomain(primitive('usize',64,false),32),'');
 for(const changed of [{...primitive('u8',8,false),proven:false},{...primitive('u8',8,false),wrappers:['reference']},{...primitive('u8',8,false),sourceOwner:{kind:'declaration',path:['u8'],arguments:[]}},{...primitive('u8',16,false)},{...primitive('u8',8,true)}])assert.equal(readRustStandardCollectionSourceScalarDomain(changed,64),'');
 const string={proven:true,wrappers:[],type:{kind:'string'},textSourceModel:'owned string',sourceOwner:{kind:'library',path:['std','string','String'],arguments:[]}};assert.equal(readRustStandardCollectionSourceScalarDomain(string,64),'String');assert.equal(readRustStandardCollectionSourceScalarDomain({...string,textSourceModel:'string slice'},64),'');
});
test('source position joins cannot erase different collection specializations behind one carrier type',()=>{
 assert.equal(readRustStandardCollectionSourcePositionError({}, {}, {}),'');assert.notEqual(readRustStandardCollectionSourcePositionError({}, {sourceCollectionTypeIdentity:'map u8 i32'},{}),'');assert.notEqual(readRustStandardCollectionSourcePositionError({}, {sourceCollectionTypeIdentity:'map u8 i32'},{sourceCollectionTypeIdentity:'map u8 u16'}),'');assert.equal(readRustStandardCollectionSourceStorage({standardCollectionSource:null},{}),false);assert.equal(readRustStandardCollectionSourceOrigin({standardCollectionSource:null},{}),null);
});
test('private generated implementation paths cannot acquire public original-source visibility',()=>{
 const parser={standardCollectionSource:{privateNames:new Set(['kernel.State','kernel.execute'])},resolve:segments=>segments};
 for(const segments of [['kernel','State'],['kernel','State','Variant'],['crate','kernel','execute']])assert.throws(()=>refuseRustStandardCollectionSourcePrivateName(parser,segments,[],{}),/private/);
 assert.doesNotThrow(()=>refuseRustStandardCollectionSourcePrivateName(parser,['std','collections','HashMap'],[],{}));
});
test('source library type projection retains complete nested actual data identities',()=>{
 const type={kind:'tuple',elements:[{kind:'data',name:'kernel.State'},{kind:'array',element:{kind:'data',name:'kernel.Scalar'}},{kind:'option',element:{kind:'data',name:'kernel.Reference'},absence:'undefined'}]};
 assert.deepEqual(projectRustStandardCollectionLibraryType(type),{kind:'tuple',elements:[{kind:'named',path:['crate','kernel','State']},{kind:'array',element:{kind:'named',path:['crate','kernel','Scalar']}},{kind:'option',element:{kind:'named',path:['crate','kernel','Reference']},absence:'undefined'}]});assert.equal(type.elements[0].kind,'data');
});
test('literal metadata authoring uses actual datatype fields without executing represented methods',()=>{
 const type={kind:'data',name:'Scalar'},library={program:{declarations:new Map([['Scalar',{k:'data',ctors:[{name:'integer',fields:[{name:'value',type:{kind:'int'}}]},{name:'boolean',fields:[{name:'value',type:{kind:'bool'}}]}]}]])}},span={start:1,end:2};
 const encoded=encodeRustStandardCollectionSourceConstant(library,{$:'integer',value:9007199254740993n},type,span);assert.deepEqual(encoded.fn.path,['crate','Scalar','integer']);assert.equal(encoded.args[0].value,'9007199254740993');assert.equal(encoded.args[0].type.kind,'int');assert.throws(()=>encodeRustStandardCollectionSourceConstant(library,{$:'invented',value:1n},type,span),/actual complete/);
 const value={k:'name',path:['held']};assert.deepEqual(constructRustStandardCollectionSourceVariant(library,type,'integer',new Map([['value',value]]),span).args,[value]);assert.throws(()=>constructRustStandardCollectionSourceVariant(library,type,'integer',new Map(),span),/every actual field/);
 let index=0;const parser={freshProductBinding:()=>String(index++)},matched=matchRustStandardCollectionSourceVariant(parser,library,type,value,'integer',fields=>fields.get('value').expression,()=>({k:'unit'}),span);assert.equal(matched.rows.length,2);assert.deepEqual(matched.rows[0].patterns[0].path,['crate','Scalar','integer']);assert.deepEqual(matched.rows[1].patterns[0].path,['crate','Scalar','boolean']);
});
