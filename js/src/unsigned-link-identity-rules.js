// Exact unsigned 64-bit identities use little-endian 32-bit limbs. Every
// intermediate integer below is <= 2^53-1; no rounded identifier is admitted.
/** @param {number} exponent @returns {number} */
export function readUnsignedLinkByteFactor(exponent){
 if(!Number.isInteger(exponent)||exponent<0||exponent>3)throw new RangeError('invalid unsigned link byte position');
 let factor=1;for(let index=0;index<exponent;index+=1)factor*=256;return factor;
}
/** @param {number[]} words @param {number} index @returns {number} */
export function readUnsignedLinkLimb(words,index){return words[index]??-1;}
/** @param {number[]} words @returns {boolean} */
export function acceptUnsignedLinkIdentity(words){
 const low=readUnsignedLinkLimb(words,0),high=readUnsignedLinkLimb(words,1);
 return words.length===2&&Number.isInteger(low)&&low>=0&&low<4294967296&&Number.isInteger(high)&&high>=0&&high<4294967296&&(low!==0||high!==0);
}
/** @param {number[]} first @param {number[]} second @returns {number} */
export function compareUnsignedLinkIdentities(first,second){
 if(!acceptUnsignedLinkIdentity(first)||!acceptUnsignedLinkIdentity(second))throw new RangeError('invalid unsigned link identity');
 if(readUnsignedLinkLimb(first,1)!==readUnsignedLinkLimb(second,1))return readUnsignedLinkLimb(first,1)<readUnsignedLinkLimb(second,1)?-1:1;
 return readUnsignedLinkLimb(first,0)===readUnsignedLinkLimb(second,0)?0:readUnsignedLinkLimb(first,0)<readUnsignedLinkLimb(second,0)?-1:1;
}
/** Empty means exhausted, separately from the largest actual identity.
 * @param {number[]} words @returns {number[]} */
export function advanceUnsignedLinkIdentity(words){
 if(!acceptUnsignedLinkIdentity(words))throw new RangeError('invalid unsigned link identity');
 if(readUnsignedLinkLimb(words,0)===4294967295&&readUnsignedLinkLimb(words,1)===4294967295)return [];
 return readUnsignedLinkLimb(words,0)===4294967295?[0,readUnsignedLinkLimb(words,1)+1]:[readUnsignedLinkLimb(words,0)+1,readUnsignedLinkLimb(words,1)];
}
/** Decimal syntax is deliberately exact: no whitespace, sign or exponent.
 * Leading zeroes are retained as syntax only, not separate identities.
 * @param {string} text @returns {number[]} */
export function parseUnsignedLinkIdentity(text){
 if(text.length===0||text.length>20)throw new RangeError('invalid unsigned link identity');
 let low=0,high=0;
 for(let index=0;index<text.length;index+=1){
  const digit=text.charCodeAt(index)-48;
  if(digit<0||digit>9)throw new RangeError('invalid unsigned link identity');
  const lower=low*10+digit;
  const upper=high*10+Math.floor(lower/4294967296);
  if(upper>=4294967296)throw new RangeError('unsigned link identity overflow');
  low=lower%4294967296;high=upper;
 }
 const result=[low,high];
 if(!acceptUnsignedLinkIdentity(result))throw new RangeError('invalid unsigned link identity');
 return result;
}
/** @param {number[]} words @returns {string} */
export function renderUnsignedLinkIdentity(words){
 if(!acceptUnsignedLinkIdentity(words))throw new RangeError('invalid unsigned link identity');
 let low=readUnsignedLinkLimb(words,0),high=readUnsignedLinkLimb(words,1),text='';
 while(low!==0||high!==0){
  const upper=Math.floor(high/10),remainder=high%10;
  const lower=remainder*4294967296+low;
  text=(lower%10)+text;low=Math.floor(lower/10);high=upper;
 }
 return text;
}
/** @param {number[]} words @returns {number[]} */
export function encodeUnsignedLinkIdentity(words){
 if(!acceptUnsignedLinkIdentity(words))throw new RangeError('invalid unsigned link identity');
 const bytes=[];
 for(let limb=0;limb<2;limb+=1)for(let byte=0;byte<4;byte+=1)bytes.push(Math.floor(readUnsignedLinkLimb(words,limb)/readUnsignedLinkByteFactor(byte))%256);
 return bytes;
}
/** @param {number[]} bytes @param {number} offset @returns {number[]} */
export function readUnsignedLinkIdentity(bytes,offset){
 if(!Number.isSafeInteger(offset)||offset<0||offset+8>bytes.length)return [];
 let low=0,high=0;
 for(let index=0;index<8;index+=1){
  const byte=readUnsignedLinkLimb(bytes,offset+index);
  if(!Number.isInteger(byte)||byte<0||byte>255)return [];
  if(index<4)low+=byte*readUnsignedLinkByteFactor(index);else high+=byte*readUnsignedLinkByteFactor(index-4);
 }
 const words=[low,high];
 if(!acceptUnsignedLinkIdentity(words))return [];
 return words;
}
/** @param {number[]} bytes @param {number} offset @returns {number[]} */
export function decodeUnsignedLinkIdentity(bytes,offset){
 const words=readUnsignedLinkIdentity(bytes,offset);
 if(words.length!==2)throw new RangeError('invalid unsigned link identity bytes');
 return words;
}
/** MLU64N01: one versioned exact-ID directory and the existing complete v2
 * network body. Canonical directory order and exact framing are shared rules.
 * No graph metadata, term-cache or syntax-cache interpretation is duplicated.
 * @param {number[]} identities @param {number[]} next @param {number[]} body
 * @param {number} byteLimit @returns {number[]} */
export function encodeUnsignedLinkEnvelope(identities,next,body,byteLimit){
 if(!Number.isSafeInteger(byteLimit)||byteLimit<33||identities.length%2!==0||identities.length/2>4294967295||body.length>4294967295||body.length===0||identities.length*4+body.length+25>byteLimit)return [-1];
 if(next.length!==0&&!acceptUnsignedLinkIdentity(next))return [-2];
 const output=[77,76,85,54,52,78,48,49,next.length===0?1:0];
 const cursor=next.length===0?[1,0]:next;
 for(const byte of encodeUnsignedLinkIdentity(cursor))output.push(byte);
 for(let byte=0;byte<4;byte+=1)output.push(Math.floor(identities.length/2/readUnsignedLinkByteFactor(byte))%256);
 for(let offset=0;offset<identities.length;offset+=2){
  const words=[readUnsignedLinkLimb(identities,offset),readUnsignedLinkLimb(identities,offset+1)];
  if(!acceptUnsignedLinkIdentity(words)||offset>0&&compareUnsignedLinkIdentities([readUnsignedLinkLimb(identities,offset-2),readUnsignedLinkLimb(identities,offset-1)],words)>=0)return [-3];
  for(const byte of encodeUnsignedLinkIdentity(words))output.push(byte);
 }
 for(let byte=0;byte<4;byte+=1)output.push(Math.floor(body.length/readUnsignedLinkByteFactor(byte))%256);
 for(const byte of body){
  if(!Number.isInteger(byte)||byte<0||byte>255)return [-4];
  output.push(byte);
 }
 return output;
}
/** Result: exhausted flag, next low/high, directory count, directory limbs,
 * followed by every original body byte. No input suffix is discarded.
 * @param {number[]} input @param {number} byteLimit @returns {number[]} */
export function decodeUnsignedLinkEnvelope(input,byteLimit){
 if(!Number.isSafeInteger(byteLimit)||byteLimit<33||input.length>byteLimit||input.length<26)return [-1];
 const header=[77,76,85,54,52,78,48,49];
 for(let index=0;index<8;index+=1)if(readUnsignedLinkLimb(input,index)!==readUnsignedLinkLimb(header,index))return [-5];
 if(readUnsignedLinkLimb(input,8)!==0&&readUnsignedLinkLimb(input,8)!==1)return [-6];
 const next=readUnsignedLinkIdentity(input,9);
 if(next.length!==2)return [-2];
 if(readUnsignedLinkLimb(input,8)===1&&(readUnsignedLinkLimb(next,0)!==1||readUnsignedLinkLimb(next,1)!==0))return [-7];
 let count=0;
 for(let byte=0;byte<4;byte+=1){const value=readUnsignedLinkLimb(input,17+byte);if(!Number.isInteger(value)||value<0||value>255)return [-4];count+=value*readUnsignedLinkByteFactor(byte);}
 if(count>Math.floor((input.length-25)/8))return [-8];
 const output=[readUnsignedLinkLimb(input,8),readUnsignedLinkLimb(next,0),readUnsignedLinkLimb(next,1),count];
 let previous=[];
 for(let index=0;index<count;index+=1){
  const words=readUnsignedLinkIdentity(input,21+index*8);
  if(words.length!==2)return [-3];
  if(previous.length!==0&&compareUnsignedLinkIdentities(previous,words)>=0)return [-3];
  output.push(readUnsignedLinkLimb(words,0),readUnsignedLinkLimb(words,1));previous=words;
 }
 const position=21+count*8;
 let length=0;
 for(let byte=0;byte<4;byte+=1){const value=readUnsignedLinkLimb(input,position+byte);if(!Number.isInteger(value)||value<0||value>255)return [-4];length+=value*readUnsignedLinkByteFactor(byte);}
 if(length===0||position+4+length!==input.length)return [-9];
 for(let at=position+4;at<input.length;at+=1){const byte=readUnsignedLinkLimb(input,at);if(!Number.isInteger(byte)||byte<0||byte>255)return [-4];output.push(byte);}
 return output;
}

/** @param {number} code @returns {string} */
export function readUnsignedLinkEnvelopeError(code){
 if(code===-1)return 'unsigned link envelope resource limit';
 if(code===-2)return 'invalid unsigned link allocator';
 if(code===-3)return 'unsigned link directory must be strictly ordered';
 if(code===-4)return 'invalid unsigned link envelope byte';
 if(code===-5)return 'unsupported unsigned link envelope';
 if(code===-6)return 'invalid unsigned link exhaustion flag';
 if(code===-7)return 'noncanonical exhausted unsigned link allocator';
 if(code===-8)return 'truncated unsigned link directory';
 if(code===-9)return 'unsigned link envelope body length mismatch';
 return 'invalid unsigned link envelope result';
}
