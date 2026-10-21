// Derive only from the captured complete source graph. Caller-supplied numeric
// schemas are not authorities. Scalar/native/borrow validation remains owned.
import assert from 'node:assert/strict';
import {readOrdinaryInversePlan} from './plan.mjs';
import {projectOrdinaryOutcome} from './owned.mjs';

const schemas=new WeakMap();

export function deriveOwnedOutcomeTypeSchema(authority) {
  if(schemas.has(authority))return structuredClone(schemas.get(authority));
  const plan=readOrdinaryInversePlan(authority),nodes=plan.graph.nodes;
  const sourceTypeReferences=nodes.map(node=>node.identity),index=new Map(sourceTypeReferences.map((id,at)=>[id,at]));
  const reference=id=>{assert.ok(index.has(id),'every type edge retains its actual complete source owner');return index.get(id);};
  const modes=[],patterns=[],patternOffsets=[0],patternTypes=[],variantOwners=[],variantReferences=[];
  const append=(children,owner=-1,variant=null)=>{
    const id=variantOwners.length;patternTypes.push(...children.map(reference));patternOffsets.push(patternTypes.length);
    variantOwners.push(owner);variantReferences.push(variant);return id;
  };
  for(const node of nodes){
    let mode=0,children=[];
    if(node.kind==='named')children=[node.owner];
    else if(node.kind==='type_item')children=[node.aliasTypeReference];
    else if(node.kind==='Box')children=[node.arguments[0]];
    else if(node.kind==='reference')children=[node.references[0]];
    else if(node.kind==='struct_item')children=plan.bindings.find(binding=>binding.declarationReference===node.identity).fields.map(field=>field.typeReference);
    else if(node.kind==='Option'){mode=1;children=[node.arguments[0]];}
    else if(node.kind==='Vec'){mode=2;children=[node.arguments[0]];}
    else if(node.kind==='IndexMap'){mode=3;children=node.arguments;}
    else if(node.kind==='enum_item')mode=4;
    else if(node.kind==='json')mode=plan.admission.opaqueJson==='null only'?0:5;
    else if(!['String','str','bool','usize','i64'].includes(node.kind))mode=-1;
    modes.push(mode);patterns.push(append(children));
  }
  for(const binding of plan.bindings)for(const variant of binding.variants??[]){
    if(plan.admission.refusedVariants.includes(variant.identity))continue;
    append((variant.fields??variant.tuple).map(field=>field.typeReference),reference(binding.declarationReference),variant.identity);
  }
  const schema={schemaVersion:1,planSha256:authority.sha256,rootReference:plan.rootReference,
    sourceTypeReferences,variantReferences,modes,patterns,patternOffsets,patternTypes,variantOwners,
    sourceRoots:[...plan.graph.roots],sourcePins:structuredClone(plan.graph.sourceInventory.pins),
    modeDefinitions:['fixed source fields','optional source child','repeated source child','ordered source key and value pairs',
      'source-qualified variant fields','recursive JSON projection children'],
    boundary:'Source-type edges only. The caller must separately authenticate this captured schema and verify root identity, preorder/resources, scalar values, unresolved native JSON variants, static references and actual DTO/physical CST ownership.'};
  assert.equal(schema.sourceRoots.length,9);assert.equal(schema.sourcePins.length,13);
  schemas.set(authority,structuredClone(schema));return schema;
}

/** Full independent owned validation precedes access to caller rows. */
export function readOwnedOutcomeTypeColumns(authority,model,options={}) {
  projectOrdinaryOutcome(authority,model,options);
  const schema=deriveOwnedOutcomeTypeSchema(authority),plan=readOrdinaryInversePlan(authority);
  const nodes=new Map(plan.graph.nodes.map(node=>[node.identity,node]));
  const types=new Map(schema.sourceTypeReferences.map((id,index)=>[id,index]));
  const variants=new Map(schema.variantReferences.flatMap((id,index)=>id===null?[]:[[id,index]]));
  const typesColumn=[],selectors=[],offsets=[0],children=[];
  for(const row of model.nodes){
    const source=nodes.get(row.typeReference);assert.ok(source);
    typesColumn.push(types.get(row.typeReference));let selector=-1;
    if(source.kind==='Option')selector=row.data?1:0;
    else if(source.kind==='enum_item'){assert.ok(variants.has(row.data));selector=variants.get(row.data);}
    else if(source.kind==='json'&&plan.admission.opaqueJson!=='null only'){
      selector=['null','boolean','string','number','array','object'].indexOf(row.data.kind);assert.ok(selector>=0);
    }
    selectors.push(selector);children.push(...row.children);offsets.push(children.length);
  }
  return {schema,types:typesColumn,selectors,offsets,children};
}
