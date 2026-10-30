// Deferred type/ownership boundaries and complete source-definition controls.
import assert from 'node:assert/strict';
import test from 'node:test';
import {parseRust} from '../src/translation/rust.js';
import {checkProgram} from '../src/translation/check.js';
import {createSourceDefinitionProgramLinks} from '../src/source-definition-catalog.js';
import {emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
test('pointer coercion retains the complete actual callable signature',()=>{assert.throws(()=>checkProgram(parseRust('fn other(value:i32)->i32{value}fn result()->i64{let read:fn(i64)->i64=other;read(7)}fn main(){}')),error=>error.kind==='type');});
test('complete pointer policy survives both codecs and every emission',()=>{const packet=createSourceDefinitionProgramLinks('meta-language/source-program/rust-function-pointer-program');for(const target of ['JavaScript','Rust','Lean','Rocq']){const expected=emitTypedProgramLinks(packet.network,packet.root,target);for(const network of [LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),expected);}});
