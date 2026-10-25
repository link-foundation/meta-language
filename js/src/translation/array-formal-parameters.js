// Actual token/term transport. The shared source plan owns every syntax choice.
import {unsupported} from './diagnostics.js';
import {JAVASCRIPT_ARRAY_BINDING_POLICY} from './array-binding-policy.js';
import {requireJavaScriptIdentifierRole} from './javascript-identifier-roles.js';
import {findJavaScriptArrayFormalPattern,acceptJavaScriptArrayFormalContext,planJavaScriptArrayFormalParametersWithDefaults,readJavaScriptArrayFormalDefaultBindingPositions,readJavaScriptArrayFormalDefaultScopeBoundary,readJavaScriptArrayFormalIterableOperation} from './array-formal-parameter-rules.js';
import {findDefaultParameterReference} from './frontend-rules.js';
import {readJavaScriptRecordParameterBindingStatement} from './javascript-record-parameter-rules.js';
import {readJavaScriptPatternParameterBody,readNamedJavaScriptRecordParameterBody} from './javascript-record-parameters.js';

export function readJavaScriptArrayFormalParameters(parser,context,doc,asynchronous){
 const c=parser.cursor,start=c.index,opening=c.tokens[start-1];
 if(opening?.kind!=='punct'||opening.value!=='(')return null;
 const close=parser.matching(start-1),closing=c.tokens[close];if(closing?.kind!=='punct'||closing.value!==')')return null;
 const tokens=c.tokens.slice(start,close+1),kinds=tokens.map(token=>token.kind),values=tokens.map(token=>token.value);
 const selected=findJavaScriptArrayFormalPattern(kinds,values,0);if(selected<0)return null;
 if(!acceptJavaScriptArrayFormalContext(context,asynchronous,doc?.params.size??0))throw unsupported('array formal parameter source profile','ordinary array patterns require synchronous callables without parameter documentation tags',tokens[selected]);
 const plan=planJavaScriptArrayFormalParametersWithDefaults(kinds,values,0,JAVASCRIPT_ARRAY_BINDING_POLICY.reservedBindings);
 if(plan[0]<0)throw unsupported('array formal parameter syntax','flat names, elisions and final named rest use distinct actual BoundNames; defaults and nested/mixed object patterns need separate interpretations (reason '+(-plan[0])+')',tokens[plan[1]]??opening);
 const parameters=[],sourceNames=[],arrayEntries=[],used=parser.recordParameterNames;
 const actual=position=>{if(!Number.isSafeInteger(position)||position<0||position>=tokens.length)throw unsupported('array formal parameter transport','every selected token belongs to its actual formal interval',opening);return tokens[position];};
 const boundNames=readJavaScriptArrayFormalDefaultBindingPositions(plan).map(position=>actual(position).value);
 const readDefault=(first,last)=>{
  const interval=tokens.slice(first,last),reference=findDefaultParameterReference(interval.map(token=>token.kind),interval.map(token=>token.value),boundNames);
  if(reference>=0)throw unsupported('array formal default reading parameter '+interval[reference].value,'the current default profile requires a closed parameter initialization environment',interval[reference]);
  const saved=c.index,outer=parser.scope;c.index=start+first;
  parser.scope={...outer,tdz:new Set([...outer.tdz,...boundNames])};
  try{const value=parser.withLexicalScope(boundNames,()=>parser.expr(),readJavaScriptArrayFormalDefaultScopeBoundary(context));if(c.index!==start+last)throw unsupported('array formal default transport','the actual expression must consume its complete planned initializer interval',actual(first));return value;}
  finally{c.index=saved;parser.scope=outer;}
 };
 const fresh=prefix=>{let name;do{if(!Number.isSafeInteger(parser.recordParameterOrdinal+1))throw unsupported('array formal parameter transport','the logical binder supply exceeds its exact ordinal range',opening);name=prefix+parser.recordParameterOrdinal++;}while(used.has(name));used.add(name);return name;};
 let cursor=2;
 while(cursor<plan.length){
  const form=plan[cursor++],slot=plan[cursor++];if(slot!==parameters.length)throw unsupported('array formal parameter transport','logical argument slots retain their actual order',opening);
  if(form===0){const token=actual(plan[cursor++]);requireJavaScriptIdentifierRole(token,true);parameters.push({name:token.value,type:null,span:{start:token.start,end:token.end}});sourceNames.push(token.value);continue;}
  if(form!==2)throw unsupported('array formal parameter transport','unknown actual formal syntax plan',opening);
  const open=actual(plan[cursor++]),end=actual(plan[cursor++]),count=plan[cursor++],where={start:open.start,end:end.end};
  const parameter=fresh('array_parameter_value'),snapshot=fresh('array_parameter_values');parameters.push({name:parameter,type:null,span:where});
  const read=name=>({k:'name',path:[name],span:where});
  arrayEntries.push({temporary:true,name:snapshot,value:{k:'math',op:readJavaScriptArrayFormalIterableOperation(),name:'',args:[{value:read(parameter),spread:false}],span:where},span:where});
  for(let index=0;index<count;index+=1){const rest=plan[cursor++]===1,position=plan[cursor++],token=actual(plan[cursor++]),defaultStart=plan[cursor++],defaultEnd=plan[cursor++];requireJavaScriptIdentifierRole(token,true);sourceNames.push(token.value);
   const offset={k:'num',value:String(position),span:{start:token.start,end:token.end}};
   const indexed={k:'index',object:read(snapshot),index:offset,span:where};
   const value=rest?{k:'arrayBindingRest',object:read(snapshot),start:offset,span:where}:defaultStart>=0?{k:'arrayBindingDefault',value:indexed,fallback:readDefault(defaultStart,defaultEnd),span:where}:indexed;
   arrayEntries.push({temporary:false,name:token.value,value,span:{start:token.start,end:token.end}});
  }
 }
 if(parameters.length!==plan[1])throw unsupported('array formal parameter transport','every actual parameter slot must be retained',opening);
 c.index=start+plan[0];return {parameters,sourceNames,arrayEntries,bindings:[]};
}
export function readJavaScriptArrayFormalParameterBody(parser,syntax,readBody){
 return readJavaScriptPatternParameterBody(parser,syntax,readBody,written=>syntax.arrayEntries.map(entry=>({s:entry.temporary?'const':readJavaScriptRecordParameterBindingStatement(written.has(entry.name)),name:entry.name,value:entry.value,span:entry.span})));
}
export function readJavaScriptOrdinaryPatternParameterBody(parser,syntax,readBody){
 if(syntax.arrayEntries)return readJavaScriptArrayFormalParameterBody(parser,syntax,readBody);
 return readNamedJavaScriptRecordParameterBody(parser,syntax,readBody);
}
