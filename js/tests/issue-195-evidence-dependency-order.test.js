import assert from 'node:assert/strict';
import test from 'node:test';
import {retainDependencyInventoryOrder} from '../scripts/dependency-inventory-order.mjs';
import {generateDependencyInventoryOrderHistory} from '../scripts/generate-dependency-inventory-order-history.mjs';
import {verifyGitArtifactCertificate} from '../scripts/git-object-certificate.mjs';

test('declaration order retention keeps current facts removes absent entries and appends each actual new identity once',()=>{
  const previous={auditDate:'old',items:[{id:'b',fact:1},{id:'a',fact:2},{id:'removed'}]};
  const current={auditDate:'current',items:[{id:'a',fact:3},{id:'new',fact:4},{id:'b',fact:5}]};
  const result=retainDependencyInventoryOrder(previous,current);
  assert.deepEqual(result,{auditDate:'current',items:[current.items[2],current.items[0],current.items[1]]});
  assert.notEqual(result.items,current.items);assert.equal(result.items[0],current.items[2]);
  assert.deepEqual(previous.items.map(row=>row.id),['b','a','removed']);
  assert.deepEqual(current.items.map(row=>row.id),['a','new','b']);
  for(const [first,second]of [[{items:[{id:'a'},{id:'a'}]},current],
    [previous,{items:[{id:'a'},{id:'a'}]}]])assert.throws(()=>retainDependencyInventoryOrder(first,second));
});

test('the actual current inventory retains both complete certified predecessors and every declared current dependency',()=>{
  const value=generateDependencyInventoryOrderHistory({check:true});
  for(const row of [value.original,value.predecessor]){
    verifyGitArtifactCertificate(row.certificate,Buffer.from(row.source),{revision:row.revision,path:row.path});
  }
  assert.equal(value.retainedIds.length,773);assert.equal(new Set(value.retainedIds).size,773);
});
