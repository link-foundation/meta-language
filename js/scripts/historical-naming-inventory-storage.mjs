// Explicit bounded Links documents; legacy single graphs retain their limits.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {resolve} from 'node:path';
import {fileURLToPath} from 'node:url';
import {readBootstrapStructuredDataLino} from '../../scripts/lib/structured-data-bootstrap.mjs';
import {serializeProjectData,writeProjectData,checkProjectData} from './project-data.mjs';

const root=fileURLToPath(new URL('../../',import.meta.url));
export const HISTORICAL_NAMING_INVENTORY_INDEX='parity/naming/array-identity-historical-inventories.lino';
const revisions=Object.freeze(['6ffff3b2dc530a5f24c128dadf877f3fa106932b','8bc0ef9725388c7927acf1854870bf46f5e365be','8f2f6c483ec32a3ffc17afec694463965dc7551d','3d8e4513e2167d0d15cca13ccaefcb4ae86669fd']);
const pathOf=revision=>'parity/naming/array-identity-historical-inventories-'+revision.slice(0,8)+'.lino';
assert.equal(new Set(revisions.map(pathOf)).size,revisions.length,'registered source documents have distinct paths');
const pin=source=>({bytes:Buffer.byteLength(source),sha256:createHash('sha256').update(source).digest('hex')});
const exactKeys=(value,keys)=>assert.deepEqual(Object.keys(value).sort(),[...keys].sort());

function inventoryFragments(history){
  const fragments=revisions.map(revision=>({schemaVersion:1,model:'historical naming inventory fragment 1',revision,
    sources:history.sources.filter(row=>row.predecessor.revision===revision),views:history.views.filter(row=>row.revision===revision)}));
  assert.deepEqual(fragments.flatMap(row=>row.sources),history.sources,'every complete source keeps its original order');
  assert.deepEqual(fragments.flatMap(row=>row.views),history.views,'every scoped view keeps its original order');
  // A fragment boundary must never split an observable container alias.
  const owners=new WeakMap();
  const visit=(value,owner)=>{
    if(value===null||typeof value!=='object')return;
    if(owners.has(value)){assert.equal(owners.get(value),owner,'a shared container cannot cross source documents');return;}
    owners.set(value,owner);for(const child of Object.values(value))visit(child,owner);
  };
  fragments.forEach((fragment,index)=>visit(fragment,index));
  return fragments;
}

/** Normal author/checker: every document uses the unchanged canonical codec. */
export function storeHistoricalNamingInventories(history,{check=false,base=root}={}){
  const partitions=[];
  for(const fragment of inventoryFragments(history)){
    const file=pathOf(fragment.revision),actual=resolve(base,file);
    if(check)checkProjectData(actual,fragment);else writeProjectData(actual,fragment);
    const bytes=readFileSync(actual);
    assert.ok(bytes.length<=64*1024*1024,'each closed source document retains a bounded physical input');
    partitions.push({revision:fragment.revision,file,...pin(bytes),sources:fragment.sources.length,views:fragment.views.length});
  }
  const index={schemaVersion:1,model:'partitioned historical naming inventories 1',revision:history.revision,boundary:history.boundary,partitions};
  const actual=resolve(base,HISTORICAL_NAMING_INVENTORY_INDEX);
  if(check)checkProjectData(actual,index);else writeProjectData(actual,index);
  return index;
}

/** Read all complete document bytes before cache selection. No page can hide
 * behind an unchanged index, and no decoded graph gets a larger node budget. */
export function readHistoricalNamingInventoryBundle({base=root}={}){
  const text=readFileSync(resolve(base,HISTORICAL_NAMING_INVENTORY_INDEX),'utf8');
  const index=readBootstrapStructuredDataLino(text),documents=[{file:HISTORICAL_NAMING_INVENTORY_INDEX,text}];
  if(index.model==='array identity historical naming inventories 1')return{legacy:index,documents};
  exactKeys(index,['schemaVersion','model','revision','boundary','partitions']);
  assert.equal(index.schemaVersion,1);assert.equal(index.model,'partitioned historical naming inventories 1');
  assert.equal(index.revision,revisions[0]);assert.equal(typeof index.boundary,'string');
  assert.deepEqual(index.partitions.map(row=>row.revision),revisions,'only complete ordered certified source epochs may be selected');
  for(const row of index.partitions){
    exactKeys(row,['revision','file','bytes','sha256','sources','views']);
    assert.equal(row.file,pathOf(row.revision));
    assert.ok(Number.isSafeInteger(row.bytes)&&row.bytes>0&&row.bytes<=64*1024*1024);
    assert.match(row.sha256,/^[a-f0-9]{64}$/u);
    assert.ok(Number.isSafeInteger(row.sources)&&row.sources>=0&&Number.isSafeInteger(row.views)&&row.views>=0);
    const text=readFileSync(resolve(base,row.file),'utf8');assert.deepEqual(pin(text),{bytes:row.bytes,sha256:row.sha256});
    documents.push({file:row.file,text});
  }
  return{index,documents};
}

/** Source certificates and naming ownership are checked by the policy reader. */
export function decodeHistoricalNamingInventoryBundle(bundle){
  if(bundle.legacy)return bundle.legacy;
  const sources=[],views=[];
  for(const[index,row]of bundle.index.partitions.entries()){
    const fragment=readBootstrapStructuredDataLino(bundle.documents[index+1].text);
    exactKeys(fragment,['schemaVersion','model','revision','sources','views']);
    assert.equal(fragment.schemaVersion,1);assert.equal(fragment.model,'historical naming inventory fragment 1');assert.equal(fragment.revision,row.revision);
    assert.equal(fragment.sources.length,row.sources);assert.equal(fragment.views.length,row.views);
    assert.ok(fragment.sources.every(source=>source.predecessor.revision===row.revision));
    assert.ok(fragment.views.every(view=>view.revision===row.revision));
    sources.push(...fragment.sources);views.push(...fragment.views);globalThis.gc?.();
  }
  return{schemaVersion:1,model:'array identity historical naming inventories 1',revision:bundle.index.revision,sources,views,boundary:bundle.index.boundary};
}

/** Reproduce every actual page and index byte through the normal serializer. */
export function verifyHistoricalNamingInventorySerialization(history,bundle,{base=root}={}){
  if(bundle.legacy){assert.equal(serializeProjectData(history,'history.lino'),bundle.documents[0].text);return;}
  const fragments=inventoryFragments(history),partitions=[];
  for(const[index,fragment]of fragments.entries()){
    const text=serializeProjectData(fragment,'fragment.lino');
    assert.equal(text,bundle.documents[index+1].text);
    partitions.push({revision:fragment.revision,file:pathOf(fragment.revision),...pin(text),sources:fragment.sources.length,views:fragment.views.length});globalThis.gc?.();
  }
  const index={schemaVersion:1,model:'partitioned historical naming inventories 1',revision:history.revision,boundary:history.boundary,partitions};
  assert.equal(serializeProjectData(index,'index.lino'),bundle.documents[0].text);
  for(const document of bundle.documents){
    assert.equal(readFileSync(resolve(base,document.file),'utf8'),document.text,'every complete archive document remains unchanged during verification');
  }
}
