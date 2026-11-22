// Explicit bounded Links documents; legacy single graphs retain their limits.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {mkdirSync,mkdtempSync,readFileSync,renameSync,rmSync,statSync,writeFileSync} from 'node:fs';
import {basename,dirname,resolve} from 'node:path';
import {fileURLToPath} from 'node:url';
import {readBootstrapStructuredDataLino} from '../../scripts/lib/structured-data-bootstrap.mjs';
import {resolveProjectDataPath,serializeProjectData} from './project-data.mjs';

const root=fileURLToPath(new URL('../../',import.meta.url));
export const HISTORICAL_NAMING_INVENTORY_INDEX='parity/naming/array-identity-historical-inventories.lino';
const revisions=Object.freeze(['6ffff3b2dc530a5f24c128dadf877f3fa106932b','8bc0ef9725388c7927acf1854870bf46f5e365be','8f2f6c483ec32a3ffc17afec694463965dc7551d','3d8e4513e2167d0d15cca13ccaefcb4ae86669fd']);
const sourcesPerPart=64,partsPerRevision=16,maximumDocumentBytes=64*1024*1024;
const maximumPartBytes=384*1024*1024;
const pathOf=(revision,part=0)=>'parity/naming/array-identity-historical-inventories-'+revision.slice(0,8)+(part?'-'+part:'')+'.lino';
const documentPaths=new Set([HISTORICAL_NAMING_INVENTORY_INDEX]);
for(const revision of revisions)for(let part=0;part<partsPerRevision;part++)documentPaths.add(pathOf(revision,part));
assert.equal(documentPaths.size,1+revisions.length*partsPerRevision,'registered source documents have distinct paths');
const pin=source=>({bytes:Buffer.byteLength(source),sha256:createHash('sha256').update(source).digest('hex')});
const exactKeys=(value,keys)=>assert.deepEqual(Object.keys(value).sort(),[...keys].sort());
let documentSnapshot;

/** Read every actual byte before reusing an equal immutable scalar string.
 * One actual root and its registered paths bound retained storage. */
function readHistoricalNamingDocumentText(base,file,expectedBytes){
  assert.ok(documentPaths.has(file));
  const directory=resolve(base),path=resolve(directory,file),size=statSync(path).size;
  if(expectedBytes===undefined)assert.ok(size<=2*maximumDocumentBytes,'bounded historical naming index');
  else assert.equal(size,expectedBytes,'the actual naming document has its exact declared physical size');
  const actual=readFileSync(path,'utf8');
  if(documentSnapshot?.directory!==directory)documentSnapshot={directory,texts:new Map()};
  const previous=documentSnapshot.texts.get(file);
  if(previous===actual)return previous;
  documentSnapshot.texts.set(file,actual);
  return actual;
}

function inventoryFragments(history,{paged=history.sources.length>sourcesPerPart}={}){
  const fragments=revisions.map(revision=>({schemaVersion:1,model:'historical naming inventory fragment 1',revision,
    sources:history.sources.filter(row=>row.predecessor.revision===revision),views:history.views.filter(row=>row.revision===revision)}));
  const pages=paged?fragments.flatMap(fragment=>{
    const parts=Math.max(1,Math.ceil(fragment.sources.length/sourcesPerPart));
    assert.ok(parts<=partsPerRevision,'bounded naming source documents per original revision');
    const sourceParts=new Map();
    const selected=Array.from({length:parts},(_,part)=>({...fragment,sources:fragment.sources.slice(part*sourcesPerPart,(part+1)*sourcesPerPart),views:[]}));
    for(const [part,page]of selected.entries()){
      for(const source of page.sources){
        assert.ok(!sourceParts.has(source.file),'each file has one source epoch per revision');
        sourceParts.set(source.file,part);
      }
    }
    for(const view of fragment.views){
      const part=sourceParts.get(view.file);
      assert.ok(part!==undefined,'every scoped view retains its original source epoch');
      selected[part].views.push(view);
    }
    return selected;
  }):fragments;
  assert.deepEqual(pages.flatMap(row=>row.sources),history.sources,'every complete source keeps its original order');
  assert.deepEqual(pages.flatMap(row=>row.views),history.views,'every scoped view keeps its original order');
  // A fragment boundary must never split an observable container alias.
  const owners=new WeakMap();
  const visit=(value,owner)=>{
    if(value===null||typeof value!=='object')return;
    if(owners.has(value)){assert.equal(owners.get(value),owner,'a shared container cannot cross source documents');return;}
    owners.set(value,owner);for(const child of Object.values(value))visit(child,owner);
  };
  pages.forEach((fragment,index)=>visit(fragment,index));
  return pages;
}

function describeDocument(fragment,part,paged,text){
  return {revision:fragment.revision,file:pathOf(fragment.revision,part),...pin(text),
    sources:fragment.sources.length,views:fragment.views.length,...(paged?{part}:{})};
}
function inventoryIndex(history,partitions,paged){
  return {schemaVersion:paged?2:1,model:'partitioned historical naming inventories '+(paged?'2':'1'),
    revision:history.revision,boundary:history.boundary,partitions};
}

/** Normal author/checker: every document uses the unchanged canonical codec. */
export function storeHistoricalNamingInventories(history,{check=false,base=root}={}){
  const paged=history.sources.length>sourcesPerPart;
  const fragments=inventoryFragments(history,{paged}),partitions=[],indexPath=resolve(base,HISTORICAL_NAMING_INVENTORY_INDEX);
  let stage;
  if(!check){
    mkdirSync(dirname(indexPath),{recursive:true});
    stage=mkdtempSync(resolve(dirname(indexPath),'.historical-naming-'));
  }
  const save=(file,data)=>{
    const target=resolveProjectDataPath(resolve(base,file));
    if(check)assert.ok(readFileSync(target,'utf8')===data,'complete naming document is current: '+file);
    else writeFileSync(resolve(stage,basename(file)),data,'utf8');
  };
  try{
    let previousRevision,part,totalBytes=0;
    for(const fragment of fragments){
      part=fragment.revision===previousRevision?part+1:0;
      previousRevision=fragment.revision;
      const text=serializeProjectData(fragment,'fragment.lino');
      const descriptor=describeDocument(fragment,part,paged,text);
      assert.ok(descriptor.bytes<=maximumDocumentBytes,'each closed source document retains a bounded physical input');
      totalBytes+=descriptor.bytes;
      assert.ok(totalBytes<=maximumPartBytes,'bounded complete historical naming source documents');
      save(descriptor.file,text);
      partitions.push(descriptor);
    }
    const index=inventoryIndex(history,partitions,paged);
    save(HISTORICAL_NAMING_INVENTORY_INDEX,serializeProjectData(index,'index.lino'));
    if(stage){
      for(const {file} of partitions)renameSync(resolve(stage,basename(file)),resolve(base,file));
      renameSync(resolve(stage,basename(indexPath)),indexPath);
    }
    return index;
  }finally{
    if(stage)rmSync(stage,{recursive:true,force:true});
  }
}

/** Read all complete document bytes before cache selection. No page can hide
 * behind an unchanged index, and no decoded graph gets a larger node budget. */
export function readHistoricalNamingInventoryBundle({base=root}={}){
  const text=readHistoricalNamingDocumentText(base,HISTORICAL_NAMING_INVENTORY_INDEX);
  const index=readBootstrapStructuredDataLino(text),documents=[{file:HISTORICAL_NAMING_INVENTORY_INDEX,text}];
  if(index.model==='array identity historical naming inventories 1')return{legacy:index,documents};
  exactKeys(index,['schemaVersion','model','revision','boundary','partitions']);
  assert.ok(index.schemaVersion===1||index.schemaVersion===2);
  const paged=index.schemaVersion===2;
  assert.equal(index.model,'partitioned historical naming inventories '+index.schemaVersion);
  assert.equal(index.revision,revisions[0]);assert.equal(typeof index.boundary,'string');
  assert.ok(Array.isArray(index.partitions)&&index.partitions.length<=revisions.length*(paged?partsPerRevision:1));
  const order=[];
  let part,previous,totalBytes=0;
  for(const row of index.partitions){
    exactKeys(row,['revision','file','bytes','sha256','sources','views',...(paged?['part']:[])]);
    part=row.revision===previous?.revision?part+1:0;
    if(part===0){
      assert.equal(row.revision,revisions[order.length],'only complete ordered certified source epochs may be selected');
      order.push(row.revision);
    }
    if(paged){
      assert.equal(row.part,part);
      assert.ok(part<partsPerRevision&&row.sources<=sourcesPerPart);
      if(part>0)assert.ok(previous.sources===sourcesPerPart&&row.sources>0,'only full prior source documents can continue');
    }else assert.equal(part,0);
    assert.equal(row.file,pathOf(row.revision,part));
    assert.ok(Number.isSafeInteger(row.bytes)&&row.bytes>0&&row.bytes<=maximumDocumentBytes);
    totalBytes+=row.bytes;
    assert.ok(totalBytes<=maximumPartBytes,'bounded complete historical naming source documents');
    assert.match(row.sha256,/^[a-f0-9]{64}$/u);
    assert.ok(Number.isSafeInteger(row.sources)&&row.sources>=0&&Number.isSafeInteger(row.views)&&row.views>=0);
    previous=row;
  }
  assert.deepEqual(order,revisions,'only complete ordered certified source epochs may be selected');
  const knownFiles=new Set([HISTORICAL_NAMING_INVENTORY_INDEX,...index.partitions.map(row=>row.file)]);
  for(const file of documentSnapshot.texts.keys())if(!knownFiles.has(file))documentSnapshot.texts.delete(file);
  for(const row of index.partitions){
    const text=readHistoricalNamingDocumentText(base,row.file,row.bytes);
    assert.deepEqual(pin(text),{bytes:row.bytes,sha256:row.sha256});
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
    sources.push(...fragment.sources);views.push(...fragment.views);
    if(bundle.index.schemaVersion===1)globalThis.gc?.();
  }
  return{schemaVersion:1,model:'array identity historical naming inventories 1',revision:bundle.index.revision,sources,views,boundary:bundle.index.boundary};
}

/** Reproduce every actual page and index byte through the normal serializer. */
export function verifyHistoricalNamingInventorySerialization(history,bundle,{base=root}={}){
  if(bundle.legacy){assert.equal(serializeProjectData(history,'history.lino'),bundle.documents[0].text);return;}
  const paged=bundle.index.schemaVersion===2;
  const fragments=inventoryFragments(history,{paged}),partitions=[];
  assert.equal(fragments.length,bundle.index.partitions.length);
  let part,previousRevision;
  for(const[index,fragment]of fragments.entries()){
    const text=serializeProjectData(fragment,'fragment.lino');
    assert.equal(text,bundle.documents[index+1].text);
    part=fragment.revision===previousRevision?part+1:0;
    previousRevision=fragment.revision;
    partitions.push(describeDocument(fragment,part,paged,text));
  }
  const index=inventoryIndex(history,partitions,paged);
  assert.equal(serializeProjectData(index,'index.lino'),bundle.documents[0].text);
  for(const document of bundle.documents){
    assert.equal(readFileSync(resolve(base,document.file),'utf8'),document.text,'every complete archive document remains unchanged during verification');
  }
}
