// Historical checks receive exact predecessor bytes; current authors stay current.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {readBootstrapStructuredDataLino} from '../../scripts/lib/structured-data-bootstrap.mjs';
import {verifyGitArtifactCertificate} from './git-object-certificate.mjs';
const archive=new URL('../../parity/self-translation/grammar-context-source-metadata.lino',import.meta.url);
const archiveSha256='ddbe4d6f7ba236f8b38c83b178c2022c64270e337c535ace2cf45b321f1a9fc6';
const revision='abde6ee604b914a895f23525946fab2234109e86';
const pin=value=>({bytes:Buffer.byteLength(value),sha256:createHash('sha256').update(value).digest('hex')});
let history;

/** Unknown or edited current metadata never acquires historical authentication. */
export function restoreGrammarContextMetadataHistory(source,file) {
  assert.equal(typeof source,'string');assert.equal(typeof file,'string');
  if(!history){const bytes=readFileSync(archive);assert.equal(pin(bytes).sha256,archiveSha256);history=readBootstrapStructuredDataLino(bytes.toString());assert.equal(history.revision,revision);}
  const row=history.sources.find(row=>row.file===file);if(!row||pin(source).sha256!==row.current.sha256)return source;
  assert.deepEqual(pin(source),row.current);const bytes=Buffer.from(source),edit=row.edit;
  assert.equal(edit.offsetUnit,'UTF-8 bytes');assert.ok(Number.isSafeInteger(edit.start)&&Number.isSafeInteger(edit.end)&&edit.start>=0&&edit.end>=edit.start&&edit.end<=bytes.length);
  assert.equal(bytes.subarray(edit.start,edit.end).toString(),edit.inserted);
  const inverse=Buffer.concat([bytes.subarray(0,edit.start),Buffer.from(edit.removed),bytes.subarray(edit.end)]);
  assert.deepEqual(pin(inverse),{bytes:row.bytes,sha256:row.sha256});assert.equal(inverse.toString(),row.sourceText);
  verifyGitArtifactCertificate(row.certificate,inverse,{revision,path:file});return row.sourceText;
}
