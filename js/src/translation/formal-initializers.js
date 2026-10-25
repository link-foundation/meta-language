// Actual source intervals and temporary scope transport shared by pattern kinds.
import {unsupported} from './diagnostics.js';
import {findDefaultParameterReference} from './frontend-rules.js';
import {partitionJavaScriptFormalInitializerBindingPositions,findJavaScriptArrayFormalInitializerAssignment} from './array-formal-parameter-rules.js';
import {readJavaScriptPatternInitializerScopeBoundary,readJavaScriptPatternInitializerSubject,readJavaScriptPatternInitializerReferenceReason,readJavaScriptPatternInitializerAssignmentReason} from './javascript-record-parameter-rules.js';
export function readJavaScriptFormalInitializer(parser,{tokens,start,first,last,binding,positions,context,profile}){
 const actual=position=>{if(!Number.isSafeInteger(position)||position<0||position>=tokens.length)throw unsupported(profile+' transport','every selected token belongs to its actual formal interval',tokens[0]);return tokens[position];};
 const names=positions.map(position=>actual(position).value);
 const initialization=partitionJavaScriptFormalInitializerBindingPositions(positions,binding),count=initialization[0];
 const available=initialization.slice(1,1+count).map(position=>actual(position).value),uninitialized=initialization.slice(1+count).map(position=>actual(position).value);
 const interval=tokens.slice(first,last),reference=findDefaultParameterReference(interval.map(token=>token.kind),interval.map(token=>token.value),uninitialized);
 if(reference>=0)throw unsupported(profile+' reading '+readJavaScriptPatternInitializerSubject(context)+' '+interval[reference].value,readJavaScriptPatternInitializerReferenceReason(context),interval[reference]);
 const c=parser.cursor,saved=c.index,outer=parser.scope,before=parser.lexicalReads.length;c.index=start+first;
 parser.scope={...outer,locals:new Set([...outer.locals,...names]),mutable:new Set([...outer.mutable,...available]),tdz:new Set([...outer.tdz].filter(name=>!available.includes(name)).concat(uninitialized))};
 try{
  const value=parser.withLexicalScope(names,()=>parser.expr(),readJavaScriptPatternInitializerScopeBoundary(context));
  if(c.index!==start+last)throw unsupported(profile+' transport','the actual expression must consume its complete planned initializer interval',actual(first));
  const assignments=parser.lexicalReads.slice(before).filter(read=>read.assignment).map(read=>read.name),assigned=findJavaScriptArrayFormalInitializerAssignment(assignments,names);
  if(assigned>=0)throw unsupported(profile+' assigning '+readJavaScriptPatternInitializerSubject(context)+' '+assignments[assigned],readJavaScriptPatternInitializerAssignmentReason(context),actual(first));
  return value;
 }finally{c.index=saved;parser.scope=outer;}
}
