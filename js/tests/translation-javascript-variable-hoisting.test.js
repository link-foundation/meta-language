// Deferred var/source scope contracts. No source compiler executes on import.
import test from 'node:test';import assert from 'node:assert/strict';
import {collectJavaScriptSourceVariableDeclarations,constructJavaScriptSourceVariableInitializer,lowerJavaScriptSourceStatements,lowerJavaScriptVariableForOf,constructJavaScriptSourcePatternBinding} from '../src/translation/javascript-error-class-source.js';
import {compileJavaScriptMutableCallSourceWithCaptures} from '../src/translation/javascript-call-source.js';
const absent={$:'sourceConstant',value:{$:'undefined'}},constant=value=>({$:'errorConstant',value:{$:'number',value}});
function context(){return {$:'errorSourceContext',registry:{$:'errorModuleRegistry',owners:[],realms:[1n],exoticKey:{$:'symbol',identity:9n}},realm:{$:'errorRealmOperations',realm:1n},capabilities:[],next:100,visits:0,error:'',mutableBindings:true,lexicalBindings:[{$:'sourceLexicalBinding',identity:1,name:'parameter',immutable:false},{$:'sourceLexicalBinding',identity:2,name:'variable',immutable:false,hoisted:true},{$:'sourceLexicalBinding',identity:3,name:'copied',immutable:false,hoisted:true,parameter:1}]};}
test('function var collection crosses block and control scopes and deduplicates canonical declarations',()=>{
 const declaration={$:'sourceVariableStatement',identity:2,value:constant(7),hasInitializer:true},statements=[{$:'sourceIfStatement',condition:constant(0),yes:[declaration],no:[]},{$:'sourceBlockStatement',statements:[declaration]},{$:'sourceVariableStatement',identity:1,value:constant(0),hasInitializer:false}];assert.deepEqual(collectJavaScriptSourceVariableDeclarations(statements,context(),0),[2]);
});
test('hoisted cells begin initialized and non-simple parameter copies read the actual parameter cell',()=>{
 let row=constructJavaScriptSourceVariableInitializer(context(),2);assert.equal(row.initialized,true);assert.equal(row.value.value.$,'undefined');row=constructJavaScriptSourceVariableInitializer(context(),3);assert.equal(row.initialized,true);assert.equal(row.value.$,'sourceIf');assert.equal(row.value.yes.object.identity,1);
});
test('function entry prepares unreachable var declarations before its early return',()=>{
 const statements=[{$:'sourceReturnStatement',value:{$:'errorModuleCell',identity:2}},{$:'sourceVariableStatement',identity:2,value:constant(7),hasInitializer:true}],lowered=lowerJavaScriptSourceStatements(statements,0,context(),absent,0);assert.equal(lowered.context.error,'');assert.equal(lowered.source.$,'sourceLexicalScope');assert.deepEqual(lowered.source.identities,[2]);assert.equal(lowered.source.initializers[0].initialized,true);assert.equal(lowered.source.body.$,'sourceReturn');
});
test('var for-of initializes the existing cell without creating per-iteration lexical copies',()=>{
 const statement={$:'sourceForOfStatement',target:7,binding:2,mutable:true,variable:true,input:constant(3),iteratorKey:{$:'errorConstant',value:{$:'symbol',identity:9n}},body:[]},lowered=lowerJavaScriptVariableForOf(statement,context(),absent,0),loop=lowered.source.value;assert.equal(loop.$,'sourceForOf');assert.notEqual(loop.binding,2);assert.equal(loop.scopeBindings,undefined);assert.equal(loop.body.$,'sourceInitializeCell');assert.equal(loop.body.identity,2);assert.equal(loop.body.value.identity,loop.binding);
});
test('flat lexical scopes compile initialized var flags with no extra runtime mechanism',()=>{
 const source={$:'sourceLexicalScope',identities:[2],initializers:[{$:'sourceCellInitializer',identity:2,value:{$:'sourceConstant',value:{$:'number',value:7}},initialized:true}],realm:1n,body:{$:'sourceGet',object:{$:'sourceBinding',identity:2},key:{$:'sourceConstant',value:{$:'text',units:[118,97,108,117,101]}}}},compiled=compileJavaScriptMutableCallSourceWithCaptures(source,[],[]);assert.equal(compiled.error,'');assert.ok(compiled.instructions.some(row=>row.$==='constant'&&row.value.$==='boolean'&&row.value.value));assert.equal(compiled.instructions.filter(row=>row.$==='ordinaryAllocate').length,1);
});
test('var declaration identity stays hoisted while its catch initializer uses a distinct existing reference',()=>{
 const statement={$:'sourceVariableStatement',identity:2,value:constant(7),hasInitializer:true,target:{$:'sourceCatchBindingTarget',identity:1,cell:false}},lowered=lowerJavaScriptSourceStatements([statement],0,context(),absent,0);assert.deepEqual(lowered.source.identities,[2]);assert.equal(lowered.source.body.$,'sourceWriteExisting');assert.equal(lowered.source.body.identity,1);assert.equal(lowered.source.body.cell,false);assert.deepEqual(collectJavaScriptSourceVariableDeclarations([statement],context(),0),[2]);
});
test('pattern catch aliases select register or captured cell writes before generic initialization',()=>{
 let source=constructJavaScriptSourcePatternBinding(context(),2,absent,absent,true,true,{$:'sourceCatchBindingTarget',identity:1,cell:false});assert.equal(source.$,'sourceWriteExisting');assert.equal(source.identity,1);source=constructJavaScriptSourcePatternBinding(context(),2,absent,absent,true,true,{$:'sourceCatchBindingTarget',identity:1,cell:true});assert.equal(source.cell,true);
});
test('existing catch writes evaluate and hold their RHS before assigning either storage form',()=>{
 for(const cell of [false,true]){const source={$:'sourceWriteExisting',identity:1,cell,value:{$:'sourceConstant',value:{$:'number',value:7}},body:absent},compiled=compileJavaScriptMutableCallSourceWithCaptures(source,[1],[]);assert.equal(compiled.error,'');assert.equal(compiled.instructions.filter(row=>row.$==='ordinarySet').length,cell?1:0);assert.equal(compiled.instructions.filter(row=>row.$==='requireWrite').length,cell?1:0);}
});
