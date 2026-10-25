// Deferred complete source-definition and scalar/function kind controls.
import assert from 'node:assert/strict';
import test from 'node:test';
import {planRustSourceConstantValue} from '../src/translation/rust-source-constant-rules.js';
import {createSourceDefinitionProgramLinks} from '../src/source-definition-catalog.js';
import {emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
test('a real function item cannot acquire a source constant read plan',()=>{assert.deepEqual(planRustSourceConstantValue('fn','chosen','temporary'),[]);assert.deepEqual(planRustSourceConstantValue('const','scope.chosen','temporary'),['let','temporary','name','scope.chosen','name','temporary']);});
test('complete source constant definitions survive both codecs and all emissions',()=>{const packet=createSourceDefinitionProgramLinks('meta-language/source-program/rust-source-constant-program');for(const target of ['JavaScript','Rust','Lean','Rocq']){const expected=emitTypedProgramLinks(packet.network,packet.root,target);for(const network of [LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),expected);}});
