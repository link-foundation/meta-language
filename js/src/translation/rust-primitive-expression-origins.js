// Actual lexical origins and literal syntax into shared source-owner decisions.
import {acceptRustPrimitiveArithmeticOrigin,selectRustPrimitiveArithmeticOrigin} from './rust-primitive-expression-rules.js';
function literal(node){
 const negative=node?.k==='unary'&&node.op==='neg'&&node.arg.k==='num',number=negative?node.arg:node;
 if(number?.k!=='num')return '';
 if(!number.type)return negative?'negative integer numeral':'integer numeral';
 return number.type.kind==='float'?(negative?'negative floating numeral':'floating numeral'):'';
}
export function selectRustPrimitiveExpressionOrigin(node,operands){
 const facts=operands.map(origin=>{
  const owner=origin.sourceOwner??(origin.type?.kind==='float'&&origin.floatSourceModel==='binary64 scalar'?{kind:'primitive',path:['f64'],arguments:[]}:null),type=origin.type,path=owner?.path.join('::')??'';
  const known=acceptRustPrimitiveArithmeticOrigin(type?.kind??'',String(type?.bits??''),type?.signed===true,owner?.kind??'',path,owner?.arguments.length??-1,origin.proven===true,origin.wrappers.length);
  return {origin,owner,path,known,signed:type?.signed===true};
 });
 const left=facts[0],right=facts[1],selected=selectRustPrimitiveArithmeticOrigin(node.op??'',left?.known===true,right?.known===true,left?.path??'',right?.path??'',left?.signed===true,right?.signed===true,literal(node.arg??node.left),literal(node.right));
 const result=facts[selected];return result?{...result.origin,sourceOwner:result.owner}:null;
}
