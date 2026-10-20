import assert from 'node:assert/strict';
import test from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';
import {generateRootTypeFixtures} from '../scripts/generate-root-type-fixtures.mjs';
import {LinkNetwork} from '../src/network.js';
import {LinkMetadata,LinkType} from '../src/primitives.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {recordIssue195Observations} from './support/issue-195-observations.js';

const fixture=readProjectData(new URL('../../parity/fixtures/root-type-behavior.lino',import.meta.url));
const refs=(network,id)=>network.link(id).references().map(value=>value.asU64());
function rootTypeNetworks(row) {
  const original=new LinkNetwork();
  for(const declaration of row.declarations){
    const id=original.insertDynamicLink(declaration.references,declaration.term);assert.equal(id.asU64(),declaration.id);
    original.link(id).setMetadata(new LinkMetadata({linkType:LinkType[declaration.type],term:declaration.term,language:fixture.language,named:true}));
  }
  assert.equal(original.toLino(),row.graph);assert.deepEqual([...encodeLinkNetworkBinary(original)],row.binary);
  const networks=[original,LinkNetwork.fromLino(row.graph),decodeLinkNetworkBinary(row.binary)];
  for(const network of networks){assert.equal(network.toLino(),row.graph);assert.deepEqual([...encodeLinkNetworkBinary(network)],row.binary);}
  return networks;
}
function checkRootTypeModel(network,row) {
  const expected=fixture.expected;
  assert.equal(network.links().length,row.typed?10:9);
  for(const declaration of row.declarations){
    assert.deepEqual(refs(network,declaration.id),declaration.references);
    assert.equal(network.link(declaration.id).metadata().linkType,LinkType[declaration.type]);
    assert.equal(network.link(declaration.id).metadata().term,declaration.term);
    assert.equal(network.link(declaration.id).metadata().language,fixture.language);
  }
  assert.deepEqual(refs(network,expected.root),[expected.root,expected.root]);
  assert.deepEqual(refs(network,expected.selection),[expected.root,expected.model]);
  assert.deepEqual(refs(network,expected.premiseSelection),[expected.model,expected.premise]);
  assert.deepEqual(refs(network,expected.rootAssignment),[expected.root,expected.rootRole]);
  assert.equal(network.findTerm('selected root').asU64(),expected.root);
  const types=network.links().filter(link=>link.metadata().linkType===LinkType.Type);
  const roles=network.links().filter(link=>link.metadata().term==='declared type role');
  assert.equal(types.length,row.typed?1:0);assert.equal(roles.length,row.typed?1:0);
  if(row.typed){
    assert.equal(types[0].id().asU64(),expected.root);
    assert.deepEqual(refs(network,expected.typeAssignment),[expected.root,expected.typeRole]);
    assert.ok(refs(network,expected.entry).includes(expected.typeAssignment));
  }
  assert.equal(network.links().some(link=>['universe membership','universal proof logic','universal self typing'].includes(link.metadata().term)),false);
  assert.match(fixture.boundary,/not a universe membership or Type:Type axiom/u);
}

test('optional root type roles and selected premises survive construction and both archives without implicit universes',context=>{
  assert.deepEqual(fixture.cases.map(row=>row.typed),[false,true]);
  for(const row of fixture.cases)for(const network of rootTypeNetworks(row))checkRootTypeModel(network,row);
  for(const [fixtureId,fixtureFile]of [
    ['conversation-contract:linksFoundationRequest','parity/sources/links-foundation-and-fractal-motivation-2026-10-12.txt'],
    ['root-type-behavior','parity/fixtures/root-type-behavior.lino'],
  ])recordIssue195Observations({requirementId:'I195-LINK-FOUNDATION-OPTIONAL-TYPE-ROOT',runtime:'javascript',suffix:'behavior',fixtureId,fixtureFile,
    assertions:['rootTypeRolesDeclared','noImplicitUniversalSelfTyping','selectedFoundationPremisesPreserved'],testName:context.name});
});
test('changing type designation model premise or explicit type assignment fails the complete root model oracle',()=>{
  for(const row of fixture.cases){
    for(const [id,references]of [[3,[1,4]],[6,[2,4]],[8,[1,4]],...(row.typed?[[10,[1,1]]]:[])]){
      const network=LinkNetwork.fromLino(row.graph);network.link(id).setReferences(references);
      assert.throws(()=>checkRootTypeModel(network,row),assert.AssertionError);
    }
    const network=LinkNetwork.fromLino(row.graph);
    network.link(1).setMetadata(new LinkMetadata({linkType:row.typed?LinkType.Concept:LinkType.Type,term:'selected root',language:fixture.language,named:true}));
    assert.throws(()=>checkRootTypeModel(network,row),assert.AssertionError);
  }
});
test('automatic root type snapshots retain both complete models and exact binary encodings',()=>{
  generateRootTypeFixtures({check:true});
  for(const row of fixture.cases)for(const network of rootTypeNetworks(row))assert.deepEqual([...encodeLinkNetworkBinary(network)],row.binary);
});
