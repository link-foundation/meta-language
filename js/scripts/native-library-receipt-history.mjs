// Generated from the exact library-relocation receipt history. Do not edit.
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {readProjectData,serializeProjectData} from './project-data.mjs';
import {verifyGitArtifactCertificate} from './git-object-certificate.mjs';

const root=new URL('../../',import.meta.url),historyFile='parity/self-translation/library-relocation-receipt-history.lino';
const policySha256='a467a8ba8fcfeba0103e8280192d3e76f9947fde8765ae605bce6f47e8fb9e21';
const pin=source=>({bytes:Buffer.byteLength(source),sha256:createHash('sha256').update(source).digest('hex')});
let policy;
function readNativeLibraryHistory() {
  if(!policy){
    const bytes=readFileSync(new URL(historyFile,root));assert.equal(pin(bytes).sha256,policySha256,'the complete reviewed native packaging edge remains exact');
    policy=readProjectData(new URL(historyFile,root));assert.equal(policy.schemaVersion,1);assert.equal(policy.sources.length,2);
  }
  return policy;
}

/** Report the actual current artifact descriptor separately from its predecessor. */
export function readNativeLibraryReceiptProfile(identity) {
  const row=readNativeLibraryHistory().sources.find(row=>row.identity===identity);assert.ok(row,'one exact native packaging identity');
  return{file:row.file,...row.current};
}

/** Restore a complete reviewed source adapter before its older source checks. */
export function restoreNativeLibrarySource(source,file) {
  if(!['js/scripts/complete-declaration-receipts.mjs','js/scripts/current-translation-source-descriptors.mjs'].includes(file))return source;
  const row=readNativeLibraryHistory().sourceAdapters.find(row=>row.file===file);assert.ok(row);
  if(pin(source).sha256!==row.current.sha256)return source;
  assert.equal(Buffer.byteLength(source),row.current.bytes);
  verifyGitArtifactCertificate(row.certificate,Buffer.from(row.sourceText),{revision:row.commit,path:file});
  assert.equal(row.edit.offsetUnit,'UTF-16 code units');
  assert.equal(source.slice(row.edit.start,row.edit.start+row.edit.inserted.length),row.edit.inserted);
  const restored=source.slice(0,row.edit.start)+row.edit.removed+source.slice(row.edit.start+row.edit.inserted.length);
  assert.equal(restored,row.sourceText);assert.deepEqual(pin(restored),{bytes:row.bytes,sha256:row.sha256});return restored;
}

/** Invert only reviewed packaging descriptors before the existing complete gate. */
export function restoreNativeLibraryReceipt(artifact,identity) {
  if(!['collection construction','raw consuming'].includes(identity))return artifact;
  const history=readNativeLibraryHistory(),row=history.sources.find(row=>row.identity===identity);assert.ok(row);
  const bytes=Buffer.from(serializeProjectData(artifact,row.file));
  if(pin(bytes).sha256!==row.current.sha256)return artifact;
  assert.equal(bytes.length,row.current.bytes);
  for(const module of history.modules){
    verifyGitArtifactCertificate(module.certificate,Buffer.from(module.sourceText),{revision:module.commit,path:module.file});
    assert.deepEqual(pin(readFileSync(new URL(module.file,root))),module.current,'the actual corrected native module remains exact');
  }
  verifyGitArtifactCertificate(row.certificate,Buffer.from(row.sourceText),{revision:row.commit,path:row.file});
  const restored=structuredClone(artifact);
  for(const change of row.changes){
    assert.ok(change.path.every(key=>typeof key==='string'&&!['__proto__','constructor','prototype'].includes(key)));
    assert.ok(change.path[0]==='library'&&change.path[1]==='files'&&change.path.length===4
      ||change.path.slice(0,4).join('.')==='predecessor.proof.producerApplicability.sources'&&change.path.length===6);
    assert.ok(['bytes','sourceSha256','sha256'].includes(change.path.at(-1)));
    let parent=restored;for(const key of change.path.slice(0,-1)){assert.ok(Object.hasOwn(parent,key));parent=parent[key];}
    const key=change.path.at(-1);assert.deepEqual(parent[key],change.after);parent[key]=change.before;
  }
  const original=Buffer.from(serializeProjectData(restored,row.file));
  assert.deepEqual(pin(original),{bytes:row.bytes,sha256:row.sha256});
  assert.equal(original.toString('utf8'),row.sourceText,'every complete predecessor receipt byte is restored');
  return restored;
}
