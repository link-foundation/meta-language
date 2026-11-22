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
export function forgetJavaScriptNominalSpreadGuard(env,name){if(env.has(present))env.set(present,env.get(present).filter(row=>row.name!==name));}
export function readJavaScriptNominalSpreadBranches(condition,env){
 const negated=condition.k==='unary'&&condition.op==='not',guard=negated?condition.arg:condition;
 if(guard.k!=='binary')return {then:env,else:env};
 const left=guard.left,right=guard.right,name=left.k==='name'&&left.path.length===1&&right.k==='none'?left.path[0]:right.k==='name'&&right.path.length===1&&left.k==='none'?right.path[0]:null;
 const absence=left.k==='none'?left.absence:right.k==='none'?right.absence:'';
 const arm=readJavaScriptNominalSpreadGuardArm(guard.k,guard.op,absence);if(!arm||name===null||!env.has(name))return {then:env,else:env};
 const selected=new Map(env);selected.set(present,[...(env.get(present)??[]),{name,absence}]);
 return (negated?3-arm:arm)===1?{then:selected,else:env}:{then:env,else:selected};
}
export function projectJavaScriptGuardedOptional(node,value,env){
 if(value.type.kind!=='option'||node.k!=='name'||node.path.length!==1)return value;
 const exclusions=(env.get(present)??[]).filter(row=>row.name===node.path[0]).map(row=>row.absence);
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
