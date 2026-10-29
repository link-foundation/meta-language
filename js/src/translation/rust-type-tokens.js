import {readRustTypeClosingTokenParts} from './rust-type-token-rules.js';
export function splitRustTypeClosingToken(parser){
 const c=parser.cursor,token=c.peek(),parts=readRustTypeClosingTokenParts(token.value);if(!parts.length)return;
 let offset=token.start;const tokens=parts.map(value=>{const start=offset;offset+=value.length;return {...token,value,raw:value,start,end:offset};});c.tokens.splice(c.index,1,...tokens);
}
