// Source/checked binder facts and actual projections only. The complete source
// library owns runtime key search; no dynamic property protocol is invented.
import {FLOAT,BOOL,STRING,option,sameType} from './types.js';
import {coerce} from './check.js';
import {unsupported} from './diagnostics.js';
import {constructRecordPropertyNameRows} from './ordered-record-emission.js';
import {readJavaScriptOwnPropertyGuardArm,acceptJavaScriptOwnPropertyGuardBindings,acceptJavaScriptGuardedRecordIndex,readJavaScriptGuardedRecordProjectionPositions,readJavaScriptOwnKeyCallbackPosition,readJavaScriptOwnKeyCallbackReceiverPosition,acceptJavaScriptOwnKeyCallbackSource,acceptJavaScriptOwnKeyCallbackBinding} from './javascript-own-property-rules.js';
const facts=Symbol('actual own-property branch binders');
const callbackOrigin=Symbol('actual own-key callback origin');
const local=node=>node?.k==='name'&&node.path.length===1?node.path[0]:null;
export function forgetJavaScriptOwnPropertyGuard(env,name){const previous=env.get(facts);if(previous)env.set(facts,previous.filter(row=>row.object!==name&&row.key!==name));}
export function createJavaScriptOwnKeyCallbackEnvironment(receiver,callback,operation,env){
 const position=readJavaScriptOwnKeyCallbackPosition(operation),receiverPosition=readJavaScriptOwnKeyCallbackReceiverPosition(operation);if(position<0||receiverPosition<0)return env;
 const args=receiver?.k==='math'?receiver.args:[],object=local(args[0]?.value),owner=env.get(object);
 if(!acceptJavaScriptOwnKeyCallbackSource(receiver?.k??'',receiver?.op??'',args.length,args.some(item=>item.spread),object!==null&&owner!==undefined,owner?.kind??'',owner?.open===true,callback?.k??''))return env;
 const selected=new Map(env);selected.set(callbackOrigin,{callback,object,position,receiverPosition});return selected;
}
export function installJavaScriptOwnKeyCallbackBindings(node,parameters,env){
 const origin=env.get(callbackOrigin);if(!origin||origin.callback!==node)return;
 const parameter=parameters[origin.position];
 if(!acceptJavaScriptOwnKeyCallbackBinding(parameters.some(parameter=>parameter.name===origin.object),parameters.length>origin.receiverPosition,parameter?.type?.kind??''))return;
 env.set(facts,[...(env.get(facts)??[]),{object:origin.object,key:parameter.name}]);
}
export function readJavaScriptOwnPropertyBranchEnvironments(condition,env){
 const negated=condition.k==='unary'&&condition.op==='not',guard=negated?condition.arg:condition;
 const arm=readJavaScriptOwnPropertyGuardArm(guard.k,guard.op??'',negated);
 if(!arm)return {then:env,else:env};
 const args=guard.args,object=local(args[0]?.value),key=local(args[1]?.value),owner=env.get(object),index=env.get(key);
 if(!acceptJavaScriptOwnPropertyGuardBindings(args.length,args.some(item=>item.spread),object!==null&&owner!==undefined,key!==null&&index!==undefined,owner?.kind??'',index?.kind??'',owner?.open===true))return {then:env,else:env};
 const selected=new Map(env);selected.set(facts,[...(env.get(facts)??[]),{object,key}]);
 return arm===1?{then:selected,else:env}:{then:env,else:selected};
}
export function checkJavaScriptGuardedRecordIndex(checker,node,object,env,path,expected){
 if(object.type.kind!=='record'||node.index.k!=='name')return null;
 const objectName=local(node.object),keyName=local(node.index),key=checker.expr(node.index,env,path,undefined);
 const guarded=objectName!==null&&keyName!==null&&(env.get(facts)??[]).some(row=>row.object===objectName&&row.key===keyName);
 if(!acceptJavaScriptGuardedRecordIndex(object.type.kind,object.type.open===true,key.type.kind,guarded))throw unsupported('dynamic record index','an actual positive Object.hasOwn branch must guard unchanged local record/String binders; inherited and exotic property domains need their complete protocol',node.span);
 const fields=object.type.fields,element=fields[0]?.type??(expected?.kind==='option'?expected.element:expected??FLOAT);
 if(fields.some(field=>!sameType(field.type,element)))throw unsupported('heterogeneous dynamic record index','every actual field needs its shared dynamic value union',node.span);
 const result=option(element,'undefined'),captureName=checker.freshName('ml_guarded_record_receiver'),textName=checker.freshName('ml_guarded_record_key'),positionName=checker.freshName('ml_guarded_record_position');
 const owned={k:'var',name:captureName,type:object.type},text={k:'var',name:textName,type:STRING};
 const position={k:'var',name:positionName,type:FLOAT},value={k:'math',op:'readJavaScriptRecordPropertyIndex',args:[constructRecordPropertyNameRows(fields),text],type:FLOAT};
 let body={k:'none',absence:'undefined',type:result};
 for(const index of readJavaScriptGuardedRecordProjectionPositions(fields.length)){
  const field=fields[index],read={k:'recordField',object:owned,field:field.name,index,type:field.type};
  body={k:'if',cond:{k:'binary',op:'eq',left:position,right:{k:'lit',value:String(index),type:FLOAT},domain:FLOAT,type:BOOL},then:coerce(read,result,'JavaScript',node.span),else:body,type:result};
 }
 body={k:'let',name:positionName,value,body,type:result};
 return {k:'let',name:captureName,value:object,body:{k:'let',name:textName,value:key,body,type:result},type:result};
}
