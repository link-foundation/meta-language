// Complete current-source succession for one immutable historical descriptor.
// This is a source-only view; current parsing always uses the actual frontend.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {readBootstrapStructuredDataLino} from '../../scripts/lib/structured-data-bootstrap.mjs';
import {verifyGitArtifactCertificate} from './git-object-certificate.mjs';
import {replayRequiredFieldNativeHostIntervals} from './required-field-native-host-succession.mjs';

const root=new URL('../../',import.meta.url);
export const RELATIVE_ARRAY_CURRENT_FRONTEND='js/src/translation/javascript.js';
export const RELATIVE_ARRAY_CURRENT_FRONTEND_OUTPUT='parity/self-translation/relative-array-current-frontend-history.lino';
export const RELATIVE_ARRAY_CURRENT_FRONTEND_ANCHORS=Object.freeze([
  Object.freeze({revision:'a8a789afb0ed569340d9079279f534f0bf7c4e72',bytes:133770,sha256:'1ee38749dcf8723b27489dbd33f6e3f177a9e52c898a85cf215aecdcbdc31211'}),
  Object.freeze({revision:'8d77e476d097448eb381ebe03e2aa173402bc03d',bytes:136201,sha256:'face5521fb69545cd5c06ff2eebed38878ccba559e69d1454cb7cc352efc43ed'}),
]);
export const RELATIVE_ARRAY_CURRENT_FRONTEND_NAMING=Object.freeze({
  path:'parity/naming/ci-39526911132-record-parameter-predecessor-source-history.lino',
  bytes:9382656,sha256:'9ecc79376cd2a778a2f07605ac394b5530439a2eb3cc2cba0ccb9bde2161e4f9',
});
const legacy={bytes:130416,sha256:'38d155ce765157039d2555c070e537e97a91f2c64587c70b6c6c578c5371ebe8'};
const packetSha256='3cb2fa1f2e2ca77a1f587ea68d7624b870b78e160af3005c9a40415fbc1853d5';
let cached;
export const pinRelativeArrayCurrentFrontend=value=>({bytes:Buffer.byteLength(value),sha256:createHash('sha256').update(value).digest('hex')});
const exactKeys=(value,keys)=>{assert.ok(value&&typeof value==='object'&&!Array.isArray(value));assert.deepEqual(Object.keys(value).sort(),[...keys].sort());};

/** Reuse the established exact interval replay/inverse. No source semantics are inferred. */
export function deriveRelativeArrayFrontendIntervals(previous,current){
  assert.equal(Buffer.from(previous).toString('utf8'),previous);
  assert.equal(Buffer.from(current).toString('utf8'),current);
  const before=[...previous],after=[...current];let start=0,oldEnd=before.length,end=after.length;
  while(start<oldEnd&&start<end&&before[start]===after[start])start++;
  while(oldEnd>start&&end>start&&before[oldEnd-1]===after[end-1]){oldEnd--;end--;}
  const edits=[{start:before.slice(0,start).join('').length,removed:before.slice(start,oldEnd).join(''),inserted:after.slice(start,end).join('')}];
  assert.equal(replayRequiredFieldNativeHostIntervals(previous,edits),current);
  return edits;
}

/** Current membership is the actual complete local file; original membership
 * is independently certified at both immutable commits. No current Git claim. */
export function verifyRelativeArrayCurrentFrontendHistory(packet){
  exactKeys(packet,['schemaVersion','model','profile','file','sourceArchive','anchors','current','edges','boundary']);
  assert.equal(packet.schemaVersion,1);assert.equal(packet.model,'relative array complete current frontend succession 1');
  assert.equal(packet.profile,'relative array');assert.equal(packet.file,RELATIVE_ARRAY_CURRENT_FRONTEND);
  assert.deepEqual(packet.sourceArchive,RELATIVE_ARRAY_CURRENT_FRONTEND_NAMING);
  assert.ok(Array.isArray(packet.anchors));assert.equal(packet.anchors.length,2);
  for(const[index,row]of packet.anchors.entries()){
    exactKeys(row,['revision','bytes','sha256','source','certificate']);
    const expected=RELATIVE_ARRAY_CURRENT_FRONTEND_ANCHORS[index];
    assert.deepEqual({revision:row.revision,bytes:row.bytes,sha256:row.sha256},expected);
    assert.deepEqual(pinRelativeArrayCurrentFrontend(row.source),{bytes:expected.bytes,sha256:expected.sha256});
    verifyGitArtifactCertificate(row.certificate,Buffer.from(row.source),{revision:expected.revision,path:packet.file});
  }
  exactKeys(packet.current,['bytes','sha256','source']);
  assert.equal(typeof packet.current.source,'string');
  assert.deepEqual(pinRelativeArrayCurrentFrontend(packet.current.source),{bytes:packet.current.bytes,sha256:packet.current.sha256});
  assert.equal(packet.current.source,readFileSync(new URL(packet.file,root),'utf8'),'only the actual complete current frontend has this source view');
  assert.ok(Array.isArray(packet.edges));assert.equal(packet.edges.length,2);
  const sources=[packet.anchors[0].source,packet.anchors[1].source,packet.current.source];
  for(const[index,edge]of packet.edges.entries()){
    exactKeys(edge,['from','to','coordinateUnit','edits']);
    assert.deepEqual(edge.from,pinRelativeArrayCurrentFrontend(sources[index]));
    assert.deepEqual(edge.to,pinRelativeArrayCurrentFrontend(sources[index+1]));
    assert.equal(edge.coordinateUnit,'UTF-16 code units');
    assert.deepEqual(edge.edits,deriveRelativeArrayFrontendIntervals(sources[index],sources[index+1]));
    assert.equal(replayRequiredFieldNativeHostIntervals(sources[index],edge.edits),sources[index+1]);
    let inverse=sources[index+1];
    for(const edit of edge.edits.toReversed()){
      assert.equal(inverse.slice(edit.start,edit.start+edit.inserted.length),edit.inserted);
      inverse=inverse.slice(0,edit.start)+edit.removed+inverse.slice(edit.start+edit.inserted.length);
    }
    assert.equal(inverse,sources[index],'every original complete source byte remains');
  }
  return packet.anchors[0].source;
}

export function readRelativeArrayCurrentFrontendHistory(){
  if(!cached){
    const bytes=readFileSync(new URL(RELATIVE_ARRAY_CURRENT_FRONTEND_OUTPUT,root));
    assert.equal(pinRelativeArrayCurrentFrontend(bytes).sha256,packetSha256);
    cached=readBootstrapStructuredDataLino(bytes.toString('utf8'));
    verifyRelativeArrayCurrentFrontendHistory(cached);
  }
  return structuredClone(cached);
}

/** The older certified897 source is already historical and remains an identity.
 * All unrelated profiles/paths are identities. Every unknown selected successor refuses. */
export function restoreRelativeArrayCurrentFrontendSource(source,file,profile){
  assert.equal(typeof source,'string');assert.equal(typeof file,'string');assert.equal(typeof profile,'string');
  if(profile!=='relative array'||file!==RELATIVE_ARRAY_CURRENT_FRONTEND)return source;
  const actual=pinRelativeArrayCurrentFrontend(source);
  if(actual.bytes===legacy.bytes&&actual.sha256===legacy.sha256)return source;
  const packet=readRelativeArrayCurrentFrontendHistory();
  for(const row of [...packet.anchors,packet.current]){
    if(source===row.source)return packet.anchors[0].source;
  }
  assert.fail('an unknown complete frontend successor has no relative-array current-source authority');
}
