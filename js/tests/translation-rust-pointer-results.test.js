// Deferred source validity and independent declared pointer-result constraints.
import assert from 'node:assert/strict';
import test from 'node:test';
import {parseRust} from '../src/translation/rust.js';
import {checkProgram} from '../src/translation/check.js';
test('a discarded closure tail does not become a function result',()=>assert.throws(()=>checkProgram(parseRust('fn make()->fn(i64)->i64{|value|value+1;}fn main(){}'))));
test('a terminal pointer return never discards later source work',()=>assert.throws(()=>checkProgram(parseRust('fn make()->fn(i64)->i64{return |value|value+1;let lost=7i64;}fn main(){}'))));
test('a written incompatible result remains a source type error',()=>assert.throws(()=>checkProgram(parseRust('fn make()->fn(i64)->i64{|value:i32|value}fn main(){}')),error=>error.kind==='type'));
