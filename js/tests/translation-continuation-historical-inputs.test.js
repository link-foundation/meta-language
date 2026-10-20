import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {generateContinuationHistoricalInputs} from '../scripts/generate-continuation-historical-inputs.mjs';
import {restoreContinuationHistoricalSource} from '../scripts/continuation-source-history.mjs';
import {verifyGitArtifactCertificate} from '../scripts/git-object-certificate.mjs';

const root=new URL('../../',import.meta.url);
const pin=value=>({bytes:Buffer.byteLength(value),sha256:createHash('sha256').update(value).digest('hex')});

test('current repair artifacts and complete certified historical inputs have independently checked identities',()=>{
  const history=generateContinuationHistoricalInputs({check:true});
  assert.equal(history.sources.length,10);
  for(const row of history.sources){
    const actual=readFileSync(new URL(row.file,root),'utf8');
    assert.deepEqual(pin(actual),row.current,'the entire current artifact is checked independently');
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
