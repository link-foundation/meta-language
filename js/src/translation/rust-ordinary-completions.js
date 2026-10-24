// Actual token/term transport into shared ordinary completion constructions.
import {unsupported} from './diagnostics.js';
import {readRustUnitStatementDisposition,readRustFunctionMacroDisposition,readRustOrdinaryCompletionConstruction,readRustOrdinaryCompletionAlternativeKind,readRustOrdinaryCompletionSequenceKind,acceptRustOrdinaryCompletedAlternative} from './rust-unit-value-rules.js';
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
  if(c.is('let')){bindings.push(...parser.letStatement(path));continue;}
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
  const terminated=Boolean(c.eat(';'));
  if(terminated&&expression.k!=='abort'){
   const origin=parser.origins.origin(expression,path);
   if(readRustUnitStatementDisposition(expression.k,parser.origins.collecting,origin.proven,origin.type?.kind??'',origin.wrappers.length>0)!=='bind')throw unsupported('expression statement','statements with effects are outside the portable core in function bodies',expression.span);
   bindings.push({name:parser.freshProductBinding(),value:expression,span:expression.span});continue;
  }
  if(!c.is('}')){
   if(expression.k==='abort')throw unsupported('unreachable continuation','statements after an abort are not removed by this model',expression.span);
   if(['if','match','block'].includes(expression.k))throw unsupported('expression statement','a block value must be its last expression',expression.span);
   throw parser.fail('expected }');
  }
  result=leaf(parser,expression.k==='abort'?'abort':'value',expression,path);break;
 }
 for(let index=bindings.length-1;index>=0;index-=1)result={node:'binding',binding:bindings[index],body:result,kind:result.kind};
 return result;
}
function conditional(parser,path){
 const c=parser.cursor,token=c.next();
 if(c.is('let'))throw unsupported('if let','write a match',range(token,c.peek()));
 const cond=parser.expr(path,{noStruct:true});parser.origins.direct(cond,path,cond.span);
 const then=block(parser,path),hasElse=Boolean(c.eat('else'));
 const otherwise=hasElse?(c.is('if')?conditional(parser,path):block(parser,path)):leaf(parser,'value',{k:'unit',span:range(token,token)},path);
 return {node:'conditional',cond,then,otherwise,kind:readRustOrdinaryCompletionAlternativeKind(then.kind,otherwise.kind),span:range(token,c.peek())};
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
 if(form==='conditional')return {k:'if',cond:tree.cond,then:lower(tree.then,terminal,context),else:lower(tree.otherwise,terminal,context),span:tree.span};
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
