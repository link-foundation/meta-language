import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import test from 'node:test';
import {areSynonyms,baseForms,loadWordNet,PARTS_OF_SPEECH,partsOfSpeech} from '../scripts/english-vocabulary.mjs';
import {checkConceptRecords,loadNamingRegisters} from '../scripts/issue-195-naming.mjs';
import {readProjectData} from '../scripts/project-data.mjs';
import {fileURLToPath} from 'node:url';

const controls=readProjectData(new URL('../../parity/fixtures/naming-candidate-controls.lino',import.meta.url));

test('indexed synonym candidates preserve complete ordered original WordNet diagnostics',()=>{
  const {baseline}=controls;
  assert.equal(baseline.revision,'61a96299e459aee352c57fb2abe04d4a77dd6f93');
  assert.equal(baseline.file,'js/scripts/issue-195-naming.mjs');
  assert.equal(baseline.bytes,12292);
  assert.equal(baseline.sha256,'d2170d64b7972903d8b66052e0274188a34c4722915088e8d6f28b7d95d41f87');
  assert.equal(createHash('sha256').update(baseline.source).digest('hex'),baseline.sha256);
  assert.equal(Buffer.byteLength(baseline.source),baseline.bytes);
  const original=new Function('areSynonyms','baseForms','PARTS_OF_SPEECH','partsOfSpeech',
    baseline.source.replace(/\bexport /gu,'')+'\nreturn checkConceptRecords;'
  )(areSynonyms,baseForms,PARTS_OF_SPEECH,partsOfSpeech);
  const wordnet=loadWordNet(),{vocabulary}=loadNamingRegisters(fileURLToPath(new URL('../../',import.meta.url)));
  const context={wordnet,vocabulary};
  let duplicates=0,distinctions=0;
  for(const batch of controls.batches){
    const records=structuredClone(batch.records);
    const orders=[records,[...records].reverse(),records.filter((_,index)=>index%2).concat(records.filter((_,index)=>index%2===0))];
    for(const ordered of orders){
      const expected=original(ordered,context),actual=checkConceptRecords(ordered,context);
      assert.deepEqual(actual,expected,batch.name);
      duplicates+=expected.filter(row=>row.kind==='duplicate').length;
      distinctions+=expected.filter(row=>row.message.includes('distinct from')||row.message.includes('no distinction')).length;
    }
  }
  assert.ok(duplicates>0);assert.ok(distinctions>0);
  const mutable=structuredClone(controls.batches[0].records);
  checkConceptRecords(mutable,context);
  mutable[0].phrase='unrecorded vocabulary';
  assert.deepEqual(checkConceptRecords(mutable,context),original(mutable,context),'a later record mutation is rechecked');
  const independent=structuredClone(context);
  independent.wordnet.synsets.noun.set('parameter',['synthetic independent synset']);
  assert.deepEqual(checkConceptRecords(controls.batches[0].records,independent),original(controls.batches[0].records,independent),'a later WordNet mutation is rechecked');
});
