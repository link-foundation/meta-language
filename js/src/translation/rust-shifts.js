import{typeError}from'./diagnostics.js';
import{acceptRustPrimitiveBitwiseType}from'./rust-primitive-bitwise-rules.js';
import{acceptRustShiftSourceOperation,readRustShiftCheckedOperation,readRustShiftOverflowMessage,
 renderRustCheckedShift,renderRustShiftIntegerArgument}from'./rust-shift-rules.js';
import{readFixedIntegerCastModulus}from'./fixed-integer-cast-rules.js';

export function checkRustShift(checker,node,env,path,expected){
 if(checker.language!=='Rust'||node.args.length!==2||node.args.some(argument=>argument.spread))throw typeError('a Rust shift requires two actual unspread integer operands',node.span);
 let left=checker.expr(node.args[0].value,env,path,expected?.kind==='fixed'?expected:undefined,true);
 let count=checker.expr(node.args[1].value,env,path,undefined,true);
 if(left.type.kind==='literal')left=checker.expr(node.args[0].value,env,path,expected?.kind==='fixed'?expected:checker.defaultNumber());
 if(count.type.kind==='literal')count=checker.expr(node.args[1].value,env,path,checker.defaultNumber());
 if(left.type.kind!=='fixed'||count.type.kind!=='fixed'||!acceptRustPrimitiveBitwiseType('fixed',left.type.bits)
  ||!acceptRustPrimitiveBitwiseType('fixed',count.type.bits))throw typeError('a Rust shift requires actual fixed integer owners',node.span);
 const type={kind:'option',element:left.type,absence:'undefined'};
 const value={k:'math',op:readRustShiftCheckedOperation(node.op),args:[left,count],type};
 return acceptRustShiftSourceOperation(node.op)?{k:'coalesce',left:value,
  right:{k:'abort',message:readRustShiftOverflowMessage(node.op),type:left.type},type:left.type}:value;
}

export function emitRustCheckedShift(target,expression,emit){
 const type=expression.type.element,count=expression.args[1].type;
 const args=expression.args.map((argument,index)=>renderRustShiftIntegerArgument(target,emit(argument),index===0?type.signed:count.signed));
 return renderRustCheckedShift(target,expression.op,args,type.bits,type.signed,count.signed,readFixedIntegerCastModulus(type.bits));
}
