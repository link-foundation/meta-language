// Historical source views preserve every predecessor assertion independently.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {readProjectData,serializeProjectData} from './project-data.mjs';
import {readStructuredDataLino} from '../src/structured-data-links.js';
import {restoreContinuationHistoricalSource} from './continuation-source-history.mjs';

const historyFile=new URL('../../parity/self-translation/array-relative-element-history.lino',import.meta.url);
const historyHash='5560db49c46a77d13893ca1fc1d3ffe582dc27a727c45d071c58109ed2519c32';
const digest=value=>createHash('sha256').update(value).digest('hex');
let history;
function arrayRelativeElementHistory() {
  if(!history){
    const bytes=readFileSync(historyFile);assert.equal(digest(bytes),historyHash,'the complete reviewed relative array source history remains exact');
    history=readProjectData(historyFile);assert.equal(history.schemaVersion,1);
  }
  return history;
}
export function restoreArrayRelativeElementSource(source,file,currentOnly=false) {
  assert.equal(typeof source,'string');
  source=restoreCurrentCiRepairSource(source,file);
  source=restoreContinuationHistoricalSource(source,file,'relative array');
  const row=arrayRelativeElementHistory().sources.find(row=>row.file===file);
  if(!row)return source;
  const hash=digest(source);
  if(hash===row.sha256){assert.equal(Buffer.byteLength(source),row.bytes);assert.equal(source,row.sourceText);return source;}
  if(currentOnly&&hash!==row.current.sha256)return source;
  assert.equal(hash,row.current.sha256,'unreviewed current relative array source changes refuse');
  assert.equal(Buffer.byteLength(source),row.current.bytes);
  const edit=row.edit,bytes=Buffer.from(source);
  assert.equal(bytes.subarray(edit.start,edit.end).toString(),edit.currentText);
  const restored=Buffer.concat([bytes.subarray(0,edit.start),Buffer.from(edit.previousText),bytes.subarray(edit.end)]);
  assert.equal(restored.length,row.bytes);assert.equal(digest(restored),row.sha256);assert.equal(restored.toString(),row.sourceText);
  return restored.toString();
}

import {restoreCurrentCiRepairSource} from './current-ci-repair-source-history.mjs';
export function restoreArrayRelativeElementProjectData(value,file) {
  const row=arrayRelativeElementHistory().sources.find(row=>row.file===file);
  assert.ok(row&&row.role==='historical descriptor','a complete recorded descriptor source is required');
  // Each current descriptor is checked independently by the normal current gates.
  // This historical input view reconstructs its exact immutable predecessor.
  return readStructuredDataLino(restoreArrayRelativeElementSource(serializeProjectData(value,file),file));
}
