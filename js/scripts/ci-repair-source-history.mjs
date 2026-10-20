// Exact historical inputs for the retained original workflow source assertions.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {readProjectData} from './project-data.mjs';
import {verifyGitArtifactCertificate} from './git-object-certificate.mjs';
const archives=[
  {file:'parity/self-translation/ci-39394008799-check-sources-history.lino',sha256:'c6b3484e763df43b965ea52e9af2a1515687a2622124f116f471969c93767f04'},
  {file:'parity/self-translation/ci-39394008799-guard-sources-history.lino',sha256:'36943a5f143f40b69f797cd3ffa095b7fb6d1d13547bc4b97fb7950ad6300f85'},
];
const digest=value=>createHash('sha256').update(value).digest('hex');
let rows;

/** Unknown edits never inherit authenticated historical source membership. */
export function restoreCiRepairSourceHistory(file,source) {
  assert.equal(typeof file,'string');assert.equal(typeof source,'string');
  source=restoreEvidenceTranslationSourceHistory(file,source);
  if(!rows){
    rows=[];
    for(const archive of archives){
      const path=new URL('../../'+archive.file,import.meta.url),bytes=readFileSync(path);
      assert.equal(digest(bytes),archive.sha256);
      rows.push(...readProjectData(path).sources);
    }
    assert.equal(new Set(rows.map(row=>row.file)).size,rows.length);
  }
  const row=rows.find(row=>row.file===file);if(!row)return source;
  const actual=digest(source);
  assert.ok(actual===row.current.sha256||actual===row.sha256,'complete changed CI source requires a reviewed historical view');
  if(actual===row.current.sha256)assert.equal(Buffer.byteLength(source),row.current.bytes);
  const original=verifyGitArtifactCertificate(row.certificate,Buffer.from(row.sourceText),{revision:row.commit,path:file});
  assert.equal(original.length,row.bytes);assert.equal(digest(original),row.sha256);
  if(actual===row.sha256)assert.equal(source,row.sourceText);
  return row.sourceText;
}

import {restoreEvidenceTranslationSourceHistory} from './evidence-translation-source-history.mjs';
