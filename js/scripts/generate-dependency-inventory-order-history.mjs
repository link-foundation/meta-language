// Restore the original inventory order; retain every current declaration/fact.
import assert from 'node:assert/strict';
import {fileURLToPath} from 'node:url';
import {captureGitArtifactCertificate} from './capture-git-artifact-certificate.mjs';
import {checkProjectData,readProjectData,writeProjectData,serializeProjectData} from './project-data.mjs';
import {readStructuredDataLino} from '../src/structured-data-links.js';
import {retainDependencyInventoryOrder} from './dependency-inventory-order.mjs';
import {collectDependencies,checkInventory,renderAuditDocument} from './dependency-inventory.mjs';
import {writeFileSync,readFileSync} from 'node:fs';

/** Prove the complete original prefix and every retained current record. */
export function generateDependencyInventoryOrderHistory({check=false}={}) {
  const root=new URL('../../',import.meta.url),path='parity/dependency-inventory.lino';
  const capture=revision=>{const value=captureGitArtifactCertificate(revision,path);
    return{revision,path,source:value.blob.toString('utf8'),certificate:value.certificate};};
  const original=capture('9ac3038132953e6a220bba6f06a3e6147d39836f');
  const predecessor=capture('394103f912aaac8e2f69147519ab97c6c5107528');
  const first=readStructuredDataLino(original.source),previous=readStructuredDataLino(predecessor.source);
  const actual=readProjectData(new URL(path,root));
  assert.equal(first.items.length,745);assert.equal(previous.items.length,773);
  const declarations=collectDependencies(fileURLToPath(root));assert.deepEqual(checkInventory(actual,declarations),[]);
  const byId=new Map(actual.items.map(row=>[row.id,row]));assert.equal(byId.size,actual.items.length);
  for(const row of previous.items)assert.deepEqual(byId.get(row.id),row,'all complete preceding facts remain exact: '+row.id);
  for(const row of first.items)assert.deepEqual(byId.get(row.id),row,'all complete original facts remain exact: '+row.id);
  assert.equal(actual.auditDate,first.auditDate);assert.equal(actual.auditDate,previous.auditDate);
  const ordered=retainDependencyInventoryOrder(first,actual);
  const prefix={...ordered,items:ordered.items.slice(0,first.items.length)};
  assert.equal(serializeProjectData(prefix,path),original.source);
  const oldIds=new Set(previous.items.map(row=>row.id));
  const inverse={...ordered,items:previous.items.map(row=>ordered.items.find(item=>item.id===row.id))};
  assert.equal(serializeProjectData(inverse,path),predecessor.source,'complete current predecessor remains reconstructible');
  const value={schemaVersion:1,original,predecessor,
    retainedIds:previous.items.map(row=>row.id),
    boundary:'Identity-based order repair preserves all 745 original and 773 preceding complete records, original audit date, every current declaration and both exact certified source inverses. Newly declared identities append. This is no live registry audit or native execution evidence.'};
  assert.equal(ordered.items.filter(row=>oldIds.has(row.id)).length,773);
  if(check)checkProjectData(new URL(path,root),ordered);else writeProjectData(new URL(path,root),ordered);
  const document=new URL('docs/dependency-audit.md',root),rendered=renderAuditDocument(ordered);
  if(check)assert.equal(readFileSync(document,'utf8'),rendered);else writeFileSync(document,rendered);
  const history=new URL('parity/fixtures/dependency-inventory-order-history.lino',root);
  if(check)checkProjectData(history,value);else writeProjectData(history,value);
  return value;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.slice(2).every(value=>value==='--check'));
  generateDependencyInventoryOrderHistory({check:process.argv.includes('--check')});
}
