// Run unchanged naming extraction in a disposable heap before Links encoding.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {once} from 'node:events';
import {readFileSync} from 'node:fs';
import {isMainThread,parentPort} from 'node:worker_threads';
const root=new URL('../../',import.meta.url);
const pin=source=>({bytes:Buffer.byteLength(source),sha256:createHash('sha256').update(source).digest('hex')});

export function deriveOwnCodebaseHistoricalNamingViews(sources,inventories,read=file=>readFileSync(new URL(file,root),'utf8')){
 const views=[],inventoriesByFile=new Map(),currentNames=new Map();
 for(const entry of inventories){
  if(entry.words)continue;
  for(const file of new Set(entry.files)){
   if(!inventoriesByFile.has(file))inventoriesByFile.set(file,[]);
   inventoriesByFile.get(file).push(entry);
  }
 }
 for(const source of sources){
  const current=read(source.file);
  assert.deepEqual(pin(current),source.current,'normal freshness checks retain every complete current source pin');
  for(const entry of inventoriesByFile.get(source.file)??[]){
   if(!currentNames.has(entry))currentNames.set(entry,new Map());
   const byFile=currentNames.get(entry),cached=byFile.get(source.file);
   const actual=cached?.sha256===source.current.sha256
    ?cached.names:new Set(entry.extract(current,source.file));
   byFile.set(source.file,{sha256:source.current.sha256,names:actual});
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
 const input=once(parentPort,'message');
 parentPort.postMessage('ready');
 const [{sources,texts}]=await input;
 const epochs=sources.map(({file,current,revision,text})=>({file,current,predecessor:{revision,source:texts[text]}}));
 parentPort.postMessage(deriveOwnCodebaseHistoricalNamingViews(epochs,NAME_INVENTORIES));
 parentPort.close();
}
