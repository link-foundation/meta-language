// Deferred exact codomain, projection and full source definition controls.
import assert from 'node:assert/strict';
import test from 'node:test';
import {acceptRustExactArrayLengthResult} from '../src/translation/rust-array-length-rules.js';
import {createSourceDefinitionProgramLinks} from '../src/source-definition-catalog.js';
import {emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
test('exact length cannot silently change width, sign or operand domain',()=>{assert.equal(acceptRustExactArrayLengthResult('length','fixed',64,false,'array'),true);for(const facts of [['length','fixed',32,false,'array'],['length','fixed',64,true,'array'],['length','fixed',64,false,'string'],['cast','fixed',64,false,'array']])assert.equal(acceptRustExactArrayLengthResult(...facts),false);});
test('complete exact length definition survives both codecs and all emissions',()=>{const packet=createSourceDefinitionProgramLinks('meta-language/source-program/rust-array-length-program');for(const target of ['JavaScript','Rust','Lean','Rocq']){const expected=emitTypedProgramLinks(packet.network,packet.root,target);for(const network of [LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),expected);}});
