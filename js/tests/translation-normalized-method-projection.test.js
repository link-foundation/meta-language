// Future whole graph controls. Never execute before the complete code draft.
import assert from 'node:assert/strict';
import test from 'node:test';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {recheckLinkedTypedProgram} from '../src/typed-program-checked-projection.js';
import {encodeTypedProgramLinks,decodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
const [major,minor]=process.versions.node.split('.').map(Number);
assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node);
const sources=[
 'export function compute(){return "a,b".split();}',
 'export function compute(){return "a,b".split(undefined,0);}',
 'export function compute(){return "a,b".split(",",2);}',
 'export function compute(){return "abc".slice(1,3);}',
 'export function compute(){return "abc".substring(2,0);}',
 'export function compute(){return "abc".indexOf("b",1);}',
 'export function compute(){return "abc".lastIndexOf("b");}',
 'export function compute(){return "abc".startsWith("b",1);}',
 'export function compute(){return "aba".replace("a","$$$&");}',
 'export function compute(){return "aba".replaceAll("a","$`");}',
 'export function compute(){return "abc".codePointAt(0);}',
 'export function compute(){return "abc".charAt("1");}',
 'export function compute(){return "abc".at(-1);}',
];
function findMethod(value,seen=new WeakSet()){
 if(!value||typeof value!=='object'||seen.has(value))return null;
 seen.add(value);if(value.k==='method'&&value.object.type.kind==='string')return value;
 for(const child of value instanceof Map?value.values():Object.values(value)){const found=findMethod(child,seen);if(found)return found;}
 return null;
}
for(const source of sources)test('normalized method projection stays exact: '+source,()=>{
 const program=checkProgram(parseJavaScript(source));
 assert.equal(recheckLinkedTypedProgram(program,{wholeProgram:true}),program);
 const packet=encodeTypedProgramLinks(program);
 for(const network of [LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]){
  const restored=decodeTypedProgramLinks(network,packet.root);
  assert.deepEqual(restored,program);
  assert.equal(recheckLinkedTypedProgram(restored,{wholeProgram:true}),restored);
  for(const target of ['JavaScript','Rust','Lean','Rocq'])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),emitTypedProgramLinks(packet.network,packet.root,target));
 }
 const original=findMethod(program);
 if(!original)return;
 for(const correction of [node=>{node.op='unregistered-method';},node=>{node.args.push({k:'lit',value:'0',type:{kind:'float'}});},node=>{node.type={kind:'bool'};}]){
  const forged=structuredClone(program);correction(findMethod(forged));
  assert.throws(()=>recheckLinkedTypedProgram(forged,{wholeProgram:true}));
 }
});
