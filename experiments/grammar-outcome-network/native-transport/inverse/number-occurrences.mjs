// Source correspondence only: supplied records do not attest a native execution.
import assert from 'node:assert/strict';
import {types} from 'node:util';
import {readOrdinaryInversePlan} from './plan.mjs';
import {projectOrdinaryOutcome} from './owned.mjs';
import {planOwnedOutcomeNativeNumbers} from './number-facts.mjs';

/** Resolve JSON number sites from actual source fields, never matching values. */
export function readOwnedNumberWireCorrespondence(authority,model,options={}) {
  const projection=projectOrdinaryOutcome(authority,model,options),plan=readOrdinaryInversePlan(authority);
  const sources=new Map(plan.graph.nodes.map(row=>[row.identity,row]));
  const bindings=new Map(plan.bindings.map(row=>[row.declarationReference,row]));
  const pending=[{id:1,path:[]}],sites=new Map();let routeBytes=0,visited=0;
  const add=(id,path)=>{
    const size=Buffer.byteLength(JSON.stringify(path));
    assert.ok(size<=1048576&&(routeBytes+=size)<=16777216,'number correspondence route byte budget');
    assert.ok(pending.length<100000,'number correspondence pending node budget');pending.push({id,path});
  };
  while(pending.length){
    const {id,path}=pending.pop(),row=model.nodes[id-1],source=sources.get(row.typeReference);
    assert.ok(++visited<=100000,'number correspondence node budget');
    if(['named','type_item','Box','reference','Option'].includes(source.kind)){
      if(row.children.length)add(row.children[0],path);
    }else if(source.kind==='Vec')row.children.forEach((child,index)=>add(child,[...path,index]));
    else if(source.kind==='IndexMap'){
      for(let index=0;index<row.children.length;index+=2){
        const key=model.nodes[row.children[index]-1];assert.equal(key.form,'String');
        add(row.children[index+1],[...path,key.data]);
      }
    }else if(source.kind==='struct_item'||source.kind==='enum_item'){
      const binding=bindings.get(source.identity),selected=source.kind==='enum_item'?binding.variants.find(item=>item.identity===row.data):binding;
      if(selected.fields){
        for(const entry of selected.wire){
          if(entry.literal)continue;
          const index=selected.fields.findIndex(field=>field.identity===entry.fieldReference);assert.ok(index>=0);
          // The sole flattened LeafText payload contains strings, not JSON.
          if(entry.flatten){
            const named=sources.get(selected.fields[index].typeReference);assert.equal(named.kind,'named');
            const leaf=sources.get(named.owner);assert.equal(leaf.name,'LeafText');
            for(const variant of bindings.get(leaf.identity).variants){
              assert.equal(variant.tuple.length,1);assert.equal(sources.get(variant.tuple[0].typeReference).kind,'String');
            }
            continue;
          }
          add(row.children[index],[...path,entry.key]);
        }
      }else{assert.equal(row.children.length,1);add(row.children[0],path);}
    }else if(source.kind==='json'){
      if(row.data.kind==='number'){
        const key=JSON.stringify(path);assert.ok(!sites.has(key),'distinct source occurrences require distinct wire paths');
        sites.set(key,{node:row.id,typeReference:row.typeReference,projectedBits:row.data.bits});
      }else if(['array','object'].includes(row.data.kind))row.children.forEach((child,index)=>add(child,[...path,row.data.kind==='array'?index:row.data.keys[index]]));
    }
  }
  assert.ok(Array.isArray(model.jsonRequirements));assert.equal(sites.size,model.jsonRequirements.length,'every checked JSON number site is represented');
  const frames=[],correspondence=[];let ordinal=0,rootValues=0;
  for(const event of projection.events){
    if(['endArray','endObject'].includes(event.kind)){
      const frame=frames.pop();assert.ok(frame);assert.equal(frame.kind,event.kind==='endArray'?'array':'object');continue;
    }
    if(event.kind==='endPair')continue;
    ordinal++;
    if(event.kind==='root'){assert.equal(frames.length,0);assert.equal(rootValues,1);continue;}
    if(event.kind==='pair'){assert.equal(frames.at(-1)?.kind,'object');continue;}
    if(event.key){const frame=frames.at(-1);assert.equal(frame?.kind,'object');assert.equal(frame.key,null);frame.key=event.value;continue;}
    let path=[];const parent=frames.at(-1);
    if(parent){
      if(parent.kind==='array')path=[...parent.path,parent.index++];
      else{assert.notEqual(parent.key,null);path=[...parent.path,parent.key];parent.key=null;}
    }else rootValues++;
    if(event.kind==='number'){
      const key=JSON.stringify(path),site=sites.get(key);
      if(site){
        const bits=Buffer.alloc(8);bits.writeDoubleBE(event.value);assert.equal(bits.toString('hex'),site.projectedBits);
        correspondence.push({wireOccurrence:ordinal,...site,path});sites.delete(key);
      }
    }
    if(['array','object'].includes(event.kind))frames.push({kind:event.kind,path,index:0,key:null});
  }
  assert.equal(sites.size,0,'every source number must occur at its exact wire path');
  assert.equal(frames.length,0);assert.equal(rootValues,1);
  return{schemaVersion:1,planSha256:authority.sha256,expandedOccurrences:ordinal,correspondence,
    boundary:'Complete checked projected source correspondence only. Expanded wire ordinals identify actual owned JSON sites through source field rules. Equal values and host integer fields cannot substitute for original number sites. Native capture, execution provenance and DTO reconstruction remain separate obligations.'};
}

/** Check passive primitive records before any supplied property is read. */
function readOriginalNumberObservationRecords(input) {
  assert.ok(input&&typeof input==='object'&&!types.isProxy(input)&&Array.isArray(input),'passive observation array required');
  assert.equal(Object.getPrototypeOf(input),Array.prototype);
  const size=Object.getOwnPropertyDescriptor(input,'length').value;assert.ok(size<=100000,'observation count budget');
  assert.deepEqual(Reflect.ownKeys(input),[...Array.from({length:size},(_,index)=>String(index)),'length']);
  const records=[];
  for(let index=0;index<size;index++){
    const entry=Object.getOwnPropertyDescriptor(input,String(index));assert.ok(Object.hasOwn(entry,'value'),'observation getters refuse');
    const row=entry.value;assert.ok(row&&typeof row==='object'&&!types.isProxy(row),'passive observation record required');
    assert.equal(Object.getPrototypeOf(row),Object.prototype);
    const keys=['wireOccurrence','signed','unsigned','floating','signedDecimal','unsignedDecimal','floatingBits'];assert.deepEqual(Reflect.ownKeys(row),keys);
    const copy={};for(const key of keys){
      const descriptor=Object.getOwnPropertyDescriptor(row,key);assert.ok(Object.hasOwn(descriptor,'value'),'observation getters refuse');
      const value=descriptor.value;assert.ok(value===null||['string','number','boolean'].includes(typeof value),'primitive observation values required');
      if(typeof value==='string')assert.ok(value.length<=20,'observation scalar budget');copy[key]=value;
    }
    records.push(copy);
  }
  return records;
}

/** Join supplied raw native records by exact occurrence, with full payload checks. */
export function bindOwnedNumberWireObservations(authority,model,input,compilerInputs,options={}) {
  const records=readOriginalNumberObservationRecords(input),correspondence=readOwnedNumberWireCorrespondence(authority,model,options);
  assert.equal(records.length,correspondence.correspondence.length,'exactly one observation per source number required');
  const byNode=new Map();
  for(let index=0;index<records.length;index++){
    const row=records[index],site=correspondence.correspondence[index];
    assert.equal(row.wireOccurrence,site.wireOccurrence,'original wire occurrence order must match exactly');
    for(const key of ['signed','unsigned','floating'])assert.equal(typeof row[key],'boolean');
    assert.equal(row.floatingBits,site.projectedBits,'every original Number as_f64 payload must match its exact source site');
    for(const [flag,key]of [['signed','signedDecimal'],['unsigned','unsignedDecimal']]){
      if(row[flag]){assert.equal(typeof row[key],'string');assert.match(row[key],/^-?(?:0|[1-9][0-9]*)$/u);}
      else assert.equal(row[key],null,'absent original integer projections remain absent');
    }
    if(row.unsigned){
      assert.ok(BigInt(row.unsignedDecimal)>=0n&&BigInt(row.unsignedDecimal)<(1n<<64n));
      assert.equal(row.signed,BigInt(row.unsignedDecimal)<(1n<<63n),'positive integer predicate overlap follows its exact payload');
      if(row.signed)assert.equal(row.signedDecimal,row.unsignedDecimal);
    }else if(row.signed)assert.ok(BigInt(row.signedDecimal)>=-(1n<<63n)&&BigInt(row.signedDecimal)<0n);
    byNode.set(site.node,{node:site.node,typeReference:site.typeReference,signed:row.signed,unsigned:row.unsigned,floating:row.floating,
      payload:row.floating?row.floatingBits:row.unsigned?row.unsignedDecimal:row.signedDecimal});
  }
  const facts=model.jsonRequirements.map(row=>{const fact=byNode.get(row.node);assert.ok(fact);return fact;});
  return{schemaVersion:1,correspondence,numberPlan:planOwnedOutcomeNativeNumbers(authority,model,facts,compilerInputs,options),
    boundary:'Supplied native observation records checked against exact source occurrences and selected compiler inputs. This join does not attest that supplied data came from a live original native execution and does not construct native DTOs.'};
}
