// Actual type/declaration traversal only; all profile decisions are shared.
import {typeChildren} from './emit-common.js';
import {acceptCallableDataNode,acceptCallableDataConstructiveNode,readCallableDataTraversalError,readCallableDataObservationError,readCallableDataRecursivePlan} from './callable-data-derivation-rules.js';
import {readFormalDatatypeConstructivePlan} from './callable-data-derivation-rules.js';
import {unsupported} from './diagnostics.js';
import {acceptJavaScriptClassOwnerTypeObservation} from './javascript-class-owners.js';
export function readCallableDataFacts(type,declarations){
 let visits=0;
 const active=new Set();
 const completed=new Map();
 function checkTraversal(depth){
  const error=readCallableDataTraversalError(depth,visits);
  if(error)throw unsupported('callable data source profile',error);
 }
 function node(kind,children,childOffset,acyclic=true){
  return {
   callable:acceptCallableDataNode(kind,children.map(child=>child.callable)),
   constructive:acceptCallableDataConstructiveNode(kind,children.map(child=>child.constructive)),
   acyclic:acyclic&&children.every(child=>child.acyclic),
   height:children.reduce((height,child)=>Math.max(height,childOffset+child.height),0),
  };
 }
 function walk(value,depth){
  visits+=1;
  checkTraversal(depth);
  if(value.kind!=='data')return node(value.kind,(value.kind==='union'?value.alternatives:typeChildren(value)).map(child=>walk(child,depth+1)),1);
  if(active.has(value.name))return node('cycle',[],0,false);
  const cached=completed.get(value.name);
  if(cached!==undefined){
   checkTraversal(depth+cached.height);
   return cached;
  }
  const entry=declarations.get(value.name);if(entry?.k!=='data')return node('missing',[],0,false);
  active.add(value.name);
  try{
   const facts=node('data',entry.ctors.map(ctor=>node('constructor',ctor.fields.map(field=>walk(field.type,depth+1)),1)),0);
   if(facts.acyclic)completed.set(value.name,facts);
   return facts;
  }
  finally{active.delete(value.name);}
 }
 const {callable,constructive,acyclic}=walk(type,0);
 return {callable,constructive,acyclic};
}
export function refuseCallableDataObservation(type,declarations,operation,span,witness=undefined){if(acceptJavaScriptClassOwnerTypeObservation(type,witness,operation))return;const reason=readCallableDataObservationError(operation,readCallableDataFacts(type,declarations).callable);if(reason)throw unsupported('callable aggregate observation',reason,span);}
function projectDatatypeConstructiveGraph(type,declarations){
 const kinds=[],ends=[],edges=[],references=[],roots=[],names=[],positions=new Map();let visits=0;
 function node(kind,children,reference=-1){const error=readCallableDataTraversalError(0,kinds.length+1);if(error)throw unsupported('callable data source profile',error);const at=kinds.length;kinds.push(kind);edges.push(...children);ends.push(edges.length);references.push(reference);return at;}
 function walk(value,depth){
  const error=readCallableDataTraversalError(depth,++visits);if(error)throw unsupported('callable data source profile',error);
  if(value.kind==='data'){
   if(!positions.has(value.name)){positions.set(value.name,names.length);names.push(value.name);roots.push(-1);}
   return node('reference',[],positions.get(value.name));
  }
  return node(value.kind,(value.kind==='union'?value.alternatives:typeChildren(value)).map(child=>walk(child,depth+1)));
 }
 const root=walk(type,0);
 for(let at=0;at<names.length;at+=1){
  const entry=declarations.get(names[at]);if(entry?.k!=='data')throw unsupported('callable data source profile','the actual callable graph needs every declaration');
  roots[at]=node('data',entry.ctors.map(ctor=>node('constructor',ctor.fields.map(field=>walk(field.type,0)))));
 }
 return {kinds,ends,edges,references,roots,root,names};
}
export function readCallableDataRecursiveProfile(type,declarations){
 const {kinds,ends,edges,references,roots,root}=projectDatatypeConstructiveGraph(type,declarations);
 const plan=readCallableDataRecursivePlan(kinds,ends,edges,references,roots,root);if(plan[0]==='error')throw unsupported('callable data field',plan[1]);
 return {callable:plan[1]==='true',recursive:plan[2]==='true',constructive:plan[3]==='true',witness:Number(plan[4])};
}
export function readFormalDatatypeConstructiveProfiles(type,declarations){
 const {kinds,ends,edges,references,roots,root,names}=projectDatatypeConstructiveGraph(type,declarations);
 const plan=readFormalDatatypeConstructivePlan(kinds,ends,edges,references,roots,root);if(plan[0]==='error')throw unsupported('formal mutual datatype',plan[1]);
 if(plan[0]!=='witnesses'||plan.length!==1+2*names.length)throw unsupported('formal mutual datatype','the actual complete graph needs each finite constructor witness');
 return new Map(names.map((name,index)=>[name,{witness:Number(plan[1+index*2]),rank:Number(plan[2+index*2])}]));
}
