import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {runInNewContext} from 'node:vm';
import {createHash} from 'node:crypto';
import {selfTranslate} from '../../../../js/src/self-translation.js';
import {acceptOwnedOutcomeIntegerDecimal,readOwnedOutcomeIntegerDecimalLimit} from './integer-decimal-rules.mjs';
import {generateOwnedOutcomeIntegerDecimalRules} from './generate-integer-decimals.mjs';
import {readSourceTextRustBlock} from '../../../../js/scripts/source-text-byte-library-style.mjs';
import {readProjectData} from '../../../../js/scripts/project-data.mjs';

test('exact integer decimal admission retains every independent BigInt value and canonical spelling before projection',()=>{
  const {fixture,files}=generateOwnedOutcomeIntegerDecimalRules({check:true});
  assert.equal(fixture.cases.length,3660);assert.equal(fixture.declarations.length,5);
  assert.equal(files.length,2);
  const workLoop=readSourceTextRustBlock(files.map(row=>row.source).join('\n'),'fn','ml_accept_owned_outcome_integer_decimal_loop1');
  assert.equal((workLoop.match(/\bloop \{/gu)??[]).length,1,'the complete lifted work loop remains iterative independently of string runtime loops');
  for(const row of fixture.cases){
    assert.equal(acceptOwnedOutcomeIntegerDecimal(row.text,row.signed,row.width),row.expected,JSON.stringify(row));
    if(row.expected)assert.equal(BigInt(row.text).toString(),row.text);
  }
  for(const signed of [false,true]){
    for(const width of [16,128,0,-1,0.5,NaN,Infinity,-Infinity]){
      assert.equal(acceptOwnedOutcomeIntegerDecimal('0',signed,width),false);
      assert.equal(readOwnedOutcomeIntegerDecimalLimit(false,signed,width),'');
    }
    for(const width of [32,64])for(const text of ['\ud800','\udfff','9'.repeat(4096)])assert.equal(acceptOwnedOutcomeIntegerDecimal(text,signed,width),false);
  }
  for(const [inside,outside,signed]of [
    ['18446744073709551615','18446744073709551616',false],
    ['9223372036854775807','9223372036854775808',true],
    ['-9223372036854775808','-9223372036854775809',true],
  ]){
    assert.equal(Number(inside),Number(outside),'the original decimal distinction would disappear in binary64');
    assert.equal(acceptOwnedOutcomeIntegerDecimal(inside,signed,64),true);
    assert.equal(acceptOwnedOutcomeIntegerDecimal(outside,signed,64),false);
  }
});

test('the complete original ordering draft retains every input and independent observation after decimal digit lookup',()=>{
  const original=readProjectData(new URL('../../../../parity/fixtures/grammar-outcome-integer-decimal-ordering-source.lino',import.meta.url));
  assert.equal(original.commit,null);assert.equal(Buffer.byteLength(original.source),original.bytes);
  assert.equal(createHash('sha256').update(original.source).digest('hex'),original.sha256);
  assert.equal(original.diagnostic.reason,'ordering on string');
  const oracle=runInNewContext(original.source.replaceAll('export function','function')+';acceptOwnedOutcomeIntegerDecimal;');
  const {fixture}=generateOwnedOutcomeIntegerDecimalRules({check:true});
  for(const row of fixture.cases)assert.equal(oracle(row.text,row.signed,row.width),row.expected,JSON.stringify(row));
});

test('the complete exact decimal module translates without carried items and restores its complete original source',()=>{
  const source=readFileSync(new URL('./integer-decimal-rules.mjs',import.meta.url),'utf8');
  const translated=selfTranslate(source,'JavaScript','Rust');
  assert.equal(translated.items.filter(row=>row.status==='carried').length,0);
  assert.equal(translated.items.filter(row=>row.status==='translated'&&row.term!=='comment').length,2);
  assert.equal(selfTranslate(translated.code,'Rust','JavaScript').code,source);
});
