// Automatic immutable source capture; reviewed edits select historical inputs
// without replacing any original assertion, declaration or current inventory gate.
import assert from 'node:assert/strict';
import {fileURLToPath} from 'node:url';
import {captureGitArtifactCertificate} from './capture-git-artifact-certificate.mjs';
import {checkProjectData,writeProjectData} from './project-data.mjs';

export function generateDependencyInventoryAppendHistory({check=false}={}) {
  const capture=(revision,path)=>{const {certificate,blob}=captureGitArtifactCertificate(revision,path);return{certificate,source:blob.toString('utf8')};};
  const policy={schemaVersion:1,
    inventory:capture('9ac3038132953e6a220bba6f06a3e6147d39836f','parity/dependency-inventory.lino'),
    test:capture('e4d60c822135ac3e4dc55d9141dabd30faf1e94a','js/tests/issue-195-evidence-node-dependency.test.js'),
    edits:[{
      before:"  const current=readProjectData(new URL('../../parity/dependency-inventory.lino',import.meta.url));",
      after:'  const current=readNodeDependencyInventoryOriginalView();',
    },{
      before:"  const inventory=readProjectData(new URL('../../parity/dependency-inventory.lino',import.meta.url));",
      after:'  const inventory=readNodeDependencyInventoryOriginalView();',
    },{
      before:"assert.deepEqual(checkInventory(inventory,collectDependencies(fileURLToPath(new URL('../../',import.meta.url)))),[]);",
      after:"assert.deepEqual(checkInventory(readProjectData(new URL('../../parity/dependency-inventory.lino',import.meta.url)),collectDependencies(fileURLToPath(new URL('../../',import.meta.url)))),[]);",
    },{
      before:"import {checkInventory} from '../scripts/dependency-inventory.mjs';",
      after:"import {checkInventory} from '../scripts/dependency-inventory.mjs';\nimport {readNodeDependencyInventoryOriginalView} from '../scripts/dependency-inventory-append-history.mjs';",
    }],
    boundary:'All original assertions, inputs and expected 744/745 historical item counts remain exact. Historical selection first validates every actual current declaration and every complete original inventory byte; appends cannot replace original records, audit facts or aliases. Current inventory gates remain active. This source view supplies no registry freshness, native execution or full-scope completion evidence.'};
  let current=policy.test.source;
  for(const edit of policy.edits){assert.equal(current.split(edit.before).length,2);current=current.replace(edit.before,()=>edit.after);}
  const file=new URL('../../parity/fixtures/dependency-inventory-append-history.lino',import.meta.url);
  if(check)checkProjectData(file,policy);else writeProjectData(file,policy);
  return policy;
}

if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.slice(2).every(value=>value==='--check'));
  generateDependencyInventoryAppendHistory({check:process.argv.includes('--check')});
  console.log('Captured complete historical inventory and four exact test-source edits.');
}
