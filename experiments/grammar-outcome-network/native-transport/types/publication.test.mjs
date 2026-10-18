// Additional source-only controls; author has not executed this file.
import assert from 'node:assert/strict';
import test from 'node:test';
import {createHash} from 'node:crypto';
import {serializeProjectData,readProjectData} from '../../../../js/scripts/project-data.mjs';
import {readStructuredDataLino} from '../../../../js/src/structured-data-links.js';
import {buildTypePublication,reverseTypePublication,checkTypePublication} from './generate.mjs';

const digest=source=>createHash('sha256').update(source).digest('hex');

test('normal Links policy conversion retains exact source literal witnesses and complete predecessor inverse',()=>{
  const packet=buildTypePublication(),record=packet.receipt;
  assert.equal(digest(record.original),record.originalModuleSha256);
  assert.equal(reverseTypePublication(packet.module,record.edits),record.original);
  assert.equal(packet.tests.slice(0,record.originalTests.length),record.originalTests);
  assert.equal(digest(record.originalTests),record.originalTestSha256);
  assert.deepEqual(readStructuredDataLino(packet.policySource),packet.policy);
  assert.equal(serializeProjectData(packet.policy,'type-policy.lino'),packet.policySource);
  assert.equal(digest(packet.policySource),record.policy.sha256);
  for(const witness of record.witnesses){
    assert.equal(Buffer.from(record.original).subarray(witness.span.start,witness.span.end).toString(),witness.source);
    assert.equal(digest(witness.source),witness.sha256);
  }
  assert.deepEqual(packet.policy.requiredArguments,Object.keys(packet.policy.genericArities));
  assert.equal(packet.policy.referenceProperties.lifetime,'static');assert.equal(packet.policy.referenceProperties.mutable,false);
  assert.equal(packet.policy.builtinOwners.String,'std::string::String');
  assert.equal(packet.policy.scalarProperties.usize.integer.bits,null);
  assert.ok(packet.module.includes("assert.ok(wellFormedString(source),'unpaired UTF-16 type source')"));
  assert.ok(packet.tests.includes('漢字 😀 é'));
});

test('portable imports have an exact complete inverse and tampered edit anchors refuse',()=>{
  const packet=buildTypePublication({portable:true}),record=packet.receipt;
  assert.equal(reverseTypePublication(packet.module,record.edits),record.original);
  for(const [from,to]of record.importRewrites){assert.ok(packet.module.includes("from '"+to+"'"));assert.ok(!packet.module.includes("from '"+from+"'"));}
  const changed=packet.module.replace(record.edits.at(-1).to,'/* changed import */');
  assert.throws(()=>reverseTypePublication(changed,record.edits),/one complete source occurrence/u);
  assert.notEqual(reverseTypePublication(packet.module+' ',record.edits),record.original,'whole-source equality is mandatory after reversal');
});

test('permanent regeneration reads its normal Links originals and checks every generated artifact',()=>{
  const archive=readProjectData(new URL('./type-publication.lino',import.meta.url));
  const direct=buildTypePublication(),explicit=buildTypePublication({archive});
  assert.equal(direct.module,explicit.module);assert.equal(direct.tests,explicit.tests);
  assert.deepEqual(direct.policy,explicit.policy);assert.deepEqual(direct.receipt,archive);
  assert.deepEqual(checkTypePublication().receipt,archive);
});

test('archival originals source witnesses portable imports and generated hashes refuse drift',()=>{
  const archive=readProjectData(new URL('./type-publication.lino',import.meta.url));
  const edits=[
    value=>{value.original+=' ';},
    value=>{value.originalTests+=' ';},
    value=>{value.importRewrites[0][1]='../foreign/generate.mjs';},
    value=>{value.witnesses[0].source+=' ';},
    value=>{value.edits[0].to+=' ';},
    value=>{value.publishedModule.sha256='0'.repeat(64);},
    value=>{value.policy.sha256='0'.repeat(64);},
    value=>{value.boundary='complete native inverse';},
    value=>{value.unreviewed=true;},
  ];
  for(const edit of edits){const changed=structuredClone(archive);edit(changed);assert.throws(()=>buildTypePublication({archive:changed}));}
});

test('runtime policy guard authenticates canonical decoded value while freshness retains raw canonical bytes',()=>{
  const packet=buildTypePublication(),decorated=packet.policySource+'\n';
  const decoded=readStructuredDataLino(decorated);
  assert.deepEqual(decoded,packet.policy);
  assert.equal(digest(serializeProjectData(decoded,'type-policy.lino')),packet.receipt.policy.sha256);
  assert.notEqual(digest(decorated),packet.receipt.policy.sha256,'raw formatting has a separate byte identity');
});
