import assert from 'node:assert/strict';
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
    const current=readStructuredDataLino(restoreCurrentCiRepairSource(readFileSync(new URL(row.file,root),'utf8'),row.file)),restored=restoreNativeLibraryReceipt(current,row.identity);
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
