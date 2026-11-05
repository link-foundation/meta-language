// Source token/scope transport only. Patterns, actual moves and exhaustiveness
// reuse existing source origins and the checked Core match operation.
import {unsupported} from './diagnostics.js';
import {withRustStandardSumSourceContext} from './rust-standard-sum-source.js';
import {readRustOrdinaryMatchCompletionPresence,readRustOrdinaryMatchArmCountError,readRustOrdinaryCompletionAlternativeKind,testRustOrdinaryCompletionFallsThrough} from './rust-unit-value-rules.js';
import {readRustPanicCompletionKind} from './rust-panic-value-rules.js';
const range=(from,to)=>({start:from.start,end:Math.max(from.end,to?.kind==='eof'?from.end:to.start)});
export function hasRustOrdinaryMatchCompletion(parser){
 if(parser.cursor.peek().raw!=='match')return false;
 let size=64;for(;;){const tokens=parser.cursor.tokens.slice(parser.cursor.index,parser.cursor.index+size),selected=readRustOrdinaryMatchCompletionPresence(tokens.map(token=>token.kind),tokens.map(token=>token.kind==='identifier'?token.raw:token.value));if(selected!==-1)return selected===1;if(tokens.length<size)return false;if(size>=100000)throw unsupported('ordinary Rust match completion','the complete source match exceeds its inspection budget',parser.cursor.peek());size=Math.min(100000,size*2);}
}
export function readRustOrdinaryMatchCompletion(parser,path,reader){
 const c=parser.cursor,start=c.next(),scrutinee=withRustStandardSumSourceContext(parser,null,()=>parser.expr(path,{noStruct:true})),ownershipBase=reader.snapshot(parser),arms=[];c.expect('{','match');let joined=new Map(),continuing=false,kind=null;
 while(!c.is('}')){
  const limit=readRustOrdinaryMatchArmCountError(arms.length+1);if(limit)throw unsupported('ordinary Rust match completion',limit,range(start,c.peek()));
  parser.origins.restoreOwnedBindings(ownershipBase);const armStart=c.peek(),aliases=[];c.eat('|');
  const patterns=withRustStandardSumSourceContext(parser,parser.origins.origin(scrutinee,path),()=>{const patterns=[parser.pattern(path,aliases)];while(c.eat('|'))patterns.push(parser.pattern(path,aliases));return patterns;});
  if(c.is('if'))throw unsupported('ordinary Rust match completion','a guarded completed arm needs its selected guard fallthrough continuation',range(armStart,c.peek()));
  if(patterns.length>1&&aliases.length)throw unsupported('binding in an or-pattern','bind the value in each alternative separately',range(armStart,c.peek()));
  const previous=parser.origins.enter(parser.origins.cases(patterns,scrutinee,path));let body,block=false;
  try{
   for(const alias of aliases)parser.origins.bind(alias.name,alias.value,path);c.expect('=>','match arm');block=c.is('{')||c.is('if')||c.is('match');
   if(c.is('{'))body=reader.block(parser,path);else if(c.is('if'))body=reader.conditional(parser,path);else if(hasRustOrdinaryMatchCompletion(parser))body=readRustOrdinaryMatchCompletion(parser,path,reader);
   else if(c.peek().raw==='return'){const token=c.next(),value=c.is(',')||c.is('}')?{k:'unit',span:range(token,token)}:parser.expr(path);body=reader.leaf(parser,'return',value,path);}
   else if(c.peek().raw==='break'||c.peek().raw==='continue'){const token=c.next();reader.requireLoop(parser,token);if(!c.is(',')&&!c.is('}'))throw unsupported('Rust loop exit','labelled exits and break values need their explicit value and target interpretation',range(token,c.peek()));body=reader.leaf(parser,token.raw,{k:'unit',span:range(token,token)},path);}
   else{const value=parser.expr(path);body=reader.leaf(parser,readRustPanicCompletionKind(value.k,value.op??'')==='abort'?'abort':'value',value,path);}
   for(const alias of [...aliases].reverse())body={node:'binding',binding:{name:alias.name,value:alias.value,span:alias.value.span},body,kind:body.kind};
   reader.expire(parser,body,ownershipBase);
   const falls=testRustOrdinaryCompletionFallsThrough(body.kind);parser.origins.joinOwnedBindings(joined,reader.snapshot(parser),continuing,falls);joined=reader.snapshot(parser);continuing=continuing||falls;kind=kind===null?body.kind:readRustOrdinaryCompletionAlternativeKind(kind,body.kind);
  }finally{parser.origins.scope=previous;}
  arms.push({patterns,body,span:range(armStart,c.peek())});if(block)c.eat(',');else if(!c.is('}'))c.expect(',','match arm');
 }
 c.expect('}','match');if(!arms.length)throw unsupported('empty match','uninhabited types are outside the portable core',range(start,c.peek()));parser.origins.restoreOwnedBindings(joined);
 return {node:'match',scrutinee,arms,kind,span:range(start,c.peek())};
}
