// Declaration refreshes replace facts by identity and append new identities.
import assert from 'node:assert/strict';

/** Preserve existing record order without retaining absent or obsolete facts. */
export function retainDependencyInventoryOrder(previous,current) {
  const before=new Set(previous.items.map(row=>row.id));
  const actual=new Map(current.items.map(row=>[row.id,row]));
  assert.equal(before.size,previous.items.length,'duplicate previous dependency identities');
  assert.equal(actual.size,current.items.length,'duplicate current dependency identities');
  return{...current,items:[
    ...previous.items.filter(row=>actual.has(row.id)).map(row=>actual.get(row.id)),
    ...current.items.filter(row=>!before.has(row.id)),
  ]};
}
