// Release only the target restored into this fresh GitHub Actions checkout.
// The frozen experiment allocates its own target later, outside this checkout.
import assert from 'node:assert/strict';
import {existsSync,lstatSync,mkdirSync,realpathSync,rmSync} from 'node:fs';
import {join} from 'node:path';
import {fileURLToPath} from 'node:url';
import {writeProjectData} from '../../../../js/scripts/project-data.mjs';
import {requireNativeCI} from './run.mjs';
import {git,within} from './plan.mjs';
import {checkRunnerPublication} from './generate-publication.mjs';

const repository=realpathSync(fileURLToPath(new URL('../../../../',import.meta.url)));
requireNativeCI({repository});
checkRunnerPublication();
assert.equal(realpathSync(git(repository,['rev-parse','--show-toplevel']).trim()),repository,'actual fresh CI checkout');
assert.equal(git(repository,['rev-parse','HEAD']).trim(),process.env.GITHUB_SHA,'checked-out CI head');
const target=join(repository,'rust/target');
const restored=lstatSync(target,{throwIfNoEntry:false});
const receipt={schemaVersion:1,scope:'Release restored regenerable native build cache; no native execution.',
  repository,executionCommit:process.env.GITHUB_SHA,path:target,operation:'remove restored Cargo target',existed:Boolean(restored)};
if(receipt.existed) {
  assert.ok(restored.isDirectory()&&!restored.isSymbolicLink(),'only a regular restored target directory');
  assert.equal(realpathSync(target),target,'target must not escape through a symbolic ancestor');
  assert.equal(realpathSync(join(repository,'rust')),join(repository,'rust'),'owned crate parent');
  rmSync(target,{recursive:true,force:false});
  assert.ok(!existsSync(target),'restored target released');
}
const directory=join(repository,'issue-195-artifacts/bulk95-outcome-overlay/cache-release');
mkdirSync(directory,{recursive:true});
assert.ok(within(repository,realpathSync(directory))&&!lstatSync(directory).isSymbolicLink(),'retained receipt owner');
writeProjectData(join(directory,'receipt.lino'),receipt);
console.log('Released only the restored target owned by this fresh CI checkout.');
