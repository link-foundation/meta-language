import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync,readdirSync} from 'node:fs';
import test from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';
import {readRustSourceItems} from '../scripts/generate-private-output-test-model.mjs';

test('native workflow queue expectation correction retains every original safety assertion and actual current header',()=>{
  const history=readProjectData(new URL('../../parity/naming/native-workflow-queue-source-history.lino',import.meta.url));
  assert.equal(history.sources.length,1);const before=history.sources[0];
  for(const row of [before,history.after]){
    assert.equal(Buffer.byteLength(row.sourceText),row.bytes);
    assert.equal(createHash('sha256').update(row.sourceText).digest('hex'),row.sha256);
  }
  assert.equal(history.capture.job,118178406222);
  assert.equal(Buffer.byteLength(history.capture.log.source),171616);
  assert.equal(createHash('sha256').update(history.capture.log.source).digest('hex'),
    'd9730bde03dbcf116c2de7566f0c7d083dac067000a2ba43f45cabb4678f59ac');
  assert.ok(history.capture.log.source.includes('workflows_separate_cancellable_checks_from_serialized_writes'));
  const current=readFileSync(new URL('../../'+before.file,import.meta.url),'utf8');
  assert.equal(current,history.after.sourceText);
  assert.equal(current.split(history.insertion).length,2);
  assert.equal(current.replace(history.insertion,'').replace('&& {pull_request_identity} || github.run_id','&& github.ref || github.run_id'),before.sourceText);
  assert.ok(history.insertion.includes('"ci.yml" | "js.yml" | "rust.yml"'));
  assert.ok(history.insertion.includes('"format(\'pull-{0}\', github.event.pull_request.number)"'));
  assert.ok(history.insertion.includes('_ => "github.ref"'));
  const parsed=readRustSourceItems(Buffer.from(current));
  assert.equal(parsed.items.filter(item=>item.kind==='function_item'&&item.name==='workflows_separate_cancellable_checks_from_serialized_writes').length,1);
  const names=readdirSync(new URL('../../.github/workflows/',import.meta.url)).filter(name=>/\.ya?ml$/u.test(name)).sort();
  assert.deepEqual(names,['ci.yml','dependency-refresh.yml','js.yml','rust.yml']);
  for(const name of names){
    const source=readFileSync(new URL('../../.github/workflows/'+name,import.meta.url),'utf8');
    const header=source.split('\njobs:\n')[0];
    const prefix=header.includes('\n  workflow_call:')?name.slice(0,-4)+'-':'${{ github.workflow }}-';
    const identity=name==='dependency-refresh.yml'?'github.ref':"format('pull-{0}', github.event.pull_request.number)";
    const expected='\nconcurrency:\n  group: '+prefix+"${{ github.event_name == 'pull_request' && "+identity+' || github.run_id }}\n  cancel-in-progress: true\n';
    assert.equal(header.split('\nconcurrency:\n').length,2);
    assert.ok(header.includes(expected),name);
    assert.ok(header.includes('permissions:\n  contents: read'));
  }
});
