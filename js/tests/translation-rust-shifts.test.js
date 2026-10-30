// Future normative/operator/codec controls. Do not run before the complete draft.
import assert from'node:assert/strict';
import test from'node:test';
import{runInNewContext}from'node:vm';
import{readProjectData}from'../scripts/project-data.mjs';
import{parseRust}from'../src/translation/rust.js';
import{checkProgram}from'../src/translation/check.js';
import{emitJavaScript}from'../src/translation/emit-javascript.js';
import{encodeTypedProgramLinks,emitTypedProgramLinks}from'../src/typed-program-links.js';
import{LinkNetwork}from'../src/network.js';
import{encodeLinkNetworkBinary,decodeLinkNetworkBinary}from'../src/link-network-binary.js';
const fixture=readProjectData(new URL('../../parity/fixtures/rust-shifts.lino',import.meta.url));
const[major,minor]=process.versions.node.split('.').map(Number);
assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.JavaScript);
for(const type of fixture.types)test(type.name+': exact shift values, aborts and complete codecs',()=>{
 const rows=fixture.functions.filter(row=>row.type===type.name),program=checkProgram(parseRust(rows.map(row=>row.source).join('\n')+'\nfn main(){}\n'));
 const js=emitJavaScript(program),packet=encodeTypedProgramLinks(program);
 for(const target of['JavaScript','Rust','Lean','Rocq']){
  const original=emitTypedProgramLinks(packet.network,packet.root,target);
  for(const network of[LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),original);
 }
 const names=rows.map(row=>js.mappings.find(mapping=>mapping.kind==='function'&&mapping.source===row.name).target);
 const functions=runInNewContext(js.text.replace(/^export /gmu,'')+'\n['+names.join(',')+'];');
 for(const[index,row]of rows.entries())for(const left of type.values)for(const count of fixture.countTypes.find(type=>type.name===row.countType).values){
  const a=BigInt(left),b=BigInt(count),fn=functions[index];
  if(b<0n||b>=BigInt(type.bits))assert.throws(()=>fn(a,b),error=>error.message==='attempt to shift '+row.direction+' with overflow');
  else{const value=row.direction==='left'?a<<b:a>>b;assert.equal(fn(a,b),type.signed?BigInt.asIntN(type.bits,value):BigInt.asUintN(type.bits,value));}
 }
});
test('Rust shift precedence and contextual literal widths',()=>{
 const js=emitJavaScript(checkProgram(parseRust(fixture.precedenceSource+fixture.contextualSource+'\nfn main(){}\n')));
 const names=['shift_precedence','shift_contextual'].map(name=>js.mappings.find(mapping=>mapping.kind==='function'&&mapping.source===name).target);
 const functions=runInNewContext(js.text.replace(/^export /gmu,'')+'\n['+names.join(',')+'];');
 assert.equal(functions[0](2n,1n),6n);assert.equal(functions[1](),4n);
});
