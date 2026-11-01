// Deferred source controls. No program/compiler is invoked during authoring.
import test from 'node:test';
import assert from 'node:assert/strict';
import {sanitizeRustSourceTypeOrigin,cloneRustSourceTypeOrigin} from '../src/translation/rust-source-type-origins.js';
import {cloneRustCallableTypeOrigin} from '../src/translation/rust-function-pointers.js';
import {readRustSourceTypeOriginRuntimeFieldPlan} from '../src/translation/rust-source-type-origin-rules.js';
import {readRustFunctionPointerTypeError,readRustFunctionPointerOwnedDomainError,readRustFunctionPointerDomainError,readRustFunctionPointerHeaderDomainError} from '../src/translation/rust-function-pointer-rules.js';
function leaf(){return {type:{kind:'unit'},path:[],wrappers:[],sourceOwner:{kind:'unit',path:[],arguments:[]},proven:true,copyEvidence:true,sourceFunctionPointerHeader:true,copyClosureCaptureCount:2,copyClosureSpan:{start:1,end:2},sourceBindingIdentity:'body binding',sourcePlaceBindingIdentity:'body place',sourceLoanBindingIdentities:['loan'],sourcePlaceLoanBindingIdentities:['place loan'],sourceExtendingTemporaryBindings:['temporary'],sourceOwnedResultProof:true,sourceBorrowedPlace:true,sourceStandardSumPatternMove:true,sourceStandardSumBorrowedField:true,freshArrayValue:{start:3,end:4},sourceLifetimes:['static']};}
test('source type clone clears every runtime lineage and preserves validation witnesses throughout actual child roles',()=>{
 const input={...leaf(),fields:[leaf()],callableParameters:[leaf()],callableResult:leaf(),standardSumSource:{owner:'std::option::Option',provisional:true,arguments:[leaf()]}},before=structuredClone(input);
 const output=cloneRustCallableTypeOrigin(input);assert.deepEqual(input,before);assert.notEqual(output,input);
 for(const value of [output,...output.fields,...output.callableParameters,output.callableResult,...output.standardSumSource.arguments]){
  assert.equal(Object.hasOwn(value,'sourceBindingIdentity'),false);assert.equal(Object.hasOwn(value,'sourcePlaceBindingIdentity'),false);
  for(const name of ['sourceLoanBindingIdentities','sourcePlaceLoanBindingIdentities','sourceExtendingTemporaryBindings'])assert.deepEqual(value[name],[]);
  for(const name of ['sourceOwnedResultProof','sourceBorrowedPlace','sourceStandardSumPatternMove','sourceStandardSumBorrowedField','freshArrayValue'])assert.equal(value[name],false);
  assert.equal(value.proven,true);assert.equal(value.copyEvidence,true);assert.equal(value.sourceFunctionPointerHeader,true);assert.equal(value.copyClosureCaptureCount,2);assert.deepEqual(value.copyClosureSpan,{start:1,end:2});assert.deepEqual(value.sourceLifetimes,['static']);assert.deepEqual(value.sourceOwner,{kind:'unit',path:[],arguments:[]});
 }
 assert.equal(output.standardSumSource.provisional,true);assert.equal(cloneRustCallableTypeOrigin(null),null);assert.equal(cloneRustCallableTypeOrigin(undefined),null);
 const inplace=leaf();assert.equal(sanitizeRustSourceTypeOrigin(inplace),inplace);assert.deepEqual(cloneRustSourceTypeOrigin(inplace),inplace);
 assert.equal(readRustSourceTypeOriginRuntimeFieldPlan().length,20);
});
test('safe owned pointer profile keeps the old Copy oracle and explicit version/domain boundaries',()=>{
 assert.equal(readRustFunctionPointerTypeError(true,[false],true),'function pointer parameters require direct immutable Copy source types');
 assert.equal(readRustFunctionPointerOwnedDomainError(true,['owned'],'copy',90),'');assert.equal(readRustFunctionPointerOwnedDomainError(true,['copy'],'owned',99),'');
 assert.notEqual(readRustFunctionPointerOwnedDomainError(true,['copy'],'copy',90),'');
 for(const role of ['','borrowed','custom Drop'])assert.notEqual(readRustFunctionPointerOwnedDomainError(true,[role],'owned',90),'');
 for(const version of [89,100])assert.match(readRustFunctionPointerOwnedDomainError(true,['owned'],'copy',version),/uncovered/);
 assert.equal(readRustFunctionPointerDomainError(true,[false],true,['owned'],'copy',90),'');
 assert.match(readRustFunctionPointerDomainError(true,[false],true,['owned'],'copy',100),/uncovered/);
 assert.notEqual(readRustFunctionPointerHeaderDomainError(true,[false],true,['owned'],'copy',100,false),'');
 assert.equal(readRustFunctionPointerHeaderDomainError(true,[false],false,[''],'',90,true),'');
});
