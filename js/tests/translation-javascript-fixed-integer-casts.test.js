// Deferred source-plan controls; independent originals live in call-source data.
import test from 'node:test';
import assert from 'node:assert/strict';
import {planFixedIntegerCastExpression} from '../src/translation/fixed-integer-cast-rules.js';
import {planJavaScriptFixedIntegerCastExpression,planJavaScriptFixedIntegerArguments} from '../src/translation/javascript-fixed-integer-cast-plan.js';
import {assertVersions} from '../../experiments/javascript-properties/controls.mjs';
assertVersions();
for(const bits of [8,16,32,64,128])for(const signed of [false,true])test('retain complete preceding fixed-width plan '+bits+' '+signed,()=>{
 assert.deepEqual(planJavaScriptFixedIntegerCastExpression(bits,signed),planFixedIntegerCastExpression(bits,signed));
});
for(const bits of [-1,-Infinity,Infinity,4097,9007199254740992])test('refuse unsupported constant construction width '+bits,()=>assert.deepEqual(planJavaScriptFixedIntegerCastExpression(bits,true),[]));
test('constant ToIndex preserves truncation and NaN behavior',()=>{
 for(const bits of [0,-0,-0.5,NaN])assert.deepEqual(planJavaScriptFixedIntegerCastExpression(bits,true),planJavaScriptFixedIntegerCastExpression(0,false));
 assert.deepEqual(planJavaScriptFixedIntegerCastExpression(32.9,true),planFixedIntegerCastExpression(32,true));
});
test('ignored cast arguments retain all source coordinates',()=>{
 assert.deepEqual(planJavaScriptFixedIntegerArguments(2),[]);
 assert.deepEqual(planJavaScriptFixedIntegerArguments(5),[1,2,3,4]);
 for(const count of [0,1,-1,3.5,4097,NaN,Infinity])assert.deepEqual(planJavaScriptFixedIntegerArguments(count),[]);
});
