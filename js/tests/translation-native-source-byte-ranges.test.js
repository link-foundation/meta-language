import assert from 'node:assert/strict';
import test from 'node:test';
import {readNativeSourceByteRange} from '../scripts/native-source-byte-ranges.mjs';
import {readRustSourceItems} from '../scripts/generate-private-output-test-model.mjs';

test('native source ranges retain actual Unicode Rust CST bytes instead of UTF16 string coordinates',()=>{
  const source='// π and 😀 before the selected item\r\nfn choose() -> &\'static str { "λ😀" }\r\n';
  const bytes=Buffer.from(source),parsed=readRustSourceItems(bytes),item=parsed.items.find(row=>row.name==='choose');
  assert.ok(item);assert.equal(parsed.text(item.node),'fn choose() -> &\'static str { "λ😀" }');
  assert.equal(readNativeSourceByteRange(bytes,item.node.start,item.node.end),parsed.text(item.node));
  assert.equal(readNativeSourceByteRange(bytes,item.start,item.end),source.slice(0,-2));
  assert.notEqual(source.slice(item.node.start,item.node.end),parsed.text(item.node),'the original string slicing loses the actual byte owner');
  assert.equal(readNativeSourceByteRange(bytes,0,bytes.length),source);
});

test('byte source ranges preserve every valid boundary BOM line ending and scalar without normalization',()=>{
  for(const source of ['', '\ufeff', '\ufefffn sample() {}\r\n', 'Aπ😀e\u0301\r\n\u0000Z']){
    const bytes=Buffer.from(source),boundaries=[0];let position=0;
    for(const scalar of source){position+=Buffer.byteLength(scalar);boundaries.push(position);}
    for(const start of boundaries)for(const end of boundaries.filter(value=>value>=start)){
      const selected=readNativeSourceByteRange(bytes,start,end);
      assert.ok(Buffer.from(selected).equals(bytes.subarray(start,end)));
    }
    for(let offset=0;offset<=bytes.length;offset++)if(!boundaries.includes(offset)){
      assert.throws(()=>readNativeSourceByteRange(bytes,offset,offset),/UTF-8 boundaries/u,'even an empty selection cannot split a scalar');
      assert.throws(()=>readNativeSourceByteRange(bytes,offset,bytes.length),/UTF-8 boundaries/u);
      assert.throws(()=>readNativeSourceByteRange(bytes,0,offset),/UTF-8 boundaries/u);
    }
  }
});

test('invalid surrounding UTF8 unknown intervals active buffers and excessive source sizes refuse',()=>{
  for(const bytes of [Buffer.from([0xff,65]),Buffer.from([65,0xc0,0xaf]),Buffer.from([65,0xed,0xa0,0x80]),Buffer.from([65,0xf4,0x90,0x80,0x80])])
    assert.throws(()=>readNativeSourceByteRange(bytes,0,0),TypeError,'invalid context is never hidden by an empty range');
  const bytes=Buffer.from('abc');
  for(const [start,end]of [[-1,0],[0,4],[2,1],[NaN,0],[0,Infinity],[0.5,1],[0,'1'],[0,null]])
    assert.throws(()=>readNativeSourceByteRange(bytes,start,end),RangeError);
  assert.equal(readNativeSourceByteRange(Buffer.alloc(131072),0,0),'');
  assert.throws(()=>readNativeSourceByteRange(Buffer.alloc(131073),0,0),/budget/u);
  let invoked=0;const active=()=>{invoked++;throw new Error('must not execute');};
  assert.throws(()=>readNativeSourceByteRange(new Proxy(bytes,{get:active,getPrototypeOf:active}),0,1),/passive/u);
  const plain={get byteLength(){return active();}};assert.throws(()=>readNativeSourceByteRange(plain,0,0),/passive/u);
  const withGetters=Buffer.from('abc');
  for(const name of ['byteLength','byteOffset','buffer','constructor'])Object.defineProperty(withGetters,name,{get:active});
  assert.equal(readNativeSourceByteRange(withGetters,0,3),'abc');assert.equal(invoked,0,'intrinsic slots do not execute supplied getters');
  assert.equal(readNativeSourceByteRange(withGetters,1,2),'b','the actual intrinsic offset retains sliced pooled-buffer coordinates');
  assert.throws(()=>readNativeSourceByteRange(new Uint8Array(new SharedArrayBuffer(4)),0,0),/shared/u);
  assert.throws(()=>readNativeSourceByteRange(new DataView(new ArrayBuffer(4)),0,0),/passive/u);
});
