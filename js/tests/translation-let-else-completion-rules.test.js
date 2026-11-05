// Deferred independent lexical selection, source definitions and codec controls.
import assert from 'node:assert/strict';
import test from 'node:test';
import {tokenize,TokenCursor} from '../src/translation/lexer.js';
import {findRustReturningLetElseCompletion} from '../src/translation/let-else-completion-rules.js';
import {createSourceDefinitionProgramLinks} from '../src/source-definition-catalog.js';
import {emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
test('only actual fallback return keywords select the stronger binding route',()=>{
 const select=source=>{const tokens=tokenize(source,'Rust').tokens,cursor=new TokenCursor(tokens,'Rust');return findRustReturningLetElseCompletion(tokens.map(token=>token.kind),tokens.map(token=>token.value),tokens.map((_,offset)=>cursor.is('else',offset)),tokens.map((_,offset)=>cursor.is('return',offset)));};
 assert.ok(select('let Value::Some(x)=source else{return 7;};')>=0);
 for(const source of ['let value=if true{3}else{4};','let Value::Some(x)=source else{panic!("return")};','let r#return=3;'])assert.equal(select(source),-1);
});
test('complete let-else source definitions retain both codecs and four emissions',()=>{const packet=createSourceDefinitionProgramLinks('meta-language/source-program/let-else-completion-program');for(const target of ['JavaScript','Rust','Lean','Rocq']){const expected=emitTypedProgramLinks(packet.network,packet.root,target);for(const network of [LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),expected);}});
