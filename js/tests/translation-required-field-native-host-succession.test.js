// Source-only controls supplement every unchanged historical reader assertion.
import assert from 'node:assert/strict';
import test from 'node:test';
import {readFileSync} from 'node:fs';
import {readProjectData} from '../scripts/project-data.mjs';
import {generateJavaScriptMutableArrayBindings} from '../scripts/generate-javascript-mutable-array-bindings.mjs';
import {restoreProgramStringHostRegistrationSource} from '../scripts/program-string-host-registration-history.mjs';
import {readCollectionSizeProjectionSchemaPacket,deriveCollectionSizeProjectionValidationSource,verifyCollectionSizeProjectionSchemaPacket} from '../scripts/collection-size-projection-schema.mjs';
import {generateRequiredFieldNativeHostSuccession} from '../scripts/generate-required-field-native-host-succession.mjs';
import {REQUIRED_FIELD_NATIVE_HOST_OUTPUT,REQUIRED_FIELD_NATIVE_HOST_SOURCES,pinRequiredFieldNativeHostSource,
  verifyRequiredFieldNativeHostSuccession,restoreRequiredFieldNativeHostSource} from '../scripts/required-field-native-host-succession.mjs';
const root=new URL('../../',import.meta.url);
const packet=()=>readProjectData(new URL(REQUIRED_FIELD_NATIVE_HOST_OUTPUT,root));
test('active Node selects the bounded native-host source assertion range',()=>{
  const[major,minor]=process.versions.node.split('.').map(Number);
  assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; supported >=22.15.0 <23 or >=24 <25');
});
test('all three actual current native hosts retain exact a8 source inverses and original independent packet bytes',()=>{
  const value=verifyRequiredFieldNativeHostSuccession(packet());assert.equal(value.sources.length,5);
  const originalSources=value.sources.slice(0,3);assert.equal(originalSources.length,3);
  for(const[index,row]of originalSources.entries()){
    const current=readFileSync(new URL(row.file,root),'utf8');assert.equal(current,row.current.source);
    for(const change of REQUIRED_FIELD_NATIVE_HOST_SOURCES[index].changes)assert.equal(current.split(change.after).length,2);
    const reader=index===2?'ASCII character host':'program string registration';
    assert.equal(restoreRequiredFieldNativeHostSource(current,row.file,reader),row.previous.source);
    assert.equal(restoreRequiredFieldNativeHostSource(row.previous.source,row.file,reader),row.previous.source);
  }
  const modules=readFileSync(new URL('rust/src/translation/mod.rs',root),'utf8');
  const tests=readFileSync(new URL('rust/tests/unit/mod.rs',root),'utf8');
  assert.equal(modules.split('pub(crate) mod javascript_record_parameter_rules;\n').length,2);
  assert.equal(tests.split('mod translation_javascript_record_parameters;\n').length,2);
  assert.ok(readFileSync(new URL('rust/src/translation/javascript_record_parameter_rules.rs',root)).length>0);
  assert.ok(readFileSync(new URL('rust/tests/unit/translation_javascript_record_parameters.rs',root)).length>0);
});
test('unknown source successors paths and reader scopes never acquire an old host identity',()=>{
  for(const[index,row]of packet().sources.entries()){
    const reader=index===2?'ASCII character host':index>=3?'mutable array binding host':'program string registration';
    for(const source of[row.current.source+'\n',row.current.source.replace(row.edits[0].inserted,row.edits[0].inserted.split('\n').toReversed().join('\n')),row.current.source.replace(row.edits[0].inserted,row.edits[0].inserted+'// unrelated\n')]){
      assert.equal(restoreRequiredFieldNativeHostSource(source,row.file,reader),source);
    }
    for(const scope of['unknown reader',index===2?'program string registration':'ASCII character host'])assert.equal(restoreRequiredFieldNativeHostSource(row.current.source,row.file,scope),row.current.source);
    assert.equal(restoreRequiredFieldNativeHostSource(row.current.source,'unregistered.rs',reader),row.current.source);
  }
});
test('coherent repins certificate substitution lost owners and extra permissions refuse',()=>{
  for(const change of[
    value=>{value.revision='0'.repeat(40);},
    value=>{value.sources.pop();},value=>{value.sources.splice(2,1);},value=>{value.sources.push(structuredClone(value.sources[0]));},value=>{value.sources.reverse();},
    value=>{value.sources[0].previous.certificate.commitText+='\n';},
    value=>{value.sources[0].previous.source+='\n';Object.assign(value.sources[0].previous,pinRequiredFieldNativeHostSource(value.sources[0].previous.source));},
    value=>{value.sources[2].current.source+='\n';Object.assign(value.sources[2].current,pinRequiredFieldNativeHostSource(value.sources[2].current.source));},
    value=>{value.sources[2].edits[0].inserted+='// unrelated\n';},
    value=>{value.sources[0].permission='arbitrary future registrations';},
    value=>{value.extraFiles=['unknown.rs'];},
  ]){const value=packet();change(value);assert.throws(()=>verifyRequiredFieldNativeHostSuccession(value));}
});
test('the normal author reproduces the exact packet while both old frozen packets remain unchanged',()=>{
  const files=['parity/self-translation/program-string-host-registration-history.lino','parity/self-translation/ascii-text-predicate-hosts.lino'];
  const before=files.map(file=>readFileSync(new URL(file,root)));
  assert.deepEqual(generateRequiredFieldNativeHostSuccession({check:true}),packet());
  files.forEach((file,index)=>assert.deepEqual(readFileSync(new URL(file,root)),before[index]));
});


test('the exact current collection validator keeps its certified program-string source view without admitting unknown successors',()=>{
  const file='js/src/typed-program-links-validation.js';
  const value=readCollectionSizeProjectionSchemaPacket(),owner=value.owners.find(row=>row.file===file);assert.ok(owner);
  const source=readFileSync(new URL(file,root),'utf8');
  assert.equal(source,deriveCollectionSizeProjectionValidationSource(owner.source));
  assert.deepEqual(pinRequiredFieldNativeHostSource(source),owner.current);assert.notEqual(source,owner.source);
  assert.equal(restoreProgramStringHostRegistrationSource(source,file),owner.source);
  assert.equal(restoreProgramStringHostRegistrationSource(owner.source,file),owner.source);
  for(const changed of[source+'\n',source.replace('acceptProgramExpressionField','unknownExpressionField')]){
    assert.notEqual(changed,source);assert.equal(restoreProgramStringHostRegistrationSource(changed,file),changed);
  }
  assert.equal(restoreProgramStringHostRegistrationSource(source,'unregistered.js'),source);
  for(const change of[
    packet=>{const row=packet.owners.find(row=>row.file===file);row.source+='\n';Object.assign(row,pinRequiredFieldNativeHostSource(row.source));},
    packet=>{packet.owners.find(row=>row.file===file).current.sha256='0'.repeat(64);},
  ]){const changed=structuredClone(value);change(changed);assert.throws(()=>verifyCollectionSizeProjectionSchemaPacket(changed));}
});


test('both mutable-array source hosts keep the actual required-field transport and certified old six-host replay',()=>{
  const value=verifyRequiredFieldNativeHostSuccession(packet());assert.equal(value.sources.length,5);
  const sources=value.sources.slice(3);assert.equal(sources.length,2);
  for(const row of sources){
    const actual=readFileSync(new URL(row.file,root),'utf8');assert.equal(actual,row.current.source);
    assert.equal(restoreRequiredFieldNativeHostSource(actual,row.file,'mutable array binding host'),row.previous.source);
    assert.notEqual(actual,row.previous.source);assert.equal(row.edits.length,1);
  }
  const file='parity/self-translation/javascript-mutable-array-binding-hosts.lino',before=readFileSync(new URL(file,root));
  assert.equal(pinRequiredFieldNativeHostSource(before).sha256,'bb2f5ca4e690014ed0a08c984854b39e13cbc7a8a7dbc9be8f7d292b30a21e7b');
  const archive=generateJavaScriptMutableArrayBindings({check:true});assert.equal(archive.hosts.length,6);
  assert.deepEqual(readFileSync(new URL(file,root)),before,'the complete old six-host packet stays byte exact');
});
