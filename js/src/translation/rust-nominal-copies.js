// Source transport only: actual declaration identities, derivations and fields.
import {planRustNominalCopyOwners,acceptRustNominalCopyFieldLeaf} from './rust-nominal-copy-rules.js';
import {unsupported} from './diagnostics.js';
export function registerRustNominalCopyDeclaration(origins,name,path,derivations){
 const traits=derivations.flatMap(attribute=>attribute.traits);
 origins.nominalCopyDerivations.set([...path,name].join('.'),{copy:traits.includes('Copy'),clone:traits.includes('Clone'),span:derivations.find(attribute=>attribute.traits.includes('Copy'))?.span});
}
export function retainRustNominalCopyOrigin(origins,origin){
 if(origin.fields)origin.fields.forEach(field=>retainRustNominalCopyOrigin(origins,field));
 if(origin.callableParameters)origin.callableParameters.forEach(field=>retainRustNominalCopyOrigin(origins,field));
 if(origin.callableResult)retainRustNominalCopyOrigin(origins,origin.callableResult);
 if(origin.type?.kind==='named'){
  const owner=origins.lookup(origin.type.path,origin.path,['data']);
  origin.copyEvidence=Boolean(owner&&origins.nominalCopyOwners.has(owner.fullName));
 }else if(origin.type?.kind==='tuple')origin.copyEvidence=Boolean(origin.fields?.every(field=>origins.immutableProduct(field)));
 return origin;
}
export function finishRustNominalCopies(origins){
 const owners=[...origins.nominalCopyDerivations.keys()].sort(),copies=[],clones=[],starts=[],counts=[],targets=[],leaves=[];
 const field=origin=>{
  if(origin.type?.kind==='tuple'&&!origin.wrappers.length&&origin.fields){origin.fields.forEach(field);return;}
  const owner=origin.type?.kind==='named'?origins.lookup(origin.type.path,origin.path,['data']):null;
  targets.push(owner?owners.indexOf(owner.fullName):-1);
  leaves.push(acceptRustNominalCopyFieldLeaf(origin.type?.kind??'',origin.wrappers,origin.copyEvidence===true&&origin.proven===true));
 };
 for(const owner of owners){
  const derive=origins.nominalCopyDerivations.get(owner);copies.push(derive.copy);clones.push(derive.clone);starts.push(targets.length);
  for(const declaration of origins.catalog.values())if(declaration.k==='ctor'&&declaration.owner===owner)declaration.fields.forEach(field);
  counts.push(targets.length-starts.at(-1));
 }
 const plan=planRustNominalCopyOwners(owners,copies,clones,starts,counts,targets,leaves);
 if(plan[0]!==0)throw unsupported('Rust nominal Copy derivation',plan[0]===-2?'a derived Copy owner requires its actual Clone derivation':plan[0]===-3?'every field of every derived Copy variant needs an actual Copy proof':'the complete nominal Copy source graph must retain valid distinct owners and fields',origins.nominalCopyDerivations.get(owners[plan[1]])?.span);
 origins.nominalCopyOwners=new Set(owners.filter((_,index)=>plan[index+1]===1));
 for(const declaration of origins.catalog.values()){
  const values=declaration.k==='fn'?[...declaration.parameters,declaration.result]:declaration.k==='ctor'?[...declaration.fields,declaration.result]:declaration.k==='const'?[declaration.result]:[];
  values.forEach(origin=>retainRustNominalCopyOrigin(origins,origin));
 }
}
