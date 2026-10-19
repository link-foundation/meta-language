// Source-bound planning does not manufacture a JavaScript static lifetime.
import assert from 'node:assert/strict';
import {types} from 'node:util';
import {readOrdinaryInversePlan} from './plan.mjs';
import {projectOrdinaryOutcome} from './owned.mjs';
import {wellFormedString} from '../../../../js/src/structured-data-model.js';
import {acceptOwnedOutcomeStaticBorrowResources,findOwnedOutcomeStaticBorrowIndex,readOwnedOutcomeStaticBorrowFailure} from './static-borrow-rules.mjs';

function readPassiveRegistry(registry,typeReference) {
  assert.ok(registry&&typeof registry==='object'&&!types.isProxy(registry)&&Object.getPrototypeOf(registry)===Object.prototype,'plain passive static registry required');
  const descriptors=Object.getOwnPropertyDescriptors(registry);
  assert.deepEqual(Reflect.ownKeys(descriptors),['typeReference','values']);
  for(const descriptor of Object.values(descriptors))assert.ok(Object.hasOwn(descriptor,'value'),'registry accessors are refused');
  assert.equal(descriptors.typeReference.value,typeReference,'registry must name the actual source-owned static string reference');
  const values=descriptors.values.value;
  assert.ok(Array.isArray(values)&&!types.isProxy(values)&&Object.getPrototypeOf(values)===Array.prototype,'passive registry array required');
  const length=Object.getOwnPropertyDescriptor(values,'length').value;
  assert.ok(acceptOwnedOutcomeStaticBorrowResources(length,0),readOwnedOutcomeStaticBorrowFailure(-3));
  const fields=Object.getOwnPropertyDescriptors(values);
  assert.deepEqual(Reflect.ownKeys(fields),[...Array.from({length},(_,index)=>String(index)),'length'],'dense closed registry array required');
  const labels=[];let bytes=0;
  for(let index=0;index<length;index++){
    const descriptor=fields[index];assert.ok(Object.hasOwn(descriptor,'value'),'registry element accessors are refused');
    const text=descriptor.value;assert.equal(typeof text,'string','registry text required');
    assert.ok(acceptOwnedOutcomeStaticBorrowResources(length,bytes+text.length),readOwnedOutcomeStaticBorrowFailure(-3));
    assert.ok(wellFormedString(text),'well-formed UTF8 registry text required');
    bytes+=Buffer.byteLength(text);assert.ok(acceptOwnedOutcomeStaticBorrowResources(length,bytes),readOwnedOutcomeStaticBorrowFailure(-3));labels.push(text);
  }
  return{labels,bytes};
}

export function planOwnedOutcomeStaticBorrows(authority,model,registry,options={}) {
  const source=readOrdinaryInversePlan(authority),references=source.graph.nodes.filter(node=>node.kind==='reference');
  assert.equal(references.length,1,'the complete captured source currently has one borrowed string type');
  const reference=references[0];assert.equal(reference.lifetime,'static');assert.equal(reference.mutable,false);
  assert.equal(source.graph.nodes.find(node=>node.identity===reference.references[0]).kind,'str');
  const {labels,bytes}=readPassiveRegistry(registry,reference.identity);
  // Complete independent source/type/value validation precedes model row access.
  projectOrdinaryOutcome(authority,model,options);
  const selections=[];
  for(const required of model.borrowRequirements){
    assert.equal(required.typeReference,reference.identity);
    const occurrence=model.nodes[required.node-1],child=model.nodes[occurrence.children[0]-1],text=child.data;
    const index=findOwnedOutcomeStaticBorrowIndex(text,labels,bytes+Buffer.byteLength(text));
    if(index<0)throw new TypeError(readOwnedOutcomeStaticBorrowFailure(index));
    selections.push({node:required.node,typeReference:reference.identity,registryIndex:index,text});
  }
  // Even an unused registry must have one unambiguous interpretation.
  const admission=findOwnedOutcomeStaticBorrowIndex('',labels,bytes);
  if(admission<-1)throw new TypeError(readOwnedOutcomeStaticBorrowFailure(admission));
  return{schemaVersion:1,model:'caller supplied existing static string selection 1',planSha256:authority.sha256,
    typeReference:reference.identity,registry:{values:labels,utf8Bytes:bytes},selections,
    boundary:'A source-owned selection plan only. Native storage must supply actual existing static references with the same ordered values. No leaked allocation, callback, original pointer provenance, complete DTO reconstruction or physical CST ownership is inferred.'};
}

export function checkOwnedOutcomeStaticBorrowPlan(authority,model,registry,selection,options={}) {
  const expected=planOwnedOutcomeStaticBorrows(authority,model,registry,options);
  const pending=[selection],seen=new Set();let count=0,bytes=0;
  while(pending.length){
    const current=pending.pop();assert.ok(++count<=800000,'static selection metadata resource limit');
    if(typeof current==='string'){assert.ok(current.length<=33554432-bytes,'static selection metadata byte limit');bytes+=Buffer.byteLength(current);assert.ok(bytes<=33554432,'static selection metadata byte limit');continue;}
    if(current===null||typeof current==='number'||typeof current==='boolean')continue;
    assert.ok(typeof current==='object'&&!types.isProxy(current),'passive static selection data required');
    assert.ok([Object.prototype,Array.prototype].includes(Object.getPrototypeOf(current)),'plain static selection data required');
    if(seen.has(current))continue;seen.add(current);const keys=Reflect.ownKeys(current);assert.ok(count+pending.length+keys.length<=800000,'static selection metadata resource limit');
    for(const key of keys){assert.equal(typeof key,'string','symbolic metadata refused');const descriptor=Object.getOwnPropertyDescriptor(current,key);assert.ok(Object.hasOwn(descriptor,'value'),'static selection accessors refused');pending.push(descriptor.value);}
  }
  assert.deepEqual(selection,expected,'the complete source-owned static reference selection must match its independent registry');
  return selection;
}
