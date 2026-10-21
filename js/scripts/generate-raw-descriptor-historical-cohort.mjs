// Normal bounded capture; the complete old body stays in its existing archive.
import assert from 'node:assert/strict';
import {readFileSync,writeFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {captureGitArtifactCertificate} from './capture-git-artifact-certificate.mjs';
import {checkProjectData,writeProjectData} from './project-data.mjs';
import {RAW_DESCRIPTOR_HISTORICAL_FILE,RAW_DESCRIPTOR_HISTORICAL_OUTPUT,RAW_DESCRIPTOR_HISTORICAL_REVISION,
  RAW_DESCRIPTOR_HISTORICAL_ARCHIVE,RAW_DESCRIPTOR_HISTORICAL_ARCHIVE_PIN,RAW_DESCRIPTOR_HISTORICAL_PREVIOUS,
  RAW_DESCRIPTOR_HISTORICAL_CURRENT,RAW_DESCRIPTOR_BORROWED_FILE,RAW_DESCRIPTOR_BORROWED_PREVIOUS,RAW_DESCRIPTOR_BORROWED_CURRENT,
  RAW_DESCRIPTOR_CONTINUATION_ARCHIVE,RAW_DESCRIPTOR_CONTINUATION_ARCHIVE_PIN,readRawDescriptorBorrowedAuthority,deriveRawDescriptorBorrowedChange,
  pinRawDescriptorHistoricalSource,readRawDescriptorHistoricalAuthority,
  deriveRawDescriptorHistoricalChanges,verifyRawDescriptorHistoricalCohort,restoreRawDescriptorCohortSource} from './raw-descriptor-historical-cohort.mjs';
const root=new URL('../../',import.meta.url);
export function generateRawDescriptorHistoricalCohort({check=false}={}){
  const old=readRawDescriptorHistoricalAuthority(),{blob,certificate}=captureGitArtifactCertificate(RAW_DESCRIPTOR_HISTORICAL_REVISION,RAW_DESCRIPTOR_HISTORICAL_FILE);
  const current=blob.toString('utf8');assert.deepEqual(Buffer.from(current),blob);
  assert.deepEqual(pinRawDescriptorHistoricalSource(blob),RAW_DESCRIPTOR_HISTORICAL_CURRENT);
  assert.deepEqual(Buffer.from(restoreRawDescriptorCohortSource(readFileSync(new URL(RAW_DESCRIPTOR_HISTORICAL_FILE,root),'utf8'),RAW_DESCRIPTOR_HISTORICAL_FILE)),blob,'unknown live descriptors require another reviewed source epoch');
  const borrowed=captureGitArtifactCertificate(RAW_DESCRIPTOR_HISTORICAL_REVISION,RAW_DESCRIPTOR_BORROWED_FILE),borrowedCurrent=borrowed.blob.toString('utf8');
  assert.deepEqual(Buffer.from(borrowedCurrent),borrowed.blob);assert.deepEqual(pinRawDescriptorHistoricalSource(borrowed.blob),RAW_DESCRIPTOR_BORROWED_CURRENT);
  assert.deepEqual(Buffer.from(restoreRawDescriptorCohortSource(readFileSync(new URL(RAW_DESCRIPTOR_BORROWED_FILE,root),'utf8'),RAW_DESCRIPTOR_BORROWED_FILE)),borrowed.blob,'unknown borrowed receipt successors refuse');
  const borrowedPrevious=readRawDescriptorBorrowedAuthority().source;
  const packet={schemaVersion:1,model:'raw descriptor historical cohort 1',file:RAW_DESCRIPTOR_HISTORICAL_FILE,revision:RAW_DESCRIPTOR_HISTORICAL_REVISION,
    archive:{file:RAW_DESCRIPTOR_HISTORICAL_ARCHIVE,...RAW_DESCRIPTOR_HISTORICAL_ARCHIVE_PIN,identity:'raw consuming'},
    previous:{revision:old.commit,...RAW_DESCRIPTOR_HISTORICAL_PREVIOUS},current:{...RAW_DESCRIPTOR_HISTORICAL_CURRENT,certificate},
    edits:deriveRawDescriptorHistoricalChanges(old.sourceText,current),
    borrowed:{file:RAW_DESCRIPTOR_BORROWED_FILE,
      archive:{file:RAW_DESCRIPTOR_CONTINUATION_ARCHIVE,...RAW_DESCRIPTOR_CONTINUATION_ARCHIVE_PIN,source:RAW_DESCRIPTOR_BORROWED_FILE,profile:'relative array'},
      previous:{...RAW_DESCRIPTOR_BORROWED_PREVIOUS},current:{...RAW_DESCRIPTOR_BORROWED_CURRENT,certificate:borrowed.certificate},
      edit:deriveRawDescriptorBorrowedChange(borrowedPrevious,borrowedCurrent)},
    boundary:'Complete committed descriptor metadata only; unchanged definitions, source identities and historical gates. This is not native execution or semantic equivalence.'};
  verifyRawDescriptorHistoricalCohort(packet);
  const output=new URL(RAW_DESCRIPTOR_HISTORICAL_OUTPUT,root);if(check)checkProjectData(output,packet);else writeProjectData(output,packet);
  const helper=new URL('js/scripts/raw-descriptor-historical-cohort.mjs',root),source=readFileSync(helper,'utf8');
  const pattern=/^const packetSha256='(?:PENDING|[a-f0-9]{64})';$/mu;assert.equal(source.split(pattern).length,2);
  const expected=source.replace(pattern,"const packetSha256='"+pinRawDescriptorHistoricalSource(readFileSync(output)).sha256+"';");
  if(check)assert.equal(source,expected);else writeFileSync(helper,expected);return packet;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.slice(2).every(value=>value==='--check'));
  generateRawDescriptorHistoricalCohort({check:process.argv.includes('--check')});
  console.log('Authenticated one whole current raw descriptor and two exact historical views without duplicating its predecessor body.');
}
