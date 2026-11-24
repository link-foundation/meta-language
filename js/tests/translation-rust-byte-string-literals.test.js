// Deferred controls; no execution during the complete code drafting phase.
import assert from 'node:assert/strict';
import test from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';
import {tokenize} from '../src/translation/lexer.js';
import {parseRust} from '../src/translation/rust.js';
const fixture=readProjectData(new URL('../../parity/fixtures/rust-byte-string-literals.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);
assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.JavaScript);
for(const row of fixture.positives)test('byte token '+row.source,()=>{
 const prefix='// 😀 absolute source\r\n';
 const source=prefix+row.source+' + following';
 const {tokens}=tokenize(source,'Rust');
 assert.deepEqual(tokens[0],{kind:'byteString',value:row.value,suffix:row.suffix??'',raw:row.source,start:prefix.length,end:prefix.length+row.source.length});
 assert.deepEqual(tokens.slice(1).map(token=>token.raw),['+','following','']);
 assert.equal(source.slice(tokens[0].start,tokens[0].end),row.source);
});
for(const row of fixture.negatives)test('byte refusal '+row.reason+' '+row.source,()=>{
 assert.throws(()=>tokenize(row.source,'Rust'),error=>error.kind==='syntax'&&error.message.includes(row.reason));
});
test('byte strings keep embedded macros braces comments and quotes opaque',()=>{
 const literal=String.raw`br##" } /* include_str!("foreign") */ "# "##`;
 const source='fn main(){ let bytes='+literal+'; } // actual';
 const {tokens,comments}=tokenize(source,'Rust');
 assert.equal(tokens.filter(token=>token.kind==='byteString').length,1);
 assert.equal(tokens.find(token=>token.kind==='byteString').raw,literal);
 assert.deepEqual(tokens.filter(token=>token.value==='{'||token.value==='}').map(token=>token.value),['{','}']);
 assert.deepEqual(comments.map(comment=>comment.text),['// actual']);
 assert.throws(()=>parseRust('fn bytes(){ b"value" }'),error=>error.kind==='unsupported'&&error.reason==='source indirection: source ownership requires an explicit dereference or matching immutable reference or Box type');
});
