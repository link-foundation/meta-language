import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import test from 'node:test';
import {createOriginalLinkSuccessorSourceArchives} from '../../experiments/original-link-complete-table-projection/successor-source-archives.mjs';
const row=(profile,source)=>({source,profile:{profile,sourceBytes:Buffer.byteLength(source),sourceSha256:createHash('sha256').update(source).digest('hex'),declarationNames:['owned original source']}});
const current=row('current','current complete source'),successors=[row('consuming','consuming complete source'),row('borrowed consuming','borrowed complete source')];
const receipt=source=>{const digest=createHash('sha256').update(source).digest('hex');return{sourceSha256:digest,textReplaySha256:digest,binaryReplaySha256:digest};};

test('every complete successor archive finishes before matrix attachment and all profiles reuse independent exact replay receipts',()=>{
  const cache=createOriginalLinkSuccessorSourceArchives(),calls=[];
  const token=cache.prepare(current,successors,source=>{calls.push(source);return receipt(source);});
  assert.deepEqual(calls,[current.source,...successors.map(row=>row.source)]);
  assert.deepEqual(cache.currentReceipt(token,current.source,current.profile),receipt(current.source));
  const plan={matrix:'built only after all complete source replays'},consuming={},borrowed={};cache.attach(plan,token);cache.inherit(plan,consuming);cache.inherit(consuming,borrowed);
  for(const [owner,source]of [[plan,current],[consuming,successors[0]],[borrowed,successors[1]]])assert.deepEqual(cache.read(owner,source.source,source.profile),receipt(source.source));
  const changed=cache.read(borrowed,successors[1].source,successors[1].profile);changed.sourceSha256='0'.repeat(64);
  assert.deepEqual(cache.read(borrowed,successors[1].source,successors[1].profile),receipt(successors[1].source));
});

test('failed or counterfeit complete replays never create an attachable token and preserve the original failure',()=>{
  const failure=new Error('actual complete archive failure');
  for(const failAt of [0,1,2]){
    const cache=createOriginalLinkSuccessorSourceArchives();let calls=0;
    assert.throws(()=>cache.prepare(current,successors,source=>{if(calls++===failAt)throw failure;return receipt(source);}),error=>error===failure);
    assert.equal(calls,failAt+1);assert.throws(()=>cache.attach({},{}));
  }
  const cache=createOriginalLinkSuccessorSourceArchives();
  for(const field of ['sourceSha256','textReplaySha256','binaryReplaySha256'])assert.throws(()=>cache.prepare(current,successors,source=>({...receipt(source),[field]:'0'.repeat(64)})));
  assert.throws(()=>cache.prepare(current,[...successors].reverse(),receipt));
});

test('unknown plans changed full profiles altered source bytes and caller-owned receipts cannot reuse archive evidence',()=>{
  const cache=createOriginalLinkSuccessorSourceArchives(),token=cache.prepare(current,successors,receipt),plan={};cache.attach(plan,token);
  assert.throws(()=>cache.read({},current.source,current.profile));assert.throws(()=>cache.inherit({},{}));
  assert.throws(()=>cache.currentReceipt({},current.source,current.profile));
  assert.throws(()=>cache.read(plan,current.source+' changed',current.profile));
  const changed=structuredClone(current.profile);changed.declarationNames.push('counterfeit owner');assert.throws(()=>cache.read(plan,current.source,changed));
  assert.throws(()=>cache.attach(plan,token));assert.throws(()=>cache.inherit(plan,plan));
});
