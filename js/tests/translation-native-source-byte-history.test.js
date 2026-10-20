import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import test from 'node:test';
import {readProjectData,serializeProjectData} from '../scripts/project-data.mjs';
import {readStructuredDataLino} from '../src/structured-data-links.js';
import {verifyGitArtifactCertificate} from '../scripts/git-object-certificate.mjs';
import {generateOriginalNumberSourceRangeHistory} from '../../experiments/grammar-outcome-network/native-transport/inverse/generate-source-range-history.mjs';
import {restoreContinuationHistoricalSource} from '../scripts/continuation-source-history.mjs';

const root=new URL('../../',import.meta.url);
const pin=source=>({bytes:Buffer.byteLength(source),sha256:createHash('sha256').update(source).digest('hex')});
const historicalSource=file=>restoreContinuationHistoricalSource(readFileSync(new URL(file,root),'utf8'),file,'original number source ranges');

test('the whole original source author and receipt remain certified after the byte range correction',()=>{
  const history=generateOriginalNumberSourceRangeHistory({check:true});
  assert.equal(history.sources.length,1);const original=history.sources[0];
  assert.equal(verifyGitArtifactCertificate(original.certificate,Buffer.from(original.sourceText),{revision:history.revision,path:original.file}).toString(),original.sourceText);
  assert.deepEqual(pin(original.sourceText),{bytes:original.bytes,sha256:original.sha256});
  const current=historicalSource(original.file);assert.deepEqual(pin(current),original.current);
  assert.ok(original.sourceText.includes('sourceText:sinkSource.slice(item.start,item.end)'));
  assert.ok(current.includes('sourceText:readNativeSourceByteRange(sinkBytes,item.start,item.end)'));
  assert.ok(current.includes('sourceText:readNativeSourceByteRange(parentBytes,found[0].start,found[0].end)'));
  assert.throws(()=>verifyGitArtifactCertificate(original.certificate,Buffer.from(original.sourceText+'\n'),{revision:history.revision,path:original.file}));
  assert.throws(()=>verifyGitArtifactCertificate(original.certificate,Buffer.from(original.sourceText),{revision:history.revision,path:'other.mjs'}));
});

test('all preceding native operations fixture bytes and complete Links receipt survive without weaker assertions',()=>{
  const history=readProjectData(new URL('parity/self-translation/original-number-source-range-history.lino',root));
  assert.equal(history.unchanged.length,4);
  for(const row of history.unchanged)assert.deepEqual(pin(historicalSource(row.file)),{bytes:row.bytes,sha256:row.sha256});
  const before=history.previousReceipt,bytes=Buffer.from(before.sourceText);
  verifyGitArtifactCertificate(before.certificate,bytes,{revision:history.revision,path:before.file});
  assert.deepEqual(pin(bytes),{bytes:before.bytes,sha256:before.sha256});
  const previous=readStructuredDataLino(before.sourceText),current=readStructuredDataLino(historicalSource(before.file));
  assert.deepEqual(pin(historicalSource(before.file)),history.currentReceipt);
  assert.equal(current.byteRangeReader.file,'js/scripts/native-source-byte-ranges.mjs');
  assert.deepEqual(pin(readFileSync(new URL(current.byteRangeReader.file,root))),{bytes:current.byteRangeReader.bytes,sha256:current.byteRangeReader.sha256});
  const inverse=structuredClone(current);inverse.author=previous.author;delete inverse.byteRangeReader;
  assert.deepEqual(inverse,previous);assert.equal(serializeProjectData(inverse,before.file),before.sourceText);
  for(const key of ['inputs','sourceInventory','wirePlan','originalVisitors','trait','jsonVisitor','observationDeclaration','observationMethod','storage','inherent','implementation','relocation','rawModel','outputs','limits','counterBoundary'])
    assert.deepEqual(current[key],previous[key],key);
});
