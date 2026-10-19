// Test and generator oracle only; immutable trusted historical source, never caller code.
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {runInNewContext} from 'node:vm';
import {readProjectData} from '../../../../js/scripts/project-data.mjs';

export function readOriginalOutcomeIntegerOracle() {
  const file=new URL('../../../../parity/naming/owned-integer-range-source-history.lino',import.meta.url);
  assert.equal(createHash('sha256').update(readFileSync(file)).digest('hex'),'fa8ff451ed9e897ca0e5fe0b231f1119ce9e8cfdb407036c0c4aa4575a1e7e63');
  const history=readProjectData(file),original=history.sources[0];
  assert.equal(Buffer.byteLength(original.sourceText),original.bytes);
  assert.equal(createHash('sha256').update(original.sourceText).digest('hex'),original.sha256);
  assert.equal(Buffer.from(original.sourceText).subarray(history.declaration.start,history.declaration.end).toString(),history.before);
  assert.equal(createHash('sha256').update(history.before).digest('hex'),history.declaration.sha256);
  const requireValue=(condition,frame,code,message)=>{if(!condition)throw Object.assign(new TypeError(message),{code});};
  const oracle=runInNewContext('('+history.before+')',{Buffer,requireValue},{timeout:1000,contextCodeGeneration:{strings:false,wasm:false}});
  return {history,oracle};
}
