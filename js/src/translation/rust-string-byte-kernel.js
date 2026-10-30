// UTF-8 bytes over Rust's Unicode scalar String domain. No host encoder needed.
/** @param {string} text @returns {number[]} */
export function encodeRustStringUtf8(text){
 return Array.from(text).flatMap(character=>{
  const scalar=character.codePointAt(0)?? -1;
  if(scalar<0||scalar>1114111||scalar>=55296&&scalar<=57343)throw new TypeError('Rust text needs Unicode scalar values');
  if(scalar<128)return [scalar];
  if(scalar<2048)return [192+Math.floor(scalar/64),128+scalar%64];
  if(scalar<65536)return [224+Math.floor(scalar/4096),128+Math.floor(scalar/64)%64,128+scalar%64];
  return [240+Math.floor(scalar/262144),128+Math.floor(scalar/4096)%64,128+Math.floor(scalar/64)%64,128+scalar%64];
 });
}
