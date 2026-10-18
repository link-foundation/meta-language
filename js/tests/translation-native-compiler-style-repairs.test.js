import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';
import {readBinaryBufferCompilerStyles} from '../scripts/binary-buffer-compiler-styles.mjs';
import {readGuardedPrivateArrayWriteLibraryStyles} from '../scripts/guarded-private-array-write-library-style.mjs';
import {formatProductRuleSource} from '../scripts/product-rule-style-corrections.mjs';
import {readSourceTextRustBlock} from '../scripts/source-text-byte-library-style.mjs';
import {DecoratorSet} from '../src/decorators.js';
import {tokenize} from '../src/translation/lexer.js';
const review=readProjectData(new URL('../../parity/self-translation/binary-buffer-ci-style-review-117536842665.lino',import.meta.url));

test('the actual binary buffer compiler correction changes only four bound names and matches the shipped whole function',()=>{
 const original=review.function.source,correction=readBinaryBufferCompilerStyles(original);
 const aliases=new Map(review.function.renames.map(row=>[row.from,row.to]));
 const before=tokenize(original,'Rust').tokens,after=tokenize(correction.source,'Rust').tokens;
 assert.deepEqual(after.map(token=>[token.kind,token.raw]),before.map(token=>[token.kind,token.kind==='identifier'&&aliases.has(token.value)?aliases.get(token.value):token.raw]));
 const styles=new DecoratorSet([{id:'binary-buffer-review-control',level:'emitter',when:[['format','Rust']],actions:correction.actions}]);
 assert.equal(DecoratorSet.fromLino(styles.toLino()).decorate('emitter',{format:'Rust',source:original}).source,correction.source);
 const shipped=readFileSync(new URL('../../rust/src/translation/link_network_binary_buffer_rules/part_2.rs',import.meta.url),'utf8');
 assert.equal(formatProductRuleSource(correction.source).trim(),readSourceTextRustBlock(shipped,'fn',review.function.name));
});

test('binary buffer compiler corrections refuse altered diagnostics captures aliases and source contexts',()=>{
 for(const mutate of [
  value=>{value.capture.job+=1;},
  value=>{value.capture.logSha256='0'.repeat(64);},
  value=>{value.diagnostics.pop();},
  value=>{value.diagnostics[0].line+=1;},
  value=>{value.function.renames[0].to='input';},
  value=>{value.function.renames[0].from='false';},
  value=>{value.function.source+='\n';},
 ]){const value=structuredClone(review);mutate(value);assert.throws(()=>readBinaryBufferCompilerStyles(review.function.source,value));}
 assert.throws(()=>readBinaryBufferCompilerStyles(review.function.source.replace('pub fn','pub const fn')));
 assert.throws(()=>readBinaryBufferCompilerStyles(review.function.source+'\n'+review.function.source));
});

test('guard planner String styles use actual recorded source and preserve opaque macros comments and unknown types',()=>{
 const listed=DecoratorSet.fromLino(readFileSync(new URL('../../parity/self-translation/guarded-private-array-write-rules-decorators.lino',import.meta.url),'utf8'));
 const controls=listed.decorators.flatMap(row=>row.actions).filter(row=>row.from?.startsWith('pub fn read_guarded_write_text(')&&row.from.includes('|| String::new()')&&!row.to.includes('|| String::new()'));
 assert.equal(controls.length,1,'the complete actual getter is the constructor correction witness');
 const result=readGuardedPrivateArrayWriteLibraryStyles(controls[0].from);
 assert.ok(result.source.includes('.unwrap_or_default()'));
 assert.ok(!result.source.includes('|| String::new()'));
 assert.equal(readGuardedPrivateArrayWriteLibraryStyles(result.source).source,result.source);
 for(const part of [1,2,3,4]){
  const shipped=readFileSync(new URL('../../rust/src/translation/guarded_private_array_write_rules/part_'+part+'.rs',import.meta.url),'utf8');
  assert.equal(readGuardedPrivateArrayWriteLibraryStyles(shipped).source,shipped,'each complete corrected module retains its original lint annotations');
 }
 const opaque='pub fn preserved(value: Option<String>) -> String { opaque!(value.unwrap_or_else(|| String::new())) }';
 assert.ok(readGuardedPrivateArrayWriteLibraryStyles(opaque).source.includes('opaque!(value.unwrap_or_else(|| String::new()))'));
 const commented='pub fn preserved(value: Option<String>) -> String { value.unwrap_or_else(|| /* retain */ String::new()) }';
 assert.ok(readGuardedPrivateArrayWriteLibraryStyles(commented).source.includes('|| /* retain */ String::new()'));
 assert.throws(()=>readGuardedPrivateArrayWriteLibraryStyles('pub struct String;\npub fn preserved() -> String { String }'));
});
