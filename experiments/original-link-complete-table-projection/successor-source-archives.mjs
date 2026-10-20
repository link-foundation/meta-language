// Keep full source replay ahead of the retained observation/carrier matrix.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
const pin=source=>({bytes:Buffer.byteLength(source),sha256:createHash('sha256').update(source).digest('hex')});

/** Actual current lifecycle code has separate before/after source provenance. */
export function readOriginalLinkSuccessorArchiveSource() {
  return Object.freeze({file:'experiments/original-link-complete-table-projection/successor-source-archives.mjs',...pin(readFileSync(new URL(import.meta.url)))});
}

/** One private planner instance owns its prepared tokens and checked plans.
 * Only compact source fingerprints, whole profiles and replay hashes survive;
 * source graphs, encoded archives and typed programs never enter this cache. */
export function createOriginalLinkSuccessorSourceArchives() {
  const prepared=new WeakMap(),plans=new WeakMap();
  const replay=(source,profile,replaySource)=>{
    assert.equal(typeof source,'string');assert.equal(source.isWellFormed(),true);
    const actual=pin(source);assert.deepEqual(actual,{bytes:profile.sourceBytes,sha256:profile.sourceSha256});
    const receipt=replaySource(source);
    assert.deepEqual(Object.keys(receipt).sort(),['binaryReplaySha256','sourceSha256','textReplaySha256']);
    for(const value of Object.values(receipt))assert.equal(value,actual.sha256,'every complete actual archive replay retains the whole source');
    return{source:actual,profile:structuredClone(profile),receipt:structuredClone(receipt)};
  };
  const select=(rows,source,profile)=>{
    assert.ok(rows,'an exact prepared token or attached checked plan is required');
    const matches=rows.filter(row=>row.profile.profile===profile.profile);assert.equal(matches.length,1,'one exact source per emission profile');
    const row=matches[0];assert.deepEqual(profile,row.profile,'the complete selected emission profile remains exact');
    assert.deepEqual(pin(source),row.source,'the complete current source retains its prepared fingerprint');
    return structuredClone(row.receipt);
  };
  return Object.freeze({
    prepare(current,successors,replaySource){
      assert.equal(typeof replaySource,'function');assert.ok(Array.isArray(successors));
      assert.equal(current.profile.profile,'current');
      assert.deepEqual(successors.map(row=>row.profile.profile),['consuming','borrowed consuming']);
      const rows=[current,...successors].map(row=>replay(row.source,row.profile,replaySource));
      const token=Object.freeze({});prepared.set(token,rows);return token;
    },
    currentReceipt(token,source,profile){return select(prepared.get(token),source,profile);},
    attach(plan,token){assert.ok(prepared.has(token));assert.ok(plan&&typeof plan==='object');assert.equal(plans.has(plan),false);plans.set(plan,prepared.get(token));},
    inherit(parent,successor){assert.ok(plans.has(parent));assert.ok(successor&&typeof successor==='object');assert.equal(plans.has(successor),false);plans.set(successor,plans.get(parent));},
    read(plan,source,profile){return select(plans.get(plan),source,profile);},
  });
}
