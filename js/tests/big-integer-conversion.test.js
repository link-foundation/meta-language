// Future exact source-kernel controls. Never run before the whole code draft.
import assert from 'node:assert/strict';
import test from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';
import {convertJavaScriptStringBigInteger,convertJavaScriptStringBigIntegerUnits,convertJavaScriptNumberBigInteger,convertJavaScriptBooleanBigInteger} from '../src/big-integer-conversion.js';
import {createBigIntegerConversionProgramLinks} from '../src/big-integer-conversion-program.js';
import {emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {divideJavaScriptBigInteger} from '../src/translation/big-integer-conversion-rules.js';
import {divideJavaScriptBigInteger as divideInteger} from '../src/big-integer-conversion.js';
const fixture=readProjectData(new URL('../../parity/fixtures/big-integer-conversion.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);
assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node);
const units=text=>Array.from({length:text.length},(_,index)=>text.charCodeAt(index));
const observe=call=>{try{return{kind:'value',value:String(call())};}catch(error){return{kind:'error',errorClass:error.name,messageUnits:units(error.message)};}};
test('every exact BigInt fixture agrees with independent built-in conversions and actual errors',()=>{
 for(const row of fixture.cases){
  const value=row.kind==='string'?String.fromCharCode(...row.units):Number(row.spelling);
  const expected=row.expected.kind==='value'?row.expected:{kind:'error',errorClass:row.expected.errorClass,messageUnits:units(row.expected.prefix+String(value)+row.expected.suffix)};
  assert.deepEqual(observe(()=>BigInt(value)),expected,row.name);
  if(row.kind==='string'){
   assert.deepEqual(observe(()=>convertJavaScriptStringBigInteger(value)),expected,row.name);
   assert.deepEqual(observe(()=>convertJavaScriptStringBigIntegerUnits(row.units)),expected,row.name);
  }else assert.deepEqual(observe(()=>convertJavaScriptNumberBigInteger(value)),expected,row.name);
 }
 for(const value of [false,true])assert.equal(convertJavaScriptBooleanBigInteger(value),BigInt(value));
 for(const units of [[-1],[65536],[1.5],[NaN],null])assert.throws(()=>convertJavaScriptStringBigIntegerUnits(units),TypeError);
 assert.throws(()=>convertJavaScriptStringBigInteger(1),TypeError);
 assert.throws(()=>convertJavaScriptNumberBigInteger('1'),TypeError);
 assert.throws(()=>convertJavaScriptBooleanBigInteger(1),TypeError);
});
test('the full editable BigInt definition retains all emission and codec fields',()=>{
 const packet=createBigIntegerConversionProgramLinks();
 for(const target of ['JavaScript','Rust','Lean','Rocq']){
  const expected=emitTypedProgramLinks(packet.network,packet.root,target);
  for(const network of [LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),expected);
 }
});
test('shared BigInt division agrees with independent operators over signs, zero and arbitrary bit widths',()=>{
 const values=[];
 for(let value=-96n;value<=96n;value+=1n)values.push(value);
 for(const bits of [53n,64n,127n,256n,1024n])for(const sign of [-1n,1n])for(const delta of [-1n,0n,1n])values.push(sign*((1n<<bits)+delta));
 for(const left of values)for(const right of [-97n,-3n,-1n,0n,1n,2n,7n,99n,1n<<256n]){
  const actual=divideJavaScriptBigInteger(left,right);
  if(right===0n){assert.deepEqual(actual,[]);assert.throws(()=>left/right,RangeError);assert.throws(()=>left%right,RangeError);assert.throws(()=>divideInteger(left,right),{name:'RangeError',message:'Division by zero'});}
  else {assert.deepEqual(actual,[left/right,left%right]);assert.deepEqual(divideInteger(left,right),[left/right,left%right]);}
 }
 for(const [left,right] of [[1,1n],[1n,1],[undefined,1n],[1n,null]])assert.throws(()=>divideInteger(left,right),TypeError);
});
