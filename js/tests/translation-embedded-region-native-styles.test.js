import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {runInNewContext} from 'node:vm';
import {DecoratorSet} from '../src/decorators.js';
import {parseJavaScript} from '../src/translation/javascript.js';
import {parseRust} from '../src/translation/rust.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {emitRust, readRustArrayPrelude} from '../src/translation/emit-rust.js';
import {tokenize} from '../src/translation/lexer.js';
import {readEmbeddedRegionCharge, acceptEmbeddedRegionSourceCoverage} from '../src/translation/embedded-region-rules.js';
import {readPortableSourceParameterStyles, readPortableSourceLintStyles} from '../scripts/portable-source-library-style.mjs';
import {readEmbeddedRegionNativeStyles} from '../scripts/embedded-region-native-styles.mjs';

function reviewEmbeddedRegionStyles(source) {
  const result = readEmbeddedRegionNativeStyles(source);
  const decorators = new DecoratorSet(result.actions.length ? [{id: 'embedded-region-style-control', level: 'emitter', order: 0,
    when: [['format', 'Rust']], actions: result.actions}] : []);
  assert.equal(decorators.decorate('emitter', {format: 'Rust', source}).source, result.source);
  assert.deepEqual(readEmbeddedRegionNativeStyles(result.source), {source: result.source, actions: []});
  const original = tokenize(source, 'Rust'), styled = tokenize(result.source, 'Rust');
  assert.deepEqual(styled.comments.map(comment => comment.text), original.comments.map(comment => comment.text));
  assert.deepEqual(styled.tokens.filter(token => ['number', 'string'].includes(token.kind)).map(token => token.raw),
    original.tokens.filter(token => ['number', 'string'].includes(token.kind)).map(token => token.raw));
  return result;
}

test('embedded-region native styles prove exactly two const bodies and four actual numeric ancestor defaults', () => {
  const source = readFileSync(new URL('../src/translation/embedded-region-rules.js', import.meta.url), 'utf8');
  const emitted = emitRust(checkProgram(parseJavaScript(source, {referenceSemantics: false})), {arrayConsumption: true});
  const header = emitted.text.split('\n').find(line => line.startsWith('#![allow('));
  const raw = [header, ...emitted.preludes, ...emitted.definitions].join('\n\n');
  const declarations = new Set([...raw.matchAll(/^pub (?:fn|enum|struct|mod) (\w+)/gmu)].map(match => match[1]));
  const actions = [...declarations].filter(name => raw.includes('crate::' + name))
    .map(name => ({op: 'replace', field: 'source', from: 'crate::' + name, to: 'self::' + name}));
  const owner = new DecoratorSet([{id: 'embedded-region-owner-control', level: 'emitter', order: 0,
    when: [['format', 'Rust']], actions}]);
  const relocated = owner.decorate('emitter', {format: 'Rust', source: raw}).source;
  const reviewed = readPortableSourceLintStyles(readPortableSourceParameterStyles(relocated).source).source;
  const result = reviewEmbeddedRegionStyles(reviewed);
  assert.equal(result.actions.length, 3, 'two qualifiers and one complete ancestor owner are recorded');
  assert.deepEqual(result.actions.map(action => /^pub fn (\w+)/u.exec(action.from)[1]), [
    'read_embedded_region_charge', 'accept_embedded_region_source_coverage', 'ml_read_embedded_region_ancestor_loop1',
  ]);
  for (const action of result.actions.slice(0, 2)) {
    assert.equal(action.to, action.from.replace(/^pub fn /u, 'pub const fn '), 'qualification preserves every original body byte');
  }
  const ancestor = result.actions[2];
  assert.equal((ancestor.from.match(/unwrap_or_else/gu) ?? []).length, 4);
  assert.equal((ancestor.to.match(/unwrap_or\(/gu) ?? []).length, 4);
  assert.doesNotMatch(ancestor.to, /unwrap_or_else/u);
  assert.equal((result.source.match(/^pub const fn /gmu) ?? []).length, 2);
});

test('retained embedded-region primitive bodies preserve NaN infinity signed zero and every Boolean charge choice', () => {
  const source = 'pub fn read_embedded_region_charge(current:f64,start:f64,end:f64,byte_charge:bool)->f64{current+if byte_charge{end-start}else{1f64}}\n'
    + 'pub fn accept_embedded_region_source_coverage(start:f64,covered_until:f64)->bool{start>=covered_until}';
  const result = reviewEmbeddedRegionStyles(source);
  assert.equal(result.actions.length, 2);
  const values = [-Infinity, -1, -0, 0, 1, Infinity, NaN];
  for (const candidate of [source, result.source.replaceAll('pub const fn ', 'pub fn ')]) {
    // Recheck the actual unchanged runtime bodies. Native const compilation is
    // a CI concern; this control does not grant a wider const source model.
    const context = {};
    runInNewContext(emitJavaScript(checkProgram(parseRust(candidate + '\nfn main() {}'))).text,
      context, {timeout: 1000});
    for (const current of values) for (const start of values) for (const end of values) for (const byteCharge of [false, true]) {
      assert.ok(Object.is(context.read_embedded_region_charge(current, start, end, byteCharge),
        readEmbeddedRegionCharge(current, start, end, byteCharge)));
    }
    for (const start of values) for (const covered of values) {
      assert.equal(context.accept_embedded_region_source_coverage(start, covered), acceptEmbeddedRegionSourceCoverage(start, covered));
    }
  }
});

test('primitive const proof leaves custom calls references shadowing effects and opaque copies unchanged', () => {
  for (const source of [
    'pub fn read_embedded_region_charge(value:Custom)->f64{value+1f64}',
    'pub fn read_embedded_region_charge(value:&f64)->f64{value+1f64}',
    'pub fn read_embedded_region_charge(value:f64)->f64{other(value)}',
    'pub fn read_embedded_region_charge(value:f64)->f64{value.abs()}',
    'pub fn read_embedded_region_charge(value:f64)->f64{unknown!(value)}',
    'pub fn read_embedded_region_charge(value:f64)->f64{let other=value;other}',
    'pub fn read_embedded_region_charge(mut value:f64)->f64{value=1f64;value}',
    'pub fn read_embedded_region_charge(value:f64)->f64{if value{1f64}else{2f64}}',
    'pub fn read_embedded_region_charge(value:f64,flag:bool)->f64{if flag{value}else{false}}',
    'pub fn read_embedded_region_charge(value:f64,flag:bool)->f64{if flag{value}else value}',
    'pub fn unselected(value:f64)->f64{value+1f64}',
    'type f64=Custom;pub fn read_embedded_region_charge(value:f64)->f64{value+1f64}',
    'use custom::*;pub fn read_embedded_region_charge(value:f64)->f64{value+1f64}',
    'custom::vec!();pub fn read_embedded_region_charge(value:f64)->f64{value+1f64}',
    'pub mod std{}pub fn read_embedded_region_charge(value:f64)->f64{value+1f64}',
    'pub mod core{}pub fn read_embedded_region_charge(value:f64)->f64{value+1f64}',
    '#[custom]pub fn read_embedded_region_charge(value:f64)->f64{value+1f64}',
    'pub unsafe fn read_embedded_region_charge(value:f64)->f64{value+1f64}',
    'pub fn read_embedded_region_charge(value:f64)->f64{value}pub fn read_embedded_region_charge(value:f64)->f64{value}',
  ]) assert.equal(reviewEmbeddedRegionStyles(source).source, source);
  const complete = 'pub fn read_embedded_region_charge(value:f64)->f64{value+1f64}';
  const opaque = complete + '\npub fn literal()->&\'static str{r#"' + complete + '"#}';
  assert.equal(reviewEmbeddedRegionStyles(opaque).source, opaque, 'exact recorded replacements cannot also alter a literal copy');
  assert.throws(() => readEmbeddedRegionNativeStyles(complete.slice(0, -1)), /delimiter/u);
});

test('numeric ancestor defaults require the authentic array provider exact numeric columns and a pure NaN closure', () => {
  const provider = readRustArrayPrelude(true);
  const ancestor = 'pub fn ml_read_embedded_region_ancestor_loop1(values:&[f64],position:f64)->f64{'
    + '(self::ml_array::optional_at(values,self::ml_array::number_index(position))).unwrap_or_else(|| /* keep fallback */ f64::NAN)}';
  const source = provider + '\n' + ancestor;
  const result = reviewEmbeddedRegionStyles(source);
  assert.equal(result.actions.length, 1);
  assert.ok(result.source.includes('.unwrap_or( /* keep fallback */ f64::NAN)'));
  for (const candidate of [
    ancestor,
    provider.replace('index.and_then(|index| values.get(index)).cloned()', 'custom(values,index)') + '\n' + ancestor,
    source.replace('values:&[f64]', 'values:&[bool]'),
    source.replace('values:&[f64]', 'values:Vec<f64>'),
    source.replace('number_index(position)', 'number_index(unknown)'),
    source.replace('optional_at(values,', 'optional_at(unknown,'),
    source.replace('f64::NAN)}', 'effect())}'),
    source.replace('f64::NAN)}', 'other::NAN)}'),
    source.replace('->f64{(', '->f64{let values=custom();('),
    source.replace('ml_read_embedded_region_ancestor_loop1', 'unrelated_loop1'),
    'type Option=Custom;\n' + source,
    'use custom::f64;\n' + source,
  ]) assert.equal(reviewEmbeddedRegionStyles(candidate).source, candidate);
});
