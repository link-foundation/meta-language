// Original whole-formal initializer intervals and existing logical parameters.
import {unsupported} from './diagnostics.js';
import {JAVASCRIPT_ARRAY_BINDING_POLICY} from './array-binding-policy.js';
import {requireJavaScriptIdentifierRole} from './javascript-identifier-roles.js';
import {constructJavaScriptRecursiveObjectEntries} from './recursive-object-bindings.js';
import {readJavaScriptFormalInitializer} from './formal-initializers.js';
import {readJavaScriptObjectBindingDepthLimit} from './recursive-binding-rules.js';
import {planJavaScriptWholeParameterDefaults,containsJavaScriptWholeParameterDefault,readJavaScriptWholeParameterBindingPositions,acceptJavaScriptWholeParameterDefaultContext} from './whole-parameter-default-rules.js';
import {containsJavaScriptScalarClosureDefault} from './scalar-closure-default-rules.js';
export function readJavaScriptWholeParameterDefaults(parser,tokens,start,context){
 const kinds=tokens.map(token=>token.kind),values=tokens.map(token=>token.value),plan=planJavaScriptWholeParameterDefaults(kinds,values,0,JAVASCRIPT_ARRAY_BINDING_POLICY.reservedBindings,readJavaScriptObjectBindingDepthLimit());
 if(!containsJavaScriptWholeParameterDefault(plan)&&!containsJavaScriptScalarClosureDefault(plan,context))return null;
 if(!acceptJavaScriptWholeParameterDefaultContext(context))throw unsupported('whole parameter default context','the actual defaulted callable requires its complete function-value invocation environment',tokens[0]);
 const positions=readJavaScriptWholeParameterBindingPositions(plan),names=positions.map(at=>tokens[at].value),parameters=[],patternEntries=[],used=parser.recordParameterNames;
 if(plan[0]!==tokens.length||parser.recordParameterTokenOwner!==parser.cursor.tokens||!(used instanceof Set))throw unsupported('whole parameter default transport','the complete actual formal interval needs its parser-owned token and name supply',tokens[0]);
 const fresh=(prefix='whole_parameter_value')=>{let name;do{if(!Number.isSafeInteger(parser.recordParameterOrdinal+1))throw unsupported('whole parameter default transport','the parser-owned logical name supply exceeds its exact range',tokens[0]);name=prefix+parser.recordParameterOrdinal++;}while(used.has(name));used.add(name);return name;};
 const readDefault=(first,last,binding)=>{
  return readJavaScriptFormalInitializer(parser,{tokens,start,first,last,binding,positions,context,profile:'whole parameter default'});
 };
 let at=2;
 while(at<plan.length){const form=plan[at++],slot=plan[at++];if(slot!==parameters.length)throw unsupported('whole parameter default transport','logical arguments retain their actual ordered slots',tokens[0]);
  if(form===0||form===2){
   const binding=plan[at++],token=tokens[binding];requireJavaScriptIdentifierRole(token,true);const span={start:token.start,end:token.end};
   if(form===0)parameters.push({name:token.value,type:null,span});
   else{const name=fresh(),fallback=readDefault(plan[at++],plan[at++],binding);parameters.push({name,type:null,optional:true,default:{k:'none',absence:'undefined',span},span});patternEntries.push({temporary:false,name:token.value,value:{k:'arrayBindingDefault',value:{k:'name',path:[name],span},fallback,span},span});}
   continue;
  }
  const length=plan[at++],tree=plan.slice(at,at+length);at+=length;
  const open=tokens[tree[8]],close=tokens[tree[9]],span={start:open.start,end:close.end},name=fresh(),parameter={name,type:null,span};
  let root={k:'name',path:[name],span};
  if(form===3){const first=plan[at++],last=plan[at++],binding=positions.find(position=>position>tree[8]&&position<tree[9]);if(binding===undefined)throw unsupported('whole parameter default binding','an empty pattern needs its separate complete coercion and initialization environment',open);const fallback=readDefault(first,last,binding),selected=fresh('record_parameter_whole_value');parameter.optional=true;parameter.default={k:'none',absence:'undefined',span};patternEntries.push({temporary:true,name:selected,value:{k:'arrayBindingDefault',value:root,fallback,span},span});root={k:'name',path:[selected],span};}
  parameters.push(parameter);
  patternEntries.push(...constructJavaScriptRecursiveObjectEntries(parser,{tokens,plan:tree,start},root,{positions,context,profile:'record formal default',fresh:()=>fresh('record_parameter_nested_value')}));
 }
 parser.cursor.index=start+plan[0];return {parameters,sourceNames:names,patternEntries,bindings:[]};
}
