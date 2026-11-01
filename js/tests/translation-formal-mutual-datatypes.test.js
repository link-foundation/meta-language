// Deferred source controls, not kernel/compiler observations.
import assert from 'node:assert/strict';
import test from 'node:test';
import * as rules from '../src/translation/callable-data-derivation-rules.js';
import {checkProgram} from '../src/translation/check.js';
import {emitLean} from '../src/translation/emit-lean.js';
import {emitRocq} from '../src/translation/emit-rocq.js';
import {readFormalDatatypeConstructiveProfiles} from '../src/translation/callable-data.js';
import {encodeTypedProgramLinks,decodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {formalMutualDatatypeDecisions as decisions,formalMutualDatatypeSurfaces as surfaces} from './fixtures/formal-mutual-datatypes.mjs';
for(const[index,row]of decisions.entries())test('independent formal mutual datatype policy '+index,()=>assert.deepEqual(rules[row.operation](...row.arguments),row.expected));
for(const row of surfaces)test(row.name,()=>{
 const checked=checkProgram(row.surface),profiles=readFormalDatatypeConstructiveProfiles({kind:'data',name:'First'},checked.declarations);
 assert.deepEqual([...profiles],[['First',{witness:0,rank:2}],['Second',{witness:0,rank:1}]]);
 const lean=emitLean(checked).text,rocq=emitRocq(checked).text;
 assert.match(lean,/mutual\ninductive First where/u);assert.match(lean,/inductive Second where/u);
 assert.ok(lean.indexOf('instance : Inhabited Second')<lean.indexOf('instance : Inhabited First'));
 assert.match(rocq,/Inductive First : Type :=/u);assert.match(rocq,/\nwith Second : Type :=/u);
 const packet=encodeTypedProgramLinks(checked);for(const network of [packet.network,LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]){
  assert.deepEqual(decodeTypedProgramLinks(network,packet.root),checked);
  for(const target of ['JavaScript','Rust','Lean','Rocq'])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),emitTypedProgramLinks(packet.network,packet.root,target));
 }
});
