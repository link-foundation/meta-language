// Current scheduling successors supply only their exact certified old inputs.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {readProjectData} from './project-data.mjs';
import {verifyGitArtifactCertificate} from './git-object-certificate.mjs';
const archive=new URL('../../parity/self-translation/ci-39403403220-scheduling-sources-history.lino',import.meta.url);
const archiveHash='f346442b7cc9fb187c434c901c429955c91ac8c7cbc3dd299159a2068334bacc';
const digest=value=>createHash('sha256').update(value).digest('hex');
let history;

export function restoreSchedulingSourceHistory(file,source) {
  assert.equal(typeof file,'string');assert.equal(typeof source,'string');
  if(!history){assert.equal(digest(readFileSync(archive)),archiveHash);history=readProjectData(archive);}
  const row=history.sources.find(row=>row.file===file);if(!row)return source;
  const actual=digest(source);
  assert.ok(actual===row.current.sha256||actual===row.sha256,'changed scheduling source requires a reviewed historical input');
  assert.equal(Buffer.byteLength(source),actual===row.current.sha256?row.current.bytes:row.bytes);
  const original=verifyGitArtifactCertificate(row.certificate,Buffer.from(row.sourceText),{revision:row.commit,path:file});
  assert.equal(original.length,row.bytes);assert.equal(digest(original),row.sha256);
  if(actual===row.sha256)assert.equal(source,row.sourceText);
  return row.sourceText;
}
