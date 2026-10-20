// Rust CST positions are UTF-8 byte offsets, including in Unicode source.
import {types} from 'node:util';

const typedArrayPrototype=Object.getPrototypeOf(Uint8Array.prototype);
const nativeByteLength=Object.getOwnPropertyDescriptor(typedArrayPrototype,'byteLength').get;
const nativeByteOffset=Object.getOwnPropertyDescriptor(typedArrayPrototype,'byteOffset').get;
const nativeBuffer=Object.getOwnPropertyDescriptor(typedArrayPrototype,'buffer').get;

export function readNativeSourceByteRange(source,start,end) {
  if(source===null||typeof source!=='object'||types.isProxy(source)||!types.isUint8Array(source))
    throw new TypeError('a passive native source byte buffer is required');
  if(types.isSharedArrayBuffer(nativeBuffer.call(source)))
    throw new TypeError('shared native source buffers are unsupported');
  const size=nativeByteLength.call(source);
  if(size>131072)throw new RangeError('the native source byte budget is exceeded');
  if(!Number.isSafeInteger(start)||!Number.isSafeInteger(end)||start<0||end<start||end>size)
    throw new RangeError('a complete bounded native source byte range is required');
  const decoder=new TextDecoder('utf-8',{fatal:true,ignoreBOM:true});
  decoder.decode(source); // Validate the complete context, including outside the selection.
  for(const offset of [start,end])if(offset<size&&(source[offset]&0xc0)===0x80)
    throw new RangeError('native source range endpoints must be UTF-8 boundaries');
  return decoder.decode(new Uint8Array(nativeBuffer.call(source),nativeByteOffset.call(source)+start,end-start));
}
