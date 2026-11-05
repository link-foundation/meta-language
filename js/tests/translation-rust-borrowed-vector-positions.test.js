// Deferred assertions only; importing this file is not execution evidence.
import assert from 'node:assert/strict';
import test from 'node:test';
import * as rules from '../src/translation/rust-borrowed-vector-rules.js';
import {checkRustSourceSignature} from '../src/translation/rust-source-signatures.js';
import {rustBorrowedVectorPositionDecisions as decisions} from './fixtures/rust-borrowed-vector-positions.mjs';
for(const[index,row]of decisions.entries())test('independent borrowed source position decision '+index,()=>{assert.equal(typeof rules[row.operation],'function');assert.deepEqual(rules[row.operation](...row.arguments),row.expected);});
const scalar={kind:'fixed',bits:8,signed:false};
const owner={kind:'primitive',path:['u8'],arguments:[]};
function signature(model){return {model,language:'Rust',parameters:[],result:{type:{kind:'array',element:scalar,reference:false},wrappers:[],lifetimes:[],owner:{kind:'library',path:['std','vec','Vec'],arguments:[owner]},arrayElementPosition:{type:scalar,wrappers:['reference'],lifetimes:['static'],owner}},lifetimeBindings:[]};}
test('version three validates the complete retained source element position',()=>{const value=signature('Rust source function signature 3');assert.deepEqual(checkRustSourceSignature(value,[],value.result.type,'Rust'),value);});
for(const model of ['Rust source function signature 1','Rust source function signature 2'])test(model+' retains its prior exact field boundary',()=>{const value=signature(model);assert.throws(()=>checkRustSourceSignature(value,[],value.result.type,'Rust'),error=>error.kind==='type');});
test('a mismatched retained source element owner refuses independently of erased core type',()=>{const value=signature('Rust source function signature 3');value.result.arrayElementPosition.owner={kind:'primitive',path:['u16'],arguments:[]};assert.throws(()=>checkRustSourceSignature(value,[],value.result.type,'Rust'),error=>error.kind==='type');});
