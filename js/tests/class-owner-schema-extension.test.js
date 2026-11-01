// Deferred schema source controls. No generator or behavior runs during drafting.
import test from 'node:test';
import assert from 'node:assert/strict';
import {extendClassOwnerSchema,extendCurrentTypedProgramSourceSchema} from '../scripts/class-owner-schema-extension.mjs';
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

test('the pending complete successor includes proposition values without changing either predecessor',()=>{
 const before={...source(),expressionFields:{bool:['k','value','type']},expressionOperands:{bool:[]},expressionOperations:{},expressionProjections:{bool:{construct:'bool',fields:[]}}};
 const saved=structuredClone(before),after=extendCurrentTypedProgramSourceSchema(before);
 assert.deepEqual(before,saved);
 assert.deepEqual(after.expressionFields,{bool:['k','value','type'],propositionValue:['k','op','left','right','domain','sameValue','type','span']});
 assert.deepEqual(after.expressionOperands,{bool:[],propositionValue:['left','right']});
 assert.deepEqual(after.expressionOperations,{propositionValue:['eq','ne','lt','le','gt','ge']});
 assert.deepEqual(after.expressionProjections,{bool:{construct:'bool',fields:[]},propositionValue:{construct:'propositionValue',fields:[['prop','proposition-comparison','left']]}});
 assert.equal(after.references.classConstructor,'source class constructor reference');
 assert.deepEqual(extendCurrentTypedProgramSourceSchema(after),after);
 const conflicting={...saved,expressionOperands:{...saved.expressionOperands,propositionValue:['right','left']}};
 const snapshot=structuredClone(conflicting);assert.throws(()=>extendCurrentTypedProgramSourceSchema(conflicting),TypeError);assert.deepEqual(conflicting,snapshot);
});
