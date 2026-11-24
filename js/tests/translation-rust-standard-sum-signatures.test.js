// Source-authored controls only. Never run during complete requirements drafting.
import assert from 'node:assert/strict';
import test from 'node:test';
import {parseRust} from '../src/translation/rust.js';
import {checkProgram} from '../src/translation/check.js';
import {checkRustSourceSignature} from '../src/translation/rust-source-signatures.js';
import {createRustSourceOwnerProgramLinks} from '../src/rust-source-owner-program.js';
import {decodeTypedProgramLinks,encodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {readRustStandardSumSignatureArgumentConstructors} from '../src/translation/rust-standard-sum-signature-rules.js';

const original='pub fn same(value:Result<Option<u8>,u16>)->Result<Option<u8>,u16>{value}fn main(){}';
function version(){const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===24||major===22&&minor>=15,'uncovered Node '+process.versions.node);}
const owner=value=>value.owner.path.join('::');
function actualResult(program){const fn=program.declarations.get('same');return program.declarations.get(fn.ret.name);}

test('standard sum signature policy keeps exact complete constructor inventories',()=>{
 version();
 for(const [path,names,counts,expected] of [
  ['std::option::Option',['None','Some'],[0,1],[1]],
  ['std::result::Result',['Ok','Err'],[1,1],[0,1]],
  ['std::option::Option',['Some','None'],[1,0],[]],
  ['std::result::Result',['Ok','Err'],[1,0],[]],
  ['std::result::Result',['Ok','Err','Other'],[1,1,0],[]],
  ['local::Result',['Ok','Err'],[1,1],[]],
 ])assert.deepEqual(readRustStandardSumSignatureArgumentConstructors(path,names,counts),expected);
});

for(const sourceVersion of ['1.90.0','1.99.0'])test('retained nested sum owners require actual data layouts / '+sourceVersion,()=>{
 version();
 const surface=parseRust(original,{sourceVersion,retainSourceTypeOwners:true}),program=checkProgram(surface),fn=program.declarations.get('same');
 assert.equal(owner(fn.sourceSignature.result),'std::result::Result');
 assert.equal(fn.sourceSignature.result.owner.arguments[0].path.join('::'),'std::option::Option');
 assert.equal(fn.sourceSignature.result.owner.arguments[0].arguments[0].path.join('::'),'u8');
 assert.equal(fn.sourceSignature.result.owner.arguments[1].path.join('::'),'u16');
 assert.throws(()=>checkRustSourceSignature(fn.sourceSignature,fn.params.map(value=>value.type),fn.ret,'Rust'),/complete source field signatures/u);
 assert.deepEqual(checkRustSourceSignature(fn.sourceSignature,fn.params.map(value=>value.type),fn.ret,'Rust',value=>value,type=>program.declarations.get(type.name)),fn.sourceSignature);
 const packet=createRustSourceOwnerProgramLinks(original,{sourceVersion}),text=packet.network.toLino(),binary=encodeLinkNetworkBinary(packet.network);
 for(const network of [packet.network,LinkNetwork.fromLino(text),decodeLinkNetworkBinary(binary)]){
  const restored=decodeTypedProgramLinks(network,packet.root),data=actualResult(restored);
  assert.deepEqual(data.ctors.map(value=>value.name),['Ok','Err']);
  assert.deepEqual(restored.declarations.get('same').sourceSignature,fn.sourceSignature);
  assert.equal(owner(data.sourceFieldSignatures.ctors[0].fields[0]),'std::option::Option');
  assert.equal(owner(data.sourceFieldSignatures.ctors[1].fields[0]),'u16');
  for(const target of ['JavaScript','Rust','Lean','Rocq'])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),emitTypedProgramLinks(packet.network,packet.root,target));
 }
});

const corruptions=[
 ['remove actual constructor provenance',program=>{delete actualResult(program).sourceFieldSignatures;}],
 ['swap actual constructors',program=>{actualResult(program).ctors.reverse();}],
 ['drop error payload',program=>{actualResult(program).ctors[1].fields=[];}],
 ['change retained payload owner',program=>{actualResult(program).sourceFieldSignatures.ctors[1].fields[0].owner.path=['u8'];}],
 ['change function error owner',program=>{program.declarations.get('same').sourceSignature.result.owner.arguments[1].path=['u8'];}],
 ['attach an unrepresented owner field',program=>{program.declarations.get('same').sourceSignature.result.owner.extra='not represented';}],
];
for(const [label,change] of corruptions)test('source-free sum signature rejects '+label,()=>{
 version();
 const packet=createRustSourceOwnerProgramLinks(original,{sourceVersion:'1.90.0'}),program=decodeTypedProgramLinks(packet.network,packet.root);
 change(program);assert.throws(()=>encodeTypedProgramLinks(program));
});
