import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {readNodeRuntimeComparison} from '../scripts/dependency-node-runtime.mjs';
import {collectDependencies,ownStatus} from '../scripts/dependency-inventory.mjs';
import {readProjectData,serializeProjectData} from '../scripts/project-data.mjs';
import {readStructuredDataLino} from '../src/structured-data-links.js';
import {fileURLToPath} from 'node:url';

test('exact Node runtime selectors compare all release coordinates and preserve moving and minimum engine profiles',()=>{
  for(const [pin,floor,expected]of [['24',false,'major'],['v24',false,'major'],['24.22',false,'minor'],
    ['24.22.0',false,'version'],['v24.22.0',false,'version'],['22',true,'floor'],['22.14.0',true,'floor']]){
    assert.equal(readNodeRuntimeComparison(pin,floor),expected);
  }
  const root=fileURLToPath(new URL('../../',import.meta.url));
  const node=collectDependencies(root).find(row=>row.id==='toolchain node@24.22.0');assert.ok(node);
  assert.equal(node.compare,'version');assert.equal(ownStatus({...node,current:'24.22.0'}),'current');
  assert.equal(ownStatus({...node,current:'24.22.1'}),'behind');
  assert.equal(ownStatus({...node,current:'24.23.0'}),'behind');
  assert.equal(ownStatus({...node,pinned:'24',compare:'major',current:'24.23.0'}),'current');
  assert.equal(ownStatus({...node,pinned:'24.22',compare:'minor',current:'24.22.1'}),'current');
});

test('one-source Node reconciliation retains every unrelated audit fact and exact original collector bytes',()=>{
  const history=readProjectData(new URL('../../parity/naming/ci-node-dependency-source-history.lino',import.meta.url));
  for(const source of history.sources){
    assert.equal(Buffer.byteLength(source.sourceText),source.bytes);
    assert.equal(createHash('sha256').update(source.sourceText).digest('hex'),source.sha256);
  }
  const collector=history.sources.find(row=>row.file==='js/scripts/dependency-inventory.mjs');
  const actual=readFileSync(new URL('../../'+collector.file,import.meta.url),'utf8');
  assert.equal(actual,collector.sourceText.replace(history.before,history.after)+history.insertion);
  assert.equal(actual.replace(history.insertion,'').replace(history.after,history.before),collector.sourceText);
  const original=history.sources.find(row=>row.file==='parity/dependency-inventory.lino');
  const before=readStructuredDataLino(original.sourceText);
  const current=readNodeDependencyInventoryOriginalView();
  const generator=readNodeDependencyGeneratorRegistrationHistory().generator;
  assert.deepEqual(current.items.filter(row=>row.id===generator.id),[generator]);
  current.items=current.items.filter(row=>row.id!==generator.id);
  assert.equal(current.items.length,before.items.length);assert.equal(current.items.length,744);
  assert.equal(current.auditDate,before.auditDate);
  const node=current.items.find(row=>row.id===history.updatedNodeId);assert.ok(node);
  assert.equal(node.compare,'version');assert.equal(node.current,history.runtime.nodeVersion);
  assert.equal(node.auditDate,history.observationDate);assert.ok(node.evidence.includes('observed '+history.observationDate));
  const restored=structuredClone(current);
  restored.items=restored.items.map(row=>{
    if(row.id===history.updatedNodeId)return before.items.find(old=>old.id===history.originalNodeId);
    if(row.dependsOn)row.dependsOn=row.dependsOn.map(id=>id===history.updatedNodeId?history.originalNodeId:id);
    for(const holder of row.heldBy??[])for(const field of ['id','requiredBy'])if(holder[field]===history.updatedNodeId)holder[field]=history.originalNodeId;
    return row;
  });
  assert.deepEqual(restored,before);
  assert.equal(serializeProjectData(restored,original.file),original.sourceText);
});

import {partitionTests,testFiles} from '../scripts/test-groups.mjs';

test('original Node dependency test discovery preserves all draft source and the unchanged prior classifier',()=>{
  const location=readProjectData(new URL('../../parity/naming/node-dependency-test-location-source-history.lino',import.meta.url));
  const original=location.sources[0];
  assert.equal(original.commit,null,'the retained draft is not misrepresented as a committed source');
  assert.equal(original.worktreeBaseRevision,location.revision);
  assert.equal(Buffer.byteLength(original.sourceText),original.bytes);
  assert.equal(createHash('sha256').update(original.sourceText).digest('hex'),original.sha256);
  assert.equal(restoreNodeDependencyGeneratorTestHistory(readFileSync(new URL(import.meta.url),'utf8')),original.sourceText+location.additions);
  const classifier=restoreOwnCodebaseTestRegistrationClassifier(readFileSync(new URL('../../'+location.classifier.file,import.meta.url)));
  assert.equal(classifier.length,location.classifier.bytes);
  assert.equal(createHash('sha256').update(classifier).digest('hex'),location.classifier.sha256);
  const files=testFiles(),partition=partitionTests(files);
  assert.deepEqual(partition.unassigned,[]);assert.deepEqual(partition.ambiguous,[]);
  assert.ok(files.includes('issue-195-evidence-node-dependency.test.js'));
  assert.deepEqual(partitionTests(['issue-195-evidence-node-dependency.test.js']).groups.tooling,['issue-195-evidence-node-dependency.test.js']);
  const registrationHistory=readOwnCodebaseTestRegistrationHistory();
  assert.equal(registrationHistory.revision,'52132727d7ce158762a4499b7032a42a9f16f3c3');
  for(const row of registrationHistory.sources){
    const current=readFileSync(new URL('../../'+row.file,import.meta.url));
    assert.equal(current.length,row.current.bytes);
    assert.equal(createHash('sha256').update(current).digest('hex'),row.current.sha256);
  }
  for(const row of registrationHistory.registrations){
    assert.ok(files.includes(row.file));
    const current=partitionTests([row.file]);
    assert.deepEqual(current.unassigned,[]);assert.deepEqual(current.ambiguous,[]);
    assert.deepEqual(current.groups[row.group],[row.file]);
  }
  assert.throws(()=>restoreOwnCodebaseTestRegistrationClassifier(Buffer.from('unknown complete classifier')));
  assert.throws(()=>restoreNodeDependencyGeneratorTestHistory(readFileSync(new URL(import.meta.url),'utf8')+'\n'));
});

import {readNodeDependencyGeneratorRegistrationHistory,restoreNodeDependencyGeneratorTestHistory} from '../scripts/node-dependency-registration-history.mjs';
import {checkInventory} from '../scripts/dependency-inventory.mjs';
import {readNodeDependencyInventoryOriginalView} from '../scripts/dependency-inventory-append-history.mjs';
import {readOwnCodebaseTestRegistrationHistory,restoreOwnCodebaseTestRegistrationClassifier} from '../scripts/own-codebase-test-registration-history.mjs';

test('the new source placement generator is independently inventoried with every preceding entry intact',()=>{
  const history=readNodeDependencyGeneratorRegistrationHistory();
  const inventory=readNodeDependencyInventoryOriginalView();
  assert.equal(inventory.items.length,history.currentItemCount);
  assert.equal(inventory.items.length,745);
  assert.equal(inventory.auditDate,history.fullAuditDate);
  assert.deepEqual(inventory.items.filter(row=>row.id===history.generator.id),[history.generator]);
  assert.deepEqual(checkInventory(readProjectData(new URL('../../parity/dependency-inventory.lino',import.meta.url)),collectDependencies(fileURLToPath(new URL('../../',import.meta.url)))),[]);
});
