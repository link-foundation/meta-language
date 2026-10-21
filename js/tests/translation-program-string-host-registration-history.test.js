// These controls supplement every original program-string assertion/input.
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';
import {generateProgramStringHostRegistrationHistory} from '../scripts/generate-program-string-host-registration-history.mjs';
import {PROGRAM_STRING_REGISTRATION_REVISION,PROGRAM_STRING_HOST_REVISION,PROGRAM_STRING_HOST_ARCHIVE,
  PROGRAM_STRING_REGISTRATION_CHANGES,PROGRAM_STRING_REGISTRATIONS,programStringHostRegistrationPin,
  replayProgramStringRegistrationChanges,verifyProgramStringHostRegistrationHistory,
  restoreProgramStringHostRegistrationSource} from '../scripts/program-string-host-registration-history.mjs';
const root=new URL('../../',import.meta.url);
const policy=()=>readProjectData(new URL('parity/self-translation/program-string-host-registration-history.lino',root));

test('active Node selects the explicit program-string registration source assertion range',()=>{
  const[major,minor]=process.versions.node.split('.').map(Number);
  assert.ok(major===22&&minor>=15||major===24,
    'uncovered Node '+process.versions.node+'; supported >=22.15.0 <23.0.0 || >=24.0.0 <25.0.0');
});

test('two complete certified current roots recover every original proposal byte while five registrations remain current',()=>{
  const packet=policy(),pairs=verifyProgramStringHostRegistrationHistory(packet);
  assert.equal(packet.revision,PROGRAM_STRING_REGISTRATION_REVISION);assert.equal(packet.hostRevision,PROGRAM_STRING_HOST_REVISION);
  assert.equal(pairs.size,2);assert.equal(packet.registrations.length,5);
  const bytes=readFileSync(new URL(PROGRAM_STRING_HOST_ARCHIVE.path,root));
  assert.deepEqual(programStringHostRegistrationPin(bytes),{bytes:PROGRAM_STRING_HOST_ARCHIVE.bytes,sha256:PROGRAM_STRING_HOST_ARCHIVE.sha256});
  const original=readProjectData(new URL(PROGRAM_STRING_HOST_ARCHIVE.path,root));
  assert.equal(original.sources.length,9);
  for(const specification of PROGRAM_STRING_REGISTRATION_CHANGES){
    const pair=pairs.get(specification.path),row=original.sources.find(row=>row.file===specification.path);
    assert.equal(pair.previous,row.proposed.source);
    assert.equal(pair.current,readFileSync(new URL(specification.path,root),'utf8'));
    assert.equal(restoreProgramStringHostRegistrationSource(pair.current,specification.path),row.proposed.source);
    assert.equal(replayProgramStringRegistrationChanges(row.sourceText,row.actions).source,row.proposed.source);
  }
  for(const registration of PROGRAM_STRING_REGISTRATIONS){
    assert.equal(pairs.get(registration.owner).current.split(registration.declaration+'\n').length,2);
    assert.ok(readFileSync(new URL(registration.path,root)).length>0);
  }
  for(const row of original.sources.filter(row=>!pairs.has(row.file))){
    assert.equal(readFileSync(new URL(row.file,root),'utf8'),row.proposed.source,'the seven other complete host proposals remain exact');
  }
  assert.deepEqual(readFileSync(new URL(PROGRAM_STRING_HOST_ARCHIVE.path,root)),bytes,'the historical artifact is unchanged');
});

test('unknown complete roots wrong paths changed declaration order and newline successors stay visible to the original strict gate',()=>{
  const packet=policy(),old=readProjectData(new URL(PROGRAM_STRING_HOST_ARCHIVE.path,root));
  for(const row of packet.sources){
    const historical=old.sources.find(owner=>owner.file===row.path);
    const other=packet.sources.find(owner=>owner.path!==row.path);
    for(const source of [row.current.source+'\n',row.current.source.replace('mod ','mod /* unknown */ '),
      row.current.source.replace(row.edits[0].inserted,row.edits[0].inserted.split('\n').toReversed().join('\n'))]){
      assert.equal(restoreProgramStringHostRegistrationSource(source,row.path),source);
      assert.ok(source!==historical.sourceText&&source!==historical.proposed.source,'the unchanged old host assertion still refuses unknown source');
    }
    assert.equal(restoreProgramStringHostRegistrationSource(row.current.source,other.path),row.current.source);
    assert.equal(restoreProgramStringHostRegistrationSource(row.current.source,'unregistered.rs'),row.current.source);
    for(const source of [historical.sourceText,historical.proposed.source]){
      assert.equal(restoreProgramStringHostRegistrationSource(source,row.path),source,'both original historical input branches remain intact');
    }
  }
});

test('coherent caller repins missing or duplicate owners forged certificates and extra source permissions refuse',()=>{
  const changes=[
    packet=>{packet.schemaVersion=2;},
    packet=>{packet.revision=PROGRAM_STRING_HOST_REVISION;},
    packet=>{packet.hostArchive.sha256='0'.repeat(64);},
    packet=>{packet.hostArchive.certificate.revision=PROGRAM_STRING_HOST_REVISION;},
    packet=>{packet.producer.source+='\n';Object.assign(packet.producer,programStringHostRegistrationPin(packet.producer.source));},
    packet=>{packet.producer.edits[0].inserted+='\n';},
    packet=>{packet.sources.pop();},
    packet=>{packet.sources.push(structuredClone(packet.sources[0]));},
    packet=>{packet.sources.reverse();},
    packet=>{packet.sources[0].current.source+='\n';Object.assign(packet.sources[0].current,programStringHostRegistrationPin(packet.sources[0].current.source));},
    packet=>{packet.sources[0].predecessor.source+='\n';Object.assign(packet.sources[0].predecessor,programStringHostRegistrationPin(packet.sources[0].predecessor.source));},
    packet=>{packet.sources[0].edits[0].inserted=packet.sources[0].edits[0].inserted.replace('ascii_text_predicate_rules','unknown_module');},
    packet=>{packet.sources[0].current.certificate.path=packet.sources[1].path;},
    packet=>{packet.sources[0].permission='arbitrary future edits';},
    packet=>{packet.registrations.pop();},
    packet=>{packet.registrations[0].owner=packet.sources[1].path;},
    packet=>{packet.registrations[0].path='rust/src/translation/ir.rs';},
    packet=>{packet.registrations[0].declaration+=' ';},
    packet=>{packet.registrations[0].certificate.blob.sha256='0'.repeat(64);},
    packet=>{packet.extraFileAllowance=['future.rs'];},
  ];
  for(const[index,change]of changes.entries()){
    const packet=policy();change(packet);
    assert.throws(()=>verifyProgramStringHostRegistrationHistory(packet),assert.AssertionError,'tamper control '+index);
  }
});

test('normal capture reproduces the exact source succession and leaves the complete old host artifact untouched',()=>{
  const before=readFileSync(new URL(PROGRAM_STRING_HOST_ARCHIVE.path,root));
  const packet=generateProgramStringHostRegistrationHistory({check:true});
  assert.deepEqual(packet,policy());
  assert.deepEqual(readFileSync(new URL(PROGRAM_STRING_HOST_ARCHIVE.path,root)),before);
});
