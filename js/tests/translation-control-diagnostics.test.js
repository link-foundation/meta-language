// Authored future controls; no subprocess or production source was run while
// drafting these tests.
import test from 'node:test';
import assert from 'node:assert/strict';
import {spawnSync} from 'node:child_process';
import {createControlDiagnostics,requireControlProduct,finishControlDiagnostics,readControlDiagnosticFallback} from './support/control-diagnostics.js';
test('failed dependencies remain failing assertions and later controls still run',()=>{
 const diagnostic=createControlDiagnostics();
 const absent=diagnostic.attempt('first measured product',()=>{throw new Error('intentional first failure');});
 diagnostic.attempt('dependent assertion',()=>requireControlProduct(absent,'first measured product'));
 const later=diagnostic.attempt('independent later product',()=>7);
 assert.equal(later,7);assert.deepEqual(diagnostic.assertions,['first measured product','dependent assertion','independent later product']);
 assert.equal(diagnostic.failures.length,2);
 const record={name:'deliberate dependency control',diagnostics:{assertions:diagnostic.assertions,failures:diagnostic.failures}};
 assert.throws(()=>finishControlDiagnostics(record),error=>error instanceof AggregateError&&error.controlRecord===record&&error.errors.length===2);
});
test('an absent diagnostic contract preserves the literal input and fails explicitly',()=>{
 const definition={name:'missing diagnostics',source:'console.log(1);',stdout:'1\n'};
 const record=readControlDiagnosticFallback(definition,()=>({}));
 assert.equal(record.source,definition.source);assert.equal(record.stdout,definition.stdout);assert.equal(record.diagnostics.failures.length,1);assert.match(record.diagnostics.failures[0].message,/declared assertion diagnostics/u);
});
test('producer completes valid JSON and retains every later fixture and target before failing',()=>{
 const result=spawnSync(process.execPath,['--max-old-space-size=352','--max-semi-space-size=1',new URL('../scripts/produce-control-diagnostic-failures.mjs',import.meta.url).pathname],{encoding:'utf8',timeout:30000,maxBuffer:4*1024*1024});
 assert.equal(result.error,undefined);assert.equal(result.status,1);const envelope=JSON.parse(result.stdout);
 assert.equal(envelope.model,'deliberately failing diagnostic transport 1');assert.deepEqual(envelope.targets,['JavaScript','Rust','Lean','Rocq']);assert.deepEqual(envelope.transports,['original','text','binary']);
 assert.equal(envelope.injectedControls.length,3);
 for(const[row,index]of envelope.injectedControls.map((row,index)=>[row,index])){assert.equal(row.input,index+1);assert.deepEqual(row.diagnostics.assertions,['JavaScript','Rust','Lean','Rocq']);}
 assert.equal(envelope.injectedControls[0].diagnostics.failures.length,1);assert.equal(envelope.injectedControls[0].diagnostics.failures[0].assertion,'Lean');
 assert.equal(envelope.injectedControls[0].measurements.Rocq,1);
 assert.equal(envelope.injectedControls[1].diagnostics.failures.length,0);assert.equal(envelope.injectedControls[1].measurements.Rocq,2);
 assert.equal(envelope.injectedControls[2].diagnostics.failures.length,1);assert.equal(envelope.injectedControls[2].diagnostics.failures[0].assertion,'Rust');assert.equal(envelope.injectedControls[2].measurements.Rocq,3);
 assert.match(result.stderr,/complete cells are in stdout JSON/u);
});
