import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';
import {generateCiGuardRepairArchive} from '../../experiments/grammar-outcome-network/native-transport/inverse/generate-ci-guard-repair-archive.mjs';
import {restoreCiRepairSourceHistory} from '../scripts/ci-repair-source-history.mjs';

test('all current CI source views retain certified complete predecessors and refuse unknown source edits',()=>{
  generateCiGuardRepairArchive({check:true});
  let count=0;
  for(const name of ['check','guard']){
    const archive=readProjectData(new URL('../../parity/self-translation/ci-39394008799-'+name+'-sources-history.lino',import.meta.url));
    for(const row of archive.sources){
      const source=restoreSchedulingSourceHistory(row.file,readFileSync(new URL('../../'+row.file,import.meta.url),'utf8'));
      assert.equal(createHash('sha256').update(source).digest('hex'),row.current.sha256);
      assert.equal(Buffer.byteLength(source),row.current.bytes);
      assert.equal(restoreCiRepairSourceHistory(row.file,source),row.sourceText);
      assert.equal(restoreCiRepairSourceHistory(row.file,row.sourceText),row.sourceText);
      assert.throws(()=>restoreCiRepairSourceHistory(row.file,source+'\n# unknown edit\n'),assert.AssertionError);
      count++;
    }
  }
  assert.equal(count,13);
  assert.equal(restoreCiRepairSourceHistory('unregistered/source.js','unchanged bytes'),'unchanged bytes');
});

import {restoreSchedulingSourceHistory} from '../scripts/scheduling-source-history.mjs';
