import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';
import {readBinaryRecordCompilerStyles} from '../scripts/binary-record-compiler-styles.mjs';
import {tokenize} from '../src/translation/lexer.js';

const review=readProjectData(new URL('../../parity/self-translation/binary-record-ci-style-review-116854044616.lino',import.meta.url));
const sources=()=>review.functions.map(row=>row.source),digest=text=>createHash('sha256').update(text).digest('hex');

test('binary compiler review retains all authentic diagnostic categories and exact function contexts',()=>{
 assert.equal(digest(readFileSync(new URL('../../parity/self-translation/binary-record-ci-style-review-116854044616.lino',import.meta.url))),'ba132477a48f94463c81edbb6bceafa2e5132be8f77139038c28781b2306d353');
 assert.equal(review.capture.logBytes,218557);assert.equal(review.capture.logSha256,'0901d3421dbc82dc7f19da99d26d4b638974b398a1f46b8e0bacba8f6761f9a3');
 assert.equal(review.sourceRevision,'61d3a766760306f3c0c2a93ef2d79450b465f242');assert.equal(review.diagnostics.length,45);
 assert.equal(review.diagnostics.filter(row=>row.message==='redundant clone').length,38);
 assert.equal(review.diagnostics.filter(row=>row.message==="binding's name is too similar to existing binding").length,5);
 assert.equal(review.diagnostics.filter(row=>row.message.startsWith('pub(crate)')).length,2);
 for(const row of review.functions)assert.equal(digest(row.source),row.sha256);
 assert.equal(readBinaryRecordCompilerStyles(sources()).actions.length,4);
});

test('binary style review preserves every token except exact clone removals and bound name spelling',()=>{
 const styles=readBinaryRecordCompilerStyles(sources());
 for(let index=0;index<review.functions.length;index++){
  const row=review.functions[index],action=styles.actions[index],renames=new Map(row.renames.map(value=>[value.from,value.to]));
  const tokens=tokenize(row.source,'Rust').tokens;
  const expected=tokens.filter(token=>!row.edits.some(edit=>token.start>=edit.start&&token.end<=edit.end)).map(token=>renames.get(token.value)??token.value);
  assert.deepEqual(tokenize(action.to,'Rust').tokens.map(token=>token.value),expected,row.name);
  assert.equal(action.from,row.source);assert.equal(action.op,'replace');assert.equal(action.field,'source');
 }
});

test('binary compiler review rejects stale ambiguous overlap and missing correction contexts',()=>{
 for(const change of [
  value=>{value.functions[0].sha256='0'.repeat(64);},
  value=>{value.functions[0].edits[0].end+=1;},
  value=>{value.functions[0].edits.push({...value.functions[0].edits[0]});},
  value=>{value.functions[0].edits.pop();},
  value=>{value.functions[0].edits[0].diagnostic=value.functions[0].renames[0].diagnostic;},
  value=>{value.functions[0].renames[0].to='input';},
  value=>{value.functions[0].renames[0].from=value.functions[0].name;},
 ]){const value=structuredClone(review);change(value);assert.throws(()=>readBinaryRecordCompilerStyles(sources(),value));}
 const stale=sources();stale[0]=stale[0].replace('pub fn','pub const fn');assert.throws(()=>readBinaryRecordCompilerStyles(stale),/source context/u);
 assert.throws(()=>readBinaryRecordCompilerStyles([...sources(),sources()[0]]),/exactly once/u);
});

test('binary compiler review preserves explicit provenance and every diagnostic category',()=>{
 const value=structuredClone(review);value.reviewKind='MachineApplicable';assert.throws(()=>readBinaryRecordCompilerStyles(sources(),value));
 const missing=structuredClone(review);missing.diagnostics.pop();assert.throws(()=>readBinaryRecordCompilerStyles(sources(),missing));
 assert.ok(review.diagnostics.some(row=>row.path==='rust/src/self_translation.rs'));
 assert.ok(review.diagnostics.some(row=>row.path==='rust/src/self_translation/representation.rs'));
});
