import assert from 'node:assert/strict';
import test from 'node:test';
import {readFileSync} from 'node:fs';
import {compileGrammar} from '../src/grammar.js';
import {parseGrammarLinks} from '../src/grammar-links.js';
import {renderSyntaxTree} from '../src/grammar-runtime/syntax-tree.js';
import {structuredDataToLino,readStructuredDataLino} from '../src/structured-data-links.js';
import {readGrammarAttributeIntegerKey,compareGrammarAttributeIntegerKeys} from '../src/translation/grammar-attribute-order-rules.js';
import {readProjectData} from '../scripts/project-data.mjs';
import {generateGrammarAttributeOrderFixtures} from '../scripts/generate-grammar-attribute-order-fixtures.mjs';
import {selectVersionedAssertion} from '../scripts/assertion-version-ranges.mjs';

import {readGrammarAttributeOrderNativeStyles} from '../scripts/generate-grammar-attribute-order-rules.mjs';
import {readSourceTextRustBlock} from '../scripts/source-text-byte-library-style.mjs';
import {DecoratorSet} from '../src/decorators.js';
import {formatProductRuleSource} from '../scripts/product-rule-style-corrections.mjs';

const fixture=readProjectData(new URL('../../parity/fixtures/grammar-attribute-order.lino',import.meta.url));
const codeUnits=text=>Array.from({length:text.length},(_,index)=>text.charCodeAt(index));

test('actual grammar actions preserve attribute insertion numeric boundaries and UTF-16 rendering',()=>{
  generateGrammarAttributeOrderFixtures({check:true});
  const versions=[...readFileSync(new URL('../../rust/Cargo.lock',import.meta.url),'utf8')
    .matchAll(/\[\[package\]\]\nname = "indexmap"\nversion = "([^"]+)"/gu)].map(([,version])=>version);
  assert.equal(versions.length,1);
  assert.equal(selectVersionedAssertion(fixture.nativeOrdering,versions[0]).id,'indexmap-2.14.2-insertion-order');
  assert.throws(()=>selectVersionedAssertion(fixture.nativeOrdering,'2.14.3'),/uncovered active version/u);
  for(const item of fixture.cases){
    const outcome=compileGrammar(parseGrammarLinks(item.grammar)).parseTree(item.source);
    assert.equal(outcome.ok,true,JSON.stringify(outcome.rejection));
    assert.deepEqual(Object.keys(outcome.tree.attributes),item.order);
    assert.deepEqual(Object.entries(outcome.tree.attributes),item.entries);
    assert.equal(renderSyntaxTree(outcome.tree),item.rendered);
    const insertion=[...new Set(item.keys)];
    const ordered=insertion.map(key=>({key,index:readGrammarAttributeIntegerKey(codeUnits(key))}))
      .sort((left,right)=>compareGrammarAttributeIntegerKeys(left.index,right.index)).map(row=>row.key);
    assert.deepEqual(ordered,item.order);
    const restored=readStructuredDataLino(structuredDataToLino(outcome));
    assert.deepEqual(restored,outcome);
    assert.deepEqual(Object.keys(restored.tree.attributes),item.order);
  }
});

test('integer attribute names retain exact decimal boundaries and reject non-canonical code units',()=>{
  for(const {name,index}of fixture.integerNames)assert.equal(readGrammarAttributeIntegerKey(codeUnits(name)),index,name);
  for(const units of [[NaN],[Infinity],[-Infinity],[48.5],[-1],[65536]])assert.equal(readGrammarAttributeIntegerKey(units),-1);
});

test('recorded attribute native styles authenticate primitive owners and finite slice length only', () => {
  const native = readFileSync(new URL('../../rust/src/translation/grammar_attribute_order_rules/part_1.rs', import.meta.url), 'utf8');
  const previousGuard = '((units.len() as f64) < 1f64) || ((units.len() as f64) > 10f64)';
  const currentGuard = '!(1usize..=10usize).contains(&units.len())';
  const raw = native.replace('pub const fn compare_grammar_attribute_integer_keys(', 'pub fn compare_grammar_attribute_integer_keys(')
    .replace(currentGuard, previousGuard);
  const styled = readGrammarAttributeOrderNativeStyles(raw);
  assert.equal(styled.actions.length, 2);
  assert.equal(formatProductRuleSource(styled.source), native);
  const recorded = new DecoratorSet([{id: 'attribute-native-control', level: 'emitter', order: 0,
    when: [['format', 'Rust']], actions: styled.actions}]);
  assert.equal(DecoratorSet.fromLino(recorded.toLino()).decorate('emitter', {format: 'Rust', source: raw}).source, styled.source);
  assert.deepEqual(readGrammarAttributeOrderNativeStyles(styled.source), {source: styled.source, actions: []});
  const compare = readSourceTextRustBlock(raw, 'fn', 'compare_grammar_attribute_integer_keys');
  const range = readSourceTextRustBlock(raw, 'fn', 'read_grammar_attribute_integer_key');
  const qualified = styled.actions.find(action => action.from === compare).to;
  assert.equal(qualified.replace('pub const fn ', 'pub fn '), compare, 'const qualification preserves every body byte');
  const ranged = readSourceTextRustBlock(styled.source, 'fn', 'read_grammar_attribute_integer_key');
  assert.equal(ranged.replace(currentGuard, previousGuard), range, 'only the finite length guard spelling changes');
  for (const length of [0, 1, 2, 9, 10, 11, 1024]) {
    const values = Array.from({length}, () => 48);
    assert.equal(values.length < 1 || values.length > 10, !(values.length >= 1 && values.length <= 10));
  }
  assert.equal(NaN < 1 || NaN > 10, false);
  assert.equal(!(NaN >= 1 && NaN <= 10), true, 'arbitrary f64 guards cannot use the finite-length proof');
  for (const source of [
    'type f64 = CustomFloat;\n' + raw,
    'type usize = CustomSize;\n' + raw,
    'use crate::custom::f64;\n' + raw,
    'mod std {}\n' + raw,
    'mod core {}\n' + raw,
    'macro_rules! format { () => { 0 }; }\n' + raw,
    raw + '\npub fn unknown_macro() { vec![custom!()]; }',
    raw + '\npub fn qualified_macro() { crate::custom::format!(); }',
  ]) assert.deepEqual(readGrammarAttributeOrderNativeStyles(source), {source, actions: []});
  for (const source of [
    compare.replace('left: f64', 'left: &f64'),
    compare.replace('if left < 0f64', 'let left = 0f64; if left < 0f64'),
    compare.replace('right < 0f64 { 0f64 }', 'right < 0f64 { 2f64 }'),
    compare.replace('if left < 0f64', 'if (left,) < 0f64'),
    compare.replace('compare_grammar_attribute_integer_keys', 'foreign_comparator'),
    range.replace('units: &[f64]', 'units: &mut [f64]'),
    range.replace('read_grammar_attribute_integer_key', 'foreign_range'),
    'pub fn read_grammar_attribute_integer_key(value: f64) -> f64 { if value < 1f64 || value > 10f64 { -1f64 } else { value } }',
  ]) assert.deepEqual(readGrammarAttributeOrderNativeStyles(source), {source, actions: []});
  const literal = '\npub fn keep_literal() -> String { String::from(r#"' + compare + '"#) }';
  const duplicate = readGrammarAttributeOrderNativeStyles(raw + literal);
  assert.equal(duplicate.actions.length, 1, 'opaque copies cannot share a complete replacement context');
  assert.ok(duplicate.source.endsWith(literal));
  assert.equal(readSourceTextRustBlock(duplicate.source, 'fn', 'compare_grammar_attribute_integer_keys'), compare);
});
