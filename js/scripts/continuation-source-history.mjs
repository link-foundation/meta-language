// Historical checks authenticate a complete successor before restoring its input.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {readProjectData} from './project-data.mjs';
import {restoreNativeCompatibilitySourceHistory} from './native-compatibility-source-history.mjs';
import {verifyGitArtifactCertificate} from './git-object-certificate.mjs';
import {restoreCurrentCiRepairSource} from './current-ci-repair-source-history.mjs';
import {restoreRawDescriptorHistoricalSource} from './raw-descriptor-historical-cohort.mjs';

const historyFile=new URL('../../parity/self-translation/ci-39389240046-historical-inputs.lino',import.meta.url);
const historyHash='05e17bf81c6304f36411073ee4853798041e2764164c999ca93888870fb61560';
const pin=value=>({bytes:Buffer.byteLength(value),sha256:createHash('sha256').update(value).digest('hex')});
let history;

/** Restore only an exact reviewed successor; existing historical gates reject others. */
export function restoreContinuationHistoricalSource(source,file,profile) {
  assert.equal(typeof source,'string');
  if(profile==='relative array')source=restoreRawDescriptorHistoricalSource(source,file,'continuation historical input');
  if(!history){
    const bytes=readFileSync(historyFile);assert.equal(pin(bytes).sha256,historyHash);
    history=readProjectData(historyFile);assert.equal(history.schemaVersion,1);
  }
  const rows=history.sources.filter(row=>row.file===file&&row.profiles.includes(profile));
  assert.ok(rows.length<=1,'one complete historical input per profile');
  if(rows.length===0)return ['relative array','original number source ranges'].includes(profile)
    ?restoreCurrentCiRepairSource(source,file):source;
  source=restoreCurrentCiRepairSource(source,file);
  source=restoreNativeCompatibilitySourceHistory(file,source);
  const row=rows[0],actual=pin(source);
  if(actual.sha256!==row.current.sha256)return source;
  assert.deepEqual(actual,row.current);
  verifyGitArtifactCertificate(row.certificate,Buffer.from(row.sourceText),{revision:row.commit,path:file});
  assert.equal(row.edit.offsetUnit,'UTF-8 bytes');
  const bytes=Buffer.from(source),edit=row.edit;
  assert.equal(bytes.subarray(edit.start,edit.end).toString('utf8'),edit.inserted);
  const restored=Buffer.concat([bytes.subarray(0,edit.start),Buffer.from(edit.removed),bytes.subarray(edit.end)]);
  assert.deepEqual(pin(restored),{bytes:row.bytes,sha256:row.sha256});
  assert.equal(restored.toString('utf8'),row.sourceText);
  return row.sourceText;
}
