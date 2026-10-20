// Explicit model choices exercise existing public links, never a proof axiom.
import assert from 'node:assert/strict';
import {fileURLToPath} from 'node:url';
import {LinkNetwork} from '../src/network.js';
import {LinkMetadata,LinkType} from '../src/primitives.js';
import {encodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {checkProjectData,writeProjectData} from './project-data.mjs';

export function generateRootTypeFixtures({check=false}={}) {
  const language='explicit optional root type model 1';
  const cases=[];
  for(const typed of [false,true]){
    const declarations=[
      [1,[1,1],'selected root',typed?'Type':'Concept'],
      [2,[2,2],'explicit root role model','Concept'],
      [3,[1,2],'selected model','Semantic'],
      [4,[4,4],'type role','Concept'],
      [5,[5,5],'root type designation permitted','Concept'],
      [6,[2,5],'selected foundation premise','Semantic'],
      [7,[7,7],'ontology root role','Concept'],
      [8,[1,7],'declared root role','Semantic'],
      [9,typed?[1,2,6,8,10]:[1,2,6,8],'model entry','Semantic'],
      ...(typed?[[10,[1,4],'declared type role','Semantic']]:[]),
    ].map(([id,references,term,type])=>({id,references,term,type}));
    const network=new LinkNetwork();
    for(const row of declarations){
      const id=network.insertDynamicLink(row.references,row.term);assert.equal(id.asU64(),row.id);
      network.link(id).setMetadata(new LinkMetadata({linkType:LinkType[row.type],term:row.term,language,named:true}));
    }
    cases.push({typed,declarations,graph:network.toLino(),binary:[...encodeLinkNetworkBinary(network)]});
  }
  const fixture={schemaVersion:1,language,cases,
    expected:{root:1,model:2,selection:3,typeRole:4,premise:5,premiseSelection:6,rootRole:7,rootAssignment:8,entry:9,typeAssignment:10},
    boundary:'Root type designation is an explicit optional model role. A self-loop is the selected vertex representation, not a universe membership or Type:Type axiom. Both cases retain their selected foundation premise as actual links. No universal proof logic, physical proposition or consistency theorem is inferred.'};
  const file=new URL('../../parity/fixtures/root-type-behavior.lino',import.meta.url);
  if(check)checkProjectData(file,fixture);else writeProjectData(file,fixture);
  return fixture;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.slice(2).every(value=>value==='--check'));
  generateRootTypeFixtures({check:process.argv.includes('--check')});
  console.log('Captured both optional root type choices and their complete explicit foundation premises.');
}
