// A finite source view for two registered module roots. No runtime semantics
// or historical host proposal is replaced by this current registration proof.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {readBootstrapStructuredDataLino} from '../../scripts/lib/structured-data-bootstrap.mjs';
import {verifyGitArtifactCertificate} from './git-object-certificate.mjs';
import {restoreRequiredFieldNativeHostSource} from './required-field-native-host-succession.mjs';
import {readCollectionSizeProjectionSchemaPacket,deriveCollectionSizeProjectionValidationSource} from './collection-size-projection-schema.mjs';

const root=new URL('../../',import.meta.url);
export const PROGRAM_STRING_REGISTRATION_REVISION='7881e1e6f83818e0b25b77e1b3ab15c2d51ca9f3';
export const PROGRAM_STRING_HOST_REVISION='90636f9fb52b1b9095defbac041c8089608b8576';
export const PROGRAM_STRING_HOST_ARCHIVE={path:'parity/self-translation/program-string-representation-hosts.lino',
  bytes:2262508,sha256:'dcc181b0ed3d71309f20745f1947a74dcda687d7dc72abf06cca1cc64b3a659f'};
export const PROGRAM_STRING_HOST_PRODUCER={path:'experiments/program-string-representation/generate.mjs',
  bytes:8105,sha256:'6da40c0f9f0fd7df3bdc40ae4b9729d975399a532f5b279d0c9271a588f87db0'};
export const PROGRAM_STRING_REGISTRATION_CHANGES=[
  {path:'rust/src/translation/mod.rs',current:{bytes:4848,sha256:'645139f6b295565bf2ba30c6bb3607c1db16a3aef8ce38c8e0c8949433358e1b'},changes:[
    {before:'pub(crate) mod array_value_rules;\n',after:'pub(crate) mod array_value_rules;\npub(crate) mod ascii_text_predicate_rules;\npub(crate) mod boolean_declaration_value_rules;\n'},
    {before:'pub(crate) mod rust_diagnostic_attribute_rules;\n',after:'pub(crate) mod rust_diagnostic_attribute_rules;\npub(crate) mod rust_literal_match_rules;\n'},
  ]},
  {path:'rust/tests/unit/mod.rs',current:{bytes:10341,sha256:'86024307a1da5a844834adadf6d4d8ce724a766866e647bd9f7565ab1faa4c3d'},changes:[
    {before:'mod translation_arrays;\n',after:'mod translation_arrays;\nmod translation_ascii_text_predicates;\n'},
    {before:'mod translation_rust_lifetimes;\n',after:'mod translation_rust_lifetimes;\nmod translation_rust_literal_matches;\n'},
  ]},
];
export const PROGRAM_STRING_REGISTRATIONS=[
  {owner:'rust/src/translation/mod.rs',declaration:'pub(crate) mod ascii_text_predicate_rules;',path:'rust/src/translation/ascii_text_predicate_rules.rs'},
  {owner:'rust/src/translation/mod.rs',declaration:'pub(crate) mod boolean_declaration_value_rules;',path:'rust/src/translation/boolean_declaration_value_rules.rs'},
  {owner:'rust/src/translation/mod.rs',declaration:'pub(crate) mod rust_literal_match_rules;',path:'rust/src/translation/rust_literal_match_rules.rs'},
  {owner:'rust/tests/unit/mod.rs',declaration:'mod translation_ascii_text_predicates;',path:'rust/tests/unit/translation_ascii_text_predicates.rs'},
  {owner:'rust/tests/unit/mod.rs',declaration:'mod translation_rust_literal_matches;',path:'rust/tests/unit/translation_rust_literal_matches.rs'},
];
export const PROGRAM_STRING_HOST_PRODUCER_CHANGES=[
  {before:"import {verifyGitArtifactCertificate} from '../../js/scripts/git-object-certificate.mjs';\n",
    after:"import {verifyGitArtifactCertificate} from '../../js/scripts/git-object-certificate.mjs';\nimport {restoreProgramStringHostRegistrationSource} from '../../js/scripts/program-string-host-registration-history.mjs';\n"},
  {before:"    const actual=readFileSync(new URL(file,root),'utf8');assert.ok(actual===sourceText||actual===source,file+': unknown host edits refuse');\n",
    after:"    const actual=restoreProgramStringHostRegistrationSource(readFileSync(new URL(file,root),'utf8'),file);assert.ok(actual===sourceText||actual===source,file+': unknown host edits refuse');\n"},
];

const policyPath='parity/self-translation/program-string-host-registration-history.lino';
const policySha256='ba99ac1be8c218a374b5f014d666f6985b7bd706a0b395b28f7aecaa30ee6f0f';
let cachedArchive,cachedPolicy;
export const programStringHostRegistrationPin=value=>({bytes:Buffer.byteLength(value),sha256:createHash('sha256').update(value).digest('hex')});
const exactFields=(record,fields)=>assert.deepEqual(Object.keys(record).sort(),[...fields].sort(),'the complete registration proof layout is exact');

export function replayProgramStringRegistrationChanges(source,changes){
  let current=source;const edits=[];
  for(const change of changes){
    assert.equal(current.split(change.before).length,2,'one exact reviewed registration/source interval');
    const start=current.indexOf(change.before);
    edits.push({start,removed:change.before,inserted:change.after});
    current=current.slice(0,start)+change.after+current.slice(start+change.before.length);
  }
  let inverse=current;
  for(const edit of edits.toReversed()){
    assert.equal(inverse.slice(edit.start,edit.start+edit.inserted.length),edit.inserted);
    inverse=inverse.slice(0,edit.start)+edit.removed+inverse.slice(edit.start+edit.inserted.length);
  }
  assert.equal(inverse,source,'every complete predecessor byte remains exact');
  return{source:current,edits};
}

function originalHosts(){
  if(!cachedArchive){
    const bytes=readFileSync(new URL(PROGRAM_STRING_HOST_ARCHIVE.path,root));
    assert.deepEqual(programStringHostRegistrationPin(bytes),{bytes:PROGRAM_STRING_HOST_ARCHIVE.bytes,sha256:PROGRAM_STRING_HOST_ARCHIVE.sha256},'the frozen nine-host artifact is unchanged');
    cachedArchive={bytes,value:readBootstrapStructuredDataLino(bytes.toString('utf8'))};
  }
  return cachedArchive;
}

/** Authenticate complete current files, then invert only the declared literal
 * registrations. Frozen host definitions, actions and assertions stay exact. */
export function verifyProgramStringHostRegistrationHistory(packet){
  exactFields(packet,['schemaVersion','model','revision','hostRevision','coordinateUnit','hostArchive','producer','sources','registrations','boundary']);
  assert.equal(packet.schemaVersion,1);assert.equal(packet.model,'program string host registration succession 1');
  assert.equal(packet.revision,PROGRAM_STRING_REGISTRATION_REVISION);assert.equal(packet.hostRevision,PROGRAM_STRING_HOST_REVISION);
  assert.equal(packet.coordinateUnit,'UTF-16 code units');
  exactFields(packet.hostArchive,['path','bytes','sha256','certificate']);
  assert.deepEqual({path:packet.hostArchive.path,bytes:packet.hostArchive.bytes,sha256:packet.hostArchive.sha256},PROGRAM_STRING_HOST_ARCHIVE);
  const archived=originalHosts();
  verifyGitArtifactCertificate(packet.hostArchive.certificate,archived.bytes,{revision:PROGRAM_STRING_REGISTRATION_REVISION,path:PROGRAM_STRING_HOST_ARCHIVE.path});
  assert.equal(archived.value.revision,PROGRAM_STRING_HOST_REVISION);assert.equal(archived.value.sources.length,9);
  exactFields(packet.producer,['path','source','bytes','sha256','certificate','current','edits']);
  assert.deepEqual({path:packet.producer.path,bytes:packet.producer.bytes,sha256:packet.producer.sha256},PROGRAM_STRING_HOST_PRODUCER);
  assert.deepEqual(programStringHostRegistrationPin(packet.producer.source),{bytes:PROGRAM_STRING_HOST_PRODUCER.bytes,sha256:PROGRAM_STRING_HOST_PRODUCER.sha256});
  verifyGitArtifactCertificate(packet.producer.certificate,Buffer.from(packet.producer.source),{revision:PROGRAM_STRING_REGISTRATION_REVISION,path:PROGRAM_STRING_HOST_PRODUCER.path});
  const producer=replayProgramStringRegistrationChanges(packet.producer.source,PROGRAM_STRING_HOST_PRODUCER_CHANGES);
  assert.deepEqual(packet.producer.edits,producer.edits);assert.deepEqual(packet.producer.current,programStringHostRegistrationPin(producer.source));
  assert.equal(readFileSync(new URL(PROGRAM_STRING_HOST_PRODUCER.path,root),'utf8'),producer.source,'the actual generator is exactly its two finite source-view edits');
  assert.deepEqual(packet.sources.map(row=>row.path),PROGRAM_STRING_REGISTRATION_CHANGES.map(row=>row.path));
  const result=new Map();
  for(const[index,row]of packet.sources.entries()){
    exactFields(row,['path','predecessor','current','edits']);
    exactFields(row.predecessor,['source','bytes','sha256']);exactFields(row.current,['source','bytes','sha256','certificate']);
    const specification=PROGRAM_STRING_REGISTRATION_CHANGES[index];
    const original=archived.value.sources.find(owner=>owner.file===row.path);assert.ok(original);
    assert.equal(original.commit,PROGRAM_STRING_HOST_REVISION);
    verifyGitArtifactCertificate(original.certificate,Buffer.from(original.sourceText),{revision:PROGRAM_STRING_HOST_REVISION,path:row.path});
    const originalReplay=replayProgramStringRegistrationChanges(original.sourceText,original.actions);
    assert.equal(originalReplay.source,original.proposed.source);
    assert.deepEqual(programStringHostRegistrationPin(originalReplay.source),{bytes:original.proposed.bytes,sha256:original.proposed.sha256});
    assert.equal(row.predecessor.source,original.proposed.source);
    assert.deepEqual(programStringHostRegistrationPin(row.predecessor.source),{bytes:row.predecessor.bytes,sha256:row.predecessor.sha256});
    const replay=replayProgramStringRegistrationChanges(row.predecessor.source,specification.changes);
    assert.deepEqual(row.edits,replay.edits);assert.equal(row.current.source,replay.source);
    assert.deepEqual(programStringHostRegistrationPin(row.current.source),specification.current);
    assert.deepEqual({bytes:row.current.bytes,sha256:row.current.sha256},specification.current);
    verifyGitArtifactCertificate(row.current.certificate,Buffer.from(row.current.source),{revision:PROGRAM_STRING_REGISTRATION_REVISION,path:row.path});
    assert.equal(restoreRequiredFieldNativeHostSource(readFileSync(new URL(row.path,root),'utf8'),row.path,'program string registration'),row.current.source,'the actual current module root retains every authenticated registration and byte');
    result.set(row.path,{previous:row.predecessor.source,current:row.current.source});
  }
  assert.equal(packet.registrations.length,PROGRAM_STRING_REGISTRATIONS.length);
  for(const[index,row]of packet.registrations.entries()){
    exactFields(row,['owner','declaration','path','bytes','sha256','certificate']);
    assert.deepEqual({owner:row.owner,declaration:row.declaration,path:row.path},PROGRAM_STRING_REGISTRATIONS[index]);
    assert.equal(result.get(row.owner).current.split(row.declaration+'\n').length,2,'one actual owned module registration');
    const bytes=readFileSync(new URL(row.path,root));
    assert.deepEqual(programStringHostRegistrationPin(bytes),{bytes:row.bytes,sha256:row.sha256});
    verifyGitArtifactCertificate(row.certificate,bytes,{revision:PROGRAM_STRING_REGISTRATION_REVISION,path:row.path});
  }
  return result;
}

/** Unknown successors remain visible to the old strict host assertion. */
export function restoreProgramStringHostRegistrationSource(source,path){
  assert.equal(typeof source,'string');assert.equal(typeof path,'string');
  if(path==='js/src/typed-program-links-validation.js'){
    const packet=readCollectionSizeProjectionSchemaPacket(),owner=packet.owners.find(row=>row.file===path);assert.ok(owner);
    const current=deriveCollectionSizeProjectionValidationSource(owner.source);
    if(source!==current)return source;
    assert.deepEqual(programStringHostRegistrationPin(source),owner.current);return owner.source;
  }
  source=restoreRequiredFieldNativeHostSource(source,path,'program string registration');
  const selected=PROGRAM_STRING_REGISTRATION_CHANGES.find(row=>row.path===path);
  if(!selected||programStringHostRegistrationPin(source).sha256!==selected.current.sha256)return source;
  assert.deepEqual(programStringHostRegistrationPin(source),selected.current);
  if(!cachedPolicy){
    const bytes=readFileSync(new URL(policyPath,root));assert.equal(programStringHostRegistrationPin(bytes).sha256,policySha256,'the whole reviewed registration succession is pinned');
    const packet=readBootstrapStructuredDataLino(bytes.toString('utf8'));
    cachedPolicy=verifyProgramStringHostRegistrationHistory(packet);
  }
  const pair=cachedPolicy.get(path);assert.equal(source,pair.current);return pair.previous;
}
