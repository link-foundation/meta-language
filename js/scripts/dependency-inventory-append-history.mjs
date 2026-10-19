// Historical assertions retain their complete original inventory; current
// declarations are checked independently before selecting that source view.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {readProjectData} from './project-data.mjs';
import {structuredDataToLino,readStructuredDataLino} from '../src/structured-data-links.js';
import {verifyGitArtifactCertificate} from './git-object-certificate.mjs';
import {checkInventory,collectDependencies} from './dependency-inventory.mjs';

const file=new URL('../../parity/fixtures/dependency-inventory-append-history.lino',import.meta.url);
const inventoryRevision='9ac3038132953e6a220bba6f06a3e6147d39836f';
const testRevision='e4d60c822135ac3e4dc55d9141dabd30faf1e94a';
const digest=value=>createHash('sha256').update(value).digest('hex');
const policyHash='ae8dbb57e3f2d2df38c2e97765bf01ca58e358c0a1aab65681b2f82c5d3a23dc';

export function readDependencyInventoryAppendHistory() {
  const bytes=readFileSync(file);assert.equal(digest(bytes),policyHash);
  return readProjectData(file);
}

export function restoreNodeDependencyInventoryTestSource(source) {
  const history=readDependencyInventoryAppendHistory();
  const original=history.test.source;
  let current=original;
  for(const edit of history.edits){assert.equal(current.split(edit.before).length,2);current=current.replace(edit.before,()=>edit.after);}
  assert.ok(source===original||source===current,'only the complete original or reviewed inventory test source is admitted');
  return original;
}

/** Check every current declaration and every old byte before removing appends. */
export function verifyDependencyInventoryAppends(inventory,declarations,history=readDependencyInventoryAppendHistory()) {
  assert.equal(digest(structuredDataToLino(history)),policyHash);
  for(const [capture,revision,path]of [[history.inventory,inventoryRevision,'parity/dependency-inventory.lino'],
    [history.test,testRevision,'js/tests/issue-195-evidence-node-dependency.test.js']]){
    verifyGitArtifactCertificate(capture.certificate,Buffer.from(capture.source),{revision,path});
  }
  assert.deepEqual(checkInventory(inventory,declarations),[],'the actual complete current declaration corpus stays checked');
  const before=readStructuredDataLino(history.inventory.source);
  assert.equal(before.items.length,745);
  assert.ok(inventory.items.length>=before.items.length);
  const prefix={...inventory,items:inventory.items.slice(0,before.items.length)};
  assert.equal(structuredDataToLino(prefix),history.inventory.source,'every complete original item, audit fact, order and alias stays exact');
  const ids=new Set(prefix.items.map(row=>row.id));
  for(const entry of inventory.items.slice(before.items.length)){
    assert.equal(ids.has(entry.id),false,'appends cannot replace or repeat an original identity');ids.add(entry.id);
    assert.ok(declarations.some(row=>row.id===entry.id),'each appended entry has an actual current declaration');
  }
  return prefix;
}

export function readNodeDependencyInventoryOriginalView() {
  const root=fileURLToPath(new URL('../../',import.meta.url));
  return verifyDependencyInventoryAppends(readProjectData(new URL('../../parity/dependency-inventory.lino',import.meta.url)),collectDependencies(root));
}

/** Keep the old source-placement meaning while current definitions stay separate. */
export function restoreNodeDependencyInventoryNamingEntry(entry) {
  const path='js/tests/issue-195-evidence-node-dependency.test.js';
  if(entry.inventory!=='B70 physical field source view names for js/tests/dependency-inventory-node-runtime.test.js'
    ||entry.files.length!==1||entry.files[0]!==path)return entry;
  return{...entry,extract:(source,file)=>entry.extract(restoreNodeDependencyInventoryTestSource(source),file)};
}
