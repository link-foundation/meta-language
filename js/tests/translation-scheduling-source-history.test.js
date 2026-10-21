import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';
import {restoreSchedulingSourceHistory} from '../scripts/scheduling-source-history.mjs';
import {generateSchedulingRepairArchive} from '../../experiments/grammar-outcome-network/native-transport/inverse/generate-scheduling-repair-archive.mjs';

test('current scheduling repairs authenticate every actual source and preserve complete certified original guard inputs',()=>{
  generateSchedulingRepairArchive({check:true});
  const history=readProjectData(new URL('../../parity/self-translation/ci-39403403220-scheduling-sources-history.lino',import.meta.url));
  assert.equal(history.sources.length,6);
  for(const row of history.sources){
    const current=restoreNativeGraphSourceHistory(row.file,readFileSync(new URL('../../'+row.file,import.meta.url),'utf8'));
    assert.equal(Buffer.byteLength(current),row.current.bytes);
    assert.equal(createHash('sha256').update(current).digest('hex'),row.current.sha256);
    assert.equal(restoreSchedulingSourceHistory(row.file,current),row.sourceText);
    assert.equal(restoreSchedulingSourceHistory(row.file,row.sourceText),row.sourceText);
    assert.throws(()=>restoreSchedulingSourceHistory(row.file,current+'\n# unknown edit\n'),assert.AssertionError);
  }
  assert.equal(restoreSchedulingSourceHistory('unknown/source.js','visible unknown source'),'visible unknown source');
});

import {restoreNativeGraphSourceHistory} from '../scripts/native-graph-source-history.mjs';
