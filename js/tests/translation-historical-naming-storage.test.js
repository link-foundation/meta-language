import assert from 'node:assert/strict';
import test from 'node:test';
import {mkdtempSync,readFileSync,writeFileSync,rmSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join} from 'node:path';
import {readBootstrapStructuredDataLino} from '../../scripts/lib/structured-data-bootstrap.mjs';
import {writeProjectData} from '../scripts/project-data.mjs';
import {HISTORICAL_NAMING_INVENTORY_INDEX,storeHistoricalNamingInventories,readHistoricalNamingInventoryBundle,
 decodeHistoricalNamingInventoryBundle,verifyHistoricalNamingInventorySerialization} from '../scripts/historical-naming-inventory-storage.mjs';

const revisions=['6ffff3b2dc530a5f24c128dadf877f3fa106932b','8bc0ef9725388c7927acf1854870bf46f5e365be','8f2f6c483ec32a3ffc17afec694463965dc7551d','3d8e4513e2167d0d15cca13ccaefcb4ae86669fd'];
// Storage-only values carry no Git certificates or execution authority.
const fixture=()=>({schemaVersion:1,model:'array identity historical naming inventories 1',revision:revisions[0],
 sources:revisions.map((revision,index)=>({file:'fixture-'+index,predecessor:{revision,source:'whole Ω source '+index},current:{bytes:1,sha256:'fixture only'}})),
 views:revisions.map((revision,index)=>({revision,inventory:'inventory '+index,file:'fixture-'+index,names:[{name:'retained name',record:'fixture record'}]})),
 boundary:'Transport fixture only; source policy verification remains mandatory.'});

test('bounded Links source documents retain every ordered row and original logical legacy value',()=>{
 const base=mkdtempSync(join(tmpdir(),'meta-language-naming-storage-'));
 try{
  const history=fixture(),before=structuredClone(history);storeHistoricalNamingInventories(history,{base});
  const bundle=readHistoricalNamingInventoryBundle({base});assert.equal(bundle.documents.length,5);
  assert.deepEqual(decodeHistoricalNamingInventoryBundle(bundle),history);
  verifyHistoricalNamingInventorySerialization(history,bundle,{base});storeHistoricalNamingInventories(history,{base,check:true});
  assert.deepEqual(history,before,'the writer cannot alter source rows or views');
  writeProjectData(join(base,HISTORICAL_NAMING_INVENTORY_INDEX),history);
  const legacy=readHistoricalNamingInventoryBundle({base});assert.equal(legacy.documents.length,1);
  assert.deepEqual(decodeHistoricalNamingInventoryBundle(legacy),history);
  verifyHistoricalNamingInventorySerialization(history,legacy,{base});
 }finally{rmSync(base,{recursive:true,force:true});}
});

test('changed missing reordered or foreign documents and cross-document aliases refuse',()=>{
 const base=mkdtempSync(join(tmpdir(),'meta-language-naming-storage-'));
 try{
  const history=fixture();storeHistoricalNamingInventories(history,{base});
  const indexFile=join(base,HISTORICAL_NAMING_INVENTORY_INDEX),indexBytes=readFileSync(indexFile),index=readBootstrapStructuredDataLino(indexBytes.toString('utf8'));
  const page=join(base,index.partitions[1].file),bytes=readFileSync(page);
  writeFileSync(page,Buffer.concat([bytes,Buffer.from('\n')]));assert.throws(()=>readHistoricalNamingInventoryBundle({base}));writeFileSync(page,bytes);
  rmSync(page);assert.throws(()=>readHistoricalNamingInventoryBundle({base}),/ENOENT/u);writeFileSync(page,bytes);
  for(const mutate of [value=>value.partitions.pop(),value=>value.partitions.reverse(),value=>{value.partitions[0].file='../foreign.lino';},value=>{value.partitions[0].bytes++;},value=>{value.extra=true;}]){
   const changed=structuredClone(index);mutate(changed);writeProjectData(indexFile,changed);assert.throws(()=>readHistoricalNamingInventoryBundle({base}));writeFileSync(indexFile,indexBytes);
  }
  const alias={same:'container'};history.sources[0].shared=alias;history.sources[1].shared=alias;
  assert.throws(()=>storeHistoricalNamingInventories(history,{base}),/shared container cannot cross/u);
 }finally{rmSync(base,{recursive:true,force:true});}
});

test('repeated document reads keep caller containers detached and observe page edits behind an unchanged index',()=>{
 const base=mkdtempSync(join(tmpdir(),'meta-language-naming-storage-'));
 try{
  const history=fixture();storeHistoricalNamingInventories(history,{base});
  const original=readHistoricalNamingInventoryBundle({base}),expected=structuredClone(original);
  original.documents[1].text='caller replacement';original.documents.push({file:'foreign',text:'caller'});original.index.partitions[0].sha256='caller';
  assert.deepEqual(readHistoricalNamingInventoryBundle({base}),expected);
  const page=join(base,expected.documents[1].file),bytes=readFileSync(page),index=readFileSync(join(base,HISTORICAL_NAMING_INVENTORY_INDEX));
  writeFileSync(page,Buffer.concat([bytes,Buffer.from('\n')]));
  assert.throws(()=>readHistoricalNamingInventoryBundle({base}));
  assert.deepEqual(readFileSync(join(base,HISTORICAL_NAMING_INVENTORY_INDEX)),index,'the index is deliberately unchanged');
  assert.throws(()=>verifyHistoricalNamingInventorySerialization(history,expected,{base}));
  writeFileSync(page,bytes);assert.deepEqual(readHistoricalNamingInventoryBundle({base}),expected);
 }finally{rmSync(base,{recursive:true,force:true});}
});

test('scalar storage reuse never selects another actual root as source authority',()=>{
 const first=mkdtempSync(join(tmpdir(),'meta-language-naming-storage-')),second=mkdtempSync(join(tmpdir(),'meta-language-naming-storage-'));
 try{
  const old=fixture(),current=fixture();current.sources[0].predecessor.source+=' different root';
  storeHistoricalNamingInventories(old,{base:first});storeHistoricalNamingInventories(current,{base:second});
  assert.deepEqual(decodeHistoricalNamingInventoryBundle(readHistoricalNamingInventoryBundle({base:first})),old);
  assert.deepEqual(decodeHistoricalNamingInventoryBundle(readHistoricalNamingInventoryBundle({base:second})),current);
  assert.deepEqual(decodeHistoricalNamingInventoryBundle(readHistoricalNamingInventoryBundle({base:first})),old);
 }finally{rmSync(first,{recursive:true,force:true});rmSync(second,{recursive:true,force:true});}
});
