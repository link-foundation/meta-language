// Host traversal of the documented shared semantic projection. No projection
// tree, clone, JSON string, source span or incidental cache field is allocated.
import {readRustStandardSumSourceMetadataRecordCost,readRustStandardSumSourceMetadataTypeForm,readRustStandardSumSourceMetadataCountError} from './rust-standard-sum-source-metadata-rules.js';

function counter(){
 let nodes=0,units=0;const active=new Set();
 const fail=message=>{throw new RangeError(message);};
 const add=(count,text,depth)=>{nodes+=count;units+=text;const error=readRustStandardSumSourceMetadataCountError(nodes,units,depth);if(error)fail(error);};
 const scalar=(value,depth)=>add(1,typeof value==='string'?value.length:0,depth);
 const enter=(value,depth,read)=>{
  if(!value||typeof value!=='object')fail('standard sum source metadata needs ordinary data containers');
  if(active.has(value))fail('standard sum source metadata cannot contain a structural cycle');
  const prototype=Object.getPrototypeOf(value);if(!Array.isArray(value)&&prototype!==Object.prototype&&prototype!==null)fail('standard sum source metadata needs ordinary data containers');
  active.add(value);try{read();}finally{active.delete(value);}
 };
 const record=(form,value,depth,read)=>enter(value,depth,()=>{const cost=readRustStandardSumSourceMetadataRecordCost(form);if(cost.length===0)fail('standard sum source metadata has an unsupported projection role');add(cost[0],cost[1],depth);read();});
 const list=(values,depth,read)=>{if(!Array.isArray(values))fail('standard sum source metadata needs an ordered column');enter(values,depth,()=>{add(1,0,depth);for(const value of values)read(value,depth+1);});};
 const strings=(values,depth)=>list(values,depth,scalar);
 const owner=(value,depth)=>{
  if(value==null){scalar(null,depth);return;}
  record('owner',value,depth,()=>{scalar(value.kind,depth+1);strings(value.path,depth+1);list(value.arguments,depth+1,owner);});
 };
 const namedPath=(value,path,lookup,header,depth)=>{
  const actual=lookup?.(value.path,path);
  if(actual?.k==='data'){
   // Count the qualified path without allocating split strings or a new array.
   add(1,0,depth);scalar('crate',depth+1);
   let parts=1;for(let at=0;at<actual.fullName.length;at+=1)if(actual.fullName[at]==='.')parts+=1;
   add(parts,actual.fullName.length-(parts-1),depth+1);
  }else strings(value.path,depth);
 };
 const type=(value,depth,path=[],lookup=null,header=false)=>{
  if(value==null){scalar(null,depth);return;}
  const form=readRustStandardSumSourceMetadataTypeForm(value.kind);if(!form)fail('standard sum source metadata has an unsupported type projection');
  record(form,value,depth,()=>{
   scalar(value.kind,depth+1);
   if(form==='fixed'){scalar(value.bits,depth+1);scalar(value.signed,depth+1);}
   else if(form==='named'){if(lookup)namedPath(value,path,lookup,header,depth+1);else strings(value.path,depth+1);}
   else if(form==='data')scalar(value.name,depth+1);
   else if(['array','set','cell','option'].includes(form)){type(value.element,depth+1);if(form==='array'||form==='set')scalar(value.reference===true,depth+1);if(form==='option')scalar(value.absence??'both',depth+1);}
   else if(form==='absence')scalar(value.absence??'both',depth+1);
   else if(form==='tuple')list(value.elements,depth+1,(child,next)=>type(child,next,path,lookup,header));
   else if(form==='union')list(value.alternatives,depth+1,(child,next)=>type(child,next));
   else if(form==='function'){list(value.params,depth+1,(child,next)=>type(child,next,path,lookup,header));type(value.ret,depth+1,path,lookup,header);}
   else if(form==='record'){list(value.fields,depth+1,(field,next)=>record('record field',field,next,()=>{scalar(field.name,next+1);type(field.type,next+1);}));scalar(value.ordered===true,depth+1);scalar(value.open===true,depth+1);scalar(value.reference===true,depth+1);}
   else if(form==='map'){type(value.key,depth+1);type(value.value,depth+1);scalar(value.reference===true,depth+1);}
  });
 };
 const optionalOrigins=(values,depth)=>{if(values==null)scalar(null,depth);else list(values,depth,origin);};
 const origin=(value,depth)=>{
  if(value==null){scalar(null,depth);return;}
  record('origin',value,depth,()=>{
   type(value.type,depth+1);strings(value.path??[],depth+1);strings(value.wrappers??[],depth+1);
   owner(value.sourceOwner,depth+1);owner(value.expandedSourceOwner,depth+1);
   optionalOrigins(value.fields,depth+1);optionalOrigins(value.callableParameters,depth+1);origin(value.callableResult,depth+1);
   scalar(value.floatSourceModel??null,depth+1);scalar(value.textSourceModel??null,depth+1);scalar(value.arraySourceModel??null,depth+1);
   strings(value.sourceLifetimes??[],depth+1);strings(value.namespaceSpelling??[],depth+1);
   const sum=value.standardSumSource;
   if(sum==null)scalar(null,depth+1);else record('sum',sum,depth+1,()=>{scalar(sum.owner,depth+2);list(sum.arguments,depth+2,origin);});
  });
 };
 const identity=(value,depth,lookup,header,admitOwned)=>{
  if(!value?.type||!header&&(!value.proven||value.diverging)){scalar(null,depth);return;}
  const entry=value.type.kind==='named'?lookup(value.type.path,value.path):null,stored=entry?.k==='data'?entry.standardSumSourceOrigin:null;
  if(!header&&stored?.proven&&stored.wrappers.length===0&&(stored.copyEvidence||admitOwned?.(stored)===true)){
   record('identity sum',value,depth,()=>{scalar(stored.standardSumSource.owner,depth+1);list(stored.standardSumSource.arguments,depth+1,(child,next)=>identity(child,next,lookup,false,admitOwned));strings(value.wrappers,depth+1);});return;
  }
  record(header?'identity header':'identity value',value,depth,()=>{
   type(value.type,depth+1,value.path,lookup,header);
   if(header)strings(value.path,depth+1);
   owner(value.expandedSourceOwner??value.sourceOwner,depth+1);strings(value.wrappers,depth+1);
   if(!header){if(value.fields==null)scalar(null,depth+1);else list(value.fields,depth+1,(child,next)=>identity(child,next,lookup,false,admitOwned));scalar(value.floatSourceModel??null,depth+1);scalar(value.textSourceModel??null,depth+1);}
  });
 };
 return {origins(values){list(values,0,origin);return [nodes,units];},identities(values,lookup,header,admitOwned){list(values,0,(value,depth)=>identity(value,depth,lookup,header,admitOwned));return [nodes,units];}};
}
export function measureRustStandardSumSourceMetadata(origins){return counter().origins(origins);}
export function measureRustStandardSumSourceIdentityMetadata(origins,lookup,header=false,admitOwned=null){return counter().identities(origins,lookup,header,admitOwned);}
