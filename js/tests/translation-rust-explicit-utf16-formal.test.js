// Deferred complete explicit-model controls; do not run in source-only phase.
import test from 'node:test';
import assert from 'node:assert/strict';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {emitLean} from '../src/translation/emit-lean.js';
import {emitRocq} from '../src/translation/emit-rocq.js';
import {requireFormalUnitStrings} from '../src/translation/formal-unit-strings.js';
import {encodeTypedProgramLinks,decodeTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
const source='/** @param {string} left @param {string} right @returns {boolean} */ function before(left,right){return left<right;}';
test('explicit code-unit models retain their formal representation in Rust-owned complete programs',()=>{
 // A source-generated helper can retain JavaScript ordering inside a Rust
 // compilation unit. The checked operation itself selects its exact semantics.
 const program={...checkProgram(parseJavaScript(source,{referenceSemantics:false})),sourceLanguage:'Rust'};
 assert.equal(requireFormalUnitStrings(program),true);
 const packet=encodeTypedProgramLinks(program);
 for(const network of [LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]){
  const restored=decodeTypedProgramLinks(network,packet.root);assert.equal(restored.sourceLanguage,'Rust');assert.equal(requireFormalUnitStrings(restored),true);
  for(const emit of [emitLean,emitRocq]){const result=emit(restored);assert.ok(result.encodings.some(row=>row.id==='utf16-formal-strings'));assert.ok(result.text.includes('MLUtf16Text.compare'));}
 }
});
test('ordinary Rust String types alone do not select a code-unit source ordering',()=>{
 const program={sourceLanguage:'Rust',declarations:new Map(),main:{effects:[{k:'lit',value:'😀',type:{kind:'string'}}]}};
 assert.equal(requireFormalUnitStrings(program),false);
});
