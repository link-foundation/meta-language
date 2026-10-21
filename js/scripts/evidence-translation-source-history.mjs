// Restore certified predecessor inputs only for historical source controls.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {readProjectData} from './project-data.mjs';
import {verifyGitArtifactCertificate} from './git-object-certificate.mjs';
import {restoreNativeCompatibilitySourceHistory} from './native-compatibility-source-history.mjs';
const archive=new URL('../../parity/self-translation/evidence-translation-source-fixtures-history.lino',import.meta.url);
const archiveHash='c8d831147e7945fc5e7579e92cdce283418122419ba88a837138af9c2f7a9b7d';
const codeArchive=new URL('../../parity/self-translation/evidence-translation-source-sources-history.lino',import.meta.url);
const codeArchiveHash='9619d475c1e2f191b8489e8d95922ead1f1c2d1c92487ab23a1dc383c17db1ab';
const digest=value=>createHash('sha256').update(value).digest('hex');
let history;

/** An unregistered or unknown successor remains visible to the original guard. */
export function restoreEvidenceTranslationSourceHistory(file,source) {
  assert.equal(typeof file,'string');assert.equal(typeof source,'string');
  source=restoreNativeCompatibilitySourceHistory(file,source);
  if(!history){
    assert.equal(digest(readFileSync(archive)),archiveHash);assert.equal(digest(readFileSync(codeArchive)),codeArchiveHash);
    history={sources:[...readProjectData(archive).sources,...readProjectData(codeArchive).sources]};
    assert.equal(new Set(history.sources.map(row=>row.file)).size,history.sources.length);
  }
  const row=history.sources.find(row=>row.file===file);if(!row)return source;
  const actual=digest(source);
  if(actual!==row.current.sha256&&actual!==row.sha256)return source;
  assert.equal(Buffer.byteLength(source),actual===row.current.sha256?row.current.bytes:row.bytes);
  const original=verifyGitArtifactCertificate(row.certificate,Buffer.from(row.sourceText),{revision:row.commit,path:file});
  assert.equal(original.length,row.bytes);assert.equal(digest(original),row.sha256);
  if(actual===row.sha256)assert.equal(source,row.sourceText);
  return row.sourceText;
}
