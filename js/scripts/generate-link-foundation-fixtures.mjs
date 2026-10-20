// Exercise existing public link constructors; both runtimes consume the same
// explicit model, role declarations, cycles and independent adjacency oracle.
import assert from 'node:assert/strict';
import {fileURLToPath} from 'node:url';
import {LinkNetwork} from '../src/network.js';
import {LinkMetadata,LinkType} from '../src/primitives.js';
import {encodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {checkProjectData,writeProjectData} from './project-data.mjs';

export function generateLinkFoundationFixtures({check=false}={}) {
  const language='explicit link foundation model 1';
  const declarations=[
    [1,[1,1],'declared vertex','Concept'],
    [2,[2,2],'self loop doublet vertex model','Concept'],
    [3,[1,2],'selected vertex model','Semantic'],
    [4,[4,4],'concept role','Concept'],
    [5,[5,5],'entity role','Concept'],
    [6,[6,6],'relationship role','Concept'],
    [7,[1,4],'declared role','Semantic'],
    [8,[1,5],'declared role','Semantic'],
    [9,[1,6],'declared role','Semantic'],
    [10,[10,10],'first root','Concept'],
    [11,[11,11],'second root','Concept'],
    [12,[10,1],'root membership','Semantic'],
    [13,[11,1],'root membership','Semantic'],
    [14,[1,1],'composed motif','Relation'],
    [15,[14,1],'first motif composition','Relation'],
    [16,[14,15],'second motif composition','Relation'],
    [17,[16,18],'first cycle member','Relation'],
    [18,[17,14],'second cycle member','Relation'],
    [19,[2,10,11,17],'explicit model entry','Semantic'],
  ].map(([id,references,term,type])=>({id,references,term,type}));
  const network=new LinkNetwork();
  for(const row of declarations){
    const id=network.insertDynamicLink(row.references,row.term);assert.equal(id.asU64(),row.id);
    network.link(id).setMetadata(new LinkMetadata({linkType:LinkType[row.type],term:row.term,language,named:true}));
  }
  const fixture={schemaVersion:1,language,selectedModel:'self loop doublet vertex model',declarations,
    graph:network.toLino(),binary:[...encodeLinkNetworkBinary(network)],
    expected:{vertex:1,model:2,modelSelection:3,roles:[{point:4,assignment:7,term:'concept role'},
      {point:5,assignment:8,term:'entity role'},{point:6,assignment:9,term:'relationship role'}],
      roots:[{root:10,membership:12},{root:11,membership:13}],motif:14,sharedOwners:[15,16],cycle:[17,18],entry:19},
    boundary:'An explicit finite n-ary link model with doublet self-loop vertices. All roles, model selection, roots, shared motif references and cycles are actual links. This fixture asserts no universal self-typing, physical fractal dimension, paradox elimination or consistency proof; complete project-wide formal concept definitions remain independent requirements.'};
  const file=new URL('../../parity/fixtures/link-foundation-behavior.lino',import.meta.url);
  if(check)checkProjectData(file,fixture);else writeProjectData(file,fixture);
  return fixture;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.slice(2).every(value=>value==='--check'));
  generateLinkFoundationFixtures({check:process.argv.includes('--check')});
  console.log('Captured nineteen explicit links with self-loop vertices, shared roles and recursive composition.');
}
