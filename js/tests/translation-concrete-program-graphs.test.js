import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import {createConcreteProgramLinks} from '../../experiments/concrete-program-graphs/projection.mjs';
import {generateConcreteProgramGraphFixtures} from '../scripts/generate-concrete-program-graph-fixtures.mjs';
import {readProjectData} from '../scripts/project-data.mjs';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {TYPED_PROGRAM_LINKS_SCHEMA,decodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';

function readConcreteProgramOrigin(network,root) {
  const schema=TYPED_PROGRAM_LINKS_SCHEMA,fields=network.fieldRelations();
  const fieldName=label=>Object.entries(schema.fields).find(([,name])=>name===label)?.[0]
    ??label.replace(/ ([a-z])/gu,(_,letter)=>letter.toUpperCase());
  const provenance=fields.find(row=>row.parent.equals(root)&&row.label==='provenance');
  assert.ok(provenance);
  function read(id){
    const link=network.link(id),metadata=link.metadata();
    if(metadata.term===schema.containers.array)return link.references().map(read);
    if(metadata.term===schema.containers.record)return Object.fromEntries(fields
      .filter(row=>row.parent.equals(id)).map(row=>[fieldName(row.label),read(row.child)]));
    if(metadata.term===schema.scalars.string)return metadata.definition;
    if(metadata.term===schema.scalars.number)return Number(metadata.definition);
    if(metadata.term===schema.scalars.boolean)return metadata.definition==='true';
    if(metadata.term===schema.scalars.null)return null;
    throw new Error('unknown linked origin scalar');
  }
  return read(provenance.child);
}

function concreteProgramSourceBytes(tree) {
  if(tree.type==='missing')return Buffer.alloc(0);
  if(tree.type==='token'||tree.type==='error')return tree.text===null?Buffer.from(tree.hex,'hex'):Buffer.from(tree.text);
  return Buffer.concat((tree.children??[tree.root]).map(concreteProgramSourceBytes));
}

function executeConcreteProgramJavaScript(code) {
  const lines=[];
  vm.runInNewContext(code,{console:{log:(...values)=>lines.push(values.map(String).join(' '))},process:{argv:[]}}, {timeout:1000});
  return lines;
}

test('JavaScript Rust and TypeScript graphs retain complete concrete source and checked behavior through both archives',()=>{
  const {fixture,profiles}=generateConcreteProgramGraphFixtures({check:true});
  for(const row of fixture.cases){
    const original=createConcreteProgramLinks(row.source,row.language,{utf16Strings:true,provenance:{purpose:'origin retention'}});
    const profile=profiles.languages.find(profile=>profile.language===row.language);
    for(const alias of profile.aliases)assert.equal(createConcreteProgramLinks(row.source,alias,{utf16Strings:true,
      provenance:{purpose:'origin retention'}}).network.toLino(),original.network.toLino());
    for(const network of [original.network,LinkNetwork.fromLino(original.network.toLino()),
      decodeLinkNetworkBinary(encodeLinkNetworkBinary(original.network))]){
      const program=decodeTypedProgramLinks(network,original.root),origin=readConcreteProgramOrigin(network,original.root);
      assert.equal(program.sourceLanguage,row.language==='TypeScript'?'JavaScript':row.language);
      assert.equal(origin.sourceProfile.language,row.language);
      assert.deepEqual(origin.userProvenance,{purpose:'origin retention'});
      assert.ok(concreteProgramSourceBytes(origin.sourceProfile.concreteSyntax.tree).equals(Buffer.from(row.source)));
      assert.deepEqual(executeConcreteProgramJavaScript(emitTypedProgramLinks(network,original.root,'JavaScript').code),row.expectedLines);
      assert.match(emitTypedProgramLinks(network,original.root,'Rust').code,/fn main/u);
    }
  }
});

test('checked operation edits change both source frontends while preserving every original concrete byte',()=>{
  const {fixture}=generateConcreteProgramGraphFixtures({check:true});
  for(const row of fixture.cases.filter(row=>row.language!=='TypeScript')){
    const packet=createConcreteProgramLinks(row.source,row.language),network=LinkNetwork.fromLino(packet.network.toLino());
    const field=network.fieldRelations().find(row=>row.label===TYPED_PROGRAM_LINKS_SCHEMA.fields.op
      &&network.link(row.child).metadata().definition==='add');assert.ok(field);
    const link=network.link(field.child);link.setMetadata(link.metadata().withDefinition('mul'));
    assert.deepEqual(executeConcreteProgramJavaScript(emitTypedProgramLinks(network,packet.root,'JavaScript').code),['6']);
    const origin=readConcreteProgramOrigin(network,packet.root);
    assert.ok(concreteProgramSourceBytes(origin.sourceProfile.concreteSyntax.tree).equals(Buffer.from(row.source)));
  }
});

test('existing versioned Rust source models retain their own profile and the complete nested concrete origin',()=>{
  const {fixture}=generateConcreteProgramGraphFixtures({check:true}),row=fixture.cases.find(row=>row.language==='Rust');
  const {arrayResultModel}=readProjectData(new URL('../../parity/fixtures/rust-array-results.lino',import.meta.url));
  const packet=createConcreteProgramLinks(row.source,'Rust',{rustArrayResultModel:arrayResultModel,provenance:{purpose:'selected source model'}});
  const network=LinkNetwork.fromLino(packet.network.toLino()),origin=readConcreteProgramOrigin(network,packet.root);
  assert.equal(origin.sourceProfile.rustArrayResultModel,arrayResultModel);
  assert.equal(origin.userProvenance.sourceProfile.language,'Rust');
  assert.deepEqual(origin.userProvenance.userProvenance,{purpose:'selected source model'});
  assert.ok(concreteProgramSourceBytes(origin.userProvenance.sourceProfile.concreteSyntax.tree).equals(Buffer.from(row.source)));
  assert.deepEqual(executeConcreteProgramJavaScript(emitTypedProgramLinks(network,packet.root,'JavaScript').code),row.expectedLines);
});

test('unregistered languages malformed source non-scalar source and parser replacement attempts refuse',()=>{
  const {fixture}=generateConcreteProgramGraphFixtures({check:true});
  assert.throws(()=>createConcreteProgramLinks('','Python'),/not registered/u);
  assert.throws(()=>createConcreteProgramLinks('',{toString:()=>{throw new Error('must not coerce');}}),/registered name/u);
  assert.throws(()=>createConcreteProgramLinks('// '+String.fromCharCode(55296),'JavaScript'),/Unicode scalar/u);
  assert.throws(()=>createConcreteProgramLinks('function missing(','JavaScript'),/parser refused/u);
  for(const row of fixture.cases){
    assert.throws(()=>createConcreteProgramLinks(row.source,row.language,{concreteLimits:{stepLimit:1}}),/parser refused/u);
    assert.throws(()=>createConcreteProgramLinks(row.source,row.language,{concreteLimits:{mode:'ast'}}),/syntax retention/u);
    assert.throws(()=>createConcreteProgramLinks(row.source,row.language,{concreteLimits:{resolveGrammar:()=>null}}),/grammar selection/u);
  }
  for(const concreteLimits of [null,[],1]){
    assert.throws(()=>createConcreteProgramLinks('','JavaScript',{concreteLimits}),/must be an object/u);
  }
});
