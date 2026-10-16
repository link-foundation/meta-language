// Replay reviewed rendered diagnostics on their exact generated functions.
// This is generation data, not a compiler run or semantic success receipt.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {tokenize} from '../src/translation/lexer.js';
import {readProjectData} from './project-data.mjs';

const reviewFile=new URL('../../parity/self-translation/binary-record-ci-style-review-116854044616.lino',import.meta.url);
const digest=text=>createHash('sha256').update(text).digest('hex');

export function readBinaryRecordCompilerStyles(styled,feedback=readProjectData(reviewFile)){
 assert.equal(feedback.schemaVersion,1);
 assert.equal(feedback.reviewKind,'Reviewed rendered CI diagnostics. No machine applicability metadata was reported or invented.');
 assert.equal(feedback.capture.logSha256,'0901d3421dbc82dc7f19da99d26d4b638974b398a1f46b8e0bacba8f6761f9a3');
 assert.equal(feedback.capture.job,116854044616);assert.equal(feedback.capture.run,38862173174);
 assert.equal(feedback.diagnostics.length,45);
 const diagnostics=new Map(feedback.diagnostics.map(row=>[row.index,row]));assert.equal(diagnostics.size,45);
 const required=feedback.diagnostics.filter(row=>row.path.includes('/link_network_binary_record_rules/'));
 assert.equal(required.length,43);const seen=new Set(),names=new Set(),actions=[];
 for(const item of feedback.functions){
  assert.ok(!names.has(item.name),'a reviewed function occurs once');names.add(item.name);
  assert.equal(digest(item.source),item.sha256,'reviewed function source hash');
  assert.ok(item.source.startsWith('pub fn '+item.name+'('),'reviewed function owns its signature');
  assert.match(item.path,/^rust\/src\/link_network_binary_record_rules\/part_\d+\.rs$/u);
  assert.ok(feedback.sources[item.path]?.sha256,'review retains its original module identity');
  assert.equal(styled.filter(source=>source.includes(item.source)).length,1,'reviewed source context is present exactly once');
  const cite=(index,message)=>{
   const diagnostic=diagnostics.get(index);assert.ok(diagnostic&&!seen.has(index),'each diagnostic has one correction');
   assert.equal(diagnostic.path,item.path);assert.equal(diagnostic.message,message);seen.add(index);
  };
  const edits=item.edits.slice().sort((left,right)=>left.start-right.start);let previous=0;
  for(const edit of edits){
   assert.ok(Number.isSafeInteger(edit.start)&&Number.isSafeInteger(edit.end)&&edit.start>=previous&&edit.end<=item.source.length,'reviewed edits have valid disjoint bounds');
   assert.equal(edit.from,'.clone()');assert.equal(edit.to,'');assert.equal(edit.end-edit.start,8);
   assert.equal(item.source.slice(edit.start,edit.end),edit.from,'only the reported redundant clone is removed');
   cite(edit.diagnostic,'redundant clone');previous=edit.end;
  }
  let corrected=item.source;
  for(const edit of edits.reverse())corrected=corrected.slice(0,edit.start)+edit.to+corrected.slice(edit.end);
  const tokens=tokenize(corrected,'Rust').tokens,renames=new Map();
  for(const rename of item.renames){
   assert.match(rename.from,/^[a-z][a-z_0-9]*$/u);assert.match(rename.to,/^[a-z]+(?:_[a-z]+)*$/u);
   assert.ok(!renames.has(rename.from),'a reviewed binding is renamed once');
   assert.ok(!tokens.some(token=>token.kind==='identifier'&&token.value===rename.to),'reviewed names cannot capture existing identifiers');
   assert.ok(tokens.some(token=>token.kind==='identifier'&&token.value===rename.from),'reviewed names must exist');
   cite(rename.diagnostic,"binding's name is too similar to existing binding");renames.set(rename.from,rename.to);
  }
  assert.equal(new Set(renames.values()).size,renames.size,'reviewed names remain bijective');
  const replacements=[];
  for(let index=0;index<tokens.length;index++)if(tokens[index].kind==='identifier'&&renames.has(tokens[index].value)){
   assert.ok(!['.','::'].includes(tokens[index-1]?.value),'binding renaming cannot rewrite properties or qualified symbols');
   replacements.push(tokens[index]);
  }
  for(const token of replacements.reverse())corrected=corrected.slice(0,token.start)+renames.get(token.value)+corrected.slice(token.end);
  assert.ok(corrected.startsWith('pub fn '+item.name+'('),'reviewed styles preserve the original declaration identity');
  actions.push({op:'replace',field:'source',from:item.source,to:corrected});
 }
 assert.deepEqual([...seen].sort((a,b)=>a-b),required.map(row=>row.index).sort((a,b)=>a-b),'every binary diagnostic is corrected');
 return {id:'binary-record-compiler-review',level:'emitter',order:950,when:[['format','Rust'],['scope','formatted-item']],actions};
}
