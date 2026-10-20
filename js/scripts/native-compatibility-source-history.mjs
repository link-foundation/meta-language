// Restore certified predecessor inputs only for historical source controls.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {readProjectData} from './project-data.mjs';
import {verifyGitArtifactCertificate} from './git-object-certificate.mjs';
const archive=new URL('../../parity/self-translation/native-compatibility-source-sources-history.lino',import.meta.url);
const archiveHash='880282077c637f2fa9a30a8c06168e06a3dfebdaf03fd521880a3ebd09c24c2f';
const digest=value=>createHash('sha256').update(value).digest('hex');
let history;

/** An unregistered or unknown successor remains visible to the original guard. */
export function restoreNativeCompatibilitySourceHistory(file,source) {
  assert.equal(typeof file,'string');assert.equal(typeof source,'string');
  if(!history){assert.equal(digest(readFileSync(archive)),archiveHash);history=readProjectData(archive);}
  const row=history.sources.find(row=>row.file===file);if(!row)return source;
  const actual=digest(source);
  if(actual!==row.current.sha256&&actual!==row.sha256)return source;
  assert.equal(Buffer.byteLength(source),actual===row.current.sha256?row.current.bytes:row.bytes);
  const original=verifyGitArtifactCertificate(row.certificate,Buffer.from(row.sourceText),{revision:row.commit,path:file});
  assert.equal(original.length,row.bytes);assert.equal(digest(original),row.sha256);
  if(actual===row.sha256)assert.equal(source,row.sourceText);
  return row.sourceText;
}
