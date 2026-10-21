import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {readProjectData} from '../scripts/project-data.mjs';
import test from 'node:test';
import {generateContinuationHistoricalInputs,restoreContinuationHistoricalAuthorInput} from '../scripts/generate-continuation-historical-inputs.mjs';
import {readRawConsumingSnapshotSourceProfile} from '../scripts/raw-consuming-snapshot-source-profile.mjs';
import {readCurrentOwnCodebaseRawDescriptorPacket} from '../scripts/current-own-codebase-raw-source-descriptors.mjs';
import {restoreContinuationHistoricalSource} from '../scripts/continuation-source-history.mjs';
import {verifyGitArtifactCertificate} from '../scripts/git-object-certificate.mjs';

const root=new URL('../../',import.meta.url);
const pin=value=>({bytes:Buffer.byteLength(value),sha256:createHash('sha256').update(value).digest('hex')});

test('current repair artifacts and complete certified historical inputs have independently checked identities',()=>{
  const history=generateContinuationHistoricalInputs({check:true});
  assert.equal(history.sources.length,10);
  for(const row of history.sources){
    const actual=readFileSync(new URL(row.file,root),'utf8');
    const historical=restoreContinuationHistoricalAuthorInput(actual,row.file);
    assert.deepEqual(pin(historical),row.current,'the entire historical successor input remains exact; current bytes have independent compatibility controls');
    assert.deepEqual(pin(row.sourceText),{bytes:row.bytes,sha256:row.sha256});
    verifyGitArtifactCertificate(row.certificate,Buffer.from(row.sourceText),{revision:row.commit,path:row.file});
    for(const profile of row.profiles){
      assert.equal(restoreContinuationHistoricalSource(actual,row.file,profile),row.sourceText);
      assert.equal(restoreContinuationHistoricalSource(row.sourceText,row.file,profile),row.sourceText);
      assert.equal(restoreContinuationHistoricalSource(actual+'\n# unreviewed\n',row.file,profile),actual+'\n# unreviewed\n',
        'unknown current edits cannot acquire an authenticated predecessor');
    }
    assert.equal(restoreContinuationHistoricalSource(actual,row.file,'unregistered profile'),actual);
    assert.throws(()=>verifyGitArtifactCertificate(row.certificate,Buffer.from(row.sourceText+'\n'),{revision:row.commit,path:row.file}));
  }
});


test('the historical author retains both complete current input authorities without adding new migration rows',()=>{
  const files=['js/src/translation/check.js','parity/self-translation/complete-declaration-receipts.lino'];
  const archive='parity/self-translation/ci-39389240046-historical-inputs.lino',protectedFiles=[...files,archive];
  const before=protectedFiles.map(file=>readFileSync(new URL(file,root),'utf8'));
  const history=readProjectData(new URL('parity/self-translation/array-relative-element-history.lino',root));
  const snapshot=readRawConsumingSnapshotSourceProfile(),raw=readCurrentOwnCodebaseRawDescriptorPacket();
  for(const[index,file]of files.entries()){
    const current=before[index],row=history.sources.find(row=>row.file===file);assert.ok(row);
    if(index===0){const owner=snapshot.sources.find(row=>row.file===file);assert.ok(owner);assert.equal(current,owner.source);assert.deepEqual(pin(current),{bytes:owner.bytes,sha256:owner.sha256});}
    else assert.deepEqual(pin(current),raw.register.current,'the physical register retains its exact independent current descriptor owner');
    const previous=restoreContinuationHistoricalAuthorInput(current,file);
    assert.notEqual(previous,current,'the actual current input and selected historical source keep distinct identities');
    assert.deepEqual(pin(previous),row.current,'the complete original array-history input remains independently pinned');
    assert.equal(restoreContinuationHistoricalAuthorInput(previous,file),previous);
    assert.equal(restoreContinuationHistoricalAuthorInput(current,'unregistered.mjs'),current);
    if(index===0)assert.equal(restoreContinuationHistoricalAuthorInput(current+'\n',file),current+'\n');
    else assert.throws(()=>restoreContinuationHistoricalAuthorInput(current+'\n',file),assert.AssertionError);
  }
  assert.deepEqual(pin(before[2]),{bytes:10759631,sha256:'05e17bf81c6304f36411073ee4853798041e2764164c999ca93888870fb61560'});
  protectedFiles.forEach((file,index)=>assert.equal(readFileSync(new URL(file,root),'utf8'),before[index],file));
});
