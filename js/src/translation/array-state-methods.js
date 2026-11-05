// Real checked type/value owners for shared range-mutation decisions.
import {coerce} from './check.js';
import {unsupported} from './diagnostics.js';
import {readArrayStateMethodResult,readArrayStateMethodRequiredArguments,readArrayStateMethodArgumentRole,
  readArrayStateMethodArgumentProfile} from './array-state-method-rules.js';

export function checkArrayStateMethod(checker,node,object,env,path){
  if(object.type.kind!=='array'||object.type.reference!==true||!readArrayStateMethodResult(node.op))return null;
  if(node.args.length<readArrayStateMethodRequiredArguments(node.op))throw unsupported('shared array range arguments','the missing value needs an explicit undefined element domain',node.span);
  const args=node.args.map((argument,index)=>{
    if(!argument)throw unsupported('shared array range argument','each actual source argument needs its expression',node.span);
    const role=readArrayStateMethodArgumentRole(node.op,index);
    const value=checker.expr(argument,env,path,role==='element'?object.type.element:undefined);
    if(role==='element')return coerce(value,object.type.element,checker.language,argument.span??node.span);
    if(role&& !readArrayStateMethodArgumentProfile(value.type.kind,value.type.element?.kind??'',value.type.absence??''))throw unsupported('shared array range index','this index needs its complete primitive conversion and error domain',argument.span??node.span);
    return value;
  });
  return{k:'method',object,op:node.op,args,type:object.type,span:node.span};
}
