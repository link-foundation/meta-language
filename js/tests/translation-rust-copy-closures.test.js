// Deferred complete definition and exact source-profile controls.
import assert from 'node:assert/strict';
import test from 'node:test';
import {readRustCopyClosureHeaderError,readRustCopyClosureCaptureError,readRustCopyClosureJoinError} from '../src/translation/rust-copy-closure-rules.js';
import {createSourceDefinitionProgramLinks} from '../src/source-definition-catalog.js';
import {emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
test('closure policy retains complete actual type and ownership requirements',()=>{assert.equal(readRustCopyClosureHeaderError(true,['value'],[true],true,true),'');assert.notEqual(readRustCopyClosureHeaderError(false,['value'],[true],true,true),'');assert.notEqual(readRustCopyClosureHeaderError(true,['value','value'],[true,true],true,true),'');assert.notEqual(readRustCopyClosureCaptureError(false,[false],0),'');assert.equal(readRustCopyClosureCaptureError(false,[true],0),'');assert.notEqual(readRustCopyClosureCaptureError(false,[true],1),'');assert.equal(readRustCopyClosureJoinError(true,true,true),'');assert.notEqual(readRustCopyClosureJoinError(true,true,false),'');});
test('complete Rust closure policy survives both codecs and every emission',()=>{const packet=createSourceDefinitionProgramLinks('meta-language/source-program/rust-copy-closure-program');for(const target of ['JavaScript','Rust','Lean','Rocq']){const expected=emitTypedProgramLinks(packet.network,packet.root,target);for(const network of [LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),expected);}});
