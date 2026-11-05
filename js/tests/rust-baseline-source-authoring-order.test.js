// Deferred orchestration controls use an injected process recorder only.
import test from 'node:test';
import assert from 'node:assert/strict';
import {fileURLToPath} from 'node:url';
import {draftRustBaselineCompleteSource,rustBaselineSourceAuthoringSteps} from '../scripts/draft-rust-baseline-complete-source.mjs';
test('schema and classifier construction precede source libraries in separate Node processes',()=>{
 const calls=[];
 draftRustBaselineCompleteSource({run:(...args)=>{calls.push(args);return {status:0,signal:null};}});
 assert.deepEqual(calls.map(([,args])=>args),rustBaselineSourceAuthoringSteps.map(step=>['--expose-gc',...step]));
 assert.equal(calls.length,10);
 assert.deepEqual(calls[2][1],['--expose-gc','js/scripts/draft-rust-baseline-source-policies.mjs','--draft','--defer-publication']);
 assert.deepEqual(calls.slice(3).map(([,args])=>args[1]),['js/scripts/draft-javascript-error-source-kernels.mjs','js/scripts/draft-javascript-module-initialization.mjs','js/scripts/draft-javascript-call-machine.mjs','js/scripts/draft-javascript-call-source.mjs','js/scripts/draft-javascript-error-class-source.mjs','js/scripts/draft-closed-source-error-composition.mjs','js/scripts/draft-rust-baseline-source-publication.mjs']);
 for(const [node,args,options]of calls){assert.equal(node,process.execPath);assert.equal(options.stdio,'inherit');assert.equal(options.cwd,fileURLToPath(new URL('../../',import.meta.url)));assert.ok(args.includes('--draft'));assert.ok(!args.some(arg=>/^(?:--check|--test|cargo|rustc)$/u.test(arg)));}
});
test('a failed or interrupted author preserves later products by stopping the batch',()=>{
 for(const failed of [{status:1,signal:null},{status:null,signal:'SIGTERM'},{error:new Error('spawn failure')}]){
  const calls=[];assert.throws(()=>draftRustBaselineCompleteSource({run:(...args)=>{calls.push(args);return calls.length===2?failed:{status:0,signal:null};}}));
  assert.equal(calls.length,2);
 }
});
