import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import test from 'node:test';
import {generateBooleanDeclarationValueHosts,verifyBooleanDeclarationValueHosts} from '../scripts/generate-boolean-declaration-value-hosts.mjs';
const digest=value=>createHash('sha256').update(value).digest('hex');

test('direct Boolean native host replay retains every certified predecessor and reviewed successor byte',()=>{
  const {archive,files}=generateBooleanDeclarationValueHosts({check:true});
  assert.equal(files.length,3);
  assert.deepEqual(files.map(row=>row.path),[
    'rust/src/formal_concept_definition.rs','rust/src/formal_function_links.rs','rust/src/shared_numeric_function_model.rs',
  ]);
  assert.deepEqual(verifyBooleanDeclarationValueHosts(structuredClone(archive)),files);
  for(const [index,file]of files.entries()){
    assert.equal(digest(file.source),archive.hosts[index].currentSha256);
    assert.equal(typeof file.predecessor,'string');
    assert.notEqual(file.source,file.predecessor);
  }
});

test('host source certificates coordinate units owner paths and coherent edit forgeries refuse',()=>{
  const {archive}=generateBooleanDeclarationValueHosts({check:true});
  const cases=[
    value=>{value.coordinateUnit='UTF-8 bytes';},
    value=>{value.revision='0'.repeat(40);},
    value=>{value.hosts.reverse();},
    value=>{value.hosts[0].path='rust/src/lib.rs';},
    value=>{value.decisionSource.sha256='0'.repeat(64);},
    value=>{value.registrations=[];},
    value=>{value.hosts[0].source+='\n';},
    value=>{value.hosts[0].edits[0].start+=1;},
    value=>{value.hosts[0].edits[0].inserted+='\n';},
    value=>{
      const row=value.hosts[0];row.edits[0].inserted+='\n// a coherent unreviewed body successor\n';
      let source=row.source;
      for(const edit of row.edits)source=source.slice(0,edit.start)+edit.inserted+source.slice(edit.start+edit.removed.length);
      row.currentBytes=Buffer.byteLength(source);row.currentSha256=digest(source);
    },
  ];
  for(const change of cases){const forged=structuredClone(archive);change(forged);assert.throws(()=>verifyBooleanDeclarationValueHosts(forged));}
});
