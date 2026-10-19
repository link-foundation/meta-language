import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import test from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';
import {createModuleContext,sourceCoverage} from '../scripts/generate-self-translation-report.mjs';
import {selfTranslate,selfTranslationSignatures} from '../src/self-translation.js';
import {checkProgram} from '../src/translation/check.js';
import {TranslationError} from '../src/translation/diagnostics.js';
import {parseJavaScript} from '../src/translation/javascript.js';

const fixture=readProjectData(new URL('../../parity/fixtures/reference-unary-update-inference.lino',import.meta.url));

test('unary reference updates infer numeric slots without a missing right operand',()=>{
 assert.equal(fixture.cases.length,16);
 for(const row of fixture.cases){
  const program=parseJavaScript(row.source),definition=program.items[0],write=definition.body.body.value;
  assert.equal(write.k,'referenceWrite',row.id);assert.equal(write.op,row.op,row.id);assert.equal(write.value,undefined,row.id);
  const result=definition.ret.kind==='option'?definition.ret.element:definition.ret;
  assert.equal(result.kind,row.numericKind,row.id);
  assert.throws(()=>checkProgram(program),error=>{
   assert.ok(error instanceof TranslationError,row.id);assert.equal(error.kind,'unsupported',row.id);
   assert.equal(error.details.construct,'reference mutation',row.id);assert.deepEqual(error.span,write.span,row.id);return true;
  });
 }
});

test('unsupported unary reference updates preserve source bytes and checked sibling signatures',()=>{
 for(const row of fixture.cases){
  const source=row.source+'\n/** @param {number} value @returns {number} */\nexport function stable(value){return value+1;}\n';
  const signatures=selfTranslationSignatures(source,'JavaScript');
  assert.deepEqual(signatures.map(value=>value.name),['stable'],row.id);assert.equal(signatures[0].ret.kind,'float',row.id);
  const translated=selfTranslate(source,'JavaScript','Rust'),carried=translated.items.filter(value=>value.status==='carried');
  assert.equal(carried.filter(value=>value.term==='export_statement').length,1,row.id);assert.equal(carried[0].reason,'unsupported',row.id);
  assert.equal(sourceCoverage(source,translated.items).unrepresentedBytes,0,row.id);
  assert.equal(selfTranslate(translated.code,'Rust','JavaScript').code,source,row.id);
 }
});

test('the captured recursive cache module reaches explicit report obligations without an internal inference exception',()=>{
 const capture=fixture.historicalModule,bytes=Buffer.from(capture.source,'utf8');
 assert.equal(bytes.length,3576);assert.equal(bytes.length,capture.bytes);
 assert.equal(createHash('sha256').update(bytes).digest('hex'),'afffcb0616fbc415740434b4f03f7d8f9bce99e78f977c2abb23897676f8eb31');
 assert.equal(capture.sha256,'afffcb0616fbc415740434b4f03f7d8f9bce99e78f977c2abb23897676f8eb31');
 assert.equal(createHash('sha1').update(Buffer.from('blob '+bytes.length+'\0')).update(bytes).digest('hex'),capture.blobSha);
 assert.equal(capture.blobSha,'1e8402a364a5f6c97def585e0c3c063b68ddcd4d');
 assert.equal(fixture.capture.logSha256,'c7e59b89c3159256118ff0bb7e6349c6094381e13a4bd8f4e255e4b334ce7f97');
 assert.ok(fixture.capture.excerpt.length>=4);
 assert.deepEqual(selfTranslationSignatures(capture.source,'JavaScript'),[]);
 const sources=new Map([['grammar-runtime/recursive-seed-memo.js',capture.source],['consumer.js',"import { advanceRecursiveMemoSeed } from './grammar-runtime/recursive-seed-memo.js';\n/** @param {number} value @returns {number} */\nexport function stable(value){return value+1;}\n"]]);
 const context=createModuleContext(name=>sources.get(name)??null),options=context('consumer.js');
 assert.deepEqual(options.imports['./grammar-runtime/recursive-seed-memo.js'],[]);
 assert.deepEqual(selfTranslationSignatures(sources.get('consumer.js'),'JavaScript',options).map(value=>value.name),['stable']);
 for(const[name,source]of sources){
  const translated=selfTranslate(source,'JavaScript','Rust',context(name));
  assert.equal(sourceCoverage(source,translated.items).unrepresentedBytes,0,name);
  assert.equal(selfTranslate(translated.code,'Rust','JavaScript').code,source,name);
 }
});
