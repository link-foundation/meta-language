// Deferred immutable PutValue source controls, no represented execution.
import test from 'node:test';import assert from 'node:assert/strict';
import {compileJavaScriptMutableCallSourceWithCaptures} from '../src/translation/javascript-call-source.js';
import {lowerJavaScriptSourceReference,constructJavaScriptSourceReferenceWrite} from '../src/translation/javascript-error-class-source.js';
test('immutable PutValue evaluates its supplied value before an actual realm TypeError request',()=>{
 const compiled=compileJavaScriptMutableCallSourceWithCaptures({$:'sourceImmutableBindingWrite',value:{$:'sourcePrimitive',operation:'increment numeric value',arguments:[{$:'sourceBinding',identity:1}]}},[1],[]);assert.equal(compiled.error,'');const code=compiled.instructions,numeric=code.findIndex(row=>row.$==='primitive'&&row.operation==='increment numeric value'),failure=code.findIndex(row=>row.$==='primitive'&&row.operation==='raiseJavaScriptTypeError');assert.ok(numeric>=0&&failure>numeric);assert.equal(code.some(row=>row.$==='copy'&&row.destination===1),false);
});
test('immutable lexical References retain their original value register and abrupt write policy',()=>{
 const absent={$:'undefined'},context={$:'errorSourceContext',registry:{$:'errorRegistry',owners:[],operations:[],intrinsics:[],realms:[1n],exoticKey:absent},realm:{$:'errorRealmOperations',realm:1n,get:absent,assign:absent},capabilities:[],next:10,visits:0,error:'',mutableBindings:true},reference=lowerJavaScriptSourceReference({$:'errorBinding',identity:1,immutable:true},context,0);assert.equal(reference.context.error,'');assert.equal(reference.kind,'immutable binding');assert.deepEqual(reference.read,{$:'sourceBinding',identity:1});assert.deepEqual(constructJavaScriptSourceReferenceWrite(reference,{$:'sourceBinding',identity:2}),{$:'sourceImmutableBindingWrite',value:{$:'sourceBinding',identity:2}});
});
