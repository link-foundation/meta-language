import assert from 'node:assert/strict';
import test from 'node:test';
import {runInNewContext} from 'node:vm';
import {readTypedBooleanParameterStyles} from '../scripts/typed-boolean-source-style.mjs';
import {parseRust} from '../src/translation/rust.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';

const operation=source=>runInNewContext(emitJavaScript(checkProgram(parseRust(source+'\nfn main() {}'))).text+';choose;');

test('recorded primitive Boolean styles retain every parameter assignment and exact complete source inverse',()=>{
  for(const expression of ['first != true','first != false','true == first','false == first',
    '(first != true) && (first != false)','(first == true) || (first == false)',
    '((first != true) && (first != false)) || other','(first != true) || ((other != true) && (other != false))',
    '!(!(first == true))','(false || first) && true','true && (first || false)',
    'first && !first','!first || first']){
    const source='// original outside context\npub fn choose(first: bool, other: bool) -> bool { if '+expression+' { true } else { false } }\n// retained trailing context\n';
    const styled=readTypedBooleanParameterStyles(source,'choose'),before=operation(source),after=operation(styled.source);
    for(const first of [false,true])for(const other of [false,true])assert.equal(after(first,other),before(first,other),expression);
    let restored=styled.source;
    for(const action of [...styled.actions].reverse()){
      assert.equal(action.op,'replace');assert.equal(action.field,'source');assert.equal(restored.split(action.to).length,2);
      restored=restored.replace(action.to,action.from);
    }
    assert.equal(restored,source);
    assert.equal(readTypedBooleanParameterStyles(styled.source,'choose').source,styled.source);
    assert.ok(styled.source.startsWith('// original outside context\n'));
    assert.ok(styled.source.endsWith('// retained trailing context\n'));
  }
});

test('Boolean source styles refuse shadowed or custom types and preserve unsupported effectful predicates',()=>{
  for(const source of [
    'pub fn choose(first: Other) -> bool { first != true }',
    'pub fn choose(first: bool) -> bool { let first = false; if first != true { true } else { false } }',
    'pub fn choose(first: bool) -> bool { if let Some(first) = other { true } else { false } }',
  ])assert.throws(()=>readTypedBooleanParameterStyles(source,'choose'));
  for(const expression of ['effect(first) != true','first.field != true','first == 0']){
    const source='pub fn choose(first: bool) -> bool { if '+expression+' { true } else { false } }';
    assert.deepEqual(readTypedBooleanParameterStyles(source,'choose'),{source,actions:[]});
  }
});


test('primitive string comparisons retain precedence under negation in Boolean conditions',()=>{
  for(const expression of ['!(version == "1.0.151")','!(version != "1.0.151") || (first != false)']){
    const source='pub fn choose(first: bool, version: &str) -> bool { if '+expression+' { true } else { false } }';
    const styled=readTypedBooleanParameterStyles(source,'choose'),before=operation(source),after=operation(styled.source);
    for(const first of [false,true])for(const version of ['','1.0.151','1.0.152'])assert.equal(after(first,version),before(first,version),expression);
  }
});
