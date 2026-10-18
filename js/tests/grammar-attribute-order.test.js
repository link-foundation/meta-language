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
