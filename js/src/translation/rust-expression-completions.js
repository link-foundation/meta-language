// Transport actual source expressions through ordered callable continuations.
// Branches retain their own continuations; lambda bodies are never hoisted.
import {unsupported} from './diagnostics.js';
import {lowerRustStandardSumTryCompletion} from './rust-standard-sum-completions.js';
import {readRustExpressionCompletionForm,readRustExpressionCompletionChildRoles,
 acceptRustExpressionCompletionInertLeaf,readRustExpressionCompletionShortCircuitValue,readRustExpressionCompletionLimit,readRustExpressionCompletionPacketOperation} from './rust-expression-completion-rules.js';

export function hasRustExpressionCompletion(expression,budget={nodes:0}){
 const pending=[expression],seen=new Set();
 while(pending.length){
  const value=pending.pop();if(value===null||typeof value!=='object'||seen.has(value))continue;
  if(seen.size>=readRustExpressionCompletionLimit('work')||++budget.nodes>readRustExpressionCompletionLimit('inspection'))throw unsupported('Rust expression completion','the source expression exceeds its finite inspection budget',expression?.span);
  seen.add(value);if(value.k==='rustStandardSumTry')return true;
  for(const child of Object.values(value))if(child!==null&&typeof child==='object')pending.push(child);
 }
 return false;
}

export function lowerRustExpressionCompletions(parser,expression,path,sourceResult,continued,returned){
 let work=0;const inspection={nodes:0},contains=node=>hasRustExpressionCompletion(node,inspection);
 const mark=(node,original)=>parser.origins.mark(node,parser.origins.origin(original,path));
 const sourceType=origin=>origin.type??(origin.fields&&origin.fields.every(field=>sourceType(field))?{kind:'tuple',elements:origin.fields.map(sourceType)}:null);
 const capture=(value,next)=>{
  if(acceptRustExpressionCompletionInertLeaf(value.k))return next(value);
  const origin=parser.origins.origin(value,path),type=sourceType(origin);
  if(!origin.proven||!type)throw unsupported('Rust ordered completion capture','an earlier evaluated child needs its complete source type before a later propagation',value.span);
  const name=parser.freshProductBinding(),reference=parser.origins.mark({k:'name',path:[name],span:value.span},origin);
  return {k:'let',name,type,value,body:next(reference),span:value.span};
 };
 function ordered(node,roles,next,depth){
  const slots=[];
  for(const role of roles){
   const value=node[role];
   if(Array.isArray(value))value.forEach((child,index)=>{
    const wrapper=node.k==='array'&&role==='items'&&child?.value&&typeof child.value==='object'?child:null;
    slots.push({role,index,child:wrapper?wrapper.value:child,wrapper});
   });
   else if(value&&typeof value==='object')slots.push({role,index:null,child:value});
  }
  const selected=new Map();
  const step=at=>{
   if(at===slots.length){
    const rebuilt={...node};for(const role of roles)if(Array.isArray(node[role]))rebuilt[role]=[...node[role]];
    for(const[slot,value]of selected){if(slot.index===null)rebuilt[slot.role]=value;else rebuilt[slot.role][slot.index]=slot.wrapper?{...slot.wrapper,value}:value;}
    return next(mark(rebuilt,node));
   }
   const slot=slots[at],later=slots.slice(at+1).some(entry=>contains(entry.child));
   return visit(slot.child,value=>{
    const resume=actual=>{selected.set(slot,actual);return step(at+1);};
    // A named callee is an actual static/local callable reference, not a call.
    // Other earlier evaluated operands are captured before the later residue.
    return later?capture(value,resume):resume(value);
   },depth+1);
  };
  return step(0);
 }
 function visit(node,next,depth){
  if(++work>readRustExpressionCompletionLimit('work')||depth>readRustExpressionCompletionLimit('depth'))throw unsupported('Rust expression completion','ordered continuation construction exceeds its finite node/depth budget',node.span);
  if(!contains(node))return next(node);
  const form=readRustExpressionCompletionForm(node.k,node.op??'');
  if(form==='propagate')return visit(node.operand,operand=>lowerRustStandardSumTryCompletion(parser,{...node,operand},path,sourceResult,visit(node.payload,next,depth+1),returned),depth+1);
  if(form==='ordered')return ordered(node,readRustExpressionCompletionChildRoles(node.k),next,depth);
  if(form==='conditional')return visit(node.cond,cond=>mark({...node,cond,then:visit(node.then,next,depth+1),else:visit(node.else,next,depth+1)},node),depth+1);
  if(form==='short circuit')return visit(node.left,left=>{
   const short=readRustExpressionCompletionShortCircuitValue(node.op),literal={k:'bool',value:short,span:node.span};
   const branch=visit(node.right,next,depth+1),done=next(literal);
   return mark({k:'if',cond:left,then:short?done:branch,else:short?branch:done,span:node.span},node);
  },depth+1);
  if(form==='binding')return visit(node.value,value=>mark({...node,value,body:visit(node.body,next,depth+1)},node),depth+1);
  if(form==='print')return visit(node.expr,expr=>mark({...node,expr,body:visit(node.body,next,depth+1)},node),depth+1);
  if(form==='match'){
   const roles=node.k==='match1'?['scrutinee']:['scrutinees'];
   return ordered(node,roles,selected=>{
    if(node.k==='match1')return mark({...selected,cases:node.cases.map(row=>({...row,body:visit(row.body,next,depth+1)}))},node);
    return mark({...selected,rows:node.rows.map(row=>({...row,body:visit(row.body,next,depth+1)}))},node);
   },depth);
  }
  if(form==='packet'){
   const operation=readRustExpressionCompletionPacketOperation(node.op);
   if(operation==='result')return packet(node.args[0].value,1,next,()=>visit(node.args[1].value,next,depth+1),depth+1);
   if(operation==='success')return visit(node.args[0].value,value=>next(mark({...node,args:[{...node.args[0],value}]},node)),depth+1);
  }
  // Unknown source forms and callable bodies remain intact for the final
  // explicit private-marker guard. No unsupported child is silently erased.
  return next(node);
 }
 function packet(node,levels,next,failed,depth){
  if(++work>readRustExpressionCompletionLimit('work')||depth>readRustExpressionCompletionLimit('depth'))throw unsupported('Rust conditional packet completion','packet context exceeds its finite node/depth budget',node.span);
  const role=node.k==='math'?readRustExpressionCompletionPacketOperation(node.op):'';
  if(role==='success')return levels===1?visit(node.args[0].value,next,depth+1):packet(node.args[0].value,levels-1,next,failed,depth+1);
  if(role==='failure')return failed();
  if(role==='result')return packet(node.args[0].value,levels+1,next,()=>packet(node.args[1].value,levels,next,failed,depth+1),depth+1);
  if(node.k==='if')return visit(node.cond,cond=>mark({...node,cond,then:packet(node.then,levels,next,failed,depth+1),else:packet(node.else,levels,next,failed,depth+1)},node),depth+1);
  if(node.k==='let')return visit(node.value,value=>mark({...node,value,body:packet(node.body,levels,next,failed,depth+1)},node),depth+1);
  if(node.k==='match'||node.k==='match1')return ordered(node,node.k==='match1'?['scrutinee']:['scrutinees'],selected=>node.k==='match1'?mark({...selected,cases:node.cases.map(row=>({...row,body:packet(row.body,levels,next,failed,depth+1)}))},node):mark({...selected,rows:node.rows.map(row=>({...row,body:packet(row.body,levels,next,failed,depth+1)}))},node),depth);
  throw unsupported('Rust conditional packet completion','the actual packet producer needs its complete source continuation interpretation',node.span);
 }
 return visit(expression,continued,0);
}
