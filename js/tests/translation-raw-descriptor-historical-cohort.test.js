import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {LinkNetwork} from '../src/network.js';
import {encodeStructuredDataLinks,decodeStructuredDataLinks,readStructuredDataLino} from '../src/structured-data-links.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {RAW_DESCRIPTOR_HISTORICAL_FILE,RAW_DESCRIPTOR_HISTORICAL_ARCHIVE,RAW_DESCRIPTOR_HISTORICAL_ARCHIVE_PIN,
  RAW_DESCRIPTOR_HISTORICAL_PREVIOUS,RAW_DESCRIPTOR_HISTORICAL_CURRENT,RAW_DESCRIPTOR_HISTORICAL_RELOCATION,
  RAW_DESCRIPTOR_BORROWED_FILE,RAW_DESCRIPTOR_BORROWED_PREVIOUS,RAW_DESCRIPTOR_BORROWED_CURRENT,
  RAW_DESCRIPTOR_CONTINUATION_ARCHIVE,RAW_DESCRIPTOR_CONTINUATION_ARCHIVE_PIN,readRawDescriptorBorrowedAuthority,
  pinRawDescriptorHistoricalSource,readRawDescriptorHistoricalAuthority,readRawDescriptorHistoricalCohort,
  verifyRawDescriptorHistoricalCohort,restoreRawDescriptorHistoricalSource,restoreRawDescriptorCohortSource} from '../scripts/raw-descriptor-historical-cohort.mjs';
import {readCurrentOwnCodebaseRawDescriptorPacket} from '../scripts/current-own-codebase-raw-source-descriptors.mjs';
import {generateRawDescriptorHistoricalCohort} from '../scripts/generate-raw-descriptor-historical-cohort.mjs';
const root=new URL('../../',import.meta.url),read=file=>readFileSync(new URL(file,root),'utf8');

test('the complete physical descriptor keeps both independently pinned historical identities and every native definition',()=>{
  const[major,minor]=process.versions.node.split('.').map(Number);
  assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; >=22.15 <23 || >=24 <25 required');
  const physical=read(RAW_DESCRIPTOR_HISTORICAL_FILE),current=readCurrentOwnCodebaseRawDescriptorPacket();
  assert.deepEqual(pinRawDescriptorHistoricalSource(physical),current.artifact.current);
  const source=restoreRawDescriptorCohortSource(physical,RAW_DESCRIPTOR_HISTORICAL_FILE),old=readRawDescriptorHistoricalAuthority();
  assert.deepEqual(pinRawDescriptorHistoricalSource(source),RAW_DESCRIPTOR_HISTORICAL_CURRENT);
  const previous=restoreRawDescriptorHistoricalSource(source,RAW_DESCRIPTOR_HISTORICAL_FILE,'relative array');
  const relocation=restoreRawDescriptorHistoricalSource(source,RAW_DESCRIPTOR_HISTORICAL_FILE,'native library relocation');
  assert.equal(previous,old.sourceText);assert.deepEqual(pinRawDescriptorHistoricalSource(previous),RAW_DESCRIPTOR_HISTORICAL_PREVIOUS);
  assert.deepEqual(pinRawDescriptorHistoricalSource(relocation),RAW_DESCRIPTOR_HISTORICAL_RELOCATION);
  const before=readStructuredDataLino(previous),actual=readStructuredDataLino(source),packaged=readStructuredDataLino(relocation),live=readStructuredDataLino(physical);
  for(const field of ['completeRust','owners','declarationNames','totals','assumptions','encodings','mappings','preludeHashes','replays','archive']){
    assert.deepEqual(actual[field],before[field],field);assert.deepEqual(packaged[field],before[field],field);
    assert.deepEqual(live[field],before[field],field);
  }
  for(const value of[before,actual,packaged])assert.equal(value.predecessor.proof.producerApplicability.sources[1],value.predecessor.proof.producerApplicability.sourceHistory.current);
  assert.equal(before.decisionSources[2].bytes,62588);assert.equal(actual.decisionSources[2].bytes,62902);
  assert.equal(live.decisionSources[2].bytes,current.sources[1].current.bytes);
  assert.equal(packaged.predecessor.proof.producerApplicability.sources[12].sha256,'36dbd7ffcf25a403deab81f489eb9c3ff5e69cbf7f084ea8d9c34a279b2ae626');
});

test('the frozen continuation reader receives exactly its two original whole successor inputs',()=>{
  const raw=read(RAW_DESCRIPTOR_HISTORICAL_FILE),physical=read(RAW_DESCRIPTOR_BORROWED_FILE),current=readCurrentOwnCodebaseRawDescriptorPacket();
  assert.deepEqual(pinRawDescriptorHistoricalSource(physical),current.loan.current);
  const borrowed=restoreRawDescriptorCohortSource(physical,RAW_DESCRIPTOR_BORROWED_FILE);
  assert.deepEqual(pinRawDescriptorHistoricalSource(restoreRawDescriptorHistoricalSource(raw,RAW_DESCRIPTOR_HISTORICAL_FILE,'continuation historical input')),RAW_DESCRIPTOR_HISTORICAL_RELOCATION);
  assert.deepEqual(pinRawDescriptorHistoricalSource(borrowed),RAW_DESCRIPTOR_BORROWED_CURRENT);
  const previous=restoreRawDescriptorHistoricalSource(borrowed,RAW_DESCRIPTOR_BORROWED_FILE,'continuation historical input');
  assert.deepEqual(pinRawDescriptorHistoricalSource(previous),RAW_DESCRIPTOR_BORROWED_PREVIOUS);
  assert.equal(previous,readRawDescriptorBorrowedAuthority().source);
  assert.equal(readStructuredDataLino(previous).predecessor.sha256,RAW_DESCRIPTOR_HISTORICAL_RELOCATION.sha256);
  assert.equal(readStructuredDataLino(borrowed).predecessor.sha256,RAW_DESCRIPTOR_HISTORICAL_CURRENT.sha256);
  for(const profile of['relative array','native library relocation','unregistered profile'])assert.equal(restoreRawDescriptorHistoricalSource(borrowed,RAW_DESCRIPTOR_BORROWED_FILE,profile),borrowed);
  for(const source of[borrowed+'\n',borrowed.slice(1),previous])assert.equal(restoreRawDescriptorHistoricalSource(source,RAW_DESCRIPTOR_BORROWED_FILE,'continuation historical input'),source);
  assert.equal(restoreRawDescriptorHistoricalSource(borrowed,'unregistered.lino','continuation historical input'),borrowed);
  assert.deepEqual(pinRawDescriptorHistoricalSource(read(RAW_DESCRIPTOR_CONTINUATION_ARCHIVE)),RAW_DESCRIPTOR_CONTINUATION_ARCHIVE_PIN);
});

test('unknown paths readers and coherent unrecorded source changes retain their original identity and refusals',()=>{
  const source=read(RAW_DESCRIPTOR_HISTORICAL_FILE),packet=readRawDescriptorHistoricalCohort();
  for(const file of['unregistered.lino','Experiments/original-link-complete-table-projection/data/raw-consuming-current-native-definitions.lino'])
    assert.equal(restoreRawDescriptorHistoricalSource(source,file,'relative array'),source);
  for(const profile of['','current source','unregistered profile'])assert.equal(restoreRawDescriptorHistoricalSource(source,RAW_DESCRIPTOR_HISTORICAL_FILE,profile),source);
  for(const changed of[source+'\n',source.slice(1),readRawDescriptorHistoricalAuthority().sourceText])
    assert.equal(restoreRawDescriptorHistoricalSource(changed,RAW_DESCRIPTOR_HISTORICAL_FILE,'relative array'),changed);
  for(const edit of[
    value=>{value.revision='0'.repeat(40);},value=>{value.archive.sha256='0'.repeat(64);},
    value=>{value.current.certificate.path='unregistered.lino';},value=>{value.edits[0].line+=1;},
    value=>{value.edits[0].after=value.edits[0].after.replace('62902','62903');},
    value=>{value.edits.pop();},value=>{value.extra='unrepresented';},
    value=>{value.borrowed.current.certificate.path=value.file;},value=>{value.borrowed.edit.line+=1;},
    value=>{value.borrowed.archive.sha256='0'.repeat(64);},value=>{value.borrowed.edit.after=value.borrowed.edit.before;},
  ]){const changed=structuredClone(packet);edit(changed);assert.throws(()=>verifyRawDescriptorHistoricalCohort(changed),assert.AssertionError);}
});

test('both Links codecs keep the exact source certificate and complete nine scalar inverse',()=>{
  const packet=readRawDescriptorHistoricalCohort(),saved=encodeStructuredDataLinks(packet);
  const canonical=LinkNetwork.fromLino(saved.network.toLino());
  for(const network of[canonical,decodeLinkNetworkBinary(encodeLinkNetworkBinary(canonical))]){
    const value=decodeStructuredDataLinks(network,saved.root);assert.deepEqual(value,packet);verifyRawDescriptorHistoricalCohort(value);
  }
});

test('the normal cohort author keeps the physical artifact and every existing archive byte unchanged',()=>{
  const files=[RAW_DESCRIPTOR_HISTORICAL_FILE,RAW_DESCRIPTOR_BORROWED_FILE,RAW_DESCRIPTOR_HISTORICAL_ARCHIVE,RAW_DESCRIPTOR_CONTINUATION_ARCHIVE];
  const before=files.map(read);generateRawDescriptorHistoricalCohort({check:true});
  files.forEach((file,index)=>assert.equal(read(file),before[index],file));
  assert.deepEqual(pinRawDescriptorHistoricalSource(read(RAW_DESCRIPTOR_HISTORICAL_ARCHIVE)),RAW_DESCRIPTOR_HISTORICAL_ARCHIVE_PIN);
});
