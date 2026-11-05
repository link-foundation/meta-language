// Closed literal char append/prefix reuses the canonical Rust lexer and the existing
// exclusive String concatenation. It creates no general char/String coercion.
import {unsupported} from './diagnostics.js';
import {STRING} from './types.js';
import {acceptRustExclusiveSourceStringReceiver} from './rust-exclusive-source-string-rules.js';
import {createRustExclusiveSourceStringMutation} from './rust-exclusive-source-strings.js';
import {selectRustExclusiveSourcePlace,recordRustExclusiveSourcePlaceWrite} from './rust-exclusive-source-calls.js';
const fail=(reason,span)=>{throw unsupported('Rust literal character String mutation',reason,span);};
export function readRustSourceStringCharacterPush(parser,receiver,method,path){
 if(parser.exclusiveSourceModel!=='Rust exclusive source loans 1'||!['push','insert'].includes(method.value))return null;
 const origin=parser.origins.origin(receiver,path);if(origin.type?.kind!=='string')return null;
 const owner=origin.expandedSourceOwner??origin.sourceOwner;
 if(!parser.origins.collecting&&!acceptRustExclusiveSourceStringReceiver(origin.type.kind,origin.textSourceModel??'',origin.wrappers.length,origin.proven===true,owner?.kind??'',owner?.path.join('::')??'',owner?.arguments.length??-1))fail('character mutation requires its actual standard owned String receiver',method);
 const place=selectRustExclusiveSourcePlace(parser,receiver,path);if(!place)fail('character mutation requires its actual mutable lexical String place',method);
 const c=parser.cursor;c.expect('(','String character mutation');const prepend=method.value==='insert';
 if(prepend){const index=c.next();if(index.kind!=='number'||!['0','0usize'].includes(index.raw))fail('this closed insert profile requires the actual zero usize byte boundary literal',index);c.expect(',','String insert character');}
 const character=c.next(),scalar=Number(character.value);
 if(character.kind!=='char'||!Number.isInteger(scalar)||scalar<0||scalar>0x10ffff||scalar>=0xd800&&scalar<=0xdfff||typeof parser.source!=='string'||parser.source.slice(character.start,character.end)!==character.raw)fail('this closed push profile requires one actual canonical scalar character literal',character);
 c.eat(',');const close=c.expect(')','String push'),span={start:receiver.span?.start??method.start,end:close.end};
 const suffix=parser.origins.mark({k:'str',value:String.fromCodePoint(scalar),span:{start:character.start,end:character.end}},{...parser.origins.typeOrigin(STRING,path),proven:true,copyEvidence:true,wrappers:['reference'],sourceLifetimes:[null],textSourceModel:'string slice',sourceOwner:{kind:'primitive',path:['str'],arguments:[]}});
 let value=createRustExclusiveSourceStringMutation(parser,place,{method:'push_str',parameters:[parser.origins.origin(suffix,path)]},[suffix],path,span);
 // Zero is a UTF8 boundary for every valid String, including empty text.
 // Both source operands are inert literals; only the actual receiver read is
 // transported. Other byte indices retain their separate panic/boundary work.
 if(prepend){if(value.k!=='binary'||value.op!=='concat')fail('insertion retains its shared text concatenation and actual state read',span);value=parser.origins.markOwnedStringConstructor({...value,left:suffix,right:value.left},parser.origins.origin(value,path));}
 return recordRustExclusiveSourcePlaceWrite(parser,place,value,path,span);
}
