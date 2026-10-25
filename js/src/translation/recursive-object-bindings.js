// Actual tokens and source terms transported through one shared owner plan.
import {unsupported,TranslationError} from './diagnostics.js';
import {JAVASCRIPT_ARRAY_BINDING_POLICY} from './array-binding-policy.js';
import {requireJavaScriptIdentifierRole} from './javascript-identifier-roles.js';
import {acceptJavaScriptObjectBindingName} from './javascript-record-parameter-rules.js';
import {readJavaScriptFormalInitializer} from './formal-initializers.js';
import {constructJavaScriptObjectRestSource} from './object-rest.js';
import {readJavaScriptRecordFormalDefaultOperation} from './javascript-record-parameter-rules.js';
import {planJavaScriptRecursiveObjectBindings,readJavaScriptRecursiveObjectBindingPositions,readJavaScriptRecursiveObjectExcludedPositions,readJavaScriptRecursiveObjectInitializerBinding,readJavaScriptObjectBindingDepthLimit} from './recursive-object-binding-rules.js';
export function readJavaScriptRecursiveObjectPlan(parser,start){
 const close=parser.matching(start),tokens=parser.cursor.tokens.slice(start,close+1);
 const plan=planJavaScriptRecursiveObjectBindings(tokens.map(token=>token.kind),tokens.map(token=>token.value),0,JAVASCRIPT_ARRAY_BINDING_POLICY.reservedBindings,[],readJavaScriptObjectBindingDepthLimit());
 return {tokens,plan,start};
}
export function requireJavaScriptRecursiveObjectPlan(parser,syntax,startToken){
 const {tokens,plan,start}=syntax;
 if(plan[0]===-4){parser.cursor.index=start+tokens.length;throw unsupported('empty object binding','object destructibility needs a checked representation',{start:startToken.start,end:parser.cursor.peek().end});}
 if(plan[0]===-6)throw unsupported('rest property','name every field',tokens[plan[1]]);
 if(plan[0]===-9)throw unsupported('object binding depth','the actual recursive pattern exceeds its shared source depth budget',tokens[plan[1]]);
 if(plan[0]===-8){const token=tokens[plan[1]];requireJavaScriptIdentifierRole(token,true);if(acceptJavaScriptObjectBindingName(token.kind,token.value,JAVASCRIPT_ARRAY_BINDING_POLICY.reservedBindings))throw new TranslationError('syntax','duplicate lexical binding '+token.value,{start:token.start,end:token.end});}
 if(plan[0]<0)throw unsupported('object binding syntax','flat actual property names and distinct lexical bindings need their complete source meanings (reason '+(-plan[0])+')',tokens[plan[1]]??parser.cursor.peek());
 if(plan[0]!==tokens.length)throw unsupported('object binding transport','the declaration needs its complete actual object pattern and lexical mode',parser.cursor.peek());
}
export function constructJavaScriptRecursiveObjectEntries(parser,{tokens,plan,start},root,{positions=readJavaScriptRecursiveObjectBindingPositions(plan),context,profile,fresh}){
 const actual=position=>{if(!Number.isSafeInteger(position)||position<0||position>=tokens.length)throw unsupported('recursive object binding transport','every selected token belongs to the actual complete pattern interval',tokens[0]);return tokens[position];};
 const owners=new Map([[0,root]]),entries=[];
 for(let index=1;index<plan[1];index+=1){
  const at=2+index*8,role=plan[at],owner=owners.get(plan[at+1]),property=actual(plan[at+2]),binding=plan[at+3]>=0?actual(plan[at+3]):null,first=plan[at+4],last=plan[at+5],open=plan[at+6],close=plan[at+7];
  if(!owner)throw unsupported('recursive object binding transport','each child needs its actual preceding selected owner',property);
  const span={start:property.start,end:binding?.end??actual(close).end};
  let value;
  if(role===2)value=constructJavaScriptObjectRestSource(owner,readJavaScriptRecursiveObjectExcludedPositions(plan,plan[at+1]).map(actual),span);
  else{
   const fallback=first>=0?readJavaScriptFormalInitializer(parser,{tokens,start,first,last,binding:readJavaScriptRecursiveObjectInitializerBinding(plan,index),positions,context,profile}):null;
   if(fallback?.span)span.end=fallback.span.end;
   value=fallback?{k:'math',op:readJavaScriptRecordFormalDefaultOperation(),name:'',args:[{value:owner,spread:false},{value:{k:'str',value:property.value,span:{start:property.start,end:property.end}},spread:false},{value:fallback,spread:false}],span}:{k:'field',object:owner,field:property.value,span};
  }
  if(binding){requireJavaScriptIdentifierRole(binding,true);entries.push({temporary:false,name:binding.value,value,span:{start:binding.start,end:binding.end}});}
  else{const name=fresh();entries.push({temporary:true,name,value,span:{start:actual(open).start,end:actual(close).end}});owners.set(index,{k:'name',path:[name],span:{start:actual(open).start,end:actual(close).end}});}
 }
 return entries;
}
