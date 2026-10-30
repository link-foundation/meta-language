// Reject unknown current sources before running historical inventory extractors.
// The original per-view checks still run afterward, including callback effects.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {resolve} from 'node:path';

export function validateCurrentHistoricalNamingSourcePins(root,inventories,additional){
 const observed=new Map();
 for(const view of additional.views){
  if(!inventories.some(entry=>entry.inventory===view.inventory&&entry.files.includes(view.file)))continue;
  const source=additional.sources.find(row=>row.file===view.file&&row.predecessor.revision===view.revision);
  assert.ok(source,'every selected historical view has its original source owner');
  if(!observed.has(view.file)){
   const bytes=readFileSync(resolve(root,view.file));
   observed.set(view.file,{bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex')});
  }
  assert.deepEqual(observed.get(view.file),source.current,'the selected root retains every complete actual source byte');
 }
}
