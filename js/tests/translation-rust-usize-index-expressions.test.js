// Deferred shared definitions; never run during the full draft.
import assert from 'node:assert/strict';
import test from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';
import {acceptRustUsizeIndexExpressionNode,readRustUsizeIndexExpressionError} from '../src/translation/rust-usize-index-expression-rules.js';
const fixture=readProjectData(new URL('../../parity/fixtures/rust-usize-index-expressions.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.JavaScript);
for(const row of fixture.forms)test('actual contextual index node: '+JSON.stringify(row),()=>assert.equal(acceptRustUsizeIndexExpressionNode(row.kind,row.operator,row.atom,row.literal,row.count),row.expected));
for(const row of fixture.decisions)test(row.name,()=>assert.equal(readRustUsizeIndexExpressionError(row.kinds,row.operators,row.atoms,row.literals,row.parents),row.expected));
test('the actual expression depth remains bounded',()=>assert.equal(readRustUsizeIndexExpressionError(Array(130).fill('match'),Array(130).fill(''),Array(130).fill(false),Array(130).fill(false),Array.from({length:130},(_,index)=>index-1)),'the index expression exceeds its source depth limit'));
test('the actual source node directory remains bounded',()=>assert.equal(readRustUsizeIndexExpressionError(Array(4097).fill('num'),[],[],[],[]),'the index expression exceeds its source node limit'));
