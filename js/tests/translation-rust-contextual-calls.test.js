// Deferred source hierarchy, callback context, real-body and codec controls.
import assert from 'node:assert/strict';
import test from 'node:test';
import {readProjectData} from '../src/project-data-files.js';
import {findRustContextualClosureParenthesisCount} from '../src/translation/rust-contextual-closure-rules.js';
import {findRustDeclarationBodyBoundary} from '../src/translation/rust-declaration-header-rules.js';
import {parseRust} from '../src/translation/rust.js';
import {checkProgram} from '../src/translation/check.js';
import {createSourceDefinitionProgramLinks} from '../src/source-definition-catalog.js';
import {emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
test('complete lexical groups retain every delimiter and window boundary',()=>{const controls=readProjectData(new URL('../src/data/rust-declaration-header-controls.lino',import.meta.url));for(const row of controls.headerCases)assert.equal(findRustDeclarationBodyBoundary(row.kinds,row.values),row.end,row.name);for(const row of controls.parenthesisCases)assert.equal(findRustContextualClosureParenthesisCount(row.syntax),row.depth);});
test('header discovery never substitutes for the actual checked source body',()=>{for(const source of ['fn broken()->i64{missing_name}fn main(){}','fn broken()->i64{loop{}}fn main(){}','fn broken()->i64{apply(|value:bool|value,7)}fn apply(read:fn(i64)->i64,value:i64)->i64{read(value)}fn main(){}'])assert.throws(()=>checkProgram(parseRust(source)));});
test('a forward callback header also reaches the actual main body',()=>assert.doesNotThrow(()=>checkProgram(parseRust('fn main(){let value=apply(|value|value+1,6);assert_eq!(value,7);}fn apply(read:fn(i64)->i64,value:i64)->i64{read(value)}'))));
test('complete header discovery definitions survive both codecs and four emissions',()=>{const packet=createSourceDefinitionProgramLinks('meta-language/source-program/rust-declaration-header-program');for(const target of ['JavaScript','Rust','Lean','Rocq']){const expected=emitTypedProgramLinks(packet.network,packet.root,target);for(const network of [LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),expected);}});
