// Authored name assembly and source provenance controls; execution deferred.
import test from 'node:test';import assert from 'node:assert/strict';
import {constructJavaScriptPropertyFunctionName,constructJavaScriptSymbolFunctionName} from '../src/translation/javascript-function-names.js';
import {compileJavaScriptMutableCallSourceWithCaptures} from '../src/translation/javascript-call-source.js';
const text=value=>({$:'text',units:Array.from(value,letter=>letter.charCodeAt(0))}),absent={$:'undefined'};
test('Symbol descriptions distinguish absence from empty text and retain accessor prefixes',()=>{
 assert.deepEqual(constructJavaScriptSymbolFunctionName(absent,'').value,text(''));assert.deepEqual(constructJavaScriptSymbolFunctionName(text(''),'').value,text('[]'));assert.deepEqual(constructJavaScriptSymbolFunctionName(text('Symbol.iterator'),'').value,text('[Symbol.iterator]'));assert.deepEqual(constructJavaScriptSymbolFunctionName(text('key'),'get').value,text('get [key]'));assert.deepEqual(constructJavaScriptSymbolFunctionName(absent,'set').value,text('set '));
});
test('completed property keys retain UTF16 name units and require actual Symbol metadata',()=>{
 assert.deepEqual(constructJavaScriptPropertyFunctionName({$:'text',units:[0xd800,120]},'get').value,{$:'text',units:[103,101,116,32,0xd800,120]});const key={$:'symbol',identity:7n};assert.deepEqual(constructJavaScriptPropertyFunctionName(key,''),{$:'functionNameSymbolDescription',key,prefix:''});assert.equal(constructJavaScriptPropertyFunctionName({$:'number',value:7},'').$,'functionNameInvalid');assert.equal(constructJavaScriptSymbolFunctionName({$:'null'},'').$,'functionNameInvalid');
});
test('computed anonymous property naming holds its converted key and allocated value before publication',()=>{
 const constant=value=>({$:'sourceConstant',value}),compiled=compileJavaScriptMutableCallSourceWithCaptures({$:'sourceObject',prototype:constant({$:'null'}),realm:1n,properties:[{$:'sourceObjectProperty',key:{$:'sourceBinding',identity:1},value:{$:'sourceBinding',identity:2},inferName:true}]},[1,2],[]);assert.equal(compiled.error,'');const code=compiled.instructions,conversion=code.findIndex(row=>row.$==='propertyKey'),naming=code.findIndex(row=>row.$==='functionName'),publication=code.findIndex(row=>row.$==='ordinaryDataProperty');assert.ok(conversion<naming&&naming<publication);assert.equal(code[naming].key,code[conversion].destination);assert.equal(code[naming].object,code[publication].value);assert.equal(code[naming].prefix,'');
});
