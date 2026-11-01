// Deferred schema source controls. No generator or behavior runs during drafting.
import test from 'node:test';
import assert from 'node:assert/strict';
import {extendClassOwnerSchema} from '../scripts/class-owner-schema-extension.mjs';
const source=()=>({references:{declaration:'declaration reference'},referenceSourceFields:{declaration:[['call','','fn']]},referenceTargetFields:{declaration:'fullName'},fields:{name:'name'},neutralTypeFieldDefaults:{record:{ordered:'false',open:'false',reference:'false'}}});
test('class constructor vocabulary is additive and leaves original schema owned by caller',()=>{
 const before=source(),snapshot=structuredClone(before),after=extendClassOwnerSchema(before);
 assert.deepEqual(before,snapshot);
 assert.deepEqual(after.references,{...snapshot.references,classConstructor:'source class constructor reference'});
 assert.deepEqual(after.referenceSourceFields,{...snapshot.referenceSourceFields,classConstructor:[['','record','sourceClassOwner']]});
 assert.deepEqual(after.referenceTargetFields,{...snapshot.referenceTargetFields,classConstructor:'fullName'});
 assert.deepEqual(after.fields,{...snapshot.fields,sourceClassOwner:'source class owner',sourceClassOwners:'source class owners'});
 assert.deepEqual(after.optionalReferenceRoles,['classConstructor']);
 assert.deepEqual(after.neutralTypeFieldDefaults,snapshot.neutralTypeFieldDefaults);
 assert.deepEqual(extendClassOwnerSchema(after),after);
});
test('plain record owners are optional and previously registered optional roles are retained',()=>{
 const before=source();before.optionalReferenceRoles=['declaration'];
 assert.deepEqual(extendClassOwnerSchema(before).optionalReferenceRoles,['declaration','classConstructor']);
});
test('conflicting vocabulary and invalid optional reference owners retain explicit refusals',()=>{
 for(const edit of [before=>before.references.classConstructor='other',before=>before.referenceSourceFields.classConstructor=[['','record','name']],before=>before.referenceTargetFields.classConstructor='name',before=>before.fields.sourceClassOwner='other',before=>before.optionalReferenceRoles=['missing'],before=>before.optionalReferenceRoles=null]){
  const before=source();edit(before);const retained=structuredClone(before);
  assert.throws(()=>extendClassOwnerSchema(before),TypeError);assert.deepEqual(before,retained);
 }
 for(const before of [{},{references:{},referenceSourceFields:{},referenceTargetFields:{}}])assert.throws(()=>extendClassOwnerSchema(before),TypeError);
});
