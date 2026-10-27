import {readRustCallableOwnerProjectionLimit,readRustCallableOwnerProjectionFailure} from './rust-callable-owner-rules.js';
// Generic source-origin tree transport; interpretation remains in shared rules.
import {readRustPointerIntegerContractOwner} from './rust-pointer-integer-owners.js';
export function projectRustCallableOwnerDescriptor(origin){
 const pending=[{origin,path:'',active:false,depth:0}],rows=[];let visits=0;
 while(pending.length){const item=pending.pop(),value=item.origin;if(!value)continue;visits+=1;if(visits>readRustCallableOwnerProjectionLimit('visits')||item.depth>readRustCallableOwnerProjectionLimit('depth'))return [readRustCallableOwnerProjectionFailure(),readRustCallableOwnerProjectionFailure()];const callable=value.type?.kind==='function',active=item.active||callable;
  if(active)rows.push(item.path,readRustPointerIntegerContractOwner(value.expandedSourceOwner??value.sourceOwner,value.integerSourceOwnerConstraint??''));
  const fields=value.fields??[];for(let at=fields.length-1;at>=0;at-=1)pending.push({origin:fields[at],path:item.path+'/field/'+at,active,depth:item.depth+1});
  if(callable){if(value.callableResult)pending.push({origin:value.callableResult,path:item.path+'/result',active:true,depth:item.depth+1});const parameters=value.callableParameters??[];for(let at=parameters.length-1;at>=0;at-=1)pending.push({origin:parameters[at],path:item.path+'/parameter/'+at,active:true,depth:item.depth+1});}
  if(rows.length>readRustCallableOwnerProjectionLimit('cells'))return [readRustCallableOwnerProjectionFailure(),readRustCallableOwnerProjectionFailure()];
 }
 return rows;
}
