// Deferred declaration controls only; this module contains no compiler command.
import assert from 'node:assert/strict';
import test from 'node:test';
import {rustStandardSumOwnedOriginals,rustStandardSumOwnedRefusals,rustStandardSumOwnedPreviousBoundaries} from './fixtures/rust-standard-sum-owned-originals.mjs';
import {readRustOriginalCompilerExpectation,describeRustOriginalCompilerClassificationCoverage} from './support/rust-original-compiler-classifications.js';
const versions=['1.90.0','1.99.0'];
const classifiedOriginals=()=>[...rustStandardSumOwnedOriginals,...rustStandardSumOwnedRefusals].filter(row=>Object.hasOwn(row,'originalCompilerExpectation'));
test('new deferred guard admission retains its exact previous source oracle and compiler contract',()=>{
 assert.equal(rustStandardSumOwnedPreviousBoundaries.length,1);
 const previous=rustStandardSumOwnedPreviousBoundaries[0];
 assert.equal(previous.previousCollection,'rustStandardSumOwnedRefusals');assert.equal(previous.previousIndex,5);
 assert.deepEqual(previous.previousDiagnosticKinds,['unsupported','type']);
 assert.equal(previous.previousTranslationExpectation,'refuse immediate owned pattern guard');
 const current=rustStandardSumOwnedOriginals.filter(row=>row.name===previous.row.name);
 assert.equal(current.length,1);assert.equal(current[0].source,previous.row.source);
 assert.deepEqual(current[0].originalCompilerExpectation,previous.row.originalCompilerExpectation);
 assert.equal(current[0].stdout,'');assert.equal(current[0].expectationHistory[0].previousIndex,5);
 assert.ok(!rustStandardSumOwnedRefusals.some(row=>row.source===previous.row.source));
});
test('owned refusal originals retain explicit compiler outcome independently of frontend admission',()=>{
 const rows=classifiedOriginals();assert.equal(rows.length,6);
 for(const name of ['a constructor consumes the original owned String binding','a consuming sum parameter cannot receive the same moved root twice','a consuming field pattern invalidates its original root','an explicit reference pattern cannot move String out of a shared sum','a shared root loan cannot survive moving its owned sum']){
  const matches=rows.filter(row=>row.name===name);assert.equal(matches.length,1);assert.equal(readRustOriginalCompilerExpectation(matches[0],versions).outcome,'reject');
 }
 const guard=rows.filter(row=>row.source==='fn bad(value:Option<String>)->usize{match value{Some(text) if text.len()>0usize=>text.len(),_=>0usize}}fn main(){}');
 assert.equal(guard.length,1);assert.equal(readRustOriginalCompilerExpectation(guard[0],versions).outcome,'accept');
});
test('unclassified historical source remains explicit coverage obligation without mutation',()=>{
 const row={name:'historical unclassified original',source:'fn main(){}',sourceVersionRange:'>=1.90.0 <1.100.0'},before=structuredClone(row);
 assert.equal(readRustOriginalCompilerExpectation(row,versions),null);
 const coverage=describeRustOriginalCompilerClassificationCoverage([row,...classifiedOriginals()],versions);
 assert.equal(coverage.classified.length,6);assert.equal(coverage.unclassified.length,1);
 assert.deepEqual(coverage.unclassified[0].versions,versions);assert.match(coverage.unclassified[0].obligation,/frontend refusal alone supplies no original compiler result/);
 assert.deepEqual(row,before);
});
test('declared compiler endpoints and validity categories cannot be silently omitted or relabeled',()=>{
 const row=structuredClone(rustStandardSumOwnedRefusals[0]);row.originalCompilerExpectation.versions=['1.90.0'];
 assert.throws(()=>readRustOriginalCompilerExpectation(row,versions));
 row.originalCompilerExpectation.versions=[...versions];row.originalCompilerExpectation.category='valid Rust source';
 assert.throws(()=>readRustOriginalCompilerExpectation(row,versions));
});
