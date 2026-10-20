// Derive inverse instructions from complete authenticated source types and wire rules.
// It does not reconstruct a value or supply native execution evidence.
import assert from 'node:assert/strict';
import {fileURLToPath} from 'node:url';
import {createHash} from 'node:crypto';
import {captureNativeOutcomeTypeInventory} from '../types/inventory.mjs';
import {buildOrderedWirePlan} from '../wire/generate.mjs';
import {serializeProjectData,writeProjectData} from '../../../../js/scripts/project-data.mjs';
import {projectHistoricalOutcomeInventory} from '../../../grammar-context-source-metadata/historical-inventory.mjs';

const repository=fileURLToPath(new URL('../../../../',import.meta.url));
const copy=value=>structuredClone(value);
const digest=value=>createHash('sha256').update(value).digest('hex');
const authorities=new WeakMap();
const canonical=value=>serializeProjectData(value,'ordinary-inverse-plan.lino');
const memberIdentity=(owner,member)=>owner+'@'+member.span.start+':'+member.span.end+'#'+member.sourceSha256;

/** Read the actual fixed source corpus; never authenticate a caller's inventory. */
export function captureOrdinaryInversePlan(root=repository) {
  const capture=captureNativeOutcomeTypeInventory(root),graph=capture.derive();
  graph.sourceInventory=projectHistoricalOutcomeInventory(capture.sourceInventory);
  const wirePlan=buildOrderedWirePlan(graph.sourceInventory);
  const byIdentity=new Map(graph.nodes.map(node=>[node.identity,node]));
  const declarations=graph.roots.map(identity=>byIdentity.get(identity));
  assert.equal(declarations.length,9,'all original declarations remain present');
  assert.equal(graph.sourceInventory.pins.length,13,'all original producer/source-owner pins remain present');
  const exactDeclaration=name=>{
    // The matching name is only a locator inside the already authenticated
    // corpus. Its qualified identity and complete declaration bytes are kept.
    const matches=declarations.filter(node=>node.name===name);
    assert.equal(matches.length,1,'wire name has exactly one authenticated declaration owner');
    return matches[0];
  };
  const field=(owner,source)=>({
    identity:memberIdentity(owner,source),nativeName:source.name??null,
    typeReference:source.typeReference,source:copy(source)
  });
  const entries=(owner,sourceFields,rows)=>{
    const fields=sourceFields.map(source=>field(owner,source));
    const members=rows.filter(row=>!row.literal).map(row=>row.native);
    assert.equal(new Set(members).size,members.length,'each source member has exactly one wire instruction');
    assert.deepEqual([...members].sort(),sourceFields.map(item=>item.name).sort(),'complete member conservation');
    return {fields,wire:rows.map(row=>row.literal?copy(row):{
      ...copy(row),fieldReference:fields.find(item=>item.nativeName===row.native).identity,
      typeReference:fields.find(item=>item.nativeName===row.native).typeReference
    })};
  };
  const bindings=wirePlan.shapes.map(shape=>{
    const owner=exactDeclaration(shape.native);
    if(shape.entries) return {
      declarationReference:owner.identity,...entries(owner.identity,owner.fields,shape.entries)
    };
    assert.equal(shape.variants.length,owner.variants.length,'every actual variant is retained');
    return {declarationReference:owner.identity,variants:shape.variants.map(row=>{
      const variants=owner.variants.filter(item=>item.name===row.name);
      assert.equal(variants.length,1,'wire variant has one complete actual source owner');
      const source=variants[0],identity=memberIdentity(owner.identity,source);
      return {identity,nativeName:source.name,source:copy(source),
        ...(row.entries?entries(identity,source.fields,row.entries):{
          tuple:source.tuple.map(item=>field(identity,item)),wire:copy(row)
        })};
    })};
  });
  const syntaxTree=exactDeclaration('SyntaxTree');
  const syntaxBinding=bindings.find(item=>item.declarationReference===syntaxTree.identity);
  const admittedNames=['Node','Token','Error','Missing'];
  const ordinaryVariants=syntaxBinding.variants.filter(item=>admittedNames.includes(item.nativeName)).map(item=>item.identity);
  assert.equal(ordinaryVariants.length,admittedNames.length,'finite ordinary profile binds all four actual variants');
  const refusedVariants=syntaxBinding.variants.filter(item=>!ordinaryVariants.includes(item.identity)).map(item=>item.identity);
  assert.equal(refusedVariants.length,1,'current finite profile explicitly excludes one source-qualified variant');
  assert.equal(syntaxBinding.variants.find(item=>refusedVariants.includes(item.identity)).nativeName,'Embed');
  const plan={schemaVersion:1,profile:'ordinary owned raw outcome 1',
    rootReference:exactDeclaration(wirePlan.root).identity,
    graph,wirePlan,bindings,
    admission:{ordinaryVariants,refusedVariants,opaqueJson:'null only',
      targetUsizeWidths:[32,64],maximumDepth:256,numberFacts:'exactly empty',
      physicalOwnership:'unproved',nativeDtoReconstruction:'not supplied',
      staticReferences:'owned text plus the exact unresolved borrowed-static descriptor'}
  };
  const text=canonical(plan),sha256=digest(text);
  const authority=Object.freeze({sha256,plan:()=>copy(plan)});
  authorities.set(authority,{text,plan:copy(plan)});
  return authority;
}

/** A valid authority cannot be fabricated by a caller-supplied verifier. */
export function authenticateOrdinaryInversePlan(authority,candidate) {
  const trusted=authorities.get(authority);
  assert.ok(trusted,'plan authority must come from an actual source capture');
  assert.equal(canonical(candidate),trusted.text,'complete generated shared Links plan conservation');
  return copy(trusted.plan);
}

/** Read a captured plan without invoking a caller-supplied method. */
export function readOrdinaryInversePlan(authority) {
  const trusted=authorities.get(authority);
  assert.ok(trusted,'plan authority must come from an actual source capture');
  return copy(trusted.plan);
}

/** Expand projection coverage without claiming that erased native facts exist. */
export function extendRecursiveInversePlan(authority) {
  const plan=readOrdinaryInversePlan(authority);
  assert.equal(plan.profile,'ordinary owned raw outcome 1','extension starts at the authenticated ordinary profile');
  plan.profile='recursive owned raw outcome projection 1';
  plan.admission.refusedVariants=[];
  plan.admission.opaqueJson='recursive owned projection with unresolved native number representation';
  plan.admission.containerAliases='preserve JSON projection aliases as checked occurrence links; other owned containers remain distinct';
  const text=canonical(plan),sha256=digest(text);
  const extended=Object.freeze({sha256,plan:()=>copy(plan)});
  authorities.set(extended,{text,plan:copy(plan)});
  return extended;
}

/** Explicit shared Links output only; importing this module never writes a file. */
export function writeOrdinaryInversePlan(authority,output) {
  assert.equal(typeof output,'string');assert.ok(output.endsWith('.lino'),'inverse plans use Links Notation');
  const plan=readOrdinaryInversePlan(authority);
  writeProjectData(output,plan);
  return {sha256:authority.sha256,output};
}
