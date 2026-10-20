// Bind explicit supplied number payloads to complete checked source occurrences.
// This reader cannot turn supplied records into observations of original objects.
import assert from 'node:assert/strict';
import {types} from 'node:util';
import {projectOrdinaryOutcome} from './owned.mjs';
import {readOwnedOutcomeNativeNumberProfile} from './number-profile.mjs';
import {readOwnedOutcomeNativeNumberKind} from './number-kind-rules.mjs';
import {acceptOwnedOutcomeIntegerProjection} from './integer-projection-rules.mjs';

function readPassiveNumberFactData(input) {
  const pending=[input],seen=new Set();let nodes=0,bytes=0;
  while(pending.length){
    const value=pending.pop();assert.ok(++nodes<=1000000,'number fact metadata node limit');
    if(typeof value==='string'){
      assert.ok(value.length<=33554432-bytes,'number fact metadata byte limit');
      bytes+=Buffer.byteLength(value);assert.ok(bytes<=33554432,'number fact metadata byte limit');continue;
    }
    if(value===null||['number','boolean','undefined'].includes(typeof value))continue;
    assert.ok(typeof value==='object'&&!types.isProxy(value),'passive number fact data required');
    assert.ok([Object.prototype,Array.prototype].includes(Object.getPrototypeOf(value)),'plain number fact data required');
    assert.ok(!seen.has(value),'number fact containers cannot alias or cycle');seen.add(value);
    if(Array.isArray(value))assert.ok(value.length+nodes+pending.length<=1000000,'number fact array reservation limit');
    const keys=Reflect.ownKeys(value);assert.ok(keys.length+nodes+pending.length<=1000000,'number fact metadata node limit');
    for(const key of keys){
      assert.equal(typeof key,'string','symbolic number fact fields refuse');
      const descriptor=Object.getOwnPropertyDescriptor(value,key);assert.ok(Object.hasOwn(descriptor,'value'),'number fact accessors refuse');
      pending.push(descriptor.value);
    }
  }
}

/** A contract over supplied native facts; an actual producer remains required. */
export function planOwnedOutcomeNativeNumbers(authority,model,facts,compilerInputs,options={}) {
  readPassiveNumberFactData(facts);readPassiveNumberFactData(compilerInputs);
  const profile=readOwnedOutcomeNativeNumberProfile(compilerInputs);
  projectOrdinaryOutcome(authority,model,options);
  assert.equal(profile.compilerProfile.usizeBits,model.targetProfile.usizeBits,'actual selected target profile must match the checked owned model');
  assert.ok(Array.isArray(model.jsonRequirements),'the recursive source profile must declare every unresolved number occurrence');
  assert.ok(Array.isArray(facts)&&facts.length===model.jsonRequirements.length&&facts.length<=100000,'one supplied fact per original number occurrence');
  assert.deepEqual(Reflect.ownKeys(facts),[...facts.map((_,index)=>String(index)),'length'],'dense closed number fact sequence required');
  const selections=[];
  for(let index=0;index<facts.length;index++){
    const fact=facts[index],required=model.jsonRequirements[index];
    assert.ok(fact&&Object.getPrototypeOf(fact)===Object.prototype);
    assert.deepEqual(Reflect.ownKeys(fact),['node','typeReference','signed','unsigned','floating','payload'],'complete closed native number fact required');
    assert.equal(fact.node,required.node,'native number fact occurrence order and ownership must match');
    assert.equal(fact.typeReference,required.typeReference,'native number fact must name the actual source type');
    const kind=readOwnedOutcomeNativeNumberKind(true,profile.version,profile.arbitraryPrecision,fact.signed,fact.unsigned,fact.floating);
    assert.notEqual(kind,'','native number predicates must select one supported representation');
    assert.equal(typeof fact.payload,'string');assert.ok(fact.payload.length<=20,'bounded exact native number payload required');
    const projected=Buffer.from(required.bits,'hex').readDoubleBE();
    if(kind==='floating point'){
      assert.match(fact.payload,/^[0-9a-f]{16}$/u,'exact binary64 bits required');
      assert.equal(fact.payload,required.bits,'native floating payload and projected bits must match exactly');
    }else{
      assert.equal(fact.payload.startsWith('-'),kind==='signed integer','the selected native signed variant is strictly negative');
      assert.ok(acceptOwnedOutcomeIntegerProjection(fact.payload,kind==='signed integer',64,projected),
        'the original exact integer decimal must survive binary64 projection without rounding');
    }
    selections.push({node:required.node,typeReference:required.typeReference,kind,payload:fact.payload,projectedBits:required.bits});
  }
  return{schemaVersion:1,model:'source occurrence native number payload contract 1',planSha256:authority.sha256,
    compilerProfile:profile.compilerProfile,selections,
    boundary:'Checked supplied payload contracts only. Exact source occurrence, selected compiler profile, native variant and projected bits agree. Original object capture, live execution provenance, complete native DTO construction and physical CST ownership are not supplied.'};
}

export function checkOwnedOutcomeNativeNumberPlan(authority,model,facts,compilerInputs,selection,options={}) {
  readPassiveNumberFactData(selection);
  assert.deepEqual(selection,planOwnedOutcomeNativeNumbers(authority,model,facts,compilerInputs,options),
    'every supplied native number selection must reconstruct from the complete checked model and original payload contract');
  return selection;
}
