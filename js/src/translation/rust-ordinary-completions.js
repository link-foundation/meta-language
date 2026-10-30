import {readRustPanicCompletionKind} from './rust-panic-value-rules.js';
import {hasRustConditionalChain,readRustConditionalChainStages,lowerRustConditionalChainStages} from './rust-conditional-chains.js';
import {findRustReturningLetElseCompletion} from './let-else-completion-rules.js';
import {readRustLetElseInitializerError} from './rust-record-rules.js';
import {rustNominalBindingNames} from './rust-records.js';
// Actual token/term transport into shared ordinary completion constructions.
import {unsupported} from './diagnostics.js';
import {readRustOrdinaryStatementDisposition,readRustImplicitBlockStatementDisposition,readRustUnitStatementDisposition,readRustFunctionMacroDisposition,readRustOrdinaryCompletionConstruction,readRustOrdinaryCompletionAlternativeKind,readRustOrdinaryCompletionSequenceKind,acceptRustOrdinaryCompletedAlternative} from './rust-unit-value-rules.js';
const range=(from,to)=>({start:from.start,end:Math.max(from.end,to?.kind==='eof'?from.end:to.start)});
const name=path=>({k:'name',path});
function leaf(parser,kind,expression,path){return {node:kind,kind:kind==='return'?'returned':kind==='abort'?'aborted':'value',expression,origin:parser.origins.origin(expression,path)};}
export function readRustOrdinaryFunctionCompletion(parser,path,ret,sourceResult){
 const tree=block(parser,path),context={parser,path,ret,sourceResult,outputs:[],data:null};
 const expression=lower(tree,true,context);
 parser.origins.mark(expression,parser.origins.join(context.outputs));
 return expression;
}
function block(parser,path){
 const c=parser.cursor,open=c.expect('{','block'),previous=parser.origins.enter();
 try{const result=sequence(parser,path,open);c.expect('}','block');return result;}
 finally{parser.origins.scope=previous;}
}
function sequence(parser,path,open){
 const c=parser.cursor,bindings=[];
 let result;
 for(;;){
  if(c.eat(';'))continue;
  if(c.is('let')){if(hasLetElseCompletion(parser)){result=letElseCompletion(parser,path,open);break;}bindings.push(...parser.letStatement(path));continue;}
  const token=c.peek();
  if(c.is('}')){result=leaf(parser,'value',{k:'unit',span:range(open,token)},path);break;}
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
 for(let index=bindings.length-1;index>=0;index-=1)result={node:'binding',binding:bindings[index],body:result,kind:result.kind};
 return result;
}
function conditional(parser,path){
 const c=parser.cursor,token=c.next();
 let cond,stages,then;
 if(hasRustConditionalChain(parser)){
  const previous=parser.origins.enter();
  try{stages=readRustConditionalChainStages(parser,path,token);then=block(parser,path);}
  finally{parser.origins.scope=previous;}
 }else{cond=parser.expr(path,{noStruct:true});parser.origins.direct(cond,path,cond.span);then=block(parser,path);}
 const hasElse=Boolean(c.eat('else'));
 const otherwise=hasElse?(c.is('if')?conditional(parser,path):block(parser,path)):leaf(parser,'value',{k:'unit',span:range(token,token)},path);
 return {node:'conditional',cond,stages,then,otherwise,kind:readRustOrdinaryCompletionAlternativeKind(then.kind,otherwise.kind),span:range(token,c.peek())};
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
 c.expect('=','let');const first=c.index,value=parser.expr(path),tokens=c.tokens.slice(first,c.index);
 const reason=readRustLetElseInitializerError(tokens.map(token=>token.kind==='punct'?'punct':'atom'),tokens.map(token=>token.value));
 if(reason)throw unsupported('Rust let-else initializer',reason,range(start,c.peek()));
 c.expect('else','let-else');const otherwise=block(parser,path);
 if(!acceptRustOrdinaryCompletedAlternative(otherwise.kind))throw unsupported('Rust let-else fallback','the fallback needs a represented diverging completion',range(start,c.peek()));
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
 const declaration={k:'data',name:identifier,generated:true,ctors:[{name:'FunctionResult',fields:[{name:'result',type:ret}]},{name:'StatementContinuation',fields:[]}]};
 context.data=[...path,identifier];
 parser.ordinaryCompletionDeclarations.push({path:[...path],declaration});
 return context.data;
}
function constructor(context,variant,args,span){return {k:'app',fn:name(['crate',...data(context),variant]),args,...(span?{span}:{})};}
function lower(tree,terminal,context){
 const {parser,path}=context,form=readRustOrdinaryCompletionConstruction(tree.node,terminal);
 if(form==='value'||form==='completed'){
  if(tree.node!=='abort')parser.origins.argument(tree.origin,context.sourceResult,tree.expression.span);
  context.outputs.push(tree.origin);
  return form==='completed'?constructor(context,'FunctionResult',[tree.expression],tree.expression.span):tree.expression;
 }
 if(form==='continuation'){
  const origin=tree.origin;
  if(readRustUnitStatementDisposition(tree.expression.k,parser.origins.collecting,origin.proven,origin.type?.kind??'',origin.wrappers.length>0)!=='bind')throw unsupported('ordinary Rust statement completion','a falling-through branch needs an actual direct Unit result before its enclosing continuation',tree.expression.span);
  return {k:'let',name:parser.freshProductBinding(),value:tree.expression,body:constructor(context,'StatementContinuation',[],tree.expression.span),span:tree.expression.span};
 }
 if(form==='binding')return {k:'let',...tree.binding,body:lower(tree.body,terminal,context),span:tree.binding.span};
 if(form==='conditional'){const then=lower(tree.then,terminal,context),otherwise=lower(tree.otherwise,terminal,context);return tree.stages?lowerRustConditionalChainStages(tree.stages,then,otherwise,tree.span):{k:'if',cond:tree.cond,then,else:otherwise,span:tree.span};}
 if(form==='join'){
  const first=lower(tree.first,false,context),binding=parser.freshProductBinding(),result=parser.freshProductBinding();
  const completed=terminal?name([result]):constructor(context,'FunctionResult',[name([result])],tree.span);
  const rest=lower(tree.rest,terminal,context),owner=['crate',...data(context)];
  return {k:'let',name:binding,value:first,body:{k:'match1',scrutinee:name([binding]),cases:[{pattern:{k:'ctor',path:[...owner,'FunctionResult'],binds:[result]},body:completed},{pattern:{k:'ctor',path:[...owner,'StatementContinuation'],binds:[]},body:rest}]},span:tree.span};
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
