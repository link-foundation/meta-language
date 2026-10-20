import assert from 'node:assert/strict';
import test from 'node:test';
import {createConcreteProgramLinks} from '../../experiments/concrete-program-graphs/projection.mjs';
import {readConcreteProgramOrigin,readConcreteProgramOriginText} from '../../experiments/concrete-program-graphs/origin.mjs';
import {generateConcreteProgramOriginFixtures} from '../scripts/generate-concrete-program-origin-fixtures.mjs';
import {generateConcreteProgramGraphFixtures} from '../scripts/generate-concrete-program-graph-fixtures.mjs';
import {readProjectData} from '../scripts/project-data.mjs';
import {LinkNetwork} from '../src/network.js';
import {LinkMetadata,LinkType} from '../src/primitives.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {TYPED_PROGRAM_LINKS_SCHEMA as schema} from '../src/typed-program-links.js';
const field=(network,parent,label)=>network.fieldRelations().find(row=>row.parent.equals(parent)&&row.label===label)?.child;
function originalPacket(){const {fixture}=generateConcreteProgramGraphFixtures({check:true});
  const row=fixture.cases[0],packet=createConcreteProgramLinks(row.source,row.language);
  return{...packet,source:row.source};}
function firstToken(network){return network.fieldRelations().find(row=>row.label==='type'
  &&network.link(row.child).metadata().definition==='token')?.parent;}

test('stored origins restore original CRLF and Unicode bytes through text and binary links without parser state',()=>{
  generateConcreteProgramOriginFixtures({check:true});
  const {fixture}=generateConcreteProgramGraphFixtures({check:true});
  for(const row of fixture.cases){
    const packet=createConcreteProgramLinks(row.source,row.language,{utf16Strings:true});
    for(const network of [LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]){
      const before=network.toLino(),origin=readConcreteProgramOriginText(network,packet.root);
      assert.equal(origin.language,row.language);assert.equal(origin.plane,'source');assert.equal(origin.text,row.source);
      assert.deepEqual(Buffer.from(origin.bytes),Buffer.from(row.source));assert.equal(network.toLino(),before);
      if(row.language==='TypeScript'){
        const lowered=readConcreteProgramOriginText(network,packet.root,{plane:'lowered'});
        assert.equal(lowered.language,'JavaScript');assert.ok(lowered.text.includes('@param'));
        assert.equal(lowered.text.includes('value: string'),false);
      }else assert.throws(()=>readConcreteProgramOrigin(network,packet.root,{plane:'lowered'}),/missing lowering/u);
    }
  }
});

test('origin reads keep the stored source after semantic edits and nested selected Rust models',()=>{
  const packet=originalPacket(),network=LinkNetwork.fromLino(packet.network.toLino());
  const operation=network.fieldRelations().find(row=>row.label===schema.fields.op
    &&network.link(row.child).metadata().definition==='add');assert.ok(operation);
  const link=network.link(operation.child);link.setMetadata(link.metadata().withDefinition('mul'));
  assert.equal(readConcreteProgramOriginText(network,packet.root).text,packet.source);
  const {fixture}=generateConcreteProgramGraphFixtures({check:true}),row=fixture.cases.find(row=>row.language==='Rust');
  const {arrayResultModel}=readProjectData(new URL('../../parity/fixtures/rust-array-results.lino',import.meta.url));
  const rust=createConcreteProgramLinks(row.source,'Rust',{rustArrayResultModel:arrayResultModel});
  assert.equal(readConcreteProgramOriginText(LinkNetwork.fromLino(rust.network.toLino()),rust.root).text,row.source);
});

test('origin decoding refuses inconsistent spans duplicate fields cycles unknown profiles and invalid resources',()=>{
  for(const edit of ['span','number','duplicate','cycle','profile']){
    const packet=originalPacket(),network=LinkNetwork.fromLino(packet.network.toLino()),token=firstToken(network);assert.ok(token);
    if(edit==='span'){const link=network.link(field(network,token,'start'));link.setMetadata(link.metadata().withDefinition('1'));}
    if(edit==='number'){const link=network.link(field(network,token,'start'));link.setMetadata(link.metadata().withDefinition(''));}
    if(edit==='duplicate')network.insertFieldRelation(token,'text',field(network,token,'text'));
    if(edit==='cycle'){
      const relation=network.fieldRelations().find(row=>row.label==='children');assert.ok(relation);
      network.link(relation.child).setReferences([relation.parent]);
    }
    if(edit==='profile'){
      const relation=network.fieldRelations().find(row=>row.label==='model');assert.ok(relation);
      const link=network.link(relation.child);link.setMetadata(link.metadata().withDefinition('unknown'));
    }
    const before=network.toLino();assert.throws(()=>readConcreteProgramOrigin(network,packet.root),TypeError);
    assert.equal(network.toLino(),before);
  }
  const packet=originalPacket(),{cases}=generateConcreteProgramOriginFixtures({check:true});
  for(const value of cases.invalidLimits)assert.throws(()=>readConcreteProgramOrigin(packet.network,packet.root,{maxReadNodes:value}),/invalid resource/u);
  for(const key of ['maxGraphLinks','maxReadNodes','maxDataDepth','maxTreeDepth','maxBytes']){
    assert.throws(()=>readConcreteProgramOrigin(packet.network,packet.root,{[key]:0}),/limit/u);
  }
  assert.throws(()=>readConcreteProgramOrigin(packet.network,packet.root,{plane:'unknown'}),/unknown source plane/u);
  assert.throws(()=>readConcreteProgramOrigin(packet.network,packet.root,{foreign:1}),/invalid resource/u);
});

test('physical byte origins preserve a binary leaf while text conversion refuses invalid UTF-8',()=>{
  const packet=originalPacket(),network=LinkNetwork.fromLino(packet.network.toLino());
  const token=network.fieldRelations().filter(row=>row.label==='type'&&network.link(row.child).metadata().definition==='token')
    .map(row=>row.parent).find(id=>network.link(field(network,id,'text')).metadata().definition===';');assert.ok(token);
  const text=network.link(field(network,token,'text'));
  text.setMetadata(text.metadata().withTerm(schema.scalars.null).withDefinition(undefined));
  const hex=network.insertLink([],LinkMetadata.new().withLinkType(LinkType.Semantic).withLanguage(schema.language)
    .withNamed(true).withTerm(schema.scalars.string).withDefinition('ff'));
  network.insertFieldRelation(token,'hex',hex);
  const bytes=readConcreteProgramOrigin(network,packet.root).bytes;assert.ok(bytes.includes(255));
  assert.throws(()=>readConcreteProgramOriginText(network,packet.root),TypeError);
});

test('deep linked origins use the active executor depth and iterative decoding within explicit limits',()=>{
  const {rules}=generateConcreteProgramOriginFixtures({check:true}),packet=originalPacket(),network=LinkNetwork.fromLino(packet.network.toLino());
  const relation=network.fieldRelations().find(row=>row.label==='tree');assert.ok(relation);
  let previous=relation.child;
  const scalar=(kind,value)=>network.insertLink([],LinkMetadata.new().withLinkType(LinkType.Semantic)
    .withLanguage(schema.language).withNamed(true).withTerm(schema.scalars[kind]).withDefinition(String(value)));
  const start=scalar('number',0),end=scalar('number',Buffer.byteLength(packet.source));
  const type=scalar('string','node'),kind=scalar('string','declared physical source context');
  const nesting=Math.max(0,rules.limits.maxTreeDepth-50);
  for(let depth=0;depth<nesting;depth++){
    const node=network.insertLink([],LinkMetadata.new().withLinkType(LinkType.Semantic).withLanguage(schema.language)
      .withNamed(true).withTerm(schema.containers.record));
    const children=network.insertLink([previous],LinkMetadata.new().withLinkType(LinkType.Semantic).withLanguage(schema.language)
      .withNamed(true).withTerm(schema.containers.array));
    for(const[label,child]of [['type',type],['kind',kind],['start',start],['end',end],['children',children]])network.insertFieldRelation(node,label,child);
    previous=node;
  }
  relation.link.setReferences([relation.parent,relation.link.references()[1],previous]);
  const restored=LinkNetwork.fromLino(network.toLino());
  assert.equal(readConcreteProgramOriginText(restored,packet.root).text,packet.source);
  assert.throws(()=>readConcreteProgramOrigin(restored,packet.root,{maxTreeDepth:Math.floor(nesting/2)}),/tree resource limit/u);
});
