// Context admission stays with the existing named/closure/callable readers.
import {unsupported} from './diagnostics.js';
import {JAVASCRIPT_ARRAY_BINDING_POLICY} from './array-binding-policy.js';
import {requireJavaScriptIdentifierRole} from './javascript-identifier-roles.js';
import {constructJavaScriptRecursiveObjectEntries} from './recursive-object-bindings.js';
import {planJavaScriptRecursiveObjectFormals,containsJavaScriptRecursiveObjectFormals,readJavaScriptRecursiveObjectFormalBindingPositions,readJavaScriptObjectBindingDepthLimit} from './recursive-object-binding-rules.js';
export function readJavaScriptRecursiveObjectFormals(parser,tokens,start,context){
 const plan=planJavaScriptRecursiveObjectFormals(tokens.map(token=>token.kind),tokens.map(token=>token.value),0,JAVASCRIPT_ARRAY_BINDING_POLICY.reservedBindings,readJavaScriptObjectBindingDepthLimit());
 if(plan[0]===-9)throw unsupported('object binding depth','the actual recursive pattern exceeds its shared source depth budget',tokens[plan[1]]);
 if(!containsJavaScriptRecursiveObjectFormals(plan))return null;
 const positions=readJavaScriptRecursiveObjectFormalBindingPositions(plan),used=parser.recordParameterNames;
 if(plan[0]!==tokens.length||parser.recordParameterTokenOwner!==parser.cursor.tokens||!(used instanceof Set))throw unsupported('record parameter source transport','the plan needs the complete actual formal interval and its parser-owned token/name inventory',tokens[0]);
 const fresh=prefix=>{let name;do{if(!Number.isSafeInteger(parser.recordParameterOrdinal+1))throw unsupported('record parameter source transport','the parser-owned logical argument name ordinal exceeds its safe integer range',tokens[0]);name=prefix+parser.recordParameterOrdinal++;}while(used.has(name));used.add(name);return name;};
 const parameters=[],patternEntries=[],sourceNames=positions.map(position=>tokens[position].value);let cursor=2;
 while(cursor<plan.length){
  const form=plan[cursor++],slot=plan[cursor++];if(slot!==parameters.length)throw unsupported('record parameter source transport','logical arguments retain their actual ordered slots',tokens[0]);
  if(form===0){const token=tokens[plan[cursor++]];requireJavaScriptIdentifierRole(token,true);parameters.push({name:token.value,type:null,span:{start:token.start,end:token.end}});continue;}
  const length=plan[cursor++],tree=plan.slice(cursor,cursor+length);cursor+=length;
  const open=tokens[tree[8]],close=tokens[tree[9]],span={start:open.start,end:close.end},name=fresh('record_parameter_value');
  parameters.push({name,type:null,span});
  patternEntries.push(...constructJavaScriptRecursiveObjectEntries(parser,{tokens,plan:tree,start},{k:'name',path:[name],span},{positions,context,profile:'record formal default',fresh:()=>fresh('record_parameter_nested_value')}));
 }
 parser.cursor.index=start+plan[0];return {parameters,sourceNames,patternEntries,bindings:[]};
}
