// Derive actual callback free bindings from the checked lexical graph. A
// stored/omitted capture list never grants immutable capture authority.
import {unsupported} from './diagnostics.js';
import {sameType} from './types.js';
const fail=(reason,node)=>{throw unsupported('Rust closed callback regions',reason,node?.span);};
function freeBindings(visitor,root,parameters){
 const result=new Set(),pending=[{node:root,bound:new Set(parameters),depth:0}];while(pending.length){const {node,bound,depth}=pending.pop();if(!node||typeof node!=='object')continue;visitor.visits++;visitor.budget(depth);
  if(node.k==='var'){if(!bound.has(node.name))result.add(node.name);continue;}
  if(node.k==='let'){pending.push({node:node.value,bound,depth:depth+1},{node:node.body,bound:new Set([...bound,node.name]),depth:depth+1});continue;}
  if(node.k==='lambda'){pending.push({node:node.body,bound:new Set([...bound,...node.params.map(parameter=>parameter.name)]),depth:depth+1});continue;}
  if(node.k==='match'){pending.push({node:node.scrutinee,bound,depth:depth+1});for(const arm of node.cases){const local=new Set(bound);if(arm.pattern.name)local.add(arm.pattern.name);for(const name of arm.pattern.binds??[])if(name!==null)local.add(name);pending.push({node:arm.body,bound:local,depth:depth+1});}continue;}
  for(const[key,value]of Object.entries(node))if(!['type','span','domain','from','to','captures'].includes(key)&&!key.startsWith('source')&&value&&typeof value==='object'){if(Array.isArray(value))for(const child of value)pending.push({node:child,bound,depth:depth+1});else pending.push({node:value,bound,depth:depth+1});}
 }
 return result;
}
export function checkRustClosedSourceCallback(visitor,argument,interface_,env){
 const callback=interface_.callback,parameter=callback&&interface_.parameters[callback.parameter],names=Array.isArray(argument?.params)?argument.params.map(parameter=>parameter.name):[];
 if(!callback||argument?.k!=='lambda'||names.length!==callback.parameters.length+callback.captures.length||new Set(names).size!==names.length||parameter?.type?.kind!=='function'||argument.type?.kind!=='function'||!sameType(argument.type,parameter.type)||visitor.contains(argument.body))fail('a closed callback retains its actual complete typed inline result/state body without erased source qualifiers',argument);
 visitor.closedOutput(callback.result,argument);for(const capture of callback.captures)visitor.closedOutput(capture.position,argument);for(const position of callback.parameters)visitor.closedTarget(position,argument);
 const actual=freeBindings(visitor,argument.body,names);if(argument.captures!==undefined&&(!Array.isArray(argument.captures)||new Set(argument.captures).size!==argument.captures.length||argument.captures.length!==actual.size||argument.captures.some(name=>!actual.has(name))))fail('a callback capture list agrees with its independently derived actual lexical bindings',argument);
 for(const name of actual){const value=env.get(name);if(!value||value.role!=='copy'||value.regions.length||value.storageIdentity)fail('an immutable callback capture retains its actual live direct Copy binder without loans; mutable roots use their explicit state carrier',argument);visitor.read(value,'read',argument);}
}
