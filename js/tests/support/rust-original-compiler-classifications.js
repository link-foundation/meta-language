// Fixture orchestration only. These expectations/coverage declarations are not
// compiler results and never execute or compile an original program here.
import assert from 'node:assert/strict';
export function readRustOriginalCompilerExpectation(row,versions){
 if(!Object.hasOwn(row,'originalCompilerExpectation'))return null;
 const value=row.originalCompilerExpectation;
 assert.ok(value&&typeof value==='object'&&!Array.isArray(value),row.name+': original compiler expectation object');
 assert.deepEqual(Object.keys(value).sort(),['category','edition','model','outcome','versions'],row.name+': complete declared compiler expectation');
 assert.equal(value.model,'Rust original compiler expectation 1');assert.equal(value.edition,'2024');
 assert.deepEqual(value.versions,versions,row.name+': every selected compiler endpoint is mandatory');
 assert.ok(typeof row.sourceVersionRange==='string'&&row.sourceVersionRange.length>0,row.name+': explicit original source version range');
 assert.ok(value.outcome==='accept'||value.outcome==='reject',row.name+': explicit compiler accept/reject expectation');
 assert.equal(value.category,value.outcome==='accept'?'valid Rust source':'invalid Rust ownership',row.name+': source validity and interpreter admission are independent');
 return value;
}
export function describeRustOriginalCompilerClassificationCoverage(rows,versions){
 const classified=[],unclassified=[];
 for(const row of rows){
  const expected=readRustOriginalCompilerExpectation(row,versions);
  if(expected)classified.push({name:row.name,sourceVersionRange:row.sourceVersionRange,versions:[...expected.versions],outcome:expected.outcome,category:expected.category});
  else unclassified.push({name:row.name,sourceVersionRange:row.sourceVersionRange??null,versions:[...versions],obligation:'Classify original Rust compiler acceptance or source rejection at every selected endpoint; frontend refusal alone supplies no original compiler result.'});
 }
 return {model:'declared original Rust refusal compiler coverage 1',selectedVersions:[...versions],classified,unclassified};
}
