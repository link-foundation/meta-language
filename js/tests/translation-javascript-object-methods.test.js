// Deferred source controls; no represented method is executed on import.
import test from 'node:test';import assert from 'node:assert/strict';
import {projectJavaScriptCallTemplateFunction} from '../src/translation/javascript-call-templates.js';
import {compileJavaScriptMutableCallSourceWithCaptures} from '../src/translation/javascript-call-source.js';
test('strict runtime methods use the call receiver while omitted template mode preserves lexical arrows',()=>{
 const absent={$:'undefined'},receiver={$:'object',identity:9n,callable:false},template={$:'callTemplate',owner:1n,ordinal:0,realm:2n,parameters:0,registers:1,captureCount:0,instructions:[{$:'return',source:0}],prototype:{$:'object',identity:3n,callable:true},name:{$:'text',units:[120]},length:0};
 assert.equal(projectJavaScriptCallTemplateFunction(template,4n,[],receiver).receiverMode,'lexical');const method=projectJavaScriptCallTemplateFunction({...template,receiverMode:'strict'},4n,[],absent);assert.equal(method.receiverMode,'strict');assert.deepEqual(method.lexicalReceiver,absent);assert.deepEqual(method.instructions,template.instructions);
});
test('literal getter and setter declarations retain separate merge operations in source order',()=>{
 const constant=value=>({$:'sourceConstant',value}),key=constant({$:'text',units:[120]}),compiled=compileJavaScriptMutableCallSourceWithCaptures({$:'sourceObject',prototype:constant({$:'null'}),realm:1n,properties:[{$:'sourceObjectAccessor',key,value:{$:'sourceBinding',identity:1},kind:'get'},{$:'sourceObjectAccessor',key,value:{$:'sourceBinding',identity:2},kind:'set'}]},[1,2],[]);
 assert.equal(compiled.error,'');const accessors=compiled.instructions.filter(row=>row.$==='literalAccessor');assert.deepEqual(accessors.map(row=>row.kind),['get','set']);assert.equal(accessors.every(row=>row.object===accessors[0].object&&row.realm===1n),true);assert.equal(compiled.instructions.some(row=>row.$==='ordinaryDataProperty'),false);
});
