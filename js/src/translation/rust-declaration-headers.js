// Header-only discovery never supplies an executable function body. The normal
// syntax and ownership passes still parse every original body before returning.
import {findRustDeclarationBodyBoundary} from './rust-declaration-header-rules.js';
export function readRustDeclarationBodyBoundary(parser){
 const c=parser.cursor;let count=64;
 for(;;){const tokens=c.tokens.slice(c.index,c.index+count),end=findRustDeclarationBodyBoundary(tokens.map(token=>token.kind),tokens.map(token=>token.value));
  if(end>=0){const last=tokens[end-1];c.index+=end;return last;}
  if(end===-1||tokens.length<count)throw parser.fail('expected a complete balanced function body');
  count*=2;
 }
}
