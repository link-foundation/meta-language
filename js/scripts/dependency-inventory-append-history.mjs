// Historical assertions retain their complete original inventory; current
// declarations are checked independently before selecting that source view.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {fileURLToPath} from 'node:url';
import {readProjectData,readProjectFileBytes} from './project-data.mjs';
import {structuredDataToLino,readStructuredDataLino} from '../src/structured-data-links.js';
import {verifyGitArtifactCertificate} from './git-object-certificate.mjs';
import {checkInventory,collectDependencies} from './dependency-inventory.mjs';
import {restoreOwnCodebaseTestRegistrationTestSource} from './own-codebase-test-registration-history.mjs';

const file=new URL('../../parity/fixtures/dependency-inventory-append-history.lino',import.meta.url);
const inventoryRevision='9ac3038132953e6a220bba6f06a3e6147d39836f';
const testRevision='e4d60c822135ac3e4dc55d9141dabd30faf1e94a';
const digest=value=>createHash('sha256').update(value).digest('hex');
const policyHash='ae8dbb57e3f2d2df38c2e97765bf01ca58e358c0a1aab65681b2f82c5d3a23dc';
const lastAppendRevision='ec9f4efb0e9ef936114e09823981352f6db13700';
let originalInventory,appendHistory;

function readVerifiedAppendHistory() {
  const bytes=readProjectFileBytes(file);assert.equal(digest(bytes),policyHash);
  appendHistory??=readStructuredDataLino(bytes.toString('utf8'));
  return appendHistory;
}

export function readDependencyInventoryAppendHistory() {
  return structuredClone(readVerifiedAppendHistory());
}

export function restoreNodeDependencyInventoryTestSource(source) {
  const history=readVerifiedAppendHistory();
  const original=history.test.source;
  let current=original;
  for(const edit of history.edits){assert.equal(current.split(edit.before).length,2);current=current.replace(edit.before,()=>edit.after);}
  if(source!==original&&source!==current)source=restoreOwnCodebaseTestRegistrationTestSource(source);
  assert.ok(source===original||source===current,'only the complete original or reviewed inventory test source is admitted');
  return original;
}

// The pinned action gained one user after the certified inventories were taken.
// Verify its exact new declaration before reconstructing a historical record.
export function restoreDependencyDeclaration(entry, original) {
  const workflow = '.github/workflows/rust.yml';
  if (entry?.id !== 'action dtolnay/rust-toolchain@1.99.0' ||
      original?.id !== entry.id || !entry.declaredIn.includes(workflow)) {
    return entry;
  }
  assert.deepEqual(entry.declaredIn, [...original.declaredIn, workflow],
    'the Rust workflow is the only added declaration for the pinned action');
  return {...entry, declaredIn: original.declaredIn};
}

/** Check every current declaration and every old byte before removing appends. */
export function verifyDependencyInventoryAppends(inventory,declarations,history) {
  // Impossible current inventories cannot select a historical view. Reject
  // them before decoding its complete certificate/source packet; successful
  // inputs still run every original historical authentication below.
  assert.deepEqual(checkInventory(inventory,declarations),[],'the actual complete current declaration corpus stays checked');
  const before=verifiedOriginalInventory(history);
  assert.equal(before.items.length,745);
  assert.ok(inventory.items.length>=before.items.length);
  const historicalItems = inventory.items.slice(0,before.items.length)
    .map((entry, index) => restoreDependencyDeclaration(entry, before.items[index]));
  const prefix={...inventory,items:historicalItems};
  assert.deepEqual(prefix,before,'every complete original item, audit fact, order and alias stays exact');
  const ids=new Set(prefix.items.map(row=>row.id));
  for(const entry of inventory.items.slice(before.items.length)){
    assert.equal(ids.has(entry.id),false,'appends cannot replace or repeat an original identity');ids.add(entry.id);
    assert.ok(declarations.some(row=>row.id===entry.id),'each appended entry has an actual current declaration');
  }
  return prefix;
}

function verifiedOriginalInventory(history) {
  if(history===undefined&&originalInventory)return originalInventory;
  const original=history??readVerifiedAppendHistory();
  assert.equal(digest(structuredDataToLino(original)),policyHash);
  for(const [capture,revision,path]of [[original.inventory,inventoryRevision,'parity/dependency-inventory.lino'],
    [original.test,testRevision,'js/tests/issue-195-evidence-node-dependency.test.js']]){
    verifyGitArtifactCertificate(capture.certificate,Buffer.from(capture.source),{revision,path});
  }
  const before=readStructuredDataLino(original.inventory.source);
  assert.equal(structuredDataToLino(before),original.inventory.source);
  if(history===undefined)originalInventory=before;
  return before;
}

/** Authenticate the complete inventory at the last append-only revision. */
export function readDependencyInventoryBeforeRefresh() {
  const history=readProjectData(new URL('../../parity/fixtures/dependency-inventory-before-refresh.lino',import.meta.url));
  assert.equal(history.revision,lastAppendRevision);
  const original=verifiedOriginalInventory();
  const preceding={...original,items:[...original.items,...history.items]};
  verifyGitArtifactCertificate(history.certificate,Buffer.from(structuredDataToLino(preceding)),
    {revision:lastAppendRevision,path:'parity/dependency-inventory.lino'});
  return structuredClone(preceding);
}

export function readNodeDependencyInventoryOriginalView() {
  const root=fileURLToPath(new URL('../../',import.meta.url));
  const current=readProjectData(new URL('../../parity/dependency-inventory.lino',import.meta.url));
  assert.deepEqual(checkInventory(current,collectDependencies(root)),[],'every current declaration remains checked');
  return structuredClone(verifiedOriginalInventory());
}

/** Keep the old source-placement meaning while current definitions stay separate. */
export function restoreNodeDependencyInventoryNamingEntry(entry) {
  const path='js/tests/issue-195-evidence-node-dependency.test.js';
  if(entry.inventory!=='B70 physical field source view names for js/tests/dependency-inventory-node-runtime.test.js'
    ||entry.files.length!==1||entry.files[0]!==path)return entry;
  return{...entry,extract:(source,file)=>entry.extract(restoreNodeDependencyInventoryTestSource(source),file)};
}
