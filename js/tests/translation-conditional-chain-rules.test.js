// Deferred structural controls. Draft authors never import this module.
import assert from 'node:assert/strict';
import test from 'node:test';
import {tokenize} from '../src/translation/lexer.js';
import {findRustConditionalChainPattern,planConditionalChainLayers,planConditionalMatchArms} from '../src/translation/conditional-chain-rules.js';
test('conditional layers preserve actual operand order and stage identity',()=>{
 assert.deepEqual(planConditionalChainLayers([0,1,0,1]),[1,3,0,2,1,1,0,0]);
 assert.deepEqual(planConditionalChainLayers([]),[]);assert.deepEqual(planConditionalChainLayers([2]),[]);
 assert.deepEqual(planConditionalChainLayers(Array(257).fill(0)),[]);
});
test('guarded alternatives retain their order without discharging exhaustiveness',()=>{
 assert.deepEqual(planConditionalMatchArms([1,0,1,0]),[2,1,3,3,2,1,0]);
 assert.deepEqual(planConditionalMatchArms([1,1]),[0,1,0]);
 assert.deepEqual(planConditionalMatchArms([0,2]),[]);
});
test('actual chain detection skips opaque text and balanced operand groups',()=>{
 const find=source=>{const tokens=tokenize(source,'Rust').tokens;const at=findRustConditionalChainPattern(tokens.map(token=>token.kind),tokens.map(token=>token.value),0);return at<0?null:tokens[at].value;};
 assert.equal(find('true && let Value::Some(x)=value {x}else{0}'),'let');
 assert.equal(find('test("let", [1,2]) {1}else{0}'),null);
 assert.equal(find('(true || false) && let Value::Some(x)=value {x}else{0}'),'let');
});
