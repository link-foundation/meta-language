import assert from 'node:assert/strict';
import test from 'node:test';
import {compareRustDefinitions,createRustDefinitionComparison,rustFunctionDefinitions} from '../scripts/generate-self-translation-report.mjs';
import {parseProgrammingLanguage} from '../src/programming-language-parser.js';

const generic='// cafe\u0301 😀\nfn first() -> i64 { 1 }\nfn café() -> i64 { 2 }\nfn first() -> i64 { 9 }\nmod nested { fn hidden() {} }\n';
const decorated='// decorated complete source\npub fn first() -> i64 {\n  1\n}\nfn café() -> i64 { 2 }\nfn first() -> i64 { 9 }\nmod nested { fn hidden() {} }\n';
const counterpart='// complete counterpart\nfn first() -> i64 {\n 1\n}\nfn café() -> i64 { 7 }\nfn unrelated() {}\n';
const observingReader=()=>{
  const inputs=[];
  return{inputs,read:source=>{inputs.push(source);return rustFunctionDefinitions(source);}};
};

test('one report row preserves both complete independent CST comparisons while parsing its exact native counterpart once',()=>{
  const observed=observingReader(),comparison=createRustDefinitionComparison(counterpart,observed.read);
  try{
    const expectedGeneric=compareRustDefinitions(generic,counterpart),expectedDecorated=compareRustDefinitions(decorated,counterpart);
    assert.deepEqual([...rustFunctionDefinitions(generic)],[['first','fn first() -> i64 { 1 }'],['café','fn café() -> i64 { 2 }']],'actual UTF-8 CST ranges retain first duplicate and exclude nested functions');
    assert.equal(expectedGeneric.functions,2);assert.equal(expectedGeneric.matched,2);assert.equal(expectedGeneric.identical,1);
    assert.deepEqual(comparison.compare(generic),expectedGeneric);
    assert.deepEqual(comparison.compare(decorated),expectedDecorated);
    assert.deepEqual(comparison.compare(generic),expectedGeneric);
    assert.deepEqual(observed.inputs,[generic,counterpart,decorated],'only three distinct complete actual parser inputs enter a two-mode row');
  }finally{comparison.clear();}
});

test('equal emitted parser bytes share one result and a changed complete source is parsed independently',()=>{
  const observed=observingReader(),comparison=createRustDefinitionComparison(null,observed.read);
  try{
    const changed=generic.replace('fn café() -> i64 { 2 }','fn café() -> i64 { 3 }');
    assert.notEqual(changed,generic);
    for(const source of[generic,generic,changed])assert.deepEqual(comparison.compare(source),compareRustDefinitions(source,null));
    assert.deepEqual(observed.inputs,[generic,changed]);
    const sameAsCounterpart=createRustDefinitionComparison(generic,observed.read);
    try{assert.deepEqual(sameAsCounterpart.compare(generic),compareRustDefinitions(generic,generic));}
    finally{sameAsCounterpart.clear();}
    assert.deepEqual(observed.inputs,[generic,changed,generic],'an emitted/counterpart byte identity still gets its full independent comparison');
  }finally{comparison.clear();}
});

test('only the original translatedCode parser input is cached while differing source bytes stay distinct',()=>{
  const observed=observingReader(),comparison=createRustDefinitionComparison(null,observed.read);
  const suffix='// meta-language:prelude end\nfn value() -> i64 { 1 }\n';
  try{
    for(const source of['// original header\n'+suffix,'// other header\n'+suffix])assert.deepEqual(comparison.compare(source),compareRustDefinitions(source,null));
    assert.deepEqual(observed.inputs,[suffix],'the original prelude delimiter remains part of each exact actual CST parse input');
    const changed=suffix.replace('{ 1 }','{ 2 }');
    assert.deepEqual(comparison.compare(changed),compareRustDefinitions(changed,null));
    assert.deepEqual(observed.inputs,[suffix,changed]);
  }finally{comparison.clear();}
});

test('the exact three-entry bound evicts old text and clear releases every per-row entry',()=>{
  const observed=observingReader(),comparison=createRustDefinitionComparison(null,observed.read);
  const sources=[1,2,3,4].map(value=>'fn value() -> i64 { '+value+' }\n');
  for(const source of sources)assert.deepEqual(comparison.compare(source),compareRustDefinitions(source,null));
  assert.deepEqual(observed.inputs,sources);
  assert.deepEqual(comparison.compare(sources[0]),compareRustDefinitions(sources[0],null));
  assert.deepEqual(observed.inputs,[...sources,sources[0]],'a fourth distinct actual source cannot retain the first entry');
  comparison.clear();
  assert.deepEqual(comparison.compare(sources[0]),compareRustDefinitions(sources[0],null));
  assert.deepEqual(observed.inputs,[...sources,sources[0],sources[0]],'a later row must perform a new actual parse');
  comparison.clear();
});

test('real recovered Rust CSTs retain complete comparison results instead of acquiring a success shortcut',()=>{
  const malformed='fn broken( {';
  assert.equal(parseProgrammingLanguage(malformed,'Rust').tree.flags.hasError,true);
  const observed=observingReader(),comparison=createRustDefinitionComparison(malformed,observed.read);
  try{
    const expected=compareRustDefinitions(generic,malformed);
    assert.deepEqual(comparison.compare(generic),expected);
    assert.deepEqual(comparison.compare(generic),expected);
    assert.deepEqual(observed.inputs,[generic,malformed]);
  }finally{comparison.clear();}
});

test('a failed actual-reader control propagates the original error without caching failure or evicting earlier successful inputs',()=>{
  const inputs=[],failure=new Error('independent reader failure after actual counterpart parsing');let reject=true;
  const comparison=createRustDefinitionComparison(counterpart,source=>{
    inputs.push(source);const actual=rustFunctionDefinitions(source);
    if(source===counterpart&&reject){reject=false;throw failure;}
    return actual;
  });
  try{
    assert.throws(()=>comparison.compare(generic),error=>error===failure);
    assert.deepEqual(comparison.compare(generic),compareRustDefinitions(generic,counterpart));
    assert.deepEqual(inputs,[generic,counterpart,counterpart],'only successful complete parses enter the cache; the exact failed input is retried');
  }finally{comparison.clear();}
});
