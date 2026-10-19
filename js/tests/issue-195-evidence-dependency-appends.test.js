import assert from 'node:assert/strict';
import {execFileSync} from 'node:child_process';
import {readFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import test from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';
import {collectDependencies,checkInventory} from '../scripts/dependency-inventory.mjs';
import {readDependencyInventoryAppendHistory,verifyDependencyInventoryAppends,
  readNodeDependencyInventoryOriginalView,restoreNodeDependencyInventoryTestSource} from '../scripts/dependency-inventory-append-history.mjs';
import {generateDependencyInventoryAppendHistory} from '../scripts/generate-dependency-inventory-append-history.mjs';
import {structuredDataToLino,readStructuredDataLino} from '../src/structured-data-links.js';

const root=new URL('../../',import.meta.url);
const actual=()=>readProjectData(new URL('parity/dependency-inventory.lino',root));
const declarations=()=>collectDependencies(fileURLToPath(root));

test('every current dependency declaration remains checked while the complete original inventory is reconstructed',()=>{
  generateDependencyInventoryAppendHistory({check:true});
  const inventory=actual(),declared=declarations(),history=readDependencyInventoryAppendHistory();
  assert.deepEqual(checkInventory(inventory,declared),[]);
  const original=verifyDependencyInventoryAppends(inventory,declared);
  assert.equal(original.items.length,745);
  assert.equal(structuredDataToLino(original),history.inventory.source);
  assert.deepEqual(readNodeDependencyInventoryOriginalView(),original);
  const ids=new Set(original.items.map(row=>row.id));
  assert.deepEqual(inventory.items.slice(original.items.length).map(row=>row.name).sort(),[
    'js/scripts/generate-string-relative-character-template.mjs',
    'js/scripts/generate-contributor-instruction-coverage.mjs',
    'js/scripts/generate-dependency-inventory-append-history.mjs',
    'src/data/string-relative-character-template.lino',
  ].sort());
  for(const row of inventory.items.slice(original.items.length)){
    assert.equal(ids.has(row.id),false);
    assert.equal(declared.filter(item=>item.id===row.id).length,1);
  }
});

test('historical inventory selection refuses changed old facts missing declarations and forged append identities',()=>{
  const original=actual(),declared=declarations(),history=readDependencyInventoryAppendHistory();
  const mutations=[value=>{value.items[0].evidence='altered';},value=>{value.auditDate='2026-10-07';},
    value=>{[value.items[0],value.items[1]]=[value.items[1],value.items[0]];},
    value=>{value.items.splice(1,1);},value=>{value.items.push(structuredClone(value.items[0]));}];
  for(let index=745;index<original.items.length;index++){
    mutations.push(value=>{value.items.splice(index,1);});
    mutations.push(value=>{value.items[index].id+=' unregistered';});
  }
  for(const mutate of mutations){const changed=structuredClone(original);mutate(changed);assert.throws(()=>verifyDependencyInventoryAppends(changed,declared));}
  const forged=structuredClone(history);forged.inventory.source+=' ';
  assert.throws(()=>verifyDependencyInventoryAppends(original,declared,forged));
  const reloaded=readStructuredDataLino(structuredDataToLino(history));
  assert.equal(structuredDataToLino(verifyDependencyInventoryAppends(original,declared,reloaded)),history.inventory.source);
});

test('dependency source views retain every original assertion and both complete preceding guard modules',()=>{
  const revision='e4d60c822135ac3e4dc55d9141dabd30faf1e94a';
  const previous=file=>execFileSync('git',['show',revision+':'+file],{cwd:root,encoding:'utf8',maxBuffer:2*1024*1024});
  const file='js/tests/issue-195-evidence-node-dependency.test.js',current=readFileSync(new URL(file,root),'utf8');
  assert.equal(restoreNodeDependencyInventoryTestSource(current),previous(file));
  for(const changed of [current+' ',current.replace('744','743'),current.replace('const node=','const other=')]){
    assert.throws(()=>restoreNodeDependencyInventoryTestSource(changed));
  }
  for(const [file,before,after]of [
    ['js/scripts/node-dependency-registration-history.mjs','export function restoreNodeDependencyGeneratorTestHistory(source) {',
      'export function restoreNodeDependencyGeneratorTestHistory(source) {\n  source=restoreNodeDependencyInventoryTestSource(source);'],
    ['js/scripts/generate-node-dependency-test-placement.mjs',"const destination=readFileSync(new URL(policy.placement.destination,root),'utf8');",
      "const destination=restoreNodeDependencyInventoryTestSource(readFileSync(new URL(policy.placement.destination,root),'utf8'));"],
  ]){
    const source=readFileSync(new URL(file,root),'utf8');
    const insertion="\nimport {restoreNodeDependencyInventoryTestSource} from './dependency-inventory-append-history.mjs';\n";
    assert.equal(source.split(insertion).length,2);assert.equal(source.split(after).length,2);
    assert.equal(source.replace(insertion,'').replace(after,before),previous(file));
  }
});
