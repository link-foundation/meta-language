// Source pattern scopes lower to the existing ordinary exhaustive match algebra.
import{unsupported}from'./diagnostics.js';
function span(from,to){return{start:from.start,end:Math.max(from.end,to&&to.kind!=='eof'?to.start:from.end)};}

export function readRustConditionalPattern(parser,path,start){
 const c=parser.cursor;c.expect('let','conditional pattern');
 const aliases=[],alternatives=[parser.pattern(path,aliases)];
 while(c.eat('|'))alternatives.push(parser.pattern(path,aliases));
 if(alternatives.length>1&&aliases.length)throw unsupported('binding in an or-pattern','bind the value in each alternative separately',span(start,c.peek()));
 c.expect('=','conditional pattern');
 const scrutinee=parser.comparison(path,{noStruct:true});
 if(c.is('&&')||c.is('||'))throw unsupported('Rust conditional pattern chain','chained conditions require their complete short-circuit scope interpretation',span(start,c.peek()));
 const previous=parser.origins.enter(parser.origins.cases(alternatives,scrutinee,path));
 let body,bodyOrigin;
 try{
  for(const alias of aliases)parser.origins.bind(alias.name,alias.value,path);
  body=parser.block(path);bodyOrigin=parser.origins.origin(body,path);
 }finally{parser.origins.scope=previous;}
 for(const alias of [...aliases].reverse())body={k:'let',name:alias.name,value:alias.value,body,span:body.span};
 if(!c.eat('else'))throw unsupported('if without else','the value of an if must be defined on both branches',span(start,c.peek()));
 const otherwise=c.is('if')?parser.ifExpr(path):parser.block(path);
 const where=span(start,c.peek());
 const result={k:'match',scrutinees:[scrutinee],rows:[...alternatives.map(pattern=>({patterns:[pattern],body})),{patterns:[{k:'wild'}],body:otherwise}],span:where};
 return parser.origins.mark(result,parser.origins.join([bodyOrigin,parser.origins.origin(otherwise,path)]));
}
