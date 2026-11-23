import assert from 'node:assert/strict';
import {spawnSync} from 'node:child_process';
import {mkdtempSync,readFileSync,rmSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {tmpdir} from 'node:os';
import {join} from 'node:path';
import test from 'node:test';
import {parseWorkflow} from '../../scripts/check-cache-policy.mjs';
import {javascriptSuiteTestGroups,javascriptSuitePartition,javascriptSuiteCommandArguments,mergeJavaScriptSuitePartitions} from '../scripts/issue-195-javascript-suite-partitions.mjs';
import {stageFiles} from '../scripts/issue-195-evidence-stages.mjs';
import {readProjectData} from '../scripts/project-data.mjs';
import {verifyGitArtifactCertificate} from '../scripts/git-object-certificate.mjs';
import {restoreEvidenceTranslationSourceHistory} from '../scripts/evidence-translation-source-history.mjs';
import {generateEvidenceTranslationHistory} from '../../experiments/grammar-outcome-network/native-transport/inverse/generate-evidence-translation-history.mjs';
import {restoreNativeCompatibilitySourceHistory} from '../scripts/native-compatibility-source-history.mjs';
import {restoreRustJobAdmissionSource} from '../scripts/current-ci-repair-source-history.mjs';
const root=new URL('../../',import.meta.url);
const pin=value=>({bytes:Buffer.byteLength(value),sha256:createHash('sha256').update(value).digest('hex')});

function evidenceTranslationPartitionRecords() {
  const groups=javascriptSuiteTestGroups(),commit='a'.repeat(40),checkpoint='pre-merge',records=new Map();
  for(const group of Object.keys(groups))for(let index=1;index<=(group==='translation'?6:1);index++){
    const selection=javascriptSuitePartition(group,index+'/'+(group==='translation'?6:1),groups);
    records.set(selection.name,{schemaVersion:1,issue:195,stage:selection.name,commit,checkpoint,javascriptPartition:selection,
      toolchainVersions:{node:'v24.22.0'},groups:{'suite:javascript':{
        commands:[['node',...javascriptSuiteCommandArguments(group,selection.shard)].join(' ')],
        artifacts:['logs/'+selection.name+'.log'],failureLogs:[]}},error:null});
  }
  return{groups,commit,checkpoint,records};
}

test('six actual evidence translation scopes preserve every original workflow byte outside their matrix block',()=>{
  const [sources,fixtures]=generateEvidenceTranslationHistory({check:true});
  assert.equal(sources.sources.length,8);assert.equal(fixtures.sources.length,1);
  for(const row of [...sources.sources,...fixtures.sources]){
    assert.deepEqual(pin(restoreNativeCompatibilitySourceHistory(row.file,readFileSync(new URL(row.file,root),'utf8'))),row.current);
    verifyGitArtifactCertificate(row.certificate,Buffer.from(row.sourceText),{revision:row.commit,path:row.file});
    const current=readFileSync(new URL(row.file,root),'utf8');
    assert.equal(restoreEvidenceTranslationSourceHistory(row.file,current),row.sourceText);
    const unknown=current+'\n# unknown current source\n';assert.equal(restoreEvidenceTranslationSourceHistory(row.file,unknown),unknown);
  }
  const row=fixtures.sources[0],actual=restoreRustJobAdmissionSource(readFileSync(new URL(row.file,root),'utf8'));
  const before='          - group: translation\n            shard: 1/1\n            partition: 1-of-1\n';
  const after=Array.from({length:6},(_,index)=>'          - group: translation\n            shard: '+(index+1)+'/6\n            partition: '+(index+1)+'-of-6\n').join('');
  assert.equal(actual.split(after).length,2);assert.equal(actual.includes(before),false);
  const reportScheduleEdits=[
    ['Self-Translation Report (${{ matrix.shard }}/8)','Self-Translation Report (${{ matrix.shard }}/12)'],
    ['shard: [1, 2, 3, 4, 5, 6, 7, 8]','shard: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]'],
    ['--shard ${{ matrix.shard }}/8 --out-dir','--shard ${{ matrix.shard }}/12 --out-dir'],
  ];
  let historical=actual.replace(after,before);
  for(const [original,current] of reportScheduleEdits){
    assert.equal(historical.split(current).length,2);
    historical=historical.replace(current,original);
  }
  assert.equal(historical,row.sourceText,'all original jobs, steps, tools, native flags, gates and artifact paths stay exact');
  assert.equal(restoreEvidenceTranslationSourceHistory(row.file,actual),row.sourceText);
  assert.equal(restoreEvidenceTranslationSourceHistory(row.file,row.sourceText),row.sourceText);
  const changed=actual+'\n# changed input\n';assert.equal(restoreEvidenceTranslationSourceHistory(row.file,changed),changed);
  const job=parseWorkflow(actual).find(job=>job.id==='javascript-suite-independent');assert.ok(job);
  assert.match(job.header,/timeout-minutes: 60/u);assert.match(job.header,/fail-fast: false/u);
  const scopes=[...job.header.matchAll(/- group: translation\n\s+shard: ([1-6]\/6)\n\s+partition: ([1-6]-of-6)/gu)];
  assert.equal(scopes.length,6);assert.deepEqual(scopes.map(row=>row[1]),['1/6','2/6','3/6','4/6','5/6','6/6']);
  assert.deepEqual(scopes.map(row=>row[2]),['1-of-6','2-of-6','3-of-6','4-of-6','5-of-6','6-of-6']);
});

test('failed delivery candidates still prepare Rust admission when Rust ran', () => {
  const workflow = readFileSync(new URL('.github/workflows/ci.yml', root), 'utf8');
  const acceptance = parseWorkflow(workflow).find(job => job.id === 'acceptance');
  const [guard, checkout, setup, dependencies] = acceptance.steps;
  const condition = step => /^ {8}if: \$\{\{ (.*) \}\}$/mu.exec(step.text)?.[1];
  assert.equal(guard.name, 'Require delivery candidates');
  assert.match(guard.run, /^\s+exit 1$/mu);
  assert.equal(condition(checkout), "!cancelled() && (needs.candidates.result == 'success' || needs.rust.result != 'skipped' || needs.rust-suite.result != 'skipped')");
  assert.equal(condition(setup), "!cancelled() && steps.acceptance-checkout.outcome == 'success'");
  assert.equal(condition(dependencies), "!cancelled() && steps.acceptance-node.outcome == 'success'");
  const admission = acceptance.steps.find(step => step.name === 'Identify admitted Rust physical profiles');
  assert.equal(condition(admission), "!cancelled() && steps.acceptance-dependencies.outcome == 'success' && (needs.rust.result != 'skipped' || needs.rust-suite.result != 'skipped')");
});

test('Rust admission retries a failed job-list request with smaller complete pages', () => {
  const workflow = readFileSync(new URL('.github/workflows/ci.yml', root), 'utf8');
  const acceptance = parseWorkflow(workflow).find(job => job.id === 'acceptance');
  const admission = acceptance.steps.find(step => step.name === 'Identify admitted Rust physical profiles');
  const shell = admission.run.split('\n').slice(1).join('\n');
  const jobs = Array.from({length: 8}, (_, index) => ({
    name: `Evidence Stage (rust-suite - ${index + 1}/8)`, status: 'completed', conclusion: 'success',
  }));
  const mock = `
gh() {
  test "$#" = 4 && test "$1" = api && test "$3" = --paginate && test "$4" = --slurp || return 2
  printf '%s\\n' "$2" >&2
  case "$2" in
    'repos/link-foundation/meta-language/actions/runs/123/attempts/2/jobs?per_page=100')
      if [ "$PAGE_RESPONSE" = large ]; then printf '%s' "$LARGE_PAGE"; else printf '['; return 1; fi
      ;;
    'repos/link-foundation/meta-language/actions/runs/123/attempts/2/jobs?per_page=20')
      if [ "$PAGE_RESPONSE" = small ]; then printf '%s' "$SMALL_PAGES"; else return 1; fi
      ;;
    *) return 2 ;;
  esac
}
`;
  for (const response of ['large', 'small', 'unavailable']) {
    const directory = mkdtempSync(join(tmpdir(), 'rust-job-admission-'));
    try {
      const output = join(directory, 'output');
      const run = spawnSync('bash', ['-e', '-c', mock + shell], {
        cwd: root, encoding: 'utf8', timeout: 10_000,
        env: {...process.env, RUNNER_TEMP: directory, GITHUB_OUTPUT: output,
          GITHUB_REPOSITORY: 'link-foundation/meta-language', GITHUB_RUN_ID: '123', GITHUB_RUN_ATTEMPT: '2',
          PAGE_RESPONSE: response, LARGE_PAGE: JSON.stringify([{jobs}]),
          SMALL_PAGES: JSON.stringify([{jobs: jobs.slice(0, 4)}, {jobs: jobs.slice(4)}]),
        },
      });
      assert.ifError(run.error);
      const pageSizes = [...run.stderr.matchAll(/per_page=(\d+)/gu)].map(([, size]) => size);
      assert.deepEqual(pageSizes, response === 'large' ? ['100'] : ['100', '20']);
      if (response === 'unavailable') {
        assert.equal(run.status, 1);
        assert.throws(() => readFileSync(output), {code: 'ENOENT'});
      } else {
        assert.equal(run.status, 0, run.stderr);
        assert.equal(readFileSync(output, 'utf8'), 'enabled=true\nprofiles=evidence\n');
      }
    } finally {
      rmSync(directory, {recursive: true, force: true});
    }
  }
});

test('six complete evidence translation partitions form the exact current file union and retain distinct physical paths',()=>{
  const value=evidenceTranslationPartitionRecords(),selections=[...value.records.values()].filter(row=>row.javascriptPartition.group==='translation').map(row=>row.javascriptPartition);
  const files=selections.flatMap(row=>row.files),paths=selections.flatMap(row=>Object.values(stageFiles(row.name)));
  assert.deepEqual([...files].sort(),[...value.groups.translation].sort());assert.equal(new Set(files).size,files.length);
  assert.equal(new Set(paths).size,12);assert.ok(selections.every(row=>row.files.length>0));
  const result=mergeJavaScriptSuitePartitions({...value,testGroups:value.groups}).get('javascript-suite');
  assert.equal(result.error,null);assert.equal(result.groups['suite:javascript'].commands.length,value.records.size);
  assert.equal(result.groups['suite:javascript'].artifacts.length,value.records.size);
});

test('incomplete failed altered or mixed evidence translation cohorts cannot supply a logical suite',()=>{
  for(const mutate of [
    value=>value.records.delete('javascript-suite-translation-6-of-6'),
    value=>value.records.get('javascript-suite-translation-2-of-6').javascriptPartition.files.pop(),
    value=>value.records.get('javascript-suite-translation-2-of-6').javascriptPartition.files.push('foreign.test.js'),
    value=>{value.records.get('javascript-suite-translation-2-of-6').commit='b'.repeat(40);},
    value=>{value.records.get('javascript-suite-translation-2-of-6').error='native translation test failed';},
    value=>{value.records.get('javascript-suite-translation-2-of-6').groups['suite:javascript'].commands[0]='node scripts/test-groups.mjs --run translation';},
    value=>{value.records.get('javascript-suite-translation-2-of-6').toolchainVersions.node='v24.0.0';},
    value=>{value.records.get('javascript-suite-translation-2-of-6').groups['suite:javascript'].artifacts=[];},
    value=>{const selection=javascriptSuitePartition('translation','1/3',value.groups);const row=structuredClone(value.records.get('javascript-suite-translation-1-of-6'));row.stage=selection.name;row.javascriptPartition=selection;value.records.set(selection.name,row);},
  ]){
    const value=evidenceTranslationPartitionRecords();mutate(value);
    const result=mergeJavaScriptSuitePartitions({...value,testGroups:value.groups}).get('javascript-suite');
    assert.equal(typeof result.error,'string');assert.ok(result.error.length>0);assert.deepEqual(result.groups,{});
  }
});
