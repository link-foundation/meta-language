// Deferred property operator source controls, no behavior on import.
import test from 'node:test';import assert from 'node:assert/strict';
import {compileJavaScriptMutableCallSourceWithCaptures} from '../src/translation/javascript-call-source.js';
import {lowerJavaScriptSourcePresence,lowerJavaScriptSourceDelete} from '../src/translation/javascript-error-class-source.js';
const absent={$:'undefined'},constant=value=>({$:'sourceConstant',value});
function context(){return {$:'errorSourceContext',registry:{$:'errorRegistry',owners:[],operations:[],intrinsics:[],realms:[1n],exoticKey:absent},realm:{$:'errorRealmOperations',realm:1n,get:absent,assign:absent},capabilities:[],next:10,visits:0,error:'',mutableBindings:true};}
test('source deletion returns the actual Boolean after requiring strict success and reads no selected value',()=>{
 const compiled=compileJavaScriptMutableCallSourceWithCaptures({$:'sourceDelete',object:{$:'sourceBinding',identity:1},key:constant({$:'text',units:[120]})},[1],[]);assert.equal(compiled.error,'');const code=compiled.instructions,remove=code.findIndex(row=>row.$==='ordinaryDelete'),check=code.findIndex(row=>row.$==='primitive'&&row.operation==='require delete success');assert.ok(remove>=0&&check>remove);assert.equal(code.some(row=>row.$==='ordinaryGet'),false);
});
test('in checks its held Object before converting the already evaluated raw key',()=>{
 const lowered=lowerJavaScriptSourcePresence({$:'errorIn',key:{$:'errorBinding',identity:1},object:{$:'errorBinding',identity:2}},context(),0);assert.equal(lowered.context.error,'');const source=lowered.source;assert.equal(source.value.identity,1);assert.equal(source.body.value.identity,2);assert.equal(source.body.body.value.operation,'require object for in');assert.equal(source.body.body.body.$,'sourceHas');assert.equal(source.body.body.body.key.$,'sourcePropertyKey');
});
test('optional deletion branches before key evaluation and Get of the selected property',()=>{
 const lowered=lowerJavaScriptSourceDelete({$:'errorDelete',target:{$:'errorOptionalGet',object:{$:'errorBinding',identity:1},key:{$:'errorConstant',value:{$:'text',units:[120]}}}},context(),0);assert.equal(lowered.context.error,'');assert.equal(lowered.source.body.$,'sourceIf');assert.deepEqual(lowered.source.body.yes,constant({$:'boolean',value:true}));assert.equal(lowered.source.body.no.no.$,'sourceLocalLet');assert.equal(lowered.source.body.no.no.body.body.$,'sourceDelete');
});
