import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {readPortableLiteralOptionStyles} from '../scripts/portable-literal-option-style.mjs';
import {readSourceTextRustBlock} from '../scripts/source-text-byte-library-style.mjs';
import {DecoratorSet} from '../src/decorators.js';

const current=readFileSync(new URL('../../rust/src/translation/owned_outcome_preorder_rules/part_1.rs',import.meta.url),'utf8');
const helper=readSourceTextRustBlock(current,'fn','optional_at');
const context='pub mod ml_array {\n'+helper+'\n}\n';

test('constant option styles retain actual helper semantics, complete source owners and signed zero literals',()=>{
  const source=context+`pub fn literal(values: &[f64]) -> f64 {
    let first = (self::ml_array::optional_at(values, Some(0usize))).unwrap_or_else(|| 0f64);
    let second = self::ml_array::optional_at(values, None).unwrap_or_else(|| (-0f64));
    first + second
  }`;
  const expected=source.replace('.unwrap_or_else(|| 0f64)','.unwrap_or(0f64)')
    .replace('.unwrap_or_else(|| (-0f64))','.unwrap_or(-0f64)');
  const result=readPortableLiteralOptionStyles(source);assert.equal(result.source,expected);
  assert.equal(result.actions.length,1);assert.deepEqual(result.proofs[0].literals,['0f64','-0f64']);
  assert.equal(result.actions[0].from,readSourceTextRustBlock(source,'fn','literal'));
  assert.equal(result.actions[0].to,readSourceTextRustBlock(expected,'fn','literal'));
  assert.equal(expected.replace(result.actions[0].to,result.actions[0].from),source);
  const styles=new DecoratorSet([{id:'constant-option-control',level:'emitter',when:[['format','Rust']],actions:result.actions}]);
  assert.equal(DecoratorSet.fromLino(styles.toLino()).decorate('emitter',{format:'Rust',source}).source,expected);
  assert.deepEqual(readPortableLiteralOptionStyles(expected),{source:expected,actions:[],proofs:[]});
});

test('constant option styles preserve effectful defaults, opaque scopes and unproved receiver types',()=>{
  const call='(self::ml_array::optional_at(values, None))';
  const wrap=body=>context+'pub fn preserved(values: &[f64]) -> f64 { '+body+' }';
  for(const body of [
    call+'.unwrap_or_else(|| side_effect())',
    call+'.unwrap_or_else(|| { 0f64 })',
    call+'.unwrap_or_else(|| /* retain */ 0f64)',
    call+'.unwrap_or_else(|| 0f64 /* retain */)',
    call+'.unwrap_or_else(|| String::from("owned"))',
    'foreign::ml_array::optional_at(values, None).unwrap_or_else(|| 0f64)',
    'value.unwrap_or_else(|| 0f64)',
    'opaque!('+call+'.unwrap_or_else(|| 0f64))',
    'mod nested { pub fn f() -> f64 { '+call+'.unwrap_or_else(|| 0f64) } } 0f64',
    'let text = "unwrap_or_else(|| 0f64)"; 0f64',
  ]){const source=wrap(body);assert.equal(readPortableLiteralOptionStyles(source).source,source,body);}
  const admitted=wrap(call+'.unwrap_or_else(|| 0f64)');
  for(const prefix of ['pub struct Option;\n','pub type Option = Custom;\n',
    'use custom::Option;\n','use custom::*;\n','trait Custom {}\n','impl Custom {}\n',
    '#![no_implicit_prelude]\n','macro_rules! custom { () => {} }\n']){
    const source=prefix+admitted;assert.equal(readPortableLiteralOptionStyles(source).source,source,prefix);
  }
  const altered=admitted.replace('values.get(index)','values.get(index + 1)');
  assert.equal(readPortableLiteralOptionStyles(altered).source,altered);
  assert.equal(readPortableLiteralOptionStyles(context+admitted).source,context+admitted);
});
