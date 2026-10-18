// Sequential CI teardown after every owned compiler/probe process has closed.
// Retain inputs and provenance before deleting only this task's scratch tree.
import assert from 'node:assert/strict';
import {existsSync,lstatSync,readFileSync,readdirSync,realpathSync,rmSync,writeFileSync} from 'node:fs';
import {basename,dirname,join} from 'node:path';
import {fileURLToPath} from 'node:url';
import {readProjectData,writeProjectData} from '../../../../js/scripts/project-data.mjs';
import {requireNativeCI} from './run.mjs';
import {git,sha256,within} from './plan.mjs';
import {checkRunnerPublication} from './generate-publication.mjs';

const repository=realpathSync(fileURLToPath(new URL('../../../../',import.meta.url)));
const temporary=requireNativeCI({repository});checkRunnerPublication();
const executionCommit=git(repository,['rev-parse','HEAD']).trim();
assert.equal(executionCommit,process.env.GITHUB_SHA,'actual checked-out CI head');
const parent=join(repository,'issue-195-artifacts/bulk95-outcome-overlay');
if(existsSync(parent)) {
  assert.ok(within(repository,realpathSync(parent))&&!lstatSync(parent).isSymbolicLink(),'retained artifact root');
  for(const name of readdirSync(parent).filter(name=>name.startsWith('run-'))) {
    const artifacts=join(parent,name);
    assert.ok(lstatSync(artifacts).isDirectory()&&!lstatSync(artifacts).isSymbolicLink());
    assert.ok(within(parent,realpathSync(artifacts)));
    const receiptPath=join(artifacts,'runner-receipt.lino');
    if(!existsSync(receiptPath))continue;
    const runner=readProjectData(receiptPath),work=runner.work;
    assert.equal(runner.executionCommit,executionCommit);
    assert.equal(runner.artifacts,artifacts);
    assert.ok(dirname(work)===temporary&&basename(work).startsWith('meta-language-bulk95-outcome-'),'direct owned runner scratch');
    if(!existsSync(work))continue;
    assert.ok(lstatSync(work).isDirectory()&&!lstatSync(work).isSymbolicLink());
    assert.equal(realpathSync(work),work);
    const marker=join(work,'.meta-language-bulk95-owned.lino'),owner=readProjectData(marker);
    assert.deepEqual(owner,{repository,executionCommit,artifacts},'whole task ownership record');
    assert.ok(runner.processes.every(process=>Object.hasOwn(process,'status')&&Object.hasOwn(process,'signal')),'owned commands have closed');
    const retained=[];
    for(const name of ['.meta-language-bulk95-owned.lino','native-oracle-assets-request.lino']) {
      const file=join(work,name);if(!existsSync(file))continue;
      assert.ok(lstatSync(file).isFile()&&!lstatSync(file).isSymbolicLink());
      const bytes=readFileSync(file),target=join(artifacts,'retained-'+name.replace(/^\./u,''));
      writeFileSync(target,bytes,{flag:'wx'});
      assert.deepEqual(readFileSync(target),bytes,'exact retained scratch input');
      retained.push({path:target,bytes:bytes.length,sha256:sha256(bytes)});
    }
    // Logs, oracle and native reports were authored directly in artifacts.
    // They are outside work and remain untouched by this fixed owned removal.
    rmSync(work,{recursive:true,force:false});assert.ok(!existsSync(work));
    writeProjectData(join(artifacts,'scratch-cleanup-receipt.lino'),{schemaVersion:1,repository,executionCommit,
      operation:'remove stopped task-owned external scratch',work,retained});
  }
}
console.log('Retained experiment evidence and released only stopped task-owned scratch.');
