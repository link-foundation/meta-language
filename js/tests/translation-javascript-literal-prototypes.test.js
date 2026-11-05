// Authored controls only. Execution is deferred until the complete code draft.
import test from 'node:test';import assert from 'node:assert/strict';
import {compileJavaScriptMutableCallSourceWithCaptures} from '../src/translation/javascript-call-source.js';
import {readJavaScriptCallCodeRegisters} from '../src/translation/javascript-call-code.js';
const constant=value=>({$:'sourceConstant',value});
test('literal prototype installation stays ordered among data and spread effects',()=>{
 const source={$:'sourceObject',prototype:constant({$:'null'}),realm:1n,properties:[{$:'sourceObjectProperty',key:constant({$:'text',units:[120]}),value:constant({$:'number',value:1})},{$:'sourceLiteralPrototype',value:{$:'sourceBinding',identity:1}},{$:'sourceObjectSpread',input:{$:'sourceBinding',identity:2}}]},compiled=compileJavaScriptMutableCallSourceWithCaptures(source,[1,2],[]);
 assert.equal(compiled.error,'');const allocate=compiled.instructions.findIndex(row=>row.$==='ordinaryAllocate'),data=compiled.instructions.findIndex(row=>row.$==='ordinaryDataProperty'),prototype=compiled.instructions.findIndex(row=>row.$==='literalPrototype'),spread=compiled.instructions.findIndex(row=>row.$==='objectSpread');assert.ok(allocate<data&&data<prototype&&prototype<spread);const instruction=compiled.instructions[prototype];assert.deepEqual(readJavaScriptCallCodeRegisters(instruction),[instruction.destination,instruction.object,instruction.value]);assert.equal(instruction.object,compiled.instructions[allocate].destination);
});
