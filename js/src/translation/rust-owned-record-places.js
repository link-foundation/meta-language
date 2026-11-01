// Actual nested source places, never speculative value parsing/rollback.
import {readRustOwnedRecordPlaceTokenCount,readRustOwnedRecordPlaceDepthLimit} from './rust-owned-record-place-rules.js';
import {readRustRecordProjection} from './rust-records.js';
import {checkRustOwnedRecordRoot} from './rust-owned-record-updates.js';
import {unsupported} from './diagnostics.js';
import {readRustOwnedRecordMutableName} from './rust-owned-record-state.js';
export function readRustOwnedRecordPlaceBase(parser,path){
 const c=parser.cursor,limit=readRustOwnedRecordPlaceDepthLimit(),tokens=c.tokens.slice(c.index,c.index+limit*4+1),forms=tokens.map(token=>['(',')','.', '}'].includes(token.value)?token.value:token.kind==='identifier'?'name':token.kind==='number'&&/^(0|[1-9][0-9]*)$/u.test(token.raw)?'number':'other'),count=readRustOwnedRecordPlaceTokenCount(forms);
 if(count<0)return null;
 // A syntactic place must start in this actual lexical value scope. A path
 // naming a declaration or function is resolved by the ordinary reader.
 const first=tokens.find(token=>token.value!=='(');if(!first||!parser.origins.scope.has(first.value))return null;
 function atom(depth){
  if(depth>=limit)throw unsupported('Rust record place','the complete nested record place exceeds its source depth budget',c.peek());
  if(c.eat('(')){const value=place(depth+1);c.expect(')','record place');return value;}
  const token=c.identifier('record place'),origin=parser.origins.scope.get(token.value);if(!origin)throw unsupported('Rust record place','a place root needs its actual lexical source binding',token);
  checkRustOwnedRecordRoot(parser.origins,origin,{start:token.start,end:token.end});return readRustOwnedRecordMutableName(parser,{k:'name',path:[token.value],span:{start:token.start,end:token.end}},path);
 }
 function place(depth){let value=atom(depth);while(c.eat('.')){const field=c.next(),name=field.kind==='number'?field.raw:field.value,span={start:value.span.start,end:field.end},selected=readRustRecordProjection(parser,value,name,path,span);if(!selected)throw unsupported('Rust record place','every selected place field needs its actual complete nominal declaration',span);value=selected;}return value;}
 return place(0);
}
