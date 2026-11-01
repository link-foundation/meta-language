import {readRustPanicCompletionKind} from './rust-panic-value-rules.js';
import {hasRustConditionalChain,readRustConditionalChainStages,lowerRustConditionalChainStages} from './rust-conditional-chains.js';
import {findRustReturningLetElseCompletion} from './let-else-completion-rules.js';
import {readRustLetElseInitializerError} from './rust-record-rules.js';
import {rustNominalBindingNames} from './rust-records.js';
import {refuseRustStandardSumPrivateCompletions} from './rust-standard-sum-completions.js';
import {lowerRustExpressionCompletions,hasRustExpressionCompletion} from './rust-expression-completions.js';
import {withRustStandardSumSourceContext} from './rust-standard-sum-source.js';
// Actual token/term transport into shared ordinary completion constructions.
import {unsupported} from './diagnostics.js';
import {beginRustOwnedRecordStateBlock,endRustOwnedRecordStateBlock,readRustOwnedRecordStateCallable} from './rust-owned-record-state.js';
import {readRustOwnedRecordFiniteLoop,readRustOwnedRecordFiniteLoopSource,lowerRustOwnedRecordLoopCompletion} from './rust-owned-record-loops.js';
import {readRustOwnedRecordLoopLeafKind,readRustOwnedRecordLoopSignal} from './rust-owned-record-loop-rules.js';
import {readRustOrdinaryStatementDisposition,readRustImplicitBlockStatementDisposition,readRustUnitStatementDisposition,readRustFunctionMacroDisposition,readRustOrdinaryCompletionConstruction,readRustOrdinaryCompletionAlternativeKind,readRustOrdinaryCompletionSequenceKind,acceptRustOrdinaryCompletedAlternative,testRustOrdinaryCompletionFallsThrough,readRustOrdinaryCompletionSnapshotError} from './rust-unit-value-rules.js';
const range=(from,to)=>({start:from.start,end:Math.max(from.end,to?.kind==='eof'?from.end:to.start)});
const name=path=>({k:'name',path});
function snapshot(parser){const budget=parser.ordinaryCompletionOwnershipBudget??={entries:0,text:0},entries=parser.origins.sourceBindingStates.size;let text=0;for(const[key,value]of parser.origins.sourceBindingStates)text+=key.length+value.length;const error=readRustOrdinaryCompletionSnapshotError(budget.entries,entries,budget.text,text);if(error)throw unsupported('ordinary Rust completion ownership',error);budget.entries+=entries;budget.text+=text;return parser.origins.snapshotOwnedBindings();}
export function readRustOrdinaryCompletionOwnershipSnapshot(parser){return snapshot(parser);}
function leaf(parser,kind,expression,path){
 const success=readRustOwnedRecordLoopLeafKind(kind)||(kind==='return'?'returned':kind==='abort'?'aborted':'value');
 const origin=parser.origins.transferOwnedResult(expression,path);parser.origins.mark(expression,origin);
 return {node:kind,kind:hasRustExpressionCompletion(expression)?readRustOrdinaryCompletionAlternativeKind(success,'returned'):success,expression,origin,ownership:snapshot(parser)};
}
export function readRustOrdinaryFunctionCompletion(parser,path,ret,sourceResult){
 const tree=block(parser,path),context={parser,path,ret,sourceResult,outputs:[],data:null};
 const expression=lower(tree,true,context);
 refuseRustStandardSumPrivateCompletions(expression);
 parser.origins.mark(expression,parser.origins.join(context.outputs));
 return expression;
}
// The source reader retains the tree itself until progress and path ownership
// are authenticated. Its generated control type does not certify any value.
export function readRustOrdinaryLoopCompletion(parser,path){
 const sourceResult=parser.standardSumReturnOrigin;if(!sourceResult?.type)throw unsupported('Rust finite record loop','completed loop exits need their actual enclosing callable result type');
 const previous=parser.ordinaryCompletionLoop;parser.ordinaryCompletionLoop={callable:readRustOwnedRecordStateCallable(parser)};let tree;
 try{tree=block(parser,path);}finally{parser.ordinaryCompletionLoop=previous;}
 const context={parser,path,ret:sourceResult.type,sourceResult,outputs:[],data:null,loop:true};
 const expression=lower(tree,false,context),owner=data(context);
 return {tree,expression,owner,returns:context.outputs.filter(origin=>!origin.diverging)};
}
function block(parser,path){
 const before=snapshot(parser),frame=beginRustOwnedRecordStateBlock(parser),c=parser.cursor,previous=parser.origins.enter();
 try{const open=c.expect('{','block'),result=sequence(parser,path,open);c.expect('}','block');const current=snapshot(parser),leaves=completionLeaves(result);for(const {leaf}of leaves){parser.origins.restoreOwnedBindings(leaf.ownership);parser.origins.expireOwnedBindings(before,leaf.origin);leaf.ownership=snapshot(parser);}parser.origins.restoreOwnedBindings(current);const values=leaves.filter(({leaf,continuing})=>continuing&&leaf.node==='value');if(values.length)parser.origins.expireOwnedBindings(before,parser.origins.join(values.map(({leaf})=>leaf.origin)));else parser.origins.restoreOwnedBindings(before);return result;}
 finally{parser.origins.scope=previous;endRustOwnedRecordStateBlock(parser,frame);}
}
function completionLeaves(tree){const result=[],pending=[{tree,continuing:true}];while(pending.length){const {tree,continuing}=pending.pop();if(tree.origin)result.push({leaf:tree,continuing});if(tree.body)pending.push({tree:tree.body,continuing});if(tree.first)pending.push({tree:tree.first,continuing:false});if(tree.rest)pending.push({tree:tree.rest,continuing});if(tree.then)pending.push({tree:tree.then,continuing});if(tree.otherwise)pending.push({tree:tree.otherwise,continuing});}return result;}
function sequence(parser,path,open){
 const c=parser.cursor,bindings=[];
 let result;
 for(;;){
  if(c.eat(';'))continue;
  if(c.is('let')){if(hasLetElseCompletion(parser)){result=letElseCompletion(parser,path,open);break;}bindings.push(...parser.letStatement(path));continue;}
  const token=c.peek();
  if(c.is('}')){result=leaf(parser,'value',{k:'unit',span:range(open,token)},path);break;}
  if(token.kind==='identifier'&&['break','continue'].includes(token.raw)){
   if(parser.ordinaryCompletionLoop?.callable!==readRustOwnedRecordStateCallable(parser))throw unsupported('Rust loop exit','a loop exit needs its actual enclosing callable and loop scope',range(token,token));
   c.next();if(!c.eat(';')&&!c.is('}'))throw unsupported('Rust loop exit','labelled exits and break values need their explicit value and target interpretation',range(token,c.peek()));while(c.eat(';')){}
   if(!c.is('}'))throw unsupported('unreachable continuation','statements after a completed loop exit are not removed by this model',range(token,c.peek()));
   result=leaf(parser,token.raw,{k:'unit',span:range(token,token)},path);break;
  }
  if(c.is('return')){
   const bare=c.is(';',1)||c.is('}',1);
   if(!bare&&!parser.isTerminalReturn())throw unsupported('early return','write the result as the tail expression of the block',range(token,token));
   c.next();
   const expression=bare?{k:'unit',span:range(token,token)}:parser.expr(path);
   if(!c.eat(';')&&!c.is('}'))throw parser.fail('expected ; or }');
   while(c.eat(';')){}
   if(!c.is('}'))throw unsupported('unreachable continuation','statements after a completed return are not removed by this model',range(token,c.peek()));
   result=leaf(parser,'return',expression,path);break;
  }
  if(c.is('if')){
   const first=conditional(parser,path),terminated=Boolean(c.eat(';')),continuation=!c.is('}');
   if(acceptRustOrdinaryCompletedAlternative(first.kind)){
    if(continuation)throw unsupported('unreachable continuation','statements after a completed return are not removed by this model',first.span);
    result=first;
   }else if(continuation||terminated){const rest=sequence(parser,path,open);result={node:'sequence',first,rest,kind:readRustOrdinaryCompletionSequenceKind(first.kind,rest.kind),span:first.span};}
   else result=first;
   break;
  }
  const finiteLoop=readRustOwnedRecordFiniteLoop(parser,path);if(finiteLoop){c.eat(';');const source=readRustOwnedRecordFiniteLoopSource(parser,finiteLoop);if(source?.returns?.length){const first={node:'loop',expression:finiteLoop,kind:'mixed',span:finiteLoop.span};if(c.is('}'))result=first;else{const rest=sequence(parser,path,open);result={node:'sequence',first,rest,kind:readRustOrdinaryCompletionSequenceKind(first.kind,rest.kind),span:first.span};}break;}bindings.push({name:parser.freshProductBinding(),type:{kind:'unit'},value:finiteLoop,span:finiteLoop.span});continue;}
  if(['while','loop','for'].includes(token.value)&&token.kind==='identifier')throw unsupported(`${token.value} loop`,'loops over mutable state are outside the portable core; use recursion',range(token,token));
  const expression=token.kind==='macro'&&readRustFunctionMacroDisposition(token.value)==='print'?parser.functionPrintValue(path):parser.expr(path,{statement:true});
  const terminated=Boolean(c.eat(';')),completionKind=readRustPanicCompletionKind(expression.k,expression.op??'');
  if(terminated&&completionKind!=='abort'){
   const origin=parser.origins.origin(expression,path);
   if(readRustOrdinaryStatementDisposition(expression.k,parser.origins.collecting,origin.proven,origin.type?.kind??'',origin.wrappers.length>0)!=='bind')throw unsupported('expression statement','statements with effects are outside the portable core in function bodies',expression.span);
   bindings.push({name:parser.freshProductBinding(),value:expression,span:expression.span});continue;
  }
  const origin=parser.origins.origin(expression,path);
  const implicit=readRustImplicitBlockStatementDisposition(expression.k,expression.block===true,!c.is('}'),parser.origins.collecting,origin.proven,origin.type?.kind??'',origin.wrappers.length>0);
  if(implicit==='bind'){bindings.push({name:parser.freshProductBinding(),type:{kind:'unit'},value:expression,span:expression.span});continue;}
  if(!c.is('}')){
   if(completionKind==='abort')throw unsupported('unreachable continuation','statements after an abort are not removed by this model',expression.span);
   if(['if','match','block'].includes(expression.k))throw unsupported('expression statement','a block value must be its last expression',expression.span);
   throw parser.fail('expected }');
  }
  result=leaf(parser,completionKind==='abort'?'abort':'value',expression,path);break;
 }
 for(let index=bindings.length-1;index>=0;index-=1){
  const binding=bindings[index];
  result={node:'binding',binding,body:result,kind:hasRustExpressionCompletion(binding.value)?readRustOrdinaryCompletionAlternativeKind(result.kind,'returned'):result.kind};
 }
 return result;
}
function conditional(parser,path){
 const c=parser.cursor,token=c.next();
 let cond,stages,then,before=snapshot(parser);
 if(hasRustConditionalChain(parser)){
  const previous=parser.origins.enter();
  try{stages=withRustStandardSumSourceContext(parser,null,()=>readRustConditionalChainStages(parser,path,token));before=stages.ownershipFailure;then=block(parser,path);}
  finally{parser.origins.scope=previous;}
 }else{cond=withRustStandardSumSourceContext(parser,null,()=>parser.expr(path,{noStruct:true}));parser.origins.direct(cond,path,cond.span);before=snapshot(parser);then=block(parser,path);}
 const thenStates=snapshot(parser);parser.origins.restoreOwnedBindings(before);
 const hasElse=Boolean(c.eat('else'));
 const otherwise=hasElse?(c.is('if')?conditional(parser,path):block(parser,path)):leaf(parser,'value',{k:'unit',span:range(token,token)},path);
 parser.origins.joinOwnedBindings(thenStates,snapshot(parser),testRustOrdinaryCompletionFallsThrough(then.kind),testRustOrdinaryCompletionFallsThrough(otherwise.kind));
 const success=readRustOrdinaryCompletionAlternativeKind(then.kind,otherwise.kind);
 return {node:'conditional',cond,stages,then,otherwise,kind:hasRustExpressionCompletion(cond??stages)?readRustOrdinaryCompletionAlternativeKind(success,'returned'):success,span:range(token,c.peek())};
}
function hasLetElseCompletion(parser){
 const c=parser.cursor;let count=64;
 for(;;){const tokens=c.tokens.slice(c.index,c.index+count),selected=findRustReturningLetElseCompletion(tokens.map(token=>token.kind),tokens.map(token=>token.value),tokens.map((_,offset)=>c.is('else',offset)),tokens.map((_,offset)=>c.is('return',offset)));
  if(selected!==-2)return selected>=0;if(tokens.length<count)return false;count*=2;
 }
}
function letElseCompletion(parser,path,open){
 const c=parser.cursor,start=c.next(),aliases=[];
 if(c.is('mut'))throw unsupported('mutable binding','mutation is outside the portable core',range(start,c.peek()));
 const pattern=parser.pattern(path,aliases);
 if(aliases.length)throw unsupported('Rust nominal binding alias','whole-value aliases require their complete move and borrow interpretation',pattern.span);
 const names=new Set();for(const item of rustNominalBindingNames(parser,pattern)){if(names.has(item.name))throw unsupported('duplicate tuple binding','each name occurs only once in a destructuring pattern',item.span);names.add(item.name);}
 const type=c.eat(':')?parser.type():undefined,sourceType=type?parser.origins.type:null;
 c.expect('=','let');const first=c.index,value=withRustStandardSumSourceContext(parser,sourceType,()=>parser.expr(path)),tokens=c.tokens.slice(first,c.index);
 const reason=readRustLetElseInitializerError(tokens.map(token=>token.kind==='punct'?'punct':'atom'),tokens.map(token=>token.value));
 if(reason)throw unsupported('Rust let-else initializer',reason,range(start,c.peek()));
 c.expect('else','let-else');const beforeFallback=snapshot(parser),otherwise=block(parser,path);
 if(!acceptRustOrdinaryCompletedAlternative(otherwise.kind))throw unsupported('Rust let-else fallback','the fallback needs a represented diverging completion',range(start,c.peek()));
 parser.origins.restoreOwnedBindings(beforeFallback);
 c.expect(';','let');
 const span=range(start,c.peek()),capture=parser.freshProductBinding(),scrutinee={k:'name',path:[capture],span};
 parser.origins.bind(capture,value,path,sourceType);
 const previous=parser.origins.enter(parser.origins.cases([pattern],scrutinee,path));let then;
 try{then=sequence(parser,path,open);}finally{parser.origins.scope=previous;}
 const kind=readRustOrdinaryCompletionAlternativeKind(then.kind,otherwise.kind),stages={forms:[1],stages:[{patterns:[pattern],scrutinee,aliases:[],span}]};
 const body={node:'conditional',stages,then,otherwise,kind,span};
 return {node:'binding',binding:{name:capture,...(type===undefined&&parser.typeNamespaceModel?{}:{type}),value,span},body,kind};
}
function data(context){
 if(context.data)return context.data;
 const {parser,path,ret}=context,identifier=parser.freshProductBinding();
 const declaration={k:'data',name:identifier,generated:true,ctors:[{name:'FunctionResult',fields:[{name:'result',type:ret}]},{name:'StatementContinuation',fields:[]},...(context.loop?[{name:'LoopBreak',fields:[]},{name:'LoopContinue',fields:[]}]:[])]};
 context.data=[...path,identifier];
 parser.ordinaryCompletionDeclarations.push({path:[...path],declaration});
 return context.data;
}
function constructor(context,variant,args,span){return {k:'app',fn:name(['crate',...data(context),variant]),args,...(span?{span}:{})};}
function lower(tree,terminal,context){
 const {parser,path}=context,form=readRustOrdinaryCompletionConstruction(tree.node,terminal);
 if(form==='loop signal'){if(!context.loop)throw unsupported('Rust loop exit','a loop signal cannot escape its actual loop completion context',tree.expression.span);return constructor(context,readRustOwnedRecordLoopSignal(tree.node),[],tree.expression.span);}
 if(form==='finite loop'){const source=readRustOwnedRecordFiniteLoopSource(parser,tree.expression);context.outputs.push(...source.returns);return lowerRustOwnedRecordLoopCompletion(parser,tree.expression,['crate',...data(context)],terminal,context.sourceResult);}
 const lift=(expression,next)=>{
  if(!terminal&&hasRustExpressionCompletion(expression))data(context);
  return lowerRustExpressionCompletions(parser,expression,path,context.sourceResult,next,(value,origin)=>lower({node:'return',kind:'returned',expression:value,origin},terminal,context));
 };
 if(form==='value'||form==='completed'){
  if(tree.node!=='abort')parser.origins.argument(tree.origin,context.sourceResult,tree.expression.span);
  context.outputs.push(tree.origin);
  if(form==='completed')data(context);
  return lift(tree.expression,expression=>form==='completed'?constructor(context,'FunctionResult',[expression],expression.span):expression);
 }
 if(form==='continuation'){
  const origin=tree.origin;
  if(readRustUnitStatementDisposition(tree.expression.k,parser.origins.collecting,origin.proven,origin.type?.kind??'',origin.wrappers.length>0)!=='bind')throw unsupported('ordinary Rust statement completion','a falling-through branch needs an actual direct Unit result before its enclosing continuation',tree.expression.span);
  const binding=parser.freshProductBinding(),body=constructor(context,'StatementContinuation',[],tree.expression.span);
  return lift(tree.expression,expression=>({k:'let',name:binding,value:expression,body,span:expression.span}));
 }
 if(form==='binding'){const body=lower(tree.body,terminal,context);return lift(tree.binding.value,value=>({k:'let',...tree.binding,value,body,span:tree.binding.span}));}
 if(form==='conditional'){const then=lower(tree.then,terminal,context),otherwise=lower(tree.otherwise,terminal,context);return tree.stages?lift(lowerRustConditionalChainStages(tree.stages,then,otherwise,tree.span),expression=>expression):lift(tree.cond,cond=>({k:'if',cond,then,else:otherwise,span:tree.span}));}
 if(form==='join'){
  const first=lower(tree.first,false,context),binding=parser.freshProductBinding(),result=parser.freshProductBinding();
  const completed=terminal?name([result]):constructor(context,'FunctionResult',[name([result])],tree.span);
  const rest=lower(tree.rest,terminal,context),owner=['crate',...data(context)];
  return {k:'let',name:binding,value:first,body:{k:'match1',scrutinee:name([binding]),cases:[{pattern:{k:'ctor',path:[...owner,'FunctionResult'],binds:[result]},body:completed},{pattern:{k:'ctor',path:[...owner,'StatementContinuation'],binds:[]},body:rest},...(context.loop?['LoopBreak','LoopContinue'].map(variant=>({pattern:{k:'ctor',path:[...owner,variant],binds:[]},body:constructor(context,variant,[],tree.span)})):[])]},span:tree.span};
 }
 throw new TypeError('unknown shared Rust ordinary completion construction');
}
export function installRustOrdinaryCompletionDeclarations(parser,items){
 for(const entry of parser.ordinaryCompletionDeclarations){
  let target=items;
  for(const component of entry.path){const module=target.find(item=>item.k==='module'&&item.name===component);if(!module)throw unsupported('ordinary Rust completion carrier','the generated carrier needs its actual owning module');target=module.items;}
  target.push(entry.declaration);
 }
}
