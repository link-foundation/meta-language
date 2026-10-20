import assert from 'node:assert/strict';
import test from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';
import {captureOrdinaryInversePlan,extendRecursiveInversePlan} from '../../experiments/grammar-outcome-network/native-transport/inverse/plan.mjs';
import {reconstructOrdinaryOutcome} from '../../experiments/grammar-outcome-network/native-transport/inverse/owned.mjs';
import {planOwnedOutcomeNativeNumbers,checkOwnedOutcomeNativeNumberPlan} from '../../experiments/grammar-outcome-network/native-transport/inverse/number-facts.mjs';
import {encodeStructuredDataLinks} from '../src/structured-data-links.js';
import {decodeStructuredDataLinks} from '../src/structured-data-graph.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';

const fixture=readProjectData(new URL('../../experiments/grammar-outcome-network/native-transport/runner/compiler-profile-fixture.lino',import.meta.url));
const records=fixture.records['compile.stdout.log'].split(/\r?\n/u).filter(Boolean).map(line=>JSON.parse(line));
const probe=records.find(row=>row.reason==='compiler-artifact'&&row.target?.name==='bulk95_outcome_probe');
const dependency=records.find(row=>row.reason==='compiler-artifact'&&row.target?.name==='serde_json');
// Explicit synthetic contracts over historical compiler bytes; no live attestation.
const compilerInputs=()=>({cargoVersion:fixture.records['cargo-version.stdout.log'],rustcVersion:fixture.records['rustc-version.stdout.log'],
  stdout:fixture.records['compile.stdout.log'],stderr:'     Running `'+['/toolchain/bin/rustc','--crate-name','bulk95_outcome_probe','--test',probe.target.src_path,
    '--extern','serde_json='+dependency.filenames.find(path=>path.endsWith('.rlib'))].map(word=>"'"+word.replaceAll("'","'\\''")+"'").join(' ')+'`\n',
  lockSource:fixture.lockSource,probeName:'bulk95_outcome_probe',manifestPath:probe.manifest_path,executable:probe.executable,usizeBits:64});
let context;
function numberFactContext() {
  if(!context){
    const authority=extendRecursiveInversePlan(captureOrdinaryInversePlan());
    const input={ok:true,tree:{type:'token',kind:'word',start:0,end:1,lexical:[0,-1,2**60,-0,1.5,Number.MIN_VALUE],text:'a'},ambiguities:[],rejection:null};
    const packet=encodeStructuredDataLinks(input),options={usizeBits:64};
    const model=reconstructOrdinaryOutcome(authority,packet.network,packet.root,options);
    assert.equal(model.jsonRequirements.length,6);
    const facts=model.jsonRequirements.map((required,index)=>({node:required.node,typeReference:required.typeReference,
      signed:index<3,unsigned:index===0||index===2,floating:index>=3,
      payload:index>=3?required.bits:BigInt([0,-1,2**60][index]).toString()}));
    context={authority,model,facts,options};
  }
  return context;
}

test('complete supplied number contracts bind exact source occurrences compiler features native variants and payloads',()=>{
  const {authority,model,facts,options}=numberFactContext();
  const plan=planOwnedOutcomeNativeNumbers(authority,model,facts,compilerInputs(),options);
  assert.deepEqual(plan.selections.map(row=>row.kind),['unsigned integer','signed integer','unsigned integer','floating point','floating point','floating point']);
  assert.equal(plan.selections[2].payload,'1152921504606846976');
  assert.equal(plan.selections[3].payload,'8000000000000000');
  assert.equal(plan.selections[5].payload,'0000000000000001');
  assert.match(plan.boundary,/Original object capture, live execution provenance/u);
  const packet=encodeStructuredDataLinks(plan);
  const canonical=LinkNetwork.fromLino(packet.network.toLino());
  for(const network of [canonical,decodeLinkNetworkBinary(encodeLinkNetworkBinary(canonical))]){
    const restored=decodeStructuredDataLinks(network,packet.root);
    assert.deepEqual(checkOwnedOutcomeNativeNumberPlan(authority,model,facts,compilerInputs(),restored,options),plan);
  }
  const floatingZero=structuredClone(facts);floatingZero[0].signed=false;floatingZero[0].unsigned=false;floatingZero[0].floating=true;floatingZero[0].payload=model.jsonRequirements[0].bits;
  const alternative=planOwnedOutcomeNativeNumbers(authority,model,floatingZero,compilerInputs(),options);
  assert.equal(alternative.selections[0].kind,'floating point','equal projections cannot erase explicitly different native variants');
  assert.notDeepEqual(alternative,plan);
});

test('rounded integer payloads missing reordered or foreign facts and changed complete plans refuse',()=>{
  const {authority,model,facts,options}=numberFactContext();
  const changes=[rows=>rows.pop(),rows=>rows.reverse(),rows=>{rows[1].node=rows[0].node;},rows=>{rows[0].typeReference+=' foreign';},
    rows=>{rows[2].payload='1152921504606846977';},rows=>{rows[1].payload='-0';},rows=>{rows[0].payload='00';},
    rows=>{rows[0].floating=true;},rows=>{rows[3].payload='0000000000000000';},rows=>{rows[3].payload='7ff0000000000000';},
    rows=>{rows[0].foreign=true;},rows=>{rows[0].unsigned=1;}];
  for(const change of changes){const altered=structuredClone(facts);change(altered);assert.throws(()=>planOwnedOutcomeNativeNumbers(authority,model,altered,compilerInputs(),options));}
  const plan=planOwnedOutcomeNativeNumbers(authority,model,facts,compilerInputs(),options);
  for(const change of [value=>value.selections.pop(),value=>{value.selections[0].kind='floating point';},value=>{value.planSha256='0'.repeat(64);}]){
    const altered=structuredClone(plan);change(altered);
    assert.throws(()=>checkOwnedOutcomeNativeNumberPlan(authority,model,facts,compilerInputs(),altered,options));
  }
  const alteredModel=structuredClone(model);alteredModel.jsonRequirements[0].node++;
  assert.throws(()=>planOwnedOutcomeNativeNumbers(authority,alteredModel,facts,compilerInputs(),options));
  assert.throws(()=>planOwnedOutcomeNativeNumbers(authority,model,facts,{...compilerInputs(),usizeBits:32},options),/target profile/u);
  const inputs=compilerInputs(),changed=structuredClone(records);
  changed.find(row=>row.reason==='compiler-artifact'&&row.filenames?.includes(dependency.filenames.find(path=>path.endsWith('.rlib')))).features.push('arbitrary_precision');
  assert.throws(()=>planOwnedOutcomeNativeNumbers(authority,model,facts,{...inputs,stdout:changed.map(row=>JSON.stringify(row)).join('\n')+'\n'},options),/uncovered native JSON number representation/u);
});

test('supplied number facts and compiler contracts cannot execute proxies or accessor fields',()=>{
  const {authority,model,facts,options}=numberFactContext();let invoked=0;
  const active=()=>{invoked++;throw new Error('must never execute');};
  const proxy=new Proxy(facts,{get:active,ownKeys:active,getPrototypeOf:active});
  assert.throws(()=>planOwnedOutcomeNativeNumbers(authority,model,proxy,compilerInputs(),options),/passive/u);
  const getter=structuredClone(facts);Object.defineProperty(getter[0],'payload',{get:active,enumerable:true});
  assert.throws(()=>planOwnedOutcomeNativeNumbers(authority,model,getter,compilerInputs(),options),/accessors/u);
  const compiler=compilerInputs();Object.defineProperty(compiler,'stdout',{get:active,enumerable:true});
  assert.throws(()=>planOwnedOutcomeNativeNumbers(authority,model,facts,compiler,options),/accessors/u);
  assert.equal(invoked,0);
});
