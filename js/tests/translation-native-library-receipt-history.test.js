import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {generateLibraryRelocationHistory} from '../../experiments/grammar-outcome-network/native-transport/inverse/generate-library-relocation-history.mjs';
import {readProjectData,serializeProjectData} from '../scripts/project-data.mjs';
import {restoreNativeLibraryReceipt,readNativeLibraryReceiptProfile,restoreNativeLibrarySource} from '../scripts/native-library-receipt-history.mjs';
import {restoreCompleteDeclarationReceipt} from '../scripts/complete-declaration-receipts.mjs';
import {restoreTranslationSourceHistory} from '../scripts/translation-source-history.mjs';
import {verifyGitArtifactCertificate} from '../scripts/git-object-certificate.mjs';
import {restoreCurrentCiRepairSource} from '../scripts/current-ci-repair-source-history.mjs';
import {readStructuredDataLino} from '../src/structured-data-links.js';
import {restoreRawDescriptorHistoricalSource} from '../scripts/raw-descriptor-historical-cohort.mjs';

const root=new URL('../../',import.meta.url);

test('the corrected library literal retains every other native byte and both complete authenticated receipts',()=>{
  const history=generateLibraryRelocationHistory({check:true});assert.equal(history.sources.length,2);assert.equal(history.modules.length,1);
  for(const row of [...history.sources,...history.modules])verifyGitArtifactCertificate(row.certificate,Buffer.from(row.sourceText),{revision:row.commit,path:row.file});
  const module=history.modules[0];assert.equal(module.edits.length,1);assert.equal(module.offsetUnit,'UTF-16 code units');
  let source=readFileSync(new URL(module.file,root),'utf8');
  for(const edit of module.edits){assert.equal(source.slice(edit.start,edit.start+edit.to.length),edit.to);source=source.slice(0,edit.start)+edit.from+source.slice(edit.start+edit.to.length);}
  assert.equal(source,module.sourceText);
});

test('actual packaging descriptors remain distinct while every complete historical gate still executes',()=>{
  const history=generateLibraryRelocationHistory({check:true});
  for(const row of history.sources){
    const actual=readProjectData(new URL(row.file,root));
    const physical=readFileSync(new URL(row.file,root),'utf8'),descriptorView=restoreRawDescriptorHistoricalSource(physical,row.file,'native library relocation');
    const current=readStructuredDataLino(descriptorView===physical?restoreCurrentCiRepairSource(physical,row.file):descriptorView),restored=restoreNativeLibraryReceipt(current,row.identity);
    assert.equal(serializeProjectData(restored,row.file),row.sourceText);
    assert.deepEqual(readNativeLibraryReceiptProfile(row.identity),{file:row.file,...row.current});
    assert.notEqual(row.current.sha256,row.sha256);
    const original=restoreCompleteDeclarationReceipt(restored,row.identity);
    assert.deepEqual(restoreCompleteDeclarationReceipt(current,row.identity),original);
    assert.deepEqual(restoreCompleteDeclarationReceipt(actual,row.identity),original,'the complete actual successor also reaches every original gate');
    assert.equal(serializeProjectData(actual,row.file),readFileSync(new URL(row.file,root),'utf8'));
  }
});

test('changed algorithms unknown fields incomplete descriptors and unrelated native owners refuse',()=>{
  const history=generateLibraryRelocationHistory({check:true});
  for(const row of history.sources)for(const mutate of[
    value=>{value.extra='unknown';},
    value=>{value.sourceSha256='0'.repeat(64);},
    value=>{if(value.completeRust)value.completeRust+=' ';else value.emitted.text+=' ';},
    value=>{if(value.library)value.library.files[0].bytes+=1;else value.predecessor.proof.producerApplicability.sources.reverse();},
    value=>{delete value.archive;},
  ]){
    const current=readProjectData(new URL(row.file,root));mutate(current);
    assert.throws(()=>restoreCompleteDeclarationReceipt(current,row.identity),assert.AssertionError);
  }
  assert.throws(()=>readNativeLibraryReceiptProfile('unregistered packaging'),/exact native packaging identity/u);
});

test('complete current source adapters preserve every original historical assertion and reject unreviewed edits',()=>{
  const history=generateLibraryRelocationHistory({check:true});assert.equal(history.sourceAdapters.length,2);
  for(const row of history.sourceAdapters){
    const actual=readFileSync(new URL(row.file,root),'utf8'),current=restoreCurrentCiRepairSource(actual,row.file);assert.equal(restoreNativeLibrarySource(current,row.file),row.sourceText);
    verifyGitArtifactCertificate(row.certificate,Buffer.from(row.sourceText),{revision:row.commit,path:row.file});
  }
  const file='js/scripts/complete-declaration-receipts.mjs',source=readFileSync(new URL(file,root),'utf8'),revision='9f867bfac32f59a3341dabf54e8c0c02fbd83314';
  assert.doesNotThrow(()=>restoreTranslationSourceHistory(source,file,revision));
  assert.throws(()=>restoreTranslationSourceHistory(source+' ',file,revision),/unreviewed current source|complete current source/u);
});


test('the actual complete raw descriptor cohort retains the exact library relocation target and original receipt inverse',()=>{
  const history=readProjectData(new URL('parity/self-translation/library-relocation-receipt-history.lino',root));
  const row=history.sources.find(value=>value.identity==='raw consuming'),library=history.sources.find(value=>value.identity==='collection construction');assert.ok(row&&library);
  const actual=readFileSync(new URL(row.file,root),'utf8'),view=restoreRawDescriptorHistoricalSource(actual,row.file,'native library relocation');
  assert.notEqual(view,actual,'the actual current descriptor cohort requires the new source view');
  assert.deepEqual({bytes:Buffer.byteLength(view),sha256:createHash('sha256').update(view).digest('hex')},{bytes:row.current.bytes,sha256:'856a9b6139960222c160b98655e192af0097df2f8aa6e251887e29fe53d729bb'});
  const packet=readStructuredDataLino(view),descriptor=packet.predecessor.proof.producerApplicability.sources.find(value=>value.file===library.file);assert.ok(descriptor);
  assert.deepEqual(descriptor,{file:library.file,...library.current},'current native library packaging is retained separately from historical decision-source descriptors');
  const original=restoreNativeLibraryReceipt(packet,'raw consuming');
  assert.equal(serializeProjectData(original,row.file),row.sourceText,'the original complete 6f32 receipt remains recoverable byte for byte');
  verifyGitArtifactCertificate(row.certificate,Buffer.from(row.sourceText),{revision:row.commit,path:row.file});
  assert.equal(readFileSync(new URL(row.file,root),'utf8'),actual,'the actual descriptor file remains current');
});

test('the library relocation source view keeps both old byte inputs and refuses unknown whole sources and profiles',()=>{
  const row=readProjectData(new URL('parity/self-translation/library-relocation-receipt-history.lino',root)).sources.find(value=>value.identity==='raw consuming');assert.ok(row);
  const actual=readFileSync(new URL(row.file,root),'utf8'),view=restoreRawDescriptorHistoricalSource(actual,row.file,'native library relocation');
  for(const source of[row.sourceText,view])assert.equal(restoreRawDescriptorHistoricalSource(source,row.file,'native library relocation'),source);
  for(const source of[actual+'\n',actual.slice(1)]){assert.equal(restoreRawDescriptorHistoricalSource(source,row.file,'native library relocation'),source);assert.notEqual(source,view,'the original strict complete-string oracle must refuse these inputs');}
  for(const profile of['','current behavior','library relocation'])assert.equal(restoreRawDescriptorHistoricalSource(actual,row.file,profile),actual);
  assert.equal(restoreRawDescriptorHistoricalSource(actual,'unregistered.lino','native library relocation'),actual);
});

test('the historical library writer retains its original refusal to overwrite a newer complete raw artifact',()=>{
  const files=['experiments/original-link-complete-table-projection/data/raw-consuming-current-native-definitions.lino','parity/self-translation/library-relocation-receipt-history.lino','js/scripts/native-library-receipt-history.mjs'],before=files.map(file=>readFileSync(new URL(file,root)));
  assert.throws(()=>generateLibraryRelocationHistory(),/a historical writer cannot replace a newer current artifact/u);
  files.forEach((file,index)=>assert.deepEqual(readFileSync(new URL(file,root)),before[index],file));
});
