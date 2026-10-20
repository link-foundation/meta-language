import assert from 'node:assert/strict';
import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {readProjectData} from '../../js/scripts/project-data.mjs';
import {readStructuredDataLino,structuredDataToLino} from '../../js/src/structured-data-links.js';
import {ORIGINAL_LINK_RAW_CONSUMING_ARTIFACT, readOriginalLinkRawConsumingProfile} from './raw-consuming-emission.mjs';
import {prepareOriginalLinkRawBorrowedEmission, readOriginalLinkRawBorrowedProfile} from './raw-borrowed-emission.mjs';
import {planOriginalLinkRawBorrowedNative} from './raw-native-observations.mjs';
import {readPortableSourceParameterStyles} from '../../js/scripts/portable-source-library-style.mjs';
import {readOriginalRawColumnLoanStyles} from './raw-column-loan-styles.mjs';
import {tokenize} from '../../js/src/translation/lexer.js';

test('raw immutable parameter loans retain the complete predecessor and every checked definition', () => {
  const before=readFileSync(ORIGINAL_LINK_RAW_CONSUMING_ARTIFACT);
  const stored=readProjectData(ORIGINAL_LINK_RAW_CONSUMING_ARTIFACT);
  const predecessor=readOriginalLinkRawConsumingProfile(stored.predecessor);
  const {source,receipt,profile}=prepareOriginalLinkRawBorrowedEmission(predecessor);
  assert.equal(receipt.definitionCount,106);assert.deepEqual(receipt.declarationNames,stored.declarationNames);
  assert.deepEqual(profile.proof.predecessor,predecessor);
  const parameters=readPortableSourceParameterStyles(stored.completeRust,receipt.options);
  const columns=readOriginalRawColumnLoanStyles(parameters.source);
  assert.equal(source,columns.source);assert.deepEqual(receipt.columnLoans,columns.record);
  assert.equal(columns.record.loans.length,40);
  assert.deepEqual(receipt.cloneCalls,{before:746,after:356});
  assert.equal(receipt.styles.length,59+columns.actions.length);
  assert.equal(receipt.publicEntryHeader,'pub fn check_original_link_raw_shape(root_high: f64, root_low: f64, columns: Vec<Vec<f64>>) -> Vec<f64> {');
  assert.equal(source.split(receipt.publicEntryHeader).length,2);
  assert.equal(receipt.nativeCompilation,false);assert.equal(receipt.nativeExecution,false);
  assert.deepEqual(readOriginalLinkRawBorrowedProfile(stored.predecessor),profile);
  assert.deepEqual(readFileSync(ORIGINAL_LINK_RAW_CONSUMING_ARTIFACT),before);
  // Retain the original parameter-only profile as an independently derived
  // historical view. The current profile additionally applies forty local
  // column loans; its complete source and receipt are checked above.
  const file='experiments/original-link-complete-table-projection/data/raw-borrowed-parameter-receipt.lino';
  const previousBytes=verifyGitArtifactCertificate(
    readProjectData(new URL('../../parity/fixtures/historical/git-raw-borrowed-18a8e0a0-proof.lino',import.meta.url)),
    readFileSync(new URL('../../parity/fixtures/historical/git-raw-borrowed-18a8e0a0.lino',import.meta.url)),
    {revision:'18a8e0a04bab43f6572be311ceb75c6596fdc147',path:file});
  const previous=readStructuredDataLino(previousBytes.toString('utf8'));
  assert.deepEqual(receipt.predecessor,{...previous.predecessor,bytes:before.length,
    sha256:createHash('sha256').update(before).digest('hex')});
  const oldAuthor=readProjectData(new URL('../../parity/self-translation/ci-39372455987-receipt-sources-history.lino',import.meta.url))
    .sources.find(row=>row.file==='experiments/original-link-complete-table-projection/raw-borrowed-emission.mjs');
  verifyGitArtifactCertificate(oldAuthor.certificate,Buffer.from(oldAuthor.sourceText),{revision:oldAuthor.commit,path:oldAuthor.file});
  const pin=value=>({bytes:Buffer.byteLength(value),sha256:createHash('sha256').update(value).digest('hex')});
  const countClones=value=>{
    const tokens=tokenize(value,'Rust').tokens;
    return tokens.filter((token,index)=>token.kind==='identifier'&&token.value==='clone'
      &&tokens[index-1]?.value==='.'&&tokens[index+1]?.value==='('&&tokens[index+2]?.value===')').length;
  };
  const parameterReceipt={...receipt,source:pin(parameters.source),
    cloneCalls:{before:countClones(stored.completeRust),after:countClones(parameters.source)},
    styles:receipt.styles.slice(0,parameters.actions.length),
    decisions:receipt.decisions.filter(row=>row.file!=='experiments/original-link-complete-table-projection/raw-column-loan-styles.mjs')
      .map(row=>row.file===oldAuthor.file?{file:row.file,bytes:oldAuthor.bytes,sha256:oldAuthor.sha256}:row)};
  delete parameterReceipt.columnLoans;
  assert.deepEqual(parameterReceipt.cloneCalls,{before:746,after:396});
  assert.equal(parameterReceipt.styles.length,59);
  assert.equal(parameters.source,columns.actions.toReversed().reduce((value,action)=>value.replace(action.to,()=>action.from),source));
  const inverse={...parameterReceipt,predecessor:previous.predecessor};
  assert.deepEqual(inverse,previous);
  assert.deepEqual(Buffer.from(structuredDataToLino(inverse)),previousBytes);
});

test('raw immutable parameter loans reject changed predecessor bytes and actual mechanical decision sources', () => {
  const stored=readProjectData(ORIGINAL_LINK_RAW_CONSUMING_ARTIFACT);
  const predecessor=readOriginalLinkRawConsumingProfile(stored.predecessor);
  for(const file of [ORIGINAL_LINK_RAW_CONSUMING_ARTIFACT,new URL('../../js/scripts/portable-source-library-style.mjs',import.meta.url)]) {
    assert.throws(()=>prepareOriginalLinkRawBorrowedEmission(predecessor,{read:path=>{
      const bytes=readFileSync(path);return String(path)===String(file)?Buffer.concat([bytes,Buffer.from('\n')]):bytes;
    }}),/complete consuming predecessor bytes changed|project data is stale/u);
  }
  const changed=structuredClone(predecessor);changed.sourceSha256='0'.repeat(64);
  assert.throws(()=>prepareOriginalLinkRawBorrowedEmission(changed),assert.AssertionError);
});

test('raw loan native planning remains CI-only and cannot launch a detached checked plan', async () => {
  await assert.rejects(planOriginalLinkRawBorrowedNative({},{}),/CI/u);
  await assert.rejects(planOriginalLinkRawBorrowedNative({},{CI:'true',ORIGINAL_LINK_RAW_NATIVE:'1'}),/exact checked consuming plan/u);
});

import {verifyGitArtifactCertificate} from '../../js/scripts/git-object-certificate.mjs';
