// Host adaptation of actual receiver, lexical binder and continuation operands.
import {BOOL,STRING,UNIT,option} from './types.js';
import {typeError,unsupported} from './diagnostics.js';
import {coerce,bindLocal} from './check.js';
import {readTypeQueryResult} from './frontend-rules.js';
import {mergeOptionalAbsence} from './structured-type-rules.js';
import {acceptJavaScriptOptionalChainOperands,readJavaScriptOptionalChainField,
 readJavaScriptOptionalChainForm,readJavaScriptOptionalChainResultForm,
 readJavaScriptOptionalChainPresenceForm} from './javascript-optional-chain-rules.js';
const absent=type=>({k:'none',absence:'undefined',type});
export function constructJavaScriptOptionalPresence(receiver,present,span){
 if(readJavaScriptOptionalChainPresenceForm(present.kind)==='truthy')return {k:'truthy',arg:receiver,type:BOOL};
 const tag=readTypeQueryResult(present.kind);
 if(!tag)throw unsupported('optional chain receiver','the present source value needs its explicit nullish observation protocol',span);
 return {k:'binary',op:'eq',left:{k:'optionalTypeOf',arg:receiver,present:tag,type:STRING},right:{k:'lit',value:tag,type:STRING},domain:STRING,type:BOOL};
}
export function checkJavaScriptOptionalChain(checker,node,env,path){
 const scopeSource=node.args[0]?.value;
 if(checker.language!=='JavaScript'||scopeSource?.k!=='let'||scopeSource.type!==undefined||!acceptJavaScriptOptionalChainOperands(node.args.length,node.args.some(item=>item.spread),scopeSource.name))throw typeError('an optional chain needs its actual receiver, lexical binding and complete continuation',node.span);
 const value=checker.expr(scopeSource.value,env,path,undefined);
 const form=readJavaScriptOptionalChainForm(value.type.kind,value.type.element?.kind??'');
 const binding=checker.freshName('ml_optional_chain_receiver'),field=readJavaScriptOptionalChainField();
 const type={kind:'record',fields:[{name:field,type:value.type}]};
 const receiver={k:'recordField',object:{k:'var',name:binding,type},field,index:0,type:value.type};
 let body;
 if(form==='absent')body=absent(option(UNIT,'undefined'));
 else{
  const present=form==='guarded'?value.type.element:value.type;
  const actual=form==='guarded'?{k:'unwrapOption',arg:receiver,type:present}:receiver;
  const scope=new Map(env);bindLocal(scope,scopeSource.name,present);
  const continuation=checker.expr(scopeSource.body,scope,path,undefined);
  const tail={k:'let',name:scopeSource.name,value:actual,body:continuation,type:continuation.type};
  if(form==='direct')body=tail;
  else{
   const outer=option(tail.type,'undefined'),resultForm=readJavaScriptOptionalChainResultForm(tail.type.kind);
   const success=resultForm==='undefined'?coerce(tail,outer,checker.language,node.span):{k:'some',value:tail,type:outer};
   const condition=constructJavaScriptOptionalPresence(receiver,present,node.span);
   body={k:'if',cond:condition,then:success,else:absent(outer),type:outer};
   if(resultForm==='flatten')body={k:'flattenOption',value:body,type:option(tail.type.element,mergeOptionalAbsence(tail.type.absence,'undefined'))};
  }
 }
 return {k:'let',name:binding,value:{k:'record',entries:[{name:field,value}],type},body,type:body.type};
}
