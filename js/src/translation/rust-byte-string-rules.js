// Rust Reference lex.token.str-byte / lex.token.str-byte-raw. Offsets address
// the original UTF-16 source; byte values remain distinct from UTF-8 text.
import {readRustCharacterByteEscapeValue,readRustCharacterSimpleEscapeValue} from './rust-character-rules.js';
import {readRustStringContinuationEnd,readRustSourceLineBreakEnd} from './rust-string-rules.js';

/** Packet: [0] absent; [1,end,...bytes] literal; [negative,at,end] error.
 * Literal identifier suffixes belong to the host's existing XID lexer.
 * @param {number[]} units @param {number} start @returns {number[]} */
export function decodeRustByteStringLiteral(units,start){
 if(!Number.isInteger(start)||start<0||start>=units.length||units[start]!==98)return [0];
 const raw=units[start+1]===114;
 let opening=start+(raw?2:1);
 if(raw)while(opening<units.length&&units[opening]===35)opening+=1;
 if(units[opening]!==34)return [0];
 const hashes=raw?opening-start-2:0;
 if(hashes>255)return [-7,start,opening+1];
 let position=opening+1;let bytes=[];
 while(position<units.length){
  const unit=units[position]??-1;
  if(unit===34){
   let matches=true;
   for(let count=0;count<hashes;count+=1)if(units[position+1+count]!==35)matches=false;
   if(matches)return [1,position+1+hashes].concat(bytes);
  }
  if(!raw&&unit===92){
   const continuation=readRustStringContinuationEnd(units,position);
   if(continuation>=0){position=continuation;continue;}
   const escaped=units[position+1]??-1;
   if(escaped===117)return [-5,position,Math.min(position+2,units.length)];
   const value=escaped===120?readRustCharacterByteEscapeValue(units[position+2]??-1,units[position+3]??-1):readRustCharacterSimpleEscapeValue(escaped);
   if(value<0)return [-6,position,Math.min(position+(escaped===120?4:2),units.length)];
   bytes=bytes.concat([value]);position+=escaped===120?4:2;continue;
  }
  const lineBreak=readRustSourceLineBreakEnd(units,position);
  if(lineBreak>=0){bytes=bytes.concat([10]);position=lineBreak;continue;}
  if(unit===13)return [-4,position,position+1];
  if(!Number.isInteger(unit)||unit<0||unit>127)return [-3,position,position+1];
  bytes=bytes.concat([unit]);position+=1;
 }
 return [-2,start,units.length];
}
/** @param {number} code @returns {string} */
export function readRustByteStringLiteralError(code){
 if(code===-2)return 'unterminated Rust byte string literal';
 if(code===-3)return 'non-ASCII source in a Rust byte string literal';
 if(code===-4)return 'isolated carriage return in a Rust byte string literal';
 if(code===-5)return 'Unicode escape in a Rust byte string literal';
 if(code===-6)return 'malformed Rust byte string escape';
 if(code===-7)return 'Rust raw byte string exceeds 255 delimiter hashes';
 return 'invalid Rust byte string token';
}
/** Canonical byte payload, independent of host String encoding.
 * @param {number[]} bytes @returns {string} */
export function renderRustByteStringValue(bytes){
 let text='';
 for(let index=0;index<bytes.length;index+=1){
  const byte=bytes[index]??-1;
  if(!Number.isInteger(byte)||byte<0||byte>255)throw new RangeError('invalid Rust byte string payload');
  text=text+'0123456789abcdef'.charAt(Math.floor(byte/16))+'0123456789abcdef'.charAt(byte%16);
 }
 return text;
}
