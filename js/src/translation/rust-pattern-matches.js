// Transport shared macro expansion into the existing complete source match path.
import {unsupported} from './diagnostics.js';
import {acceptRustPatternMatchMacroOwner,planRustPatternMatchRows,readRustPatternMatchTokenValue,readRustPatternMatchTokenKind} from './rust-pattern-match-rules.js';
export function readRustPatternMatch(parser,token,path,receiver,close){
 const c=parser.cursor,owner=parser.resolve([token.value],path).join('::');
 if(!acceptRustPatternMatchMacroOwner(owner,parser.ordinaryCompletionContext()))throw unsupported('Rust pattern matches!','the actual standard macro needs its ordinary source interpretation',{start:token.start,end:c.peek().end});
 const original=c.tokens,plan=planRustPatternMatchRows(original.map(value=>value.kind),original.map(value=>value.value),c.index,close);
 if(plan.length===0)throw unsupported('Rust pattern matches!','a complete delimited source pattern and optional guard are required',{start:token.start,end:c.peek().end});
 const end=plan[0],anchor=original[end-1],generated=plan.slice(1).map(code=>{
  if(code>=0)return original[code];
  const value=readRustPatternMatchTokenValue(code),kind=readRustPatternMatchTokenKind(code);if(!value||!kind)throw new TypeError('unknown shared matches expansion token');
  return {kind,value,raw:value,start:anchor.start,end:anchor.end};
 });
 generated.push({...original.at(-1),kind:'eof',value:'',raw:'',start:anchor.end,end:anchor.end});
 c.tokens=generated;c.index=0;
 try{const result=parser.match(path,{token,scrutinee:receiver}),origin=parser.origins.origin(result,path);return parser.origins.mark({...result,span:{start:token.start,end:anchor.end}},origin);}
 finally{c.tokens=original;c.index=end;}
}
