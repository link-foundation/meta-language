import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {mkdtempSync,mkdirSync,readFileSync,rmSync,writeFileSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {dirname,join} from 'node:path';
import {fileURLToPath} from 'node:url';
import test from 'node:test';
import {extractNameInventory,NAME_INVENTORIES} from '../scripts/issue-195-naming.mjs';
import {extractOwnCodebaseHistoricalNames,readOwnCodebaseHistoricalNamingPolicy,
  readOwnCodebaseHistoricalNamingSources,verifyOwnCodebaseHistoricalNamingSources} from '../scripts/own-codebase-baseline-naming-history.mjs';
const root=fileURLToPath(new URL('../../',import.meta.url));
const digest=bytes=>createHash('sha256').update(bytes).digest('hex');

test('historical naming retains complete certified definitions and every independent current inventory name',()=>{
  const history=readOwnCodebaseHistoricalNamingSources(),before=structuredClone(history);
  const entries=NAME_INVENTORIES.filter(entry=>history.views.some(view=>entry.inventory===view.inventory&&entry.files.length===1&&entry.files[0]===view.file));
  assert.equal(entries.length,2);
  const current=extractNameInventory(root,entries),historical=extractOwnCodebaseHistoricalNames(root,entries,history);
  assert.deepEqual(historical.map(row=>({name:row.name,record:row.record})),history.views.flatMap(view=>view.names));
  const combined=[...current,...historical];assert.deepEqual(combined.slice(0,current.length),current);
  assert.equal(combined.length,current.length+2);
  for(const row of historical){
    const view=history.views.find(view=>view.file===row.file);
    assert.equal(row.sourceView.kind,'complete certified historical source');
    assert.equal(row.sourceView.path,view.file);assert.equal(row.sourceView.revision,history.revision);
    assert.equal(row.sourceView.sha256,digest(view.predecessor.source));
    assert.notEqual(row.sourceView.sha256,view.current.sha256);
  }
  assert.deepEqual(history,before,'extractors cannot replace the retained original source or current pins');
  const changed=readOwnCodebaseHistoricalNamingPolicy();changed.views[0].current.sha256='0'.repeat(64);
  assert.deepEqual(readOwnCodebaseHistoricalNamingPolicy().views,before.views.map(({predecessor,...view})=>view));
});

test('unknown whole current naming sources and driver bytes cannot borrow historical authority',()=>{
  const history=readOwnCodebaseHistoricalNamingSources(),scratch=mkdtempSync(join(tmpdir(),'meta-language-own-naming-'));
  const files=[...history.views.map(view=>view.file),history.driver.file];
  try{
    for(const file of files){const destination=join(scratch,file);mkdirSync(dirname(destination),{recursive:true});writeFileSync(destination,readFileSync(join(root,file)));}
    const expected=extractOwnCodebaseHistoricalNames(root,NAME_INVENTORIES,history);
    assert.deepEqual(extractOwnCodebaseHistoricalNames(scratch,NAME_INVENTORIES,history),expected);
    for(const file of files){
      const destination=join(scratch,file),source=readFileSync(destination);writeFileSync(destination,Buffer.concat([source,Buffer.from('\n')]));
      assert.throws(()=>extractOwnCodebaseHistoricalNames(scratch,NAME_INVENTORIES,history));
      writeFileSync(destination,source);
    }
    const missing=NAME_INVENTORIES.filter(entry=>entry.inventory!==history.views[0].inventory);
    assert.throws(()=>extractOwnCodebaseHistoricalNames(scratch,missing,history));
    assert.throws(()=>extractOwnCodebaseHistoricalNames(scratch,[...NAME_INVENTORIES,NAME_INVENTORIES.find(entry=>entry.inventory===history.views[0].inventory)],history));
  }finally{rmSync(scratch,{recursive:true,force:true});}
});

test('complete certificate and coherent historical source forgeries refuse without replacing original definitions',()=>{
  const history=readOwnCodebaseHistoricalNamingSources();
  const mutations=[
    value=>{value.revision='0'.repeat(40);},
    value=>{value.sourceArchive.sha256='0'.repeat(64);},
    value=>{value.views.reverse();},
    value=>{value.views[0].file='rust/src/lib.rs';},
    value=>{value.views[0].names[0].record='repository.invented-source-owner';},
    value=>{value.views[0].current.sha256='0'.repeat(64);},
    value=>{value.views[0].predecessor.certificate.commitText+='\n';},
    value=>{value.views[0].predecessor.certificate.trees[0].entries.pop();},
    value=>{value.views[0].predecessor.certificate.blob.objectId='0'.repeat(40);},
    value=>{
      const row=value.views[0].predecessor;row.source+='\n';row.bytes=Buffer.byteLength(row.source);row.sha256=digest(row.source);
      row.certificate.blob.bytes=row.bytes;row.certificate.blob.sha256=row.sha256;
      row.certificate.blob.objectId=createHash('sha1').update('blob '+row.bytes+'\0').update(row.source).digest('hex');
    },
    value=>{value.driver.source+='\n';value.driver.original.bytes=Buffer.byteLength(value.driver.source);value.driver.original.sha256=digest(value.driver.source);},
    value=>{value.driver.edits[0].after+='\n';},
    value=>{value.unregistered='extra authority';},
  ];
  for(const mutate of mutations){const forged=structuredClone(history);mutate(forged);assert.throws(()=>verifyOwnCodebaseHistoricalNamingSources(forged));}
  assert.deepEqual(verifyOwnCodebaseHistoricalNamingSources(structuredClone(history)),history);
});
