// Transport actual typed operands into existing product/optional operations.
import {typeError} from './diagnostics.js';
import {coerce} from './check.js';
import {readConditionalChainOperation} from './conditional-chain-rules.js';
const packet=element=>({kind:'option',element:{kind:'tuple',elements:[element]},absence:'undefined'});
const product=value=>({k:'tuple',items:[value],frozen:true,type:{kind:'tuple',elements:[value.type]}});
const element=type=>type?.kind==='option'&&type.absence==='undefined'&&type.element.kind==='tuple'&&type.element.elements.length===1?type.element.elements[0]:null;
export function checkConditionalChainValue(checker,node,env,path,expected){
 const result=node.op===readConditionalChainOperation('result'),success=node.op===readConditionalChainOperation('success');
 if(checker.language!=='Rust'||node.args.length!==(result?2:success?1:0)||node.args.some(arg=>arg.spread))throw typeError('a conditional continuation needs its complete actual operands',node.span);
 if(result){
  const left=checker.expr(node.args[0].value,env,path,expected?packet(expected):undefined),selected=element(left.type);
  if(!selected)throw typeError('a conditional continuation needs a single-field success packet',node.span);
  const right=checker.expr(node.args[1].value,env,path,selected),fallback=product(coerce(right,selected,checker.language,node.span));
  return {k:'tupleIndex',object:{k:'coalesce',left,right:fallback,type:left.type.element},index:0,type:selected};
 }
 const selected=element(expected);
 if(success){const raw=checker.expr(node.args[0].value,env,path,selected??undefined),value=selected?coerce(raw,selected,checker.language,node.span):raw,wrapped=product(value);return {k:'some',value:wrapped,type:packet(value.type)};}
 if(!selected)throw typeError('a conditional failure needs its actual success result context',node.span);
 return {k:'none',absence:'undefined',type:expected};
}
