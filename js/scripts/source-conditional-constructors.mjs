// Source construction only. No condition, field expression or represented
// program is executed. Actual checked declarations supply all type owners.
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {readAuthoredJavaScriptModuleDependencies} from './source-module-syntax.mjs';
import {sameType,typeKey,FLOAT,BOOL} from '../src/translation/types.js';
import {testConditionalConstructorOwners,planConditionalConstructorFieldSelection} from '../src/translation/conditional-constructor-sharing-rules.js';
const excluded=new Set(['type','domain','from','to','span','pattern']);
const conditionalKeys=new Set(['k','cond','then','else','type','span']);
const constructorKeys=new Set(['k','data','ctor','args','type','span','frozen']);
export function shareSourceConditionalConstructors(program){
 if(!(program.declarations instanceof Map)||!Array.isArray(program.items)||program.sourceLanguage!=='JavaScript'||![undefined,false].includes(program.referenceSemantics))
  throw new TypeError('conditional constructor sharing requires the checked JavaScript value model');
 const proofs=[],reserved=new Set([...program.declarations.keys()]);let visits=0,ordinal=0;
 const normalized=value=>value.replace(/([a-z0-9])([A-Z])/gu,'$1_$2').replace(/[^A-Za-z0-9_]/gu,'_').toLowerCase();
 const seen=new WeakSet();
 function reserve(value){
  if(typeof value==='string'){if(value.length<=256)reserved.add(normalized(value));return;}
  if(!value||typeof value!=='object'||seen.has(value))return;seen.add(value);
  if(++visits>1000000)throw new TypeError('conditional constructor identifiers exceed their work budget');
  for(const child of Array.isArray(value)?value:Object.values(value))reserve(child);
 }
 for(const entry of program.declarations.values())reserve(entry);
 for(const external of program.externals??[])reserve(external);
 function fresh(){let name;do{name='shared_constructor_selection_'+(++ordinal);}while(reserved.has(name));reserved.add(name);return name;}
 const closedCache=new Map();
 function closed(type){
  const pending=[type],objects=new WeakSet(),names=new Set();let result=true;
  while(pending.length){
   const value=pending.pop();
   if(++visits>1000000)throw new TypeError('constructor field types exceed their work budget');
   if(!value||typeof value!=='object'||![undefined,false].includes(value.reference)){result=false;break;}
   if(objects.has(value))continue;objects.add(value);
   if(['bool','float','int','nat','string','unit'].includes(value.kind))continue;
   if(['array','option'].includes(value.kind)){pending.push(value.element);continue;}
   if(value.kind==='tuple'&&Array.isArray(value.elements)&&value.elements.length<=256){pending.push(...value.elements);continue;}
   if(value.kind==='record'&&[undefined,false].includes(value.open)&&Array.isArray(value.fields)&&value.fields.length<=256){for(const field of value.fields)pending.push(field.type);continue;}
   if(value.kind!=='data'){result=false;break;}
   if(closedCache.has(value.name)){if(!closedCache.get(value.name)){result=false;break;}continue;}
   if(names.has(value.name))continue;names.add(value.name);
   const declaration=program.declarations.get(value.name);
   if(declaration?.k!=='data'||!Array.isArray(declaration.ctors)||declaration.ctors.length>256||declaration.ctors.some(constructor=>constructor.fields.length>256)){result=false;break;}
   for(const constructor of declaration.ctors)for(const field of constructor.fields)pending.push(field.type);
  }
  if(result)for(const name of names)closedCache.set(name,true);
  else if(type?.kind==='data')closedCache.set(type.name,false);
  return result;
 }
 // Typed structural framing preserves absent fields, exact scalar bits and
 // isolated UTF16 units. Hashes bind source receipts, never atom equivalence.
 function fingerprint(value,maximumDepth=240){
  let size=0,count=0;const hash=createHash('sha256'),active=new WeakSet();
  function feed(text){const bytes=Buffer.from(text,'utf16le');size+=bytes.length;if(size>2097152)throw new RangeError('source fingerprint budget');hash.update(bytes);}
  function walk(current,depth=0){
   if(depth>maximumDepth)throw new RangeError('source fingerprint depth');
   if(++count>100000)throw new RangeError('source fingerprint visits');
   const kind=typeof current;
   if(current===null){feed('null;');return;}
   if(kind==='string'){feed('text:'+current.length+':');feed(current);return;}
   if(kind==='undefined'){feed('undefined;');return;}
   if(kind==='boolean'){feed(current?'true;':'false;');return;}
   if(kind==='bigint'){feed('integer:'+String(current)+';');return;}
   if(kind==='number'){const bytes=Buffer.alloc(8);bytes.writeDoubleBE(current);feed('number:');hash.update(bytes);size+=8;return;}
   if(kind!=='object'||active.has(current))throw new TypeError('source fingerprint needs acyclic data');
   const prototype=Object.getPrototypeOf(current),array=Array.isArray(current);
   if(!array&&prototype!==Object.prototype&&prototype!==null)throw new TypeError('source fingerprint rejects opaque objects');
   active.add(current);const keys=Reflect.ownKeys(current);
   feed((array?'array:':prototype===null?'null record:':'record:')+keys.length+':');
   for(const key of keys){const descriptor=Object.getOwnPropertyDescriptor(current,key);
    if(typeof key!=='string'||!Object.hasOwn(descriptor,'value')||!descriptor.enumerable&&!(array&&key==='length'))throw new TypeError('source fingerprint rejects hidden fields');
    feed('key:'+key.length+':');feed(key);walk(descriptor.value,depth+1);}
   active.delete(current);
  }
  try{walk(value);if(size>2097152)return null;return {size,sha256:hash.digest('hex')};}catch(error){if(error instanceof TypeError||error instanceof RangeError)return null;throw error;}
 }
 function atom(value){
  if(value?.k==='var'&&Object.keys(value).every(key=>['k','name','type','span'].includes(key)))return true;
  if(value?.k==='unit'&&Object.keys(value).every(key=>['k','type','span'].includes(key)))return true;
  if(value?.k==='none'&&Object.keys(value).every(key=>['k','absence','type','span'].includes(key)))return true;
  if(value?.k==='ctor'&&Array.isArray(value.args)&&value.args.length===0&&Object.keys(value).every(key=>constructorKeys.has(key))){
   const declaration=program.declarations.get(value.data);
   return value.type?.kind==='data'&&value.type.name===value.data&&declaration?.k==='data'
    &&declaration.ctors.some(constructor=>constructor.name===value.ctor&&constructor.fields.length===0)&&closed(value.type);
  }
  return value?.k==='lit'&&Object.keys(value).every(key=>['k','value','type','span'].includes(key))
   &&['bool','string','float','int','nat'].includes(value.type?.kind)&&['boolean','string'].includes(typeof value.value);
 }
 function equalAtoms(left,right){
  if(!atom(left)||!atom(right)||left.k!==right.k||!sameType(left.type,right.type))return false;
  if(left.k==='var')return left.name===right.name;
  if(left.k==='unit')return true;
  if(left.k==='none')return left.absence===right.absence;
  if(left.k==='ctor')return left.data===right.data&&left.ctor===right.ctor
   &&Object.hasOwn(left,'frozen')===Object.hasOwn(right,'frozen')&&left.frozen===right.frozen;
  return left.value===right.value;
 }
 function factor(node,owner,path,depth){
  if(depth>240||node?.k!=='if'||node.type?.kind!=='data'||!closed(node.type))return null;
  const leaves=[],positions=new WeakMap();let work=0;
  function collect(value,at,level=0){
   if(level>64||++work>1024||!value||typeof value!=='object'||!sameType(value.type,node.type))return false;
   if(value.k==='if'&&Object.keys(value).every(key=>conditionalKeys.has(key))&&value.cond?.type?.kind==='bool')
    return collect(value.then,[...at,'then'],level+1)&&collect(value.else,[...at,'else'],level+1);
   if(leaves.length>=256||positions.has(value))return false;
   positions.set(value,leaves.length);leaves.push({value,path:at});return true;
  }
  if(!collect(node,path)||leaves.length<2)return null;
  const groups=[];
  for(const [index,leaf]of leaves.entries()){
   const value=leaf.value;
   if(value.k!=='ctor'||!Array.isArray(value.args)||!Object.keys(value).every(key=>constructorKeys.has(key)))continue;
   const declaration=program.declarations.get(value.data);
   const constructor=declaration?.k==='data'?declaration.ctors.find(entry=>entry.name===value.ctor):null;
   // Keep bare IEEE values at their original allocation sites; this profile
   // introduces no new floating-value selections or NaN boxing assumptions.
   const bareFloat=type=>{const seen=new WeakSet();while(type?.kind==='option'){if(seen.has(type))return true;seen.add(type);type=type.element;}return type?.kind==='float';};
   if(!constructor||constructor.fields.some(field=>bareFloat(field.type))||value.data!==node.type.name||constructor.fields.length!==value.args.length
    ||value.args.some((argument,slot)=>!sameType(argument.type,constructor.fields[slot].type)))continue;
   let group=groups.find(entry=>entry.first.data===value.data&&entry.first.ctor===value.ctor
    &&Object.hasOwn(entry.first,'frozen')===Object.hasOwn(value,'frozen')&&entry.first.frozen===value.frozen);
   if(!group){group={first:value,constructor,indices:[]};groups.push(group);}group.indices.push(index);
  }
  const shared=groups.filter(group=>group.indices.length>=2);
  if(shared.length===0)return null;
  const name=fresh(),selection={k:'var',name,type:FLOAT},literal=value=>({k:'lit',value:String(value),type:FLOAT});
  const equals=index=>({k:'binary',op:'eq',left:selection,right:literal(index),type:BOOL,domain:FLOAT});
  function membership(indices){let value=equals(indices[0]);for(const index of indices.slice(1))value={k:'binary',op:'or',left:value,right:equals(index),type:BOOL};return value;}
  function choose(value){
   const position=positions.get(value);
   if(position!==undefined)return {...literal(position),...(Object.hasOwn(value,'span')?{span:value.span}:{})};
   return {...value,then:choose(value.then),else:choose(value.else),type:FLOAT};
  }
  const fields=[],covered=new Set(),blocks=[];
  for(const group of shared){
   const values=group.indices.map(index=>leaves[index].value),first=group.first,constructor=group.constructor;
   if(!testConditionalConstructorOwners(first.data,first.ctor,constructor.fields.length,
    values.map(value=>value.data),values.map(value=>value.ctor),values.map(value=>value.args.length)))return null;
   const args=constructor.fields.map((field,index)=>{
    const atoms=[];
    const classes=values.map(value=>{const argument=value.args[index];let identity=-1;
     for(let at=0;at<atoms.length;at+=1){if(++visits>1000000)throw new TypeError('constructor atom grouping exceeds its work budget');if(equalAtoms(argument,atoms[at])){identity=at;break;}}
     if(identity<0){identity=atoms.length;atoms.push(argument);}return identity;});
    const plan=planConditionalConstructorFieldSelection(classes);
    if(plan.length===0)throw new TypeError('a complete constructor field lost its finite selection');
    let value=values[plan[0]].args[index];
    for(let at=plan.length-1;at>=1;at-=1){const selected=plan[at];value={k:'if',cond:equals(group.indices[selected]),then:values[selected].args[index],else:value,type:field.type};}
    fields.push({datatype:first.data,constructor:first.ctor,index,type:typeKey(field.type),leaves:group.indices,classes,selection:plan});return value;
   });
   const body={...first,args};if(Object.hasOwn(node,'span'))body.span=node.span;else delete body.span;
   blocks.push({indices:group.indices,body});for(const index of group.indices)covered.add(index);
  }
  for(const [index,leaf]of leaves.entries())if(!covered.has(index))blocks.push({indices:[index],body:leaf.value});
  // The largest already complete alternative supplies the finite fallback;
  // other bodies retain their actual source expressions and argument order.
  blocks.sort((left,right)=>right.indices.length-left.indices.length||left.indices[0]-right.indices[0]);
  let body=blocks[0].body;
  for(let index=blocks.length-1;index>=1;index-=1)body={k:'if',cond:membership(blocks[index].indices),then:blocks[index].body,else:body,type:node.type,...(Object.hasOwn(node,'span')?{span:node.span}:{})};
  const result={k:'let',name,value:choose(node),body,type:node.type,...(Object.hasOwn(node,'span')?{span:node.span}:{})};
  const before=fingerprint(node,240-depth),after=fingerprint(result,240-depth);
  if(!before||!after||after.size>=before.size)return null;
  proofs.push({operation:'select complete value constructors',owner,path,inputSha256:before.sha256,outputSha256:after.sha256,
   selectionBinding:name,fields,blocks:blocks.map(block=>({leaves:block.indices})),
   leaves:leaves.map((leaf,index)=>({index,path:leaf.path,arguments:leaf.value.k==='ctor'&&Array.isArray(leaf.value.args)?leaf.value.args.map((argument,slot)=>({slot,spanPresent:Object.hasOwn(argument,'span'),spanKind:argument.span===undefined?'undefined':argument.span===null?'null':'span',span:argument.span??null})):[],spanPresent:Object.hasOwn(leaf.value,'span'),spanKind:leaf.value.span===undefined?'undefined':leaf.value.span===null?'null':'span',span:leaf.value.span??null})),
   preserveConditionOrder:true,preserveArgumentOrder:true,preserveCompleteOwners:true});
  return result;
 }
 function rewrite(node,owner,path,depth=0){
  if(!node||typeof node!=='object')return node;
  if(depth>256||++visits>1000000)throw new TypeError('conditional constructor rewriting exceeds its source budget');
  const selected=factor(node,owner,path,depth);if(selected)return selected;
  let changed=false;
  const result=Array.isArray(node)?node.map((value,index)=>{const next=rewrite(value,owner,[...path,index],depth+1);changed||=next!==value;return next;}):Object.fromEntries(Object.entries(node).map(([key,value])=>{const next=excluded.has(key)?value:rewrite(value,owner,[...path,key],depth+1);changed||=next!==value;return [key,next];}));
  return changed?result:node;
 }
 const declarations=new Map();
 for(const [name,entry]of program.declarations){const body=entry.k==='fn'?rewrite(entry.body,name,['body']):entry.body;declarations.set(name,body===entry.body?entry:{...entry,body});}
 const items=values=>values.map(item=>{if(item.k==='module')return {...item,items:items(item.items)};
  if(program.declarations.get(item.fullName)!==item)throw new TypeError('constructor sharing needs actual declaration items');
  return declarations.get(item.fullName);});
 const root=new URL('../../',import.meta.url),paths=new Set(['js/scripts/source-conditional-constructors.mjs','js/scripts/source-module-syntax.mjs']);
 let sourceBytes=0;
 function capture(path){
  if(paths.has(path))return;
  if(!/^js\/src\/(?:[a-z0-9-]+\/)*[a-z0-9-]+\.js$/u.test(path)||paths.size>=128)throw new TypeError('constructor-sharing dependencies need bounded actual project modules');
  paths.add(path);const url=new URL(path,root),source=readFileSync(url,'utf8');sourceBytes+=Buffer.byteLength(source);
  if(sourceBytes>16777216)throw new RangeError('constructor-sharing dependencies exceed 16 MiB');
  for(const specifier of readAuthoredJavaScriptModuleDependencies(source,path)){
   if(!specifier.startsWith('./')&&!specifier.startsWith('../'))throw new TypeError('constructor-sharing dependencies require relative source modules');
   const dependency=new URL(specifier,url);
   if(!dependency.href.startsWith(root.href))throw new TypeError('constructor-sharing dependency escapes the project');
   capture(decodeURIComponent(dependency.href.slice(root.href.length)));
  }
 }
 capture('js/src/translation/types.js');capture('js/src/translation/conditional-constructor-sharing-rules.js');
 const inputs=[...paths].sort().map(path=>{const source=readFileSync(new URL(path,root));return {path,bytes:source.length,sha256:createHash('sha256').update(source).digest('hex')};});
 return {program:{...program,declarations,items:items(program.items)},proof:{model:'closed value constructor selection 1',
  producer:{nodeVersion:process.versions.node,nodeVersionRange:'>=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',inputs},proofs}};
}
