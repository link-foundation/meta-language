import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {readProjectData} from '../../js/scripts/project-data.mjs';
import {verifyGitArtifactCertificate} from '../../js/scripts/git-object-certificate.mjs';
const file=new URL('../../parity/self-translation/program-string-overlay-provider-revision.lino',import.meta.url);
const packetSha256='b7337ffb703cbbc834245c9c761ad3d1475ef7a7998c80f5257a85602c0ce196';
const hash=value=>createHash('sha256').update(value).digest('hex');

/** Admit the complete source profile only after all current and old Git proofs. */
export function readProgramStringOverlayProviderRevision(previous) {
  assert.equal(hash(readFileSync(file)),packetSha256);const packet=readProjectData(file);
  assert.equal(packet.previousSourceCommit,previous.sourceCommit);
  assert.deepEqual(packet.providers.map(row=>row.module),previous.providers.map(row=>row.module));
  assert.equal(packet.certificates.length,5);assert.equal(packet.inverse.length,2);
  for(const [index,row]of packet.certificates.entries()){
    assert.equal(row.module,packet.providers[index].module);
    for(const side of ['current','previous']){
      const source=row[side];verifyGitArtifactCertificate(source.certificate,Buffer.from(source.source),{revision:source.revision,path:'js/src/'+row.module});
      assert.equal(source.revision,side==='current'?packet.sourceCommit:packet.previousSourceCommit);
    }
    assert.equal(hash(row.current.source),packet.providers[index].sourceSha256);
    assert.equal(hash(row.previous.source),previous.providers[index].sourceSha256);
    const actions=packet.inverse.filter(action=>action.module===row.module);
    assert.equal(actions.length,row.current.source===row.previous.source?0:1);
    if(actions.length)assert.deepEqual(actions[0],{module:row.module,from:row.current.source,to:row.previous.source});
  }
  return packet;
}
