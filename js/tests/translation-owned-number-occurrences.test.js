import assert from 'node:assert/strict';
import test from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';
import {captureOrdinaryInversePlan,extendRecursiveInversePlan} from '../../experiments/grammar-outcome-network/native-transport/inverse/plan.mjs';
import {reconstructOrdinaryOutcome} from '../../experiments/grammar-outcome-network/native-transport/inverse/owned.mjs';
import {readOwnedNumberWireCorrespondence,bindOwnedNumberWireObservations} from '../../experiments/grammar-outcome-network/native-transport/inverse/number-occurrences.mjs';
import {encodeStructuredDataLinks} from '../src/structured-data-links.js';
import {decodeStructuredDataLinks} from '../src/structured-data-graph.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';

const compilerFixture=readProjectData(new URL('../../experiments/grammar-outcome-network/native-transport/runner/compiler-profile-fixture.lino',import.meta.url));
const compilerRecords=compilerFixture.records['compile.stdout.log'].trim().split(/\r?\n/u).map(row=>JSON.parse(row));
const probe=compilerRecords.find(row=>row.reason==='compiler-artifact'&&row.target?.name==='bulk95_outcome_probe');
const dependency=compilerRecords.find(row=>row.reason==='compiler-artifact'&&row.target?.name==='serde_json');
function occurrenceCompilerInputs() {
  const args=['/toolchain/bin/rustc','--crate-name','bulk95_outcome_probe','--test',probe.target.src_path,'--extern','serde_json='+dependency.filenames.find(path=>path.endsWith('.rlib'))];
  return{cargoVersion:compilerFixture.records['cargo-version.stdout.log'],rustcVersion:compilerFixture.records['rustc-version.stdout.log'],
    stdout:compilerFixture.records['compile.stdout.log'],stderr:'     Running `'+args.map(word=>"'"+word.replaceAll("'","'\\''")+"'").join(' ')+'`\n',
    lockSource:compilerFixture.lockSource,probeName:'bulk95_outcome_probe',manifestPath:probe.manifest_path,executable:probe.executable,usizeBits:64};
}
let authority;
function occurrenceContext(input,rootName=null) {
  authority??=extendRecursiveInversePlan(captureOrdinaryInversePlan());
  const packet=encodeStructuredDataLinks(input),options={usizeBits:64};
  if(rootName){const plan=authority.plan();options.rootReference=plan.graph.roots.find(id=>plan.graph.nodes.find(row=>row.identity===id)?.name===rootName);assert.ok(options.rootReference);}
  return{authority,options,model:reconstructOrdinaryOutcome(authority,packet.network,packet.root,options)};
}
const occurrenceToken=lexical=>({type:'token',kind:'word',start:0,end:1,lexical,text:'a'});
const occurrenceOutcome=tree=>({ok:true,tree,ambiguities:[],rejection:null});

test('actual original number controls join by source sites and exact expanded wire ordinals through both Links archives',()=>{
  const fixture=readProjectData(new URL('../../parity/fixtures/original-native-number-observations.lino',import.meta.url));
  const {authority,model,options}=occurrenceContext(occurrenceOutcome(occurrenceToken([0,-1,2**60,-0,1.5,Number.MIN_VALUE])));
  const correspondence=readOwnedNumberWireCorrespondence(authority,model,options);
  assert.equal(correspondence.expandedOccurrences,38);assert.equal(correspondence.expandedOccurrences,fixture.expandedOccurrences);
  assert.deepEqual(correspondence.correspondence.map(row=>row.wireOccurrence),[23,24,25,26,27,28]);
  assert.deepEqual(correspondence.correspondence.map(row=>row.path),Array.from({length:6},(_,index)=>['tree','lexical',index]));
  assert.deepEqual(correspondence.correspondence.map(row=>row.node),model.jsonRequirements.map(row=>row.node));
  const result=bindOwnedNumberWireObservations(authority,model,fixture.observations,occurrenceCompilerInputs(),options);
  assert.deepEqual(result.numberPlan.selections.map(row=>row.kind),['unsigned integer','signed integer','unsigned integer','floating point','floating point','floating point']);
  assert.deepEqual(result.numberPlan.selections.map(row=>row.projectedBits),fixture.observations.map(row=>row.floatingBits));
  assert.match(result.boundary,/does not attest/u);
  const encoded=encodeStructuredDataLinks(result),canonical=LinkNetwork.fromLino(encoded.network.toLino());
  for(const graph of [canonical,decodeLinkNetworkBinary(encodeLinkNetworkBinary(canonical))]){
    const restored=decodeStructuredDataLinks(graph,encoded.root);assert.deepEqual(restored,result);
    assert.deepEqual(restored,bindOwnedNumberWireObservations(authority,model,fixture.observations,occurrenceCompilerInputs(),options));
  }
});

test('recursive equal numbers retain distinct paths across every actual JSON owning field and ordered key',()=>{
  const input=occurrenceOutcome({type:'embed',language:'child',start:0,end:1,root:occurrenceToken(1),sourceChildren:[occurrenceToken(1)],
    selection:{mode:'boundary',mapping:'identity',start:0,end:1,candidates:[1],provenance:1,
      alternatives:[{language:'other',root:occurrenceToken(1),provenance:1}],sourceMapping:1,ambiguous:true}});
  input.sourcePlanes=[1];input.ambiguities=[{rule:'boundary',start:0,end:1,sourceRoute:[1]}];
  input.rejection={reason:'boundary',diagnostics:[1]};
  const {authority,model,options}=occurrenceContext(input),result=readOwnedNumberWireCorrespondence(authority,model,options);
  const expected=[['tree','root','lexical'],['tree','sourceChildren',0,'lexical'],['tree','selection','candidates',0],
    ['tree','selection','provenance'],['tree','selection','alternatives',0,'root','lexical'],['tree','selection','alternatives',0,'provenance'],
    ['tree','selection','sourceMapping'],['sourcePlanes',0],['ambiguities',0,'sourceRoute',0],['rejection','diagnostics',0]];
  assert.equal(result.correspondence.length,10);assert.equal(new Set(result.correspondence.map(row=>row.node)).size,10);
  assert.deepEqual(result.correspondence.map(row=>JSON.stringify(row.path)).sort(),expected.map(row=>JSON.stringify(row)).sort());
  assert.ok(result.correspondence.every(row=>row.projectedBits==='3ff0000000000000'));
  assert.ok(result.correspondence.every((row,index,rows)=>index===0||rows[index-1].wireOccurrence<row.wireOccurrence));
  const records=result.correspondence.map(row=>({wireOccurrence:row.wireOccurrence,signed:true,unsigned:true,floating:false,signedDecimal:'1',unsignedDecimal:'1',floatingBits:row.projectedBits}));
  const bound=bindOwnedNumberWireObservations(authority,model,records,occurrenceCompilerInputs(),options);
  assert.deepEqual(bound.numberPlan.selections.map(row=>row.node),model.jsonRequirements.map(row=>row.node));
  const aliased={value:[1,1], '0':1, ['__proto__']:null};
  const repeated=occurrenceOutcome(occurrenceToken(aliased));repeated.sourcePlanes=[aliased,structuredClone(aliased)];
  const context=occurrenceContext(repeated),shared=readOwnedNumberWireCorrespondence(context.authority,context.model,context.options);
  assert.equal(shared.correspondence.length,9);assert.equal(new Set(shared.correspondence.map(row=>JSON.stringify(row.path))).size,9);
  assert.ok(context.model.jsonAliases.length>0,'actual shared containers retain their existing checked alias manifest');
  assert.ok(shared.correspondence.some(row=>row.path.at(-1)==='0'),'object keys stay strings while array positions stay numeric');
});

test('foreign host ordinals reordered records payload changes and false native predicate overlap refuse',()=>{
  const fixture=readProjectData(new URL('../../parity/fixtures/original-native-number-observations.lino',import.meta.url));
  const {authority,model,options}=occurrenceContext(occurrenceOutcome(occurrenceToken([0,-1,2**60,-0,1.5,Number.MIN_VALUE])));
  const mutations=[rows=>rows.pop(),rows=>rows.reverse(),rows=>{rows[0].wireOccurrence=8;},rows=>{rows[1].wireOccurrence=rows[0].wireOccurrence;},
    rows=>{rows[0].signed=false;rows[0].signedDecimal=null;},rows=>{rows[1].signedDecimal='-2';},
    rows=>{rows[2].unsignedDecimal='1152921504606846977';},rows=>{rows[2].signedDecimal='1152921504606846977';rows[2].unsignedDecimal='1152921504606846977';},
    rows=>{rows[3].floatingBits='0000000000000000';},rows=>{rows[3].signedDecimal='0';},rows=>{rows[0].foreign=true;}];
  for(const mutate of mutations){const altered=structuredClone(fixture.observations);mutate(altered);assert.throws(()=>bindOwnedNumberWireObservations(authority,model,altered,occurrenceCompilerInputs(),options));}
  let invoked=0;const active=()=>{invoked++;throw new Error('must not execute');};
  assert.throws(()=>bindOwnedNumberWireObservations(authority,model,new Proxy(fixture.observations,{get:active,ownKeys:active}),occurrenceCompilerInputs(),options),/passive/u);
  const getters=structuredClone(fixture.observations);Object.defineProperty(getters[0],'wireOccurrence',{get:active,enumerable:true});
  assert.throws(()=>bindOwnedNumberWireObservations(authority,model,getters,occurrenceCompilerInputs(),options),/getters/u);
  const sequence=structuredClone(fixture.observations);Object.defineProperty(sequence,'0',{get:active,enumerable:true});
  assert.throws(()=>bindOwnedNumberWireObservations(authority,model,sequence,occurrenceCompilerInputs(),options),/getters/u);assert.equal(invoked,0);
  const altered=structuredClone(model);altered.jsonRequirements[0].node++;
  assert.throws(()=>readOwnedNumberWireCorrespondence(authority,altered,options));
});

test('empty original manifests host integer maps and explicit correspondence route limits retain their boundaries',()=>{
  for(const context of [occurrenceContext(occurrenceOutcome(null)),occurrenceContext({count:7,text:'a'},'SyntaxAttributes')]){
    const {authority,model,options}=context,result=readOwnedNumberWireCorrespondence(authority,model,options);
    assert.deepEqual(result.correspondence,[]);assert.ok(result.expandedOccurrences>0);
    assert.deepEqual(bindOwnedNumberWireObservations(authority,model,[],occurrenceCompilerInputs(),options).numberPlan.selections,[]);
  }
  const {authority,model,options}=occurrenceContext(occurrenceOutcome(occurrenceToken({['x'.repeat(1048576)]:1})));
  assert.equal(model.jsonRequirements.length,1,'the existing owned model still admits its original source input');
  assert.throws(()=>readOwnedNumberWireCorrespondence(authority,model,options),/route byte budget/u,'the finite new correspondence profile refuses an excessive route without weakening the existing model');
});
