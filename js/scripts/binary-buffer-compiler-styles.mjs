// Four bijective local renames from the actual rendered native CI diagnostics.
// Formatting proves the complete original function before any name is changed.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {tokenize} from '../src/translation/lexer.js';
import {readProjectData,serializeProjectData} from './project-data.mjs';
import {readSourceTextRustBlock} from './source-text-byte-library-style.mjs';
import {formatProductRuleSource} from './product-rule-style-corrections.mjs';
const reviewFile=new URL('../../parity/self-translation/binary-buffer-ci-style-review-117536842665.lino',import.meta.url);
const digest=source=>createHash('sha256').update(source).digest('hex');

function verifyReviewedBinaryBufferSource(current,previous){
 if(current===previous)return new Map();
 assert.equal(digest(current),'fc538108ce75bfa56d830e0a7feb2fb295a72adc213956acbf99b7a5e7930d98',
  'the complete pre-style function is the recorded emitter successor');
 const tokens=tokenize(current,'Rust').tokens, aliases=new Map();
 const identifiers=new Set(tokens.filter(token=>token.kind==='identifier').map(token=>token.raw));
 let restored=current;
 for(const token of tokens.toReversed()){
  if(token.kind!=='identifier')continue;
  const original=token.raw
   .replace(/^source_ml_source1_(s[0-9]+)$/u,'ml_$1')
   .replace(/^source_ml_temporary_(result[0-9]+|transcode_link_network_binary_loop[0-9]+)$/u,'ml_$1')
   .replace(/^MlTemporary(TranscodeLinkNetworkBinary(?:Join|Loop)[0-9]+[A-Za-z]*)$/u,'Ml$1');
  if(original===token.raw)continue;
  assert.ok(!identifiers.has(original),'the original identifier must not already be bound or referenced');
  assert.equal(aliases.get(original)??token.raw,token.raw,'each original identifier has one successor');
  aliases.set(original,token.raw);
  restored=restored.slice(0,token.start)+original+restored.slice(token.end);
 }
 assert.equal(aliases.size,51);
 const call='let ml_s6 = self::ml_transcode_link_network_binary_loop40(input, &(header), index);';
 assert.equal(restored.split(call).length,2,'there is one formatting-only call change');
 restored=restored.replace(call,[
  'let ml_s6 =',
  '                                                self::ml_transcode_link_network_binary_loop40(',
  '                                                    input,',
  '                                                    &(header),',
  '                                                    index,',
  '                                                );',
 ].join('\n'));
 assert.equal(restored,previous,'the complete reviewed function is recovered byte for byte');
 return aliases;
}

export function readBinaryBufferCompilerStyles(source,feedback=readProjectData(reviewFile)){
 assert.equal(digest(serializeProjectData(feedback,reviewFile)),'e78283bb105f5e1d7703e15e89a360beb3ec162850a4a192a68128d0e546a97d','the complete reviewed diagnostic packet is immutable');
 assert.equal(feedback.schemaVersion,1);
 assert.equal(feedback.reviewKind,'Reviewed rendered CI diagnostics. No machine applicability metadata was reported or invented.');
 assert.equal(feedback.capture.run,39079123225);assert.equal(feedback.capture.job,117536842665);
 assert.equal(feedback.capture.logSha256,'99b99c1ace473d50111777ea1b12dd76b17839d999eda57949c78ed585676fad');
 assert.equal(feedback.diagnostics.length,4);
 const item=feedback.function;assert.equal(item.name,'transcode_link_network_binary');
 assert.equal(digest(item.source),item.sha256);
 const from=readSourceTextRustBlock(source,'fn',item.name);
 const aliases=verifyReviewedBinaryBufferSource(formatProductRuleSource(from).trim(),item.source);
 const tokens=tokenize(from,'Rust').tokens,renames=new Map(),seen=new Set();
 assert.equal(item.renames.length,4);
 for(const row of item.renames){
  assert.match(row.from,/^[a-z][a-z_0-9]*$/u);assert.match(row.to,/^[a-z]+(?:_[a-z]+)+$/u);
  assert.ok(!renames.has(row.from)&&!seen.has(row.diagnostic));
  const diagnostic=feedback.diagnostics[row.diagnostic];assert.equal(diagnostic.index,row.diagnostic);
  assert.equal(diagnostic.path,feedback.capture.path);
  assert.equal(diagnostic.message,"binding's name is too similar to existing binding");seen.add(row.diagnostic);
  const actual=aliases.get(row.from)??row.from;
  assert.equal(tokens.filter(token=>token.kind==='identifier'&&token.value===actual).length,2,'one actual binding and one actual use');
  assert.ok(!tokens.some(token=>token.kind==='identifier'&&token.value===row.to),'new names cannot capture existing identifiers');
  assert.ok(!tokens.some(token=>token.kind==='string'&&new RegExp('\\b'+actual+'\\b','u').test(token.value)),'implicit format captures remain outside this finite rename');
  renames.set(actual,row.to);
 }
 assert.equal(new Set(renames.values()).size,renames.size);
 assert.deepEqual([...seen].sort((a,b)=>a-b),[0,1,2,3],'every reported binding has one correction');
 const edits=[];
 for(let at=0;at<tokens.length;at+=1){const token=tokens[at];if(token.kind!=='identifier'||!renames.has(token.value))continue;
  assert.ok(!['.','::'].includes(tokens[at-1]?.value),'properties and qualified names retain their exact identity');
  edits.push({start:token.start,end:token.end,from:token.value,to:renames.get(token.value)});
 }
 assert.equal(edits.length,8);
 let to=from;for(const edit of [...edits].reverse())to=to.slice(0,edit.start)+edit.to+to.slice(edit.end);
 const result=tokenize(to,'Rust').tokens;assert.equal(result.length,tokens.length);
 for(let at=0;at<tokens.length;at+=1){assert.equal(result[at].kind,tokens[at].kind);assert.equal(result[at].raw,tokens[at].kind==='identifier'&&renames.has(tokens[at].value)?renames.get(tokens[at].value):tokens[at].raw);}
 let inverse=to;
 for(let at=result.length-1;at>=0;at-=1)if(result[at].kind==='identifier'&&[...renames.values()].includes(result[at].value)){
  const original=[...renames].find(([,value])=>value===result[at].value)[0];inverse=inverse.slice(0,result[at].start)+original+inverse.slice(result[at].end);
 }
 assert.equal(inverse,from,'the whole original function byte sequence is recovered');
 assert.equal(source.split(from).length,2);
 return {source:source.replace(from,()=>to),actions:[{op:'replace',field:'source',from,to}]};
}
