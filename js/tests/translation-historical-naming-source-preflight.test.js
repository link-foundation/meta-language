import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {mkdtempSync,writeFileSync,rmSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join} from 'node:path';
import test from 'node:test';
import {validateCurrentHistoricalNamingSourcePins} from '../scripts/historical-naming-source-preflight.mjs';
const pin=source=>({bytes:Buffer.byteLength(source),sha256:createHash('sha256').update(source).digest('hex')});

test('historical naming preflight authenticates every selected whole source without executing extractors',()=>{
 const directory=mkdtempSync(join(tmpdir(),'meta-language-naming-preflight-'));
 try{
  const sources=['export const first=1;\n','export const second=2;\n'];
  sources.forEach((source,index)=>writeFileSync(join(directory,index+'.js'),source));
  const rows=sources.map((source,index)=>({file:index+'.js',predecessor:{revision:'original'},current:pin(source)}));
  const views=rows.map(row=>({inventory:'inventory '+row.file,file:row.file,revision:'original'}));
  const inventories=views.map(view=>({inventory:view.inventory,files:[view.file],extract(){throw new Error('extractors run only after preflight');}}));
  const additional={sources:rows,views:[...views,views[0]]},before=structuredClone(additional);
  assert.equal(validateCurrentHistoricalNamingSourcePins(directory,inventories,additional),undefined);
  assert.deepEqual(additional,before);
  for(const index of [0,1]){
   writeFileSync(join(directory,index+'.js'),sources[index].replace(String(index+1),'9'));
   assert.throws(()=>validateCurrentHistoricalNamingSourcePins(directory,inventories,additional),/complete actual source byte/u);
   writeFileSync(join(directory,index+'.js'),sources[index]);
   validateCurrentHistoricalNamingSourcePins(directory,inventories,additional);
  }
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test('unselected historical views cannot select current files and every selected epoch keeps its own pin',()=>{
 const directory=mkdtempSync(join(tmpdir(),'meta-language-naming-preflight-'));
 try{
  writeFileSync(join(directory,'source.js'),'actual');
  const view={inventory:'selected',file:'source.js',revision:'old'},inventory={inventory:'selected',files:['source.js']};
  const row={file:'source.js',predecessor:{revision:'old'},current:pin('actual')};
  const extra={inventory:'unselected',file:'missing.js',revision:'old'};
  validateCurrentHistoricalNamingSourcePins(directory,[inventory],{sources:[row],views:[view,extra]});
  const another={...view,revision:'another'},changed={...row,predecessor:{revision:'another'},current:pin('forged')};
  assert.throws(()=>validateCurrentHistoricalNamingSourcePins(directory,[inventory],{sources:[row,changed],views:[view,another]}),/complete actual source byte/u);
  assert.throws(()=>validateCurrentHistoricalNamingSourcePins(directory,[inventory],{sources:[],views:[view]}),/original source owner/u);
  rmSync(join(directory,'source.js'));
  assert.throws(()=>validateCurrentHistoricalNamingSourcePins(directory,[inventory],{sources:[row],views:[view]}),/ENOENT/u);
 }finally{rmSync(directory,{recursive:true,force:true});}
});
