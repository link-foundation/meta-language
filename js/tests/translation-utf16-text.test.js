// Future kernel controls. No execution before the complete requirements draft.
import assert from 'node:assert/strict';
import test from 'node:test';
import {sliceUtf16Text,substringUtf16Text,characterUtf16Text,relativeCharacterUtf16Text,codePointUtf16Text,pointsUtf16Text,indexUtf16Text,lastIndexUtf16Text,compareUtf16Text,startsUtf16Text,endsUtf16Text,includesUtf16Text,trimUtf16Text,characterUnitsUtf16Text,codePointUnitsUtf16Text,displayUtf8FromUtf16Text} from '../src/translation/utf16-text-rules.js';
import {createUtf16TextProgramLinks} from '../src/utf16-text-program.js';
import {createStringConstructionProgramLinks} from '../src/string-construction-program.js';
import {createFormalStringUnitProgramLinks} from '../src/formal-string-unit-program.js';
import {emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
const [major,minor]=process.versions.node.split('.').map(Number);
assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node);
const units=text=>Array.from({length:text.length},(_,at)=>text.charCodeAt(at));
const texts=['','abc','A😀B','\ud800x\udfff','\ud800','\udfff','\ufeff x \u2029','\u0000\uffff'];
const positions=[-Infinity,-10,-1,-0.9,-0,0,1,1.9,2,10,Infinity,NaN];
test('complete UTF16 definitions retain original string operations and scalar display boundaries',()=>{
 for(const text of texts){
  const input=units(text);
  assert.deepEqual(pointsUtf16Text(input),Array.from(text,units));
  assert.deepEqual(displayUtf8FromUtf16Text(input),Array.from(Buffer.from(text,'utf8')));
  for(const position of positions){
   assert.deepEqual(characterUtf16Text(input,position),units(text.charAt(position)));
   assert.deepEqual(relativeCharacterUtf16Text(input,position),text.at(position)===undefined?[]:[units(text.at(position))]);
   assert.deepEqual(codePointUtf16Text(input,position),text.codePointAt(position)===undefined?[]:[text.codePointAt(position)]);
   for(const end of positions){
    assert.deepEqual(sliceUtf16Text(input,position,end),units(text.slice(position,end)));
    assert.deepEqual(substringUtf16Text(input,position,end),units(text.substring(position,end)));
   }
   for(const search of texts){
    assert.equal(indexUtf16Text(input,units(search),position),text.indexOf(search,position));
    assert.equal(lastIndexUtf16Text(input,units(search),position),text.lastIndexOf(search,position));
   }
  }
  for(const search of texts){
   assert.equal(compareUtf16Text(input,units(search)),text===search?0:text<search?-1:1);
   assert.equal(startsUtf16Text(input,units(search)),text.startsWith(search));
   assert.equal(endsUtf16Text(input,units(search)),text.endsWith(search));
   assert.equal(includesUtf16Text(input,units(search)),text.includes(search));
  }
  assert.deepEqual(trimUtf16Text(input,true,true),units(text.trim()));
  assert.deepEqual(trimUtf16Text(input,true,false),units(text.trimStart()));
  assert.deepEqual(trimUtf16Text(input,false,true),units(text.trimEnd()));
 }
 for(const values of [[],[0,-0,NaN,Infinity,-Infinity],[65,65535,65536,65537,-1,1.9,2**100]])assert.deepEqual(characterUnitsUtf16Text(values),units(String.fromCharCode(...values)));
 for(const values of [[],[0,-0,55296,57343,65535,65536,1114111],[65,128512]])assert.deepEqual(codePointUnitsUtf16Text(values),[-1,...units(String.fromCodePoint(...values))]);
 for(const value of [-1,1.5,NaN,Infinity,1114112]){assert.deepEqual(codePointUnitsUtf16Text([65,value,0]),[1]);assert.throws(()=>String.fromCodePoint(65,value,0),RangeError);}
});
test('complete UTF16 operation graphs retain every definition in both codecs and all emissions',()=>{
 for(const create of [createUtf16TextProgramLinks,createStringConstructionProgramLinks,createFormalStringUnitProgramLinks]){
 const packet=create();
 for(const language of ['JavaScript','Rust','Lean','Rocq']){
  const expected=emitTypedProgramLinks(packet.network,packet.root,language);
  for(const network of [LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))])assert.deepEqual(emitTypedProgramLinks(network,packet.root,language),expected);
 }
 }
});
