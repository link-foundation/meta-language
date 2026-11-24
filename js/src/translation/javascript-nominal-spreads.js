// Actual checked declaration/child transport into the shared copy policy and
// existing constructor, exhaustive field projection and ordered let algebra.
import {readJavaScriptNominalSpreadError,readJavaScriptNominalSpreadGuardArm,readJavaScriptNominalSumSpreadError} from './javascript-nominal-spread-rules.js';
import {checkJavaScriptDataField} from './javascript-data-fields.js';
import {coerce} from './check.js';
import {typeError} from './diagnostics.js';
import {constructJavaScriptOwnedConstructorField,javaScriptConstructorFieldValueType} from './javascript-constructor-presence.js';
import {constructJavaScriptOptionalPresence} from './javascript-optional-chains.js';
import {typeKey} from './types.js';
const present=Symbol('unchanged nominal spread absence guards');
const constructors=Symbol('unchanged nominal constructor guards');
const ownProperties=Symbol('unchanged nominal own property guards');
function fieldPath(node){
 const fields=[];
 while(node.k==='field'&&fields.length<256){fields.push(node.field);node=node.object;}
 if(node.k!=='name'||node.path.length!==1)return null;
 return {name:node.path[0],fields:fields.reverse()};
}
function samePath(left,right){return left.name===right.name&&left.fields.length===right.fields.length&&left.fields.every((field,index)=>field===right.fields[index]);}
export function readJavaScriptGuardedConstructors(node,env){
 if(!env.has(constructors))return undefined;
 const path=fieldPath(node);
 return path&&(env.get(constructors)??[]).find(fact=>samePath(fact,path));
}
export function hasJavaScriptGuardedOwnField(node,env){
 if(!env.has(ownProperties))return false;
 const path=fieldPath(node);
 return path&&(env.get(ownProperties)??[]).some(fact=>samePath(fact,path));
}
export function forgetJavaScriptNominalSpreadGuard(env,name){
 for(const kind of [present,constructors,ownProperties])if(env.has(kind))env.set(kind,env.get(kind).filter(row=>row.name!==name));
}
function immutableFieldPath(checker,node,env){
 const path=fieldPath(node);
 if(!path)return null;
 let type=env.get(path.name);
 for(const field of path.fields){
  const owner=type?.kind==='option'?type.element:type;
  const variants=owner?.kind==='data'?checker.items.get(owner.name)?.ctors:undefined;
  const declared=variants?.flatMap(variant=>variant.fields.filter(candidate=>candidate.name===field));
  if(!declared?.length)return null;
  const selected=javaScriptConstructorFieldValueType(declared[0]);
  if(declared.some(candidate=>typeKey(javaScriptConstructorFieldValueType(candidate))!==typeKey(selected)))return null;
  type=declared.length===variants.length?selected:{kind:'option',element:selected,absence:'undefined'};
 }
 return {...path,type};
}
function joinedBranchFacts(original,left,right){
 if(left===original||right===original)return original;
 const absence=(left.get(present)??[]).filter(fact=>(right.get(present)??[]).some(other=>fact.absence===other.absence&&samePath(fact,other)));
 const tags=(left.get(constructors)??[]).flatMap(fact=>{
  const other=(right.get(constructors)??[]).find(other=>fact.owner===other.owner&&samePath(fact,other));
  return other?[{...fact,names:[...new Set([...fact.names,...other.names])]}]:[];
 });
 const own=(left.get(ownProperties)??[]).filter(fact=>(right.get(ownProperties)??[]).some(other=>samePath(fact,other)));
 return new Map(original).set(present,absence).set(constructors,tags).set(ownProperties,own);
}
function directTagTest(condition){
 const test=condition.tagTest;
 if(!test||condition.k!=='match'||condition.scrutinees.length!==1||condition.rows.length!==2)return null;
 const actual=fieldPath(condition.scrutinees[0]),declared=fieldPath(test.object);
 if(!actual||!declared||!samePath(actual,declared))return null;
 const [matching,remaining]=condition.rows;
 const pattern=matching.patterns[0];
 if(matching.patterns.length!==1||pattern.k!=='ctor'||pattern.path.at(-1)!==test.tag||!pattern.args.every(arg=>arg.k==='wild')
   ||remaining.patterns.length!==1||remaining.patterns[0].k!=='wild')return null;
 if(matching.body.k!=='bool'||matching.body.value!==!test.negated||remaining.body.k!=='bool'||remaining.body.value!==test.negated)return null;
 return test;
}
export function readJavaScriptOptionalGuardBranch(checker,condition,env,outcome,depth=0){
 if(depth>=256)return env;
 if(condition.k==='unary'&&condition.op==='not')return readJavaScriptOptionalGuardBranch(checker,condition.arg,env,!outcome,depth+1);
 if(checker.checkedProjection&&condition.k==='math'
   &&(condition.op==='strictEqual:false'||condition.op==='strictEqual:true')
   &&condition.args.length===2&&condition.args.every(arg=>!arg.spread)){
  const [left,right]=condition.args.map(arg=>arg.value);
  return readJavaScriptOptionalGuardBranch(checker,{k:'binary',op:condition.op==='strictEqual:true'?'ne':'eq',left,right},env,outcome,depth+1);
 }
 if(outcome&&condition.k==='math'&&condition.op==='testJavaScriptOwnedProperty'&&condition.args.length===2&&condition.args.every(arg=>!arg.spread)){
  const [object,key]=condition.args.map(arg=>arg.value);
  const path=key.k==='str'?immutableFieldPath(checker,object,env):null;
  if(path?.type?.kind==='data'&&path.fields.length<256){
   const property={name:path.name,fields:[...path.fields,key.value]},facts=env.get(ownProperties)??[];
   if(!facts.some(fact=>samePath(fact,property)))return new Map(env).set(ownProperties,[...facts,property]);
  }
  return env;
 }
 const test=directTagTest(condition);
 if(test){
  const path=immutableFieldPath(checker,test.object,env);
  const owner=path?.type?.kind==='option'?path.type.element:path?.type;
  const variants=owner?.kind==='data'?checker.items.get(owner.name)?.ctors:undefined;
  if(!variants?.some(variant=>variant.name===test.tag))return env;
  const facts=env.get(constructors)??[],previous=facts.find(fact=>samePath(fact,path));
  const names=(previous?.names??variants.map(variant=>variant.name)).filter(name=>(name===test.tag)===(outcome!==test.negated));
  return new Map(env).set(constructors,[...facts.filter(fact=>fact!==previous),{...path,owner:owner.name,names}]);
 }
 if(condition.k!=='binary')return env;
 const {op,left,right}=condition;
 if(op==='and'||op==='or'){
  const first=readJavaScriptOptionalGuardBranch(checker,left,env,outcome,depth+1);
  if((op==='and')===outcome)return readJavaScriptOptionalGuardBranch(checker,right,first,outcome,depth+1);
  const second=readJavaScriptOptionalGuardBranch(checker,right,env,outcome,depth+1);
  return joinedBranchFacts(env,first,second);
 }
 const nameNode=left.k==='name'||left.k==='field'?left:right;
 const literal=nameNode===left?right:left;
 const absence=literal.k==='none'?literal.absence:literal.k==='unit'?'undefined':'';
 const arm=readJavaScriptNominalSpreadGuardArm(condition.k,op,absence);
 if(arm!==(outcome?1:2))return env;
 const path=immutableFieldPath(checker,nameNode,env);
 if(!path)return env;
 const facts=env.get(present)??[];
 if(facts.some(fact=>fact.absence===absence&&samePath(fact,path)))return env;
 return new Map(env).set(present,[...facts,{...path,absence}]);
}
export function projectJavaScriptGuardedOptional(node,value,env){
 if(value.type.kind!=='option'||!env.has(present))return value;
 const path=fieldPath(node);
 if(!path)return value;
 const exclusions=(env.get(present)??[]).filter(row=>samePath(row,path)).map(row=>row.absence);
 const required=value.type.absence==='both'?['undefined','null']:[value.type.absence];
 return required.every(absence=>exclusions.includes(absence))?{k:'unwrapOption',arg:value,type:value.type.element}:value;
}
export function checkJavaScriptNominalSpread(checker,node,env,path,expected){
 if(checker.language!=='JavaScript'||!node.entries[0]?.spread)return null;
 const selected=expected??(node.type?checker.resolveType(node.type,path,node.span):undefined);
 if(selected?.kind!=='data')return null;
 const declaration=checker.items.get(selected.name),constructors=declaration?.k==='data'?declaration.ctors:[],ctor=constructors[0];
 const kinds=node.entries.map(entry=>entry.spread?'spread':'field');
 if(constructors.length>1){
  const overrides=node.entries.slice(1).map(entry=>ctor.fields.find(field=>field.name===entry.name));
  const common=overrides.every(field=>field&&constructors.every(variant=>variant.fields.some(candidate=>
   candidate.name===field.name&&(candidate.optional===true)===(field.optional===true)&&typeKey(candidate.type)===typeKey(field.type))));
  const error=readJavaScriptNominalSumSpreadError(constructors.length,kinds,common);
  if(error)throw typeError(error,node.span);
 }
 const values=[],bindings=[],fields=new Map();
 for(const entry of node.entries){
  const declared=!entry.spread?ctor?.fields.find(field=>field.name===entry.name):undefined;
  const value=checker.expr(entry.value,env,path,declared&&javaScriptConstructorFieldValueType(declared));values.push(entry.spread?projectJavaScriptGuardedOptional(entry.value,value,env):value);
 }
 const error=readJavaScriptNominalSpreadError(checker.language,Math.min(1,constructors.length),ctor?.fields.map(field=>field.name)??[],
  kinds,node.entries.map(entry=>entry.spread?'':entry.name),
  node.entries.map((entry,index)=>entry.spread?(values[index].type.kind==='data'?values[index].type.name:'') : ''),selected.name,
  node.prototypeDefinition!==undefined||node.recordProjection!==undefined||node.propertyOrder!==undefined);
 if(error)throw typeError(error,node.span);
 if(constructors.length>1){
  const cases=constructors.map(variant=>{
   const binds=variant.fields.map(()=>checker.freshName('ml_nominal_field'));
   const args=variant.fields.map((field,index)=>({k:'var',name:binds[index],type:field.type}));
   const overrides=[];
   for(const [index,entry]of node.entries.slice(1).entries()){
    const position=variant.fields.findIndex(field=>field.name===entry.name);
    const field=variant.fields[position],name=checker.freshName('ml_nominal_override');
    const expected=javaScriptConstructorFieldValueType(field);
    overrides.push({name,value:coerce(values[index+1],expected,checker.language,node.span)});
    const value={k:'var',name,type:expected};
    args[position]=field.optional?constructJavaScriptOwnedConstructorField(value,field.type):value;
   }
   const body={k:'ctor',data:declaration.fullName,ctor:variant.name,args,type:selected};
   return {pattern:{k:'ctor',data:declaration.fullName,ctor:variant.name,binds},body:overrides.reduceRight((body,binding)=>({k:'let',...binding,body,type:selected}),body)};
  });
  return {k:'match',scrutinee:values[0],cases,type:selected};
 }
 for(const [index,entry]of node.entries.entries()){
  const value=values[index],name=checker.freshName(entry.spread?'ml_nominal_spread':'ml_nominal_override');bindings.push({name,value});
  const variable={k:'var',name,type:value.type};
  if(entry.spread){
   for(const field of ctor.fields){
    const name=checker.freshName('ml_nominal_field');
    const value=checkJavaScriptDataField(checker,variable,field.name,node.span,field.optional===true);
    bindings.push({name,value});
    const current={k:'var',name,type:field.type},previous=fields.get(field.name);
    fields.set(field.name,field.optional&&previous
      ?{k:'if',cond:constructJavaScriptOptionalPresence(current,field.type.element,node.span),then:current,else:previous,type:field.type}
      :current);
   }
  }else{
   const field=ctor.fields.find(field=>field.name===entry.name);
   const checked=coerce(variable,javaScriptConstructorFieldValueType(field),checker.language,node.span);
   fields.set(entry.name,field.optional?constructJavaScriptOwnedConstructorField(checked,field.type):checked);
  }
 }
 let body={k:'ctor',data:declaration.fullName,ctor:ctor.name,args:ctor.fields.map(field=>fields.get(field.name)),type:selected};
 for(const binding of bindings.toReversed())body={k:'let',name:binding.name,value:binding.value,body,type:selected};
 return body;
}
