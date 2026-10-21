// Preserve the complete original author before correcting its CST byte spans.
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {fileURLToPath} from 'node:url';
import {captureGitArtifactCertificate} from '../../../../js/scripts/capture-git-artifact-certificate.mjs';
import {verifyGitArtifactCertificate} from '../../../../js/scripts/git-object-certificate.mjs';
import {readProjectData,serializeProjectData,checkProjectData,writeProjectData} from '../../../../js/scripts/project-data.mjs';
import {readStructuredDataLino} from '../../../../js/src/structured-data-links.js';
import {restoreContinuationHistoricalSource} from '../../../../js/scripts/continuation-source-history.mjs';

const root=new URL('../../../../',import.meta.url),revision='6fba3503a710325310e359995a1a77fcedf8011a';
const pin=bytes=>({bytes:Buffer.byteLength(bytes),sha256:createHash('sha256').update(bytes).digest('hex')});
const historicalSource=file=>restoreContinuationHistoricalSource(readFileSync(new URL(file,root),'utf8'),file,'original number source ranges');
export function generateOriginalNumberSourceRangeHistory({check=false}={}) {
  const file='experiments/grammar-outcome-network/native-transport/inverse/generate-original-number-observations.mjs';
  const original=captureGitArtifactCertificate(revision,file),sourceText=original.blob.toString('utf8');
  verifyGitArtifactCertificate(original.certificate,original.blob,{revision,path:file});
  assert.ok(sourceText.includes('sourceText:sinkSource.slice(item.start,item.end)'));
  const current=historicalSource(file);
  assert.ok(check||current===readFileSync(new URL(file,root),'utf8'),
    'a historical generator cannot overwrite a newer current source receipt; use --check');
  assert.ok(current.includes('sourceText:readNativeSourceByteRange(sinkBytes,item.start,item.end)'));
  assert.ok(current.includes('sourceText:readNativeSourceByteRange(parentBytes,found[0].start,found[0].end)'));
  const receiptFile='parity/self-translation/original-native-number-observations.lino';
  const before=captureGitArtifactCertificate(revision,receiptFile),previous=readStructuredDataLino(before.blob.toString('utf8'));
  verifyGitArtifactCertificate(before.certificate,before.blob,{revision,path:receiptFile});
  const after=readStructuredDataLino(historicalSource(receiptFile)),inverse=structuredClone(after);
  assert.ok(inverse.byteRangeReader);inverse.author=previous.author;delete inverse.byteRangeReader;
  assert.deepEqual(inverse,previous,'all original source inputs numeric validation and native output metadata remain exact');
  assert.ok(Buffer.from(serializeProjectData(inverse,receiptFile)).equals(before.blob),'the complete original Links receipt survives its exact byte inverse');
  const unchangedFiles=['rust/tests/unit/native_json_number_observation_model.rs','rust/tests/unit/native_json_number_observation_visitors.rs',
    'parity/fixtures/original-native-number-observations.lino','parity/self-translation/original-native-number-observation-decorators.lino'];
  const unchanged=unchangedFiles.map(file=>{
    const previous=captureGitArtifactCertificate(revision,file),current=Buffer.from(historicalSource(file));
    assert.ok(current.equals(previous.blob),'every complete native operation and original fixture byte remains unchanged: '+file);
    return{file,...pin(current)};
  });
  const history={schemaVersion:1,revision,sources:[{file,commit:revision,...pin(original.blob),sourceText,certificate:original.certificate,current:pin(current)}],
    previousReceipt:{file:receiptFile,...pin(before.blob),sourceText:before.blob.toString('utf8'),certificate:before.certificate},
    currentReceipt:pin(historicalSource(receiptFile)),unchanged,
    correction:{original:'JavaScript string coordinates applied to actual Rust CST byte offsets',
      replacement:'Validated UTF-8 byte ranges with complete-context validation and explicit scalar boundaries',
      originalCorpus:'Both complete original sink and parent inputs are ASCII and retain every original output byte',
      boundary:'Current Unicode CST byte controls extend admitted source transport. The original complete qualified author remains independently reconstructable. No semantic identity equivalence, native execution, compiler provenance or complete DTO reconstruction is inferred.'}};
  const output=new URL('../../../../parity/self-translation/original-number-source-range-history.lino',import.meta.url);
  if(check)checkProjectData(output,history);else writeProjectData(output,history);
  return history;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.slice(2).every(value=>value==='--check'));
  generateOriginalNumberSourceRangeHistory({check:process.argv.includes('--check')});
  console.log('Preserved the complete certified original source author and every native output fixture and numeric operation.');
}
