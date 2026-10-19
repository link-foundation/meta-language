// Independent original JavaScript observations and complete source-free replay.
import assert from 'node:assert/strict';
import test from 'node:test';
import {runInNewContext} from 'node:vm';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {emitRust} from '../src/translation/emit-rust.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {encodeTypedProgramLinks,decodeTypedProgramLinks} from '../src/typed-program-links.js';
import {recheckLinkedTypedProgram} from '../src/typed-program-checked-projection.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {preservesTranslationBehavior} from './support/translation-behavior.js';
const header='/** @param {number} value @param {number} position @returns {number[]} */\n';
const guard='Number.isInteger(position) && position >= 0 && position < data.length';
const kernel=body=>header+'export function updated(value,position){'+body+'}';
test('guarded writes execute emitted Rust in the native CI stage',()=>{
 const source=header+'function updated(value,position){const data=[2,4,6];if('+guard+'){data[position]=value;}return data;}';
 preservesTranslationBehavior(source+"\nconsole.log(updated(7,1).join(','));console.log(updated(7,-1).join(','));console.log(updated(7,0.5).join(','));console.log(updated(7,NaN).join(','));console.log(updated(Infinity,2).join(','));console.log(1/(updated(-0,0)[0]??1));",
  '2,7,6\n2,4,6\n2,4,6\n2,4,6\n2,4,Infinity\n-Infinity\n');
});
test('direct guarded writes preserve all valid and out-of-bounds Number paths through complete source-free programs',async()=>{
 const source=kernel('const data=[2,4,6];if('+guard+'){data[position]=value;}return data;');
 const original=await import('data:text/javascript;base64,'+Buffer.from(source).toString('base64'));
 const checked=checkProgram(parseJavaScript(source)),native=emitRust(checked),packet=encodeTypedProgramLinks(checked);
 const variants=[checked,decodeTypedProgramLinks(LinkNetwork.fromLino(packet.network.toLino()),packet.root),
  decodeTypedProgramLinks(decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network)),packet.root)];
 for(const variant of variants){
  const program=recheckLinkedTypedProgram(variant,{wholeProgram:true}),rust=emitRust(program);assert.equal(rust.text,native.text);
  for(const field of ['definitions','preludes','mappings','assumptions','encodings'])assert.deepEqual(rust[field],native[field]);
  const emitted=runInNewContext(emitJavaScript(program).text+'\n;updated',{}, {timeout:1000});
  for(const position of [-Infinity,-2,-1,-0,0,0.5,1,2,3,4,NaN,Infinity])for(const value of [-0,0,7,NaN,Infinity]){
   assert.deepEqual(Array.from(emitted(value,position)),original.updated(value,position));
  }
 }
});
test('the guarded profile leaves missing guards alias escapes captures and stateful loops as explicit translation gaps',()=>{
 for(const body of [
  'const data=[2,4,6];data[position]=value;return data;',
  'const data=[2,4,6];if(position>=0&&position<data.length){data[position]=value;}return data;',
  'const data=[2,4,6];if(Number.isInteger(position)&&position>=0){data[position]=value;}return data;',
  'const data=[2,4,6],alias=data;if('+guard+'){data[position]=value;}return alias;',
  'const data=[2,4,6];function capture(){return data;}if('+guard+'){data[position]=value;}return capture();',
  'const data=[2,4,6];while(value>0){if('+guard+'){data[position]=value;}value-=1;}return data;',
  'const data=[];if('+guard+'){data[position]=value;}return data;',
  'const data=[2,4,6];function shift(){position=100;return true;}if('+guard+'&&shift()){data[position]=value;}return data;',
  'const data=[2,4,6];function next(){return value+1;}if('+guard+'){data[position]=next();}return data;',
 ])assert.throws(()=>emitRust(checkProgram(parseJavaScript(kernel(body)))));
});
test('guarded source replays preserve guard order primitive element types and successive guarded writes',async()=>{
 const terms=['Number.isInteger(position)','position >= 0','position < data.length'];
 const orders=[[0,1,2],[0,2,1],[1,0,2],[1,2,0],[2,0,1],[2,1,0]];
 const sources=orders.map(order=>({source:kernel('const data=[2,4,6];if('+order.map(index=>terms[index]).join('&&')+'){data[position]=Math.abs(value);}return data;'),values:[-0,-3,NaN,Infinity]}));
 sources.push({source:kernel('const data=[2,4,6];if('+guard+'){data[position]=value;}if(Number.isSafeInteger(position)&&position>=0&&position<data.length){data[position]=value+data.length;}return data;'),values:[-0,0,7,NaN]});
 for(const[type,literal,values]of [
  ['boolean','[false,true]',[false,true]],
  ['string',"['first','second']",['','前😀','\ud800']],
 ])sources.push({source:'/** @param {'+type+'} value @param {number} position @returns {'+type+'[]} */\nexport function updated(value,position){const data='+literal+';if('+guard+'){data[position]=value;}return data;}',values});
 for(const {source,values}of sources){
  const original=await import('data:text/javascript;base64,'+Buffer.from(source).toString('base64'));
  const checked=checkProgram(parseJavaScript(source)),native=emitRust(checked),packet=encodeTypedProgramLinks(checked);
  for(const variant of [checked,decodeTypedProgramLinks(LinkNetwork.fromLino(packet.network.toLino()),packet.root),decodeTypedProgramLinks(decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network)),packet.root)]){
   const program=recheckLinkedTypedProgram(variant,{wholeProgram:true});assert.equal(emitRust(program).text,native.text);
   const emitted=runInNewContext(emitJavaScript(program).text+'\n;updated',{}, {timeout:1000});
   for(const position of [-1,-0,0,0.5,1,2,3,NaN,Infinity])for(const value of values)assert.deepEqual(Array.from(emitted(value,position)),original.updated(value,position));
  }
 }
});
