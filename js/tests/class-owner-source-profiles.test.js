// Deferred complete-source/receipt assertions, independent of represented
// program behavior. Run only after the explicit future author prerequisites.
import assert from 'node:assert/strict';
import test from 'node:test';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {readStructuredDataLino} from '../src/structured-data-links.js';
import {serializeProjectData} from '../scripts/project-data.mjs';
import {readClassOwnerSourceProfile,restoreClassOwnerSchema,restoreClassOwnerNativeTypeSource,restoreClassOwnerOwnedSource} from '../scripts/class-owner-schema-profile.mjs';
import {readOrderedRecordSchemaPredecessor,restoreOrderedRecordSchema,restoreOrderedRecordTypeSource,restoreOrderedRecordOwnedSource,readOrderedRecordSourceProfiles} from '../scripts/ordered-record-schema-profile.mjs';
import {readProgramTypeNeutralDefaults} from '../scripts/read-program-type-neutral-defaults.mjs';
const root=new URL('../../',import.meta.url),hash=bytes=>createHash('sha256').update(bytes).digest('hex');
test('all three native Type profiles retain their independent complete source guards',async()=>{
 const legacy=readProgramTypeNeutralDefaults({version:1}),ordered=readProgramTypeNeutralDefaults({version:2}),current=readProgramTypeNeutralDefaults({version:3}),receipt=readClassOwnerSourceProfile();
 assert.equal(legacy.source.sha256,'329dfababaff646a3ad870e76abbf026924b8c367cde6b9a5364def05fdd50d0');
 assert.equal(ordered.source.sha256,'89d8ca15420493e3e9210a1b0a5b359d55d8a9821a7445806ea82a98fd99f92f');
 assert.equal(current.source.sha256,receipt.currentNativeTypes.sha256);assert.equal(receipt.predecessorRevision,'397c2ea47e8531558857dc54caf88358fd51033a');
 assert.equal(legacy.fields.length,5);assert.equal(ordered.fields.length,6);assert.equal(current.fields.length,6);
 assert.deepEqual(current.defaults,ordered.defaults);assert.equal(Object.hasOwn(legacy,'optionalFields'),false);assert.equal(Object.hasOwn(ordered,'optionalFields'),false);
 assert.deepEqual(current.optionalFields.map(row=>[row.kind,row.field,row.nativeField,row.absence]),[['record','sourceClassOwner','source_class_owner','omitted field']]);
 assert.equal(Object.hasOwn(current.defaults.record,'sourceClassOwner'),false);
 const profiles=await readOrderedRecordSourceProfiles();assert.equal(profiles.version,3);assert.deepEqual(profiles.legacy,legacy);assert.deepEqual(profiles.ordered,ordered);assert.deepEqual(profiles.current,current);
 assert.throws(()=>readProgramTypeNeutralDefaults({version:4}),RangeError);
});
test('complete current native bytes restore both historical source owners and reject unrelated edits',()=>{
 const receipt=readClassOwnerSourceProfile(),old=readOrderedRecordSchemaPredecessor(),bytes=readFileSync(new URL('rust/src/translation/types.rs',root));
 assert.equal(restoreClassOwnerNativeTypeSource(bytes,'rust/src/translation/types.rs'),receipt.previousNativeTypes.source);
 assert.equal(restoreOrderedRecordTypeSource(bytes,'rust/src/translation/types.rs'),old.previousNativeTypes.source);
 const altered=Buffer.concat([bytes,Buffer.from('\n// independent unknown edit\n')]);
 assert.throws(()=>restoreClassOwnerNativeTypeSource(altered,'rust/src/translation/types.rs'),/uncovered complete native Type successor/u);
 assert.throws(()=>restoreOrderedRecordTypeSource(altered,'rust/src/translation/types.rs'));
 assert.throws(()=>restoreClassOwnerNativeTypeSource(bytes,'rust/src/translation/invented.rs'));
});
test('each schema keeps exact ordered-record and original predecessors while new constructor edges remain guarded',()=>{
 const receipt=readClassOwnerSourceProfile(),original=readOrderedRecordSchemaPredecessor();
 for(const row of receipt.schemas){const bytes=readFileSync(new URL(row.file,root)),current=readStructuredDataLino(bytes.toString('utf8'));
  assert.equal(hash(bytes),row.current.sha256);assert.equal(restoreClassOwnerSchema(bytes,row.file),row.previous.source);assert.equal(restoreOrderedRecordSchema(bytes,row.file),original.previousSchema.source);
  assert.equal(current.references.classConstructor,'source class constructor reference');assert.deepEqual(current.referenceSourceFields.classConstructor,[['','record','sourceClassOwner']]);assert.equal(current.referenceTargetFields.classConstructor,'fullName');assert.ok(current.optionalReferenceRoles.includes('classConstructor'));
  for(const action of ['wrong role','lost optional role','unknown field']){const changed=structuredClone(current);if(action==='wrong role')changed.references.classConstructor='invented reference';else if(action==='lost optional role')changed.optionalReferenceRoles=changed.optionalReferenceRoles.filter(role=>role!=='classConstructor');else changed.fields.invented='invented';const altered=Buffer.from(serializeProjectData(changed,row.file));assert.throws(()=>restoreClassOwnerSchema(altered,row.file));assert.throws(()=>restoreOrderedRecordSchema(altered,row.file));}
 }
});
test('neutral-default reader source retains the exact version-two and original source assertion owners',()=>{
 const receipt=readClassOwnerSourceProfile(),original=readOrderedRecordSchemaPredecessor(),row=receipt.ownedSources[0],bytes=readFileSync(new URL(row.file,root));
 assert.equal(restoreClassOwnerOwnedSource(row.file,bytes).toString('utf8'),row.previous.source);assert.equal(restoreOrderedRecordOwnedSource(row.file,bytes).toString('utf8'),original.ownedSources.find(peer=>peer.file===row.file).source);
 assert.throws(()=>restoreClassOwnerOwnedSource(row.file,Buffer.concat([bytes,Buffer.from('\n// unknown owner edit\n')])));
});
