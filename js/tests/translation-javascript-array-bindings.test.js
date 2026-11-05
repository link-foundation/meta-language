// Deferred Array binding scope and region controls. No execution at import.
import test from 'node:test';import assert from 'node:assert/strict';
import {compileJavaScriptMutableCallSourceWithCaptures} from '../src/translation/javascript-call-source.js';
const absent={$:'undefined'},constant=value=>({$:'sourceConstant',value}),number=value=>({$:'number',value});
const declaration={$:'sourceBindingDeclaration',identity:1,mutable:false};
const initialized=value=>({$:'sourceInitializeBinding',identity:1,value:constant(number(value)),body:constant(absent)});
const scope=initializer=>({$:'sourceBindingScope',declarations:[declaration],initializer,body:{$:'sourceBinding',identity:1}});
test('binding scope outputs initialize once and retain constant-body assignment refusal',()=>{
 const compiled=compileJavaScriptMutableCallSourceWithCaptures(scope(initialized(7)),[],[]);assert.equal(compiled.error,'');assert.equal(compiled.instructions.some(row=>row.$==='copy'),true);assert.notEqual(compileJavaScriptMutableCallSourceWithCaptures(scope(constant(absent)),[],[]).error,'');assert.notEqual(compileJavaScriptMutableCallSourceWithCaptures(scope({$:'sourceInitializeBinding',identity:1,value:{$:'sourceBinding',identity:1},body:constant(absent)}),[],[]).error,'');const assigned={...scope(initialized(7)),body:{$:'sourceAssignBinding',identity:1,value:constant(number(9))}};assert.notEqual(compileJavaScriptMutableCallSourceWithCaptures(assigned,[],[]).error,'');
});
test('binding scope definite initialization requires both conditional paths',()=>{
 const conditional={$:'sourceIf',condition:{$:'sourceBinding',identity:2},yes:initialized(7),no:constant(absent)};assert.notEqual(compileJavaScriptMutableCallSourceWithCaptures(scope(conditional),[2],[]).error,'');const both={...conditional,no:initialized(9)};assert.equal(compileJavaScriptMutableCallSourceWithCaptures(scope(both),[2],[]).error,'');
});
test('Array binding output code follows iterator close rather than extending its protected lifetime',()=>{
 const initializer={$:'sourceIteratorBinding',slot:3,input:{$:'sourceBinding',identity:2},iteratorKey:constant({$:'symbol',identity:99n}),realm:1n,body:{$:'sourceInitializeBinding',identity:1,value:{$:'sourceIteratorStep',slot:3,realm:1n,readValue:true},body:constant(absent)}},compiled=compileJavaScriptMutableCallSourceWithCaptures(scope(initializer),[2],[]);assert.equal(compiled.error,'');const operations=compiled.instructions.filter(row=>row.$==='iteratorProtocol');assert.deepEqual(operations.map(row=>row.operation),['acquire','step','close']);const close=compiled.instructions.findIndex(row=>row.$==='iteratorProtocol'&&row.operation==='close'),returned=compiled.instructions.findIndex(row=>row.$==='return');assert.ok(close<returned);assert.equal(compiled.instructions.some(row=>row.$==='finishFinally'),true);
});
test('Array rest consumes its actual cached region without another iterator acquisition',()=>{
 const source={$:'sourceIteratorBinding',slot:3,input:{$:'sourceBinding',identity:2},iteratorKey:constant({$:'symbol',identity:99n}),realm:1n,body:{$:'sourceIteratorRemainder',slot:3,prototype:constant({$:'object',identity:1n,callable:false}),realm:1n}},compiled=compileJavaScriptMutableCallSourceWithCaptures(source,[2],[]);assert.equal(compiled.error,'');const operations=compiled.instructions.filter(row=>row.$==='iteratorProtocol');assert.deepEqual(operations.map(row=>row.operation),['acquire','step','close']);assert.equal(compiled.instructions.filter(row=>row.$==='arrayAllocate').length,1);assert.equal(compiled.instructions.filter(row=>row.$==='ordinaryDataProperty').length,1);
});
