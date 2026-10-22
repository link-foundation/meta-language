// Run unchanged naming extraction in a disposable heap before Links encoding.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {isMainThread,parentPort,workerData} from 'node:worker_threads';
const root=new URL('../../',import.meta.url);
const pin=source=>({bytes:Buffer.byteLength(source),sha256:createHash('sha256').update(source).digest('hex')});

export function deriveOwnCodebaseHistoricalNamingViews(sources,inventories,read=file=>readFileSync(new URL(file,root),'utf8')){
 const views=[];
 for(const source of sources){
  globalThis.gc?.();
  const current=read(source.file);
  assert.deepEqual(pin(current),source.current,'normal freshness checks retain every complete current source pin');
  for(const entry of inventories.filter(entry=>!entry.words&&entry.files.includes(source.file))){
   const actual=new Set(entry.extract(current,source.file));
   const names=[...new Set(entry.extract(source.predecessor.source,source.file))].filter(name=>!actual.has(name))
    .map(name=>({name,record:entry.recordOf?entry.recordOf(name):name}));
   if(names.length)views.push({inventory:entry.inventory,file:source.file,revision:source.predecessor.revision,names});
  }
 }
 return views;
}

if(!isMainThread){
 assert.ok(parentPort);
 const {NAME_INVENTORIES}=await import('./issue-195-naming.mjs');
 parentPort.postMessage(deriveOwnCodebaseHistoricalNamingViews(workerData,NAME_INVENTORIES));
 parentPort.close();
}
