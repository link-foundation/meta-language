import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {selfTranslate} from '../../../../js/src/self-translation.js';
import {acceptOwnedOutcomeIntegerValue} from './integer-rules.mjs';
import {generateOwnedOutcomeIntegerRules} from './generate-integers.mjs';
import {readOriginalOutcomeIntegerOracle} from './integer-history.mjs';

test('shared outcome integer ranges retain every original BigInt boundary and exact native fixture bits',()=>{
  const {fixture,files}=generateOwnedOutcomeIntegerRules({check:true});
  assert.equal(fixture.cases.length,1688);assert.equal(files.length,2);
  for(const row of fixture.cases){
    assert.equal(acceptOwnedOutcomeIntegerValue(row.value,row.signed,row.width),row.expected,row.name);
    const bits=Buffer.alloc(8);bits.writeDoubleBE(row.value);assert.equal(bits.toString('hex'),row.bits);
    if(row.originalOutput!==null)assert.equal(row.originalOutput.bits,row.bits);
  }
  for(const signed of [false,true]){
    for(const width of [32,64])for(const value of [NaN,Infinity,-Infinity])assert.equal(acceptOwnedOutcomeIntegerValue(value,signed,width),false);
    for(const width of [16,128,0,-1,0.5,NaN,Infinity,-Infinity])assert.equal(acceptOwnedOutcomeIntegerValue(0,signed,width),false);
  }
  assert.equal(acceptOwnedOutcomeIntegerValue(2**63-1024,true,64),true);
  assert.equal(acceptOwnedOutcomeIntegerValue(2**63,true,64),false);
  assert.equal(acceptOwnedOutcomeIntegerValue(2**64-2048,false,64),true);
  assert.equal(acceptOwnedOutcomeIntegerValue(2**64,false,64),false);
  const {history}=readOriginalOutcomeIntegerOracle();
  const current=readFileSync(new URL('./owned.mjs',import.meta.url),'utf8');
  assert.equal(current,history.sources[0].sourceText.replace(history.before,history.after)+history.insertion);
  assert.equal(current.replace(history.insertion,'').replace(history.after,history.before),history.sources[0].sourceText);
  assert.ok(history.after.includes("requireValue(typeof value==='number'&&Number.isFinite(value)&&Number.isInteger(value)&&!Object.is(value,-0)"));
  assert.ok(history.after.includes('const integer=BigInt(value)'));
  assert.ok(history.after.includes("buffer.writeDoubleBE(value)"));
});

test('the complete integer range module translates without carried items and restores every original source byte',()=>{
  const source=readFileSync(new URL('./integer-rules.mjs',import.meta.url),'utf8');
  const translated=selfTranslate(source,'JavaScript','Rust');
  assert.equal(translated.items.filter(item=>item.status==='carried').length,0);
  assert.equal(translated.items.filter(item=>item.status==='translated'&&item.term!=='comment').length,1);
  assert.equal(selfTranslate(translated.code,'Rust','JavaScript').code,source);
});
