import test from 'node:test';
import assert from 'node:assert/strict';
import vm from 'node:vm';
import {readProjectData} from '../scripts/project-data.mjs';
import {sourceSpanNumberArchive,sourceSpanNumberFixture} from '../scripts/generate-source-span-fixtures.mjs';
import {acceptSourceSpanOffset,acceptSourceSpanInterval} from '../src/source-span-rules.js';
import {acceptSourceSpanRecord,validateSourceSpans,SOURCE_SPAN_POLICY} from '../src/source-spans.js';
import {decodeTypedProgramLinks,encodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';

const fixture=readProjectData(new URL('../../parity/fixtures/source-span-numbers.lino',import.meta.url));
const output=code=>{const lines=[];vm.runInNewContext(code,{console:{log:value=>lines.push(String(value))}},{timeout:1000});return lines;};

test('shared source offsets distinguish integral values, signed zero and unsafe or nonfinite numbers',()=>{
  for(const value of [0,1,32,Number.MAX_SAFE_INTEGER])assert.equal(acceptSourceSpanOffset(value,Object.is(value,-0)),true,String(value));
  for(const value of [-0,-1,0.5,NaN,Infinity,-Infinity,Number.MAX_SAFE_INTEGER+1])assert.equal(acceptSourceSpanOffset(value,Object.is(value,-0)),false,String(value));
  assert.equal(acceptSourceSpanInterval(0,32),true);assert.equal(acceptSourceSpanInterval(32,0),false);
  assert.equal(SOURCE_SPAN_POLICY.offsetMaximum,Number.MAX_SAFE_INTEGER);
});

test('the host source-span record bridge preserves named shape and ordered safe offsets',()=>{
  for(const span of [{start:0,end:0},{start:0,end:32},{start:32,end:Number.MAX_SAFE_INTEGER}])assert.equal(acceptSourceSpanRecord(span),true);
  for(const span of [null,[],{start:0},{start:0,end:1,extra:false},{start:1,end:0},{start:-0,end:1},{start:0,end:-0},{start:'0',end:1},{start:0,end:Infinity}])assert.equal(acceptSourceSpanRecord(span),false);
});

test('span inspection does not execute source accessors or declaration iterator overrides',()=>{
  let calls=0;
  const span={end:1};Object.defineProperty(span,'start',{enumerable:true,get(){calls++;return 0;}});
  assert.equal(acceptSourceSpanRecord(span),false);assert.equal(calls,0);
  const root={};Object.defineProperty(root,'span',{enumerable:true,get(){calls++;return {start:0,end:1};}});
  assert.throws(()=>validateSourceSpans(root),/source span is invalid/u);assert.equal(calls,0);
  const declarations=new Map([['keep',{span:{start:0,end:1}}]]);
  declarations[Symbol.iterator]=()=>{calls++;throw new Error('custom iterator');};
  assert.doesNotThrow(()=>validateSourceSpans({declarations}));assert.equal(calls,0);
  assert.doesNotThrow(()=>validateSourceSpans({span:undefined}));
});

test('actual archived source offsets accept equivalent decimal and exponent spellings without touching floating values',()=>{
  assert.deepEqual(sourceSpanNumberFixture(),fixture);
  for(const spelling of ['32.0','3.2e1','0e0'])assert.ok(fixture.cases.some(entry=>entry.aliases.some(edit=>edit.spelling===spelling)),spelling);
  let observed=0;
  for(const entry of fixture.cases){
    const expected=decodeTypedProgramLinks(LinkNetwork.fromLino(entry.archive),entry.root);
    for(const edit of entry.aliases){
      const archive=sourceSpanNumberArchive(entry,edit),program=decodeTypedProgramLinks(archive.network,archive.root);
      assert.deepEqual(program,expected,edit.id);
      const rebuilt=encodeTypedProgramLinks(program);
      assert.deepEqual(output(emitTypedProgramLinks(rebuilt.network,rebuilt.root,'JavaScript').code),entry.output,edit.id);
      observed++;
    }
  }
  assert.ok(observed>40);
  const primitive=decodeTypedProgramLinks(LinkNetwork.fromLino(fixture.cases[0].archive),fixture.cases[0].root);
  assert.ok(JSON.stringify([...primitive.declarations]).includes('"kind":"float"'));
  const code=emitTypedProgramLinks(LinkNetwork.fromLino(fixture.cases[0].archive),fixture.cases[0].root,'JavaScript').code;
  assert.equal(vm.runInNewContext(`${code}\nObject.is(keep(-0),-0)`,{console:{log(){}}},{timeout:1000}),true);
});

test('nonintegral, unsafe, negative and signed-zero source offsets still refuse through actual archived roles',()=>{
  for(const entry of fixture.cases)for(const edit of entry.invalid){
    const archive=sourceSpanNumberArchive(entry,edit);
    assert.throws(()=>decodeTypedProgramLinks(archive.network,archive.root),undefined,edit.id);
    for(const target of ['JavaScript','Rust'])assert.throws(()=>emitTypedProgramLinks(archive.network,archive.root,target),undefined,`${edit.id}:${target}`);
  }
});
