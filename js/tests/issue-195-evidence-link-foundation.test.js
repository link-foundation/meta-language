import assert from 'node:assert/strict';
import test from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';
import {generateLinkFoundationFixtures} from '../scripts/generate-link-foundation-fixtures.mjs';
import {LinkNetwork} from '../src/network.js';
import {LinkMetadata,LinkType} from '../src/primitives.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {recordIssue195Observations} from './support/issue-195-observations.js';

const fixture=readProjectData(new URL('../../parity/fixtures/link-foundation-behavior.lino',import.meta.url));
const refs=(network,id)=>network.link(id).references().map(value=>value.asU64());
function constructLinkFoundationFixture() {
  const network=new LinkNetwork();
  for(const row of fixture.declarations){
    const id=network.insertDynamicLink(row.references,row.term);assert.equal(id.asU64(),row.id);
    network.link(id).setMetadata(new LinkMetadata({linkType:LinkType[row.type],term:row.term,language:fixture.language,named:true}));
  }
  return network;
}
function linkFoundationNetworks() {
  const original=constructLinkFoundationFixture();
  assert.equal(original.toLino(),fixture.graph);
  assert.deepEqual([...encodeLinkNetworkBinary(original)],fixture.binary);
  const networks=[original,LinkNetwork.fromLino(fixture.graph),decodeLinkNetworkBinary(fixture.binary)];
  for(const network of networks)assert.equal(network.toLino(),fixture.graph);
  return networks;
}
function checkLinkFoundationVertex(network) {
  const {vertex,model,modelSelection}=fixture.expected;
  assert.deepEqual(refs(network,vertex),[vertex,vertex]);
  assert.deepEqual(refs(network,modelSelection),[vertex,model]);
  assert.equal(network.link(model).metadata().term,fixture.selectedModel);
  assert.equal(network.link(vertex).metadata().language,fixture.language);
  assert.equal(network.link(vertex).id().asU64(),vertex);
  assert.equal(network.findTerm('declared vertex').asU64(),vertex);
}
function checkLinkFoundationRoles(network) {
  const {vertex,roles,roots,entry}=fixture.expected;
  const assignments=network.links().filter(link=>link.metadata().term==='declared role');
  assert.equal(assignments.length,3);
  assert.deepEqual(assignments.map(link=>refs(network,link.id())),roles.map(role=>[vertex,role.point]));
  for(const role of roles){
    assert.deepEqual(refs(network,role.assignment),[vertex,role.point]);
    assert.equal(network.link(role.point).metadata().term,role.term);
    assert.equal(network.link(refs(network,role.assignment)[0]),network.link(vertex));
  }
  assert.equal(new Set(roots.map(row=>row.root)).size,2);
  for(const row of roots){
    assert.ok(refs(network,entry).includes(row.root));
    assert.deepEqual(refs(network,row.membership),[row.root,vertex]);
    assert.equal(network.link(row.root).id().asU64(),row.root);
  }
}
function checkLinkFoundationComposition(network) {
  const {vertex,motif,sharedOwners,cycle,entry}=fixture.expected;
  assert.equal(network.links().length,fixture.declarations.length);
  for(const row of fixture.declarations){
    assert.deepEqual(refs(network,row.id),row.references);
    for(const child of row.references)assert.ok(network.link(child));
  }
  assert.deepEqual(refs(network,motif),[vertex,vertex]);
  assert.ok(sharedOwners.every(id=>refs(network,id)[0]===motif));
  assert.equal(network.link(refs(network,sharedOwners[0])[0]),network.link(refs(network,sharedOwners[1])[0]));
  assert.ok(refs(network,cycle[0]).includes(cycle[1]));
  assert.ok(refs(network,cycle[1]).includes(cycle[0]));
  assert.ok(refs(network,entry).includes(cycle[0]));
  assert.match(fixture.boundary,/no universal self-typing, physical fractal dimension, paradox elimination or consistency proof/u);
  assert.equal(network.links().some(link=>link.metadata().linkType===LinkType.Type),false,'cycles and roots do not invent type declarations');
}
function observeLinkFoundation(requirementId,assertions,testName) {
  for(const [fixtureId,fixtureFile]of [
    ['conversation-contract:linksFoundationRequest','parity/sources/links-foundation-and-fractal-motivation-2026-10-12.txt'],
    ['link-foundation-behavior','parity/fixtures/link-foundation-behavior.lino'],
  ])recordIssue195Observations({requirementId,runtime:'javascript',suffix:'behavior',fixtureId,fixtureFile,assertions,testName});
}

test('explicit doublet self-loop vertices retain both endpoints identity and selected model through both archives',context=>{
  for(const network of linkFoundationNetworks())checkLinkFoundationVertex(network);
  observeLinkFoundation('I195-LINK-FOUNDATION-SELF-LOOP-VERTEX',[
    'selfLoopVertexRepresentable','selfReferencingEndpointsPreserved','vertexModelAndIdentityRetained'],context.name);
});
test('one link keeps concept entity and relationship roles and multiple linked root selections',context=>{
  for(const network of linkFoundationNetworks())checkLinkFoundationRoles(network);
  observeLinkFoundation('I195-LINK-FOUNDATION-MULTIPLE-ROLES',[
    'linkIdentitySharedAcrossRoles','rolesAreExplicitLinks','multipleRootRolesPreserved'],context.name);
});
test('recursive link composition retains the complete motif sharing and explicit cycle without a consistency claim',context=>{
  for(const network of linkFoundationNetworks())checkLinkFoundationComposition(network);
  observeLinkFoundation('I195-LINK-FOUNDATION-RECURSIVE-COMPOSITION',[
    'recursiveLinkCompositionPreserved','sharedMotifAndCyclesExplicit','noUnprovedConsistencyInference'],context.name);
});
test('self-loop role model and cycle edits cannot satisfy the independent link foundation assertions',()=>{
  for(const [id,references,check]of [
    [1,[1,2],checkLinkFoundationVertex],[3,[1,4],checkLinkFoundationVertex],
    [7,[14,4],checkLinkFoundationRoles],[13,[11,14],checkLinkFoundationRoles],
    [16,[1,15],checkLinkFoundationComposition],[18,[16,14],checkLinkFoundationComposition],
  ]){
    const network=LinkNetwork.fromLino(fixture.graph);network.link(id).setReferences(references);
    assert.throws(()=>check(network),assert.AssertionError);
  }
});
test('automatic link foundation capture retains the complete public-constructor graph and binary snapshot',()=>{
  generateLinkFoundationFixtures({check:true});
  for(const network of linkFoundationNetworks())assert.deepEqual([...encodeLinkNetworkBinary(network)],fixture.binary);
});
