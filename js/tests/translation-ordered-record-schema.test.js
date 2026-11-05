// Deferred complete source/schema profile guards; retain both version inputs.
import assert from 'node:assert/strict';
import test from 'node:test';
import {readFileSync} from 'node:fs';
import {readOrderedRecordSourceProfiles,restoreOrderedRecordSchema,restoreOrderedRecordTypeSource,restoreOrderedRecordOwnedSource,readOrderedRecordSchemaPredecessor} from '../scripts/ordered-record-schema-profile.mjs';
test('ordered record schema retains the original five facets and guards all six active facets',async()=>{
 const profiles=await readOrderedRecordSourceProfiles();
 assert.equal(profiles.legacy.fields.length,5);assert.equal(profiles.current.fields.length,6);
 assert.equal(profiles.current.defaults.record.ordered,'false');
 const previous=readOrderedRecordSchemaPredecessor();
 for(const file of ['js/src/data/typed-program-links-schema.lino','rust/src/data/typed-program-links-schema.lino']){
  const bytes=readFileSync(new URL('../../'+file,import.meta.url));
  assert.equal(restoreOrderedRecordSchema(bytes,file),previous.previousSchema.source);
  assert.throws(()=>restoreOrderedRecordSchema(Buffer.from(bytes.toString('utf8').replace('optional-expression','expressions')),file));
 }
 const native=readFileSync(new URL('../../rust/src/translation/types.rs',import.meta.url));
 assert.equal(restoreOrderedRecordTypeSource(native,'rust/src/translation/types.rs'),previous.previousNativeTypes.source);
 assert.throws(()=>restoreOrderedRecordTypeSource(Buffer.concat([native,Buffer.from('\n')]),'rust/src/translation/types.rs'));
 for(const entry of previous.ownedSources){const bytes=readFileSync(new URL('../../'+entry.file,import.meta.url));assert.equal(restoreOrderedRecordOwnedSource(entry.file,bytes).toString('utf8'),entry.source);assert.throws(()=>restoreOrderedRecordOwnedSource(entry.file,Buffer.concat([bytes,Buffer.from('\n')])));}
});
