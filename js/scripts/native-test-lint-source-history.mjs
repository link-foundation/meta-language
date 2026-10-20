// Historical checks restore only a complete authenticated predecessor view.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readProjectData} from './project-data.mjs';
import {verifyGitArtifactCertificate} from './git-object-certificate.mjs';

/** Recheck complete current bytes before restoring the recorded original interval. */
export function restoreNativeTestLintSource(file,source) {
  const history=readProjectData(new URL('../../parity/self-translation/native-test-lint-corrections.lino',import.meta.url));
  const selected=history.sources.filter(row=>row.file===file);assert.equal(selected.length,1,'one authenticated native test lint source');
  const row=selected[0];verifyGitArtifactCertificate(row.certificate,Buffer.from(row.sourceText),{revision:row.commit,path:file});
  const pin=value=>({bytes:Buffer.byteLength(value),sha256:createHash('sha256').update(value).digest('hex')});
  assert.deepEqual(pin(source),row.current,'the complete actual current module must match before historical restoration');
  assert.equal(row.edit.offsetUnit,'UTF-16 code units');
  assert.equal(source.slice(row.edit.start,row.edit.start+row.edit.inserted.length),row.edit.inserted);
  const restored=source.slice(0,row.edit.start)+row.edit.removed+source.slice(row.edit.start+row.edit.inserted.length);
  assert.equal(restored,row.sourceText);assert.deepEqual(pin(restored),{bytes:row.bytes,sha256:row.sha256});
  return restored;
}
