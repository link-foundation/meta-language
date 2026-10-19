import assert from 'node:assert/strict';
import test from 'node:test';
import {Executor} from '../src/grammar-runtime/executor.js';
import {createGrammarParser} from '../src/grammar-runtime.js';
import {parseGrammarLinks} from '../src/grammar-links.js';
import {nativeGrammarText} from '../src/native-grammar-parser.js';
import {createCompleteRustSourceArchives} from '../../experiments/original-link-complete-table-projection/archive-transport.mjs';

test('complete source diagnostics report actual budget failures under original limits',()=>{
  const parser=createGrammarParser(parseGrammarLinks(nativeGrammarText('native-rust'))),source='fn f(){}';
  const original=Executor.prototype.run,retain=Executor.prototype.retain;
  for(const[options,reason]of[[{memoryLimit:0},'memoryBudget'],[{stepLimit:0},'stepLimit'],[{maxDepth:0},'nestingDepth']]){
    for(const refuseObserver of[false,true]){
      const events=[],failure=new Error('actual rejected parser outcome');let encoded=false;
      assert.throws(()=>createCompleteRustSourceArchives(source,{
        parseSource:text=>{const outcome=parser.parseTree(text,options);assert.equal(outcome.rejection.reason,reason);throw failure;},
        encodeBinary:()=>{encoded=true;throw new Error('a rejected source cannot be encoded');},
        onNativeRun:event=>{events.push(event);if(refuseObserver&&event.phase.startsWith('failed-'))throw new Error('observer cannot mask parser rejection');},
      }),error=>error===failure);
      assert.equal(encoded,false);assert.equal(Executor.prototype.run,original);assert.equal(Executor.prototype.retain,retain);
      const event=events.find(event=>event.phase==='failed-'+reason);assert.ok(event,reason);
      assert.equal(event.inputBytes,Buffer.byteLength(source));assert.equal(event.memoLimit,1000000);
      assert.equal(event.memoryLimit,options.memoryLimit??2000000);assert.equal(event.stepLimit,options.stepLimit??100000+1000*Buffer.byteLength(source));
      assert.equal(event.retiredRules,0);for(const value of Object.values(event))assert.ok(['string','number','boolean'].includes(typeof value));
    }
  }
});
