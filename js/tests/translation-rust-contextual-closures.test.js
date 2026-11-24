// Deferred actual-context and complete source definition controls.
import assert from 'node:assert/strict';
import test from 'node:test';
import {parseRust} from '../src/translation/rust.js';
import {checkProgram} from '../src/translation/check.js';
import {createSourceDefinitionProgramLinks} from '../src/source-definition-catalog.js';
import {emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
test('written annotations remain independent constraints on an actual pointer context',()=>{for(const source of ['fn f()->i64{let read:fn(i64)->i64=|value:i32|value;read(7)}fn main(){}','fn f()->i64{let read:fn(i64)->i64=|value|true;read(7)}fn main(){}','fn f()->i64{let read:fn(i64,i64)->i64=|value|value;read(3,4)}fn main(){}'])assert.throws(()=>checkProgram(parseRust(source)),error=>error.kind==='type');});
test('contextual closure definitions survive both codecs and all target emissions',()=>{const packet=createSourceDefinitionProgramLinks('meta-language/source-program/rust-contextual-closure-program');for(const target of ['JavaScript','Rust','Lean','Rocq']){const expected=emitTypedProgramLinks(packet.network,packet.root,target);for(const network of [LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),expected);}});
