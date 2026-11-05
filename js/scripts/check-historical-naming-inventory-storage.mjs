// Read-only source/certificate/serialization freshness, not a naming-policy or
// runtime observation. Complete vocabulary and inventory checks remain in CI.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {readOwnCodebaseHistoricalNamingPolicy} from './own-codebase-baseline-naming-history.mjs';
import {readHistoricalNamingInventoryBundle,verifyHistoricalNamingInventorySerialization} from './historical-naming-inventory-storage.mjs';
assert.ok(process.argv.slice(2).every(arg=>arg==='--check'));
const root=new URL('../../',import.meta.url),history=readOwnCodebaseHistoricalNamingPolicy().additionalSourceViews;
const actual=new Map();
for(const row of history.sources){
 if(!actual.has(row.file)){
  const bytes=readFileSync(new URL(row.file,root));actual.set(row.file,{bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex')});
 }
 assert.deepEqual(actual.get(row.file),row.current,'every complete current source remains fresh: '+row.file);
}
globalThis.gc?.();
const bundle=readHistoricalNamingInventoryBundle();verifyHistoricalNamingInventorySerialization(history,bundle);
console.log('Historical naming storage is current: '+history.sources.length+' certified source epochs, '+history.views.length+' views, '+bundle.documents.length+' complete Links documents.');
