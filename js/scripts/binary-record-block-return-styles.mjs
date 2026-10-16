// Append reviewed block-return corrections after the earlier immutable review.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {tokenize} from '../src/translation/lexer.js';
import {readProjectData} from './project-data.mjs';

const reviewFile=new URL('../../parity/self-translation/binary-record-ci-block-return-review-116894548503.lino',import.meta.url);
const digest=source=>createHash('sha256').update(source).digest('hex');

export function readBinaryRecordBlockReturnStyles(styled,review=readProjectData(reviewFile)){
  assert.equal(review.schemaVersion,1);
  assert.equal(review.reviewKind,'Reviewed rendered CI diagnostics. No machine applicability metadata was reported or invented.');
  assert.equal(review.capture.run,38874601505);assert.equal(review.capture.job,116894548503);
  assert.equal(review.capture.logSha256,'2793cdbc870582f3b5c17180175f92e1158e05e75415100ba33af7634074cfac');
  assert.equal(review.diagnostics.length,3);assert.equal(review.sourceTestFindings.length,5);
  assert.equal(review.functions.length,1);const item=review.functions[0];
  assert.equal(item.name,'transcode_link_network_binary');
  assert.equal(item.path,'rust/src/link_network_binary_record_rules/part_1.rs');
  assert.equal(digest(item.source),item.sha256);assert.ok(item.source.startsWith('pub fn '+item.name+'('));
  assert.ok(review.sources[item.path]?.sha256);
  assert.equal(styled.filter(source=>source.includes(item.source)).length,1,'reviewed source occurs exactly once after earlier corrections');
  const seen=new Set(),edits=item.edits.slice().sort((left,right)=>left.start-right.start);let previous=0;
  assert.equal(edits.length,3);
  for(const edit of edits){
    assert.ok(Number.isSafeInteger(edit.start)&&Number.isSafeInteger(edit.end)&&edit.start>=previous&&edit.end<=item.source.length);
    assert.equal(item.source.slice(edit.start,edit.end),edit.from,'exact reviewed block context');
    const diagnostic=review.diagnostics.find(row=>row.index===edit.diagnostic);assert.ok(diagnostic&&!seen.has(edit.diagnostic));
    assert.equal(diagnostic.path,item.path);assert.equal(diagnostic.message,'returning the result of a `let` binding from a block');seen.add(edit.diagnostic);
    const tokens=tokenize(edit.from,'Rust').tokens.filter(token=>token.kind!=='eof');
    const expression=tokenize(edit.to,'Rust').tokens.filter(token=>token.kind!=='eof');
    assert.equal(tokens[0]?.value,'let');assert.equal(tokens[1]?.kind,'identifier');assert.equal(tokens[1].value,edit.binding);
    assert.equal(tokens[2]?.value,'=');assert.equal(tokens.at(-2)?.value,';');assert.equal(tokens.at(-1)?.value,edit.binding);
    assert.deepEqual(expression.map(token=>[token.kind,token.value]),tokens.slice(3,-2).map(token=>[token.kind,token.value]),'the complete returned expression is unchanged');
    assert.ok(!expression.some(token=>token.kind==='identifier'&&token.value===edit.binding),'the removed declaration has no self reference');
    previous=edit.end;
  }
  assert.deepEqual([...seen].sort(),review.diagnostics.map(row=>row.index).sort());
  let corrected=item.source;for(const edit of edits.reverse())corrected=corrected.slice(0,edit.start)+edit.to+corrected.slice(edit.end);
  return {id:'binary-record-block-return-compiler-review',level:'emitter',order:960,when:[['format','Rust'],['scope','formatted-item']],actions:[{op:'replace',field:'source',from:item.source,to:corrected}]};
}
