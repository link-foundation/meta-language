import assert from 'node:assert/strict';
import test from 'node:test';
import {percentEncode,percentDecode} from '../src/lino-serialization-write.js';
const encoder=new TextEncoder();
function originalByteEncoding(value){
 const source=String(value);if(source.length===0)return'%';
 return[...encoder.encode(source)].map(byte=>(byte>=48&&byte<=57||byte>=65&&byte<=90||byte>=97&&byte<=122||byte===45||byte===95||byte===46)?String.fromCharCode(byte):'%'+byte.toString(16).toUpperCase().padStart(2,'0')).join('');
}
test('bounded encoding preserves every UTF16 unit ASCII escape and supplementary scalar byte',()=>{
 const[major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; >=22.15 <23 or >=24 <25 required');
 for(let unit=0;unit<=65535;unit++){const value=String.fromCharCode(unit);assert.equal(percentEncode(value),originalByteEncoding(value));}
 for(const value of ['',null,undefined,false,true,0,-0,123,'!\'()*~/%','\uD83D\uDE00','\uDBFF\uDFFF','\uD800\uD800\uDC00\uDC00','\uFEFFπ𐀀\r\n'])assert.equal(percentEncode(value),originalByteEncoding(value));
});
test('encoding boundaries preserve complete output and Unicode across repeated bounded pieces',()=>{
 for(const length of[4095,4096,4097,8191,8192,8193]){
  const value='a'.repeat(length)+'π😀\n%'+'.'.repeat(length);
  assert.equal(percentEncode(value),originalByteEncoding(value));assert.equal(percentDecode(percentEncode(value)),value);
 }
 const source=('source 😀\n'.repeat(128*1024));
 const encoded=percentEncode(source);assert.equal(encoded,'source%20%F0%9F%98%80%0A'.repeat(128*1024));assert.equal(percentDecode(encoded),source);
});
