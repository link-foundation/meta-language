// Ordinary source control flow; private parser results never escape as IR ops.
import {unsupported} from './diagnostics.js';
import {readRustUnitStatementDisposition,readRustFunctionMacroDisposition,readRustOrdinaryConditionalCompletion,readRustOrdinaryCompletionKind,acceptRustOrdinaryElseIf} from './rust-unit-value-rules.js';
const range=(from,to)=>({start:from.start,end:Math.max(from.end,to?.kind==='eof'?from.end:to.start)});

export function readRustOrdinaryFunctionCompletion(parser,path){return block(parser,path).expression;}
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
  if(c.is('}')){result={kind:'value',expression:{k:'unit',span:range(open,token)}};break;}
  if(c.is('return')){
   const bare=c.is(';',1)||c.is('}',1);
   if(!bare&&!parser.isTerminalReturn())throw unsupported('early return','write the result as the tail expression of the block',range(token,token));
   c.next();
   const expression=bare?{k:'unit',span:range(token,token)}:parser.expr(path);
   if(!c.eat(';')&&!c.is('}'))throw parser.fail('expected ; or }');
   while(c.eat(';')){}
   if(!c.is('}'))throw unsupported('unreachable continuation','statements after a completed return are not removed by this model',range(token,c.peek()));
   result={kind:'returned',expression};break;
  }
  if(c.is('if')){result=conditional(parser,path,open);break;}
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
  result={kind:expression.k==='abort'?'aborted':'value',expression};break;
 }
 const origin=parser.origins.origin(result.expression,path);
 result.expression=bindings.reduceRight((body,binding)=>({k:'let',...binding,body,span:binding.span}),result.expression);
 parser.origins.mark(result.expression,origin);
 return result;
}
function conditional(parser,path,open){
 const c=parser.cursor,token=c.next();
 if(c.is('let'))throw unsupported('if let','write a match',range(token,c.peek()));
 const cond=parser.expr(path,{noStruct:true});parser.origins.direct(cond,path,cond.span);
 const then=block(parser,path),hasElse=Boolean(c.eat('else'));
 if(hasElse&&c.is('if')&&!acceptRustOrdinaryElseIf(then.kind))throw unsupported('guarded return completion','an else-if after a continuing alternative needs its complete joined completion carrier',range(token,c.peek()));
 let otherwise=hasElse?(c.is('if')?conditional(parser,path,open):block(parser,path)):null;
 const terminated=Boolean(c.eat(';')),continuation=!c.is('}');
 const action=readRustOrdinaryConditionalCompletion(then.kind,otherwise?.kind??'',hasElse,continuation,terminated);
 if(action==='guarded continuation')otherwise=sequence(parser,path,open);
 else if(action==='implicit Unit fallback')otherwise={kind:'value',expression:{k:'unit',span:range(token,token)}};
 else if(action==='returned conditional'||action==='aborted conditional'||action==='terminal mixed conditional'){}
 else if(action==='conditional value'){
  if(terminated)throw unsupported('expression statement','statements with effects are outside the portable core in function bodies',range(token,c.peek()));
 }else if(action==='unreachable continuation')throw unsupported('unreachable continuation','statements after a completed return are not removed by this model',range(token,c.peek()));
 else if(action==='discarded conditional value')throw unsupported('expression statement','a block value must be its last expression',range(token,c.peek()));
 else throw unsupported('guarded return completion','every guard branch must complete a return or abort, with an enclosing continuation; mixed return/value branches need a separate completion proof',range(token,c.peek()));
 const expression={k:'if',cond,then:then.expression,else:otherwise.expression,span:range(token,c.peek())};
 parser.origins.mark(expression,parser.origins.join([parser.origins.origin(then.expression,path),parser.origins.origin(otherwise.expression,path)]));
 return{kind:readRustOrdinaryCompletionKind(action,then.kind,otherwise.kind),expression};
}
