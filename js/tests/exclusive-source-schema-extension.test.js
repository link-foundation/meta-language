// Deferred additive vocabulary controls; no schema generation during drafting.
import test from 'node:test';import assert from 'node:assert/strict';
import {extendExclusiveSourceSchema} from '../scripts/exclusive-source-schema-extension.mjs';
test('exclusive signature vocabulary preserves predecessor fields without mutating input',()=>{
 const before={fields:{sourceSignature:'source signature',position:'position'},expressionFields:{name:['type']},references:{},referenceSourceFields:{},referenceTargetFields:{}},after=extendExclusiveSourceSchema(before);assert.deepEqual(before,{fields:{sourceSignature:'source signature',position:'position'},expressionFields:{name:['type']},references:{},referenceSourceFields:{},referenceTargetFields:{}});assert.equal(after.fields.sourceSignature,'source signature');assert.equal(after.fields.sourceExclusiveSignature,'exclusive source signature');assert.equal(after.fields.rustExclusiveSourceModel,'Rust exclusive source model');assert.deepEqual(extendExclusiveSourceSchema(after),after);
});
test('exclusive vocabulary refuses conflicting names labels and incomplete schemas',()=>{
 for(const before of [{},{fields:[]},{fields:{sourceExclusiveSignature:'another meaning'}},{fields:{different:'exclusive source signature'}},{fields:{different:'Rust exclusive source model'}}])assert.throws(()=>extendExclusiveSourceSchema(before),TypeError);
});
test('FnMut specialization vocabulary retains its distinct source facet and profile spelling',()=>{
 const base={fields:{sourceSignature:'source signature'},references:{},referenceSourceFields:{},referenceTargetFields:{}},after=extendExclusiveSourceSchema(base);
 assert.equal(after.fields.sourceFnMutSpecialization,'source mutable function specialization');assert.equal(after.fields.rustFnMutSourceModel,'Rust source mutable function model');assert.equal(after.fields.sourceExclusiveSignature,'exclusive source signature');assert.equal(after.fields.sourceSignature,'source signature');
 for(const fields of [{sourceFnMutSpecialization:'source signature'},{another:'source mutable function specialization'},{rustFnMutSourceModel:'different profile'}])assert.throws(()=>extendExclusiveSourceSchema({...base,fields}),TypeError);
 assert.equal(after.references.standardCollectionLibraryDeclaration,'standard collection library declaration reference');assert.equal(after.referenceTargetFields.standardCollectionLibraryDeclaration,'fullName');
});
