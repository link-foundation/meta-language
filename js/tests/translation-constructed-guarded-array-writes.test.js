import assert from 'node:assert/strict';
import test from 'node:test';
import {runInNewContext} from 'node:vm';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {emitRust} from '../src/translation/emit-rust.js';
import {encodeTypedProgramLinks,decodeTypedProgramLinks} from '../src/typed-program-links.js';
import {recheckLinkedTypedProgram} from '../src/typed-program-checked-projection.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {preservesTranslationBehavior} from './support/translation-behavior.js';
const guard='Number.isInteger(position)&&position>=0&&position<data.length';
const moduleSource=(body,type='number')=>'/** @param {'+type+'} value @param {number} position @param {number} count @returns {'+type+'[]} */ export function updated(value,position,count){'+body+'}';
test('constructed guarded array programs execute the emitted Rust in native CI',()=>{
 const numeric=moduleSource('const data=[];for(let index=0;index<count;index+=1){data.push(0);}if('+guard+'){data[position]=value;}return data;').replace('export function','function');
 preservesTranslationBehavior(numeric+"\nconsole.log(updated(7,1,3).join(','));console.log(updated(7,-1,3).join(','));console.log(updated(7,0.5,3).join(','));console.log(updated(7,NaN,3).join(','));console.log(updated(7,0,0).join(','));console.log(1/(updated(-0,0,1)[0]??1));",'0,7,0\n0,0,0\n0,0,0\n0,0,0\n\n-Infinity\n');
 for(const[type,literal,value,expected]of [
  ['boolean','false','true','false,true,false\n'],
  ['string',"''","'前😀'",',前😀,\n'],
 ]){
  const source=moduleSource('const data=[];for(let index=0;index<count;index+=1){data.push('+literal+');}if('+guard+'){data[position]=value;}return data;',type).replace('export function','function');
  preservesTranslationBehavior(source+"\nconsole.log(updated("+value+",1,3).join(','));",expected);
 }
});
async function verify(source,values){
 const original=await import('data:text/javascript;base64,'+Buffer.from(source).toString('base64'));
 const checked=checkProgram(parseJavaScript(source)),rust=emitRust(checked),packet=encodeTypedProgramLinks(checked);
 for(const program of [checked,decodeTypedProgramLinks(LinkNetwork.fromLino(packet.network.toLino()),packet.root),decodeTypedProgramLinks(decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network)),packet.root)]){
  const restored=recheckLinkedTypedProgram(program,{wholeProgram:true}),native=emitRust(restored);
  for(const key of ['text','definitions','preludes','mappings','assumptions','encodings'])assert.deepEqual(native[key],rust[key]);
  const operation=runInNewContext(emitJavaScript(restored).text+'\n;updated;',{}, {timeout:1000});
  for(const count of [-2,0,0.5,1,3,5])for(const position of [-Infinity,-1,-0,0,0.5,1,2,3,5,NaN,Infinity])for(const value of values)assert.deepEqual(Array.from(operation(value,position,count)),original.updated(value,position,count));
 }
}
test('primitive fill loops and later guarded updates keep complete original observations and both archives',async()=>{
 await verify(moduleSource('const data=[];for(let index=0;index<count;index+=1){data.push(0);}if('+guard+'){data[position]=value;}return data;'),[-0,0,7,NaN,Infinity]);
 await verify(moduleSource('const data=[];data.push(0);if('+guard+'){data[position]=value;}data.push(1);if('+guard+'){data[position]=value+data.length;}return data;'),[-0,0,7,NaN]);
});
test('construction and guarded writes retain Boolean and exact string code-unit values',async()=>{
 await verify(moduleSource('const data=[];for(let index=0;index<count;index+=1){data.push(false);}if('+guard+'){data[position]=value;}return data;','boolean'),[false,true]);
 await verify(moduleSource("const data=[];for(let index=0;index<count;index+=1){data.push('');}if("+guard+'){data[position]=value;}return data;','string'),['','前😀','\ud800']);
});
test('joint proof refuses aliases captures unknown push arguments observed lengths rebinding and nested writes',()=>{
 for(const body of [
  'const data=[];const alias=data;data.push(0);if('+guard+'){data[position]=value;}return alias;',
  'const data=[];data.push(0);const capture=()=>data;if('+guard+'){data[position]=value;}return capture();',
  'const data=[];function next(){return 0;}data.push(next());if('+guard+'){data[position]=value;}return data;',
  'const data=[];const length=data.push(0);if('+guard+'){data[position]=value;}return data;',
  'let data=[];data.push(0);data=[1];if('+guard+'){data[position]=value;}return data;',
  'const data=[];data.push(0);while(value>0){if('+guard+'){data[position]=value;}value-=1;}return data;',
  'const data=[];if('+guard+'){data[position]=value;}return data;',
 ])assert.throws(()=>emitRust(checkProgram(parseJavaScript(moduleSource(body)))));
});
