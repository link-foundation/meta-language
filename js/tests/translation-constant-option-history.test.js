import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';
import {readPortableLiteralOptionStyles} from '../scripts/portable-literal-option-style.mjs';
import {formatProductRuleSource} from '../scripts/product-rule-style-corrections.mjs';

test('actual constant option and empty root CI repairs retain every complete preceding source and assertion',()=>{
  const history=readProjectData(new URL('../../parity/naming/constant-option-source-history.lino',import.meta.url));
  assert.equal(history.sources.length,5);
  assert.equal(history.capture.run,39283021387);assert.equal(history.capture.job,118176565888);
  assert.equal(history.capture.testedCommit,'cd31e285781c2e149f04b667be98e1b88f8d648a');
  assert.deepEqual(history.capture.observedVersionRange,{minimumInclusive:'1.99.1',maximumExclusive:'1.99.2'});
  assert.equal(Buffer.byteLength(history.capture.log.source),187509);
  assert.equal(createHash('sha256').update(history.capture.log.source).digest('hex'),
    'c266479c8afe44e39e26b43306e7ae666b19502b8241ee83774d6e0aa3e9856e');
  assert.equal(history.capture.errors.length,4);
  assert.ok(history.capture.log.source.includes('rustc 1.99.1'));
  for(const old of history.sources){
    assert.equal(Buffer.byteLength(old.sourceText),old.bytes);
    assert.equal(createHash('sha256').update(old.sourceText).digest('hex'),old.sha256);
    const current=restoreSourceTypeBindingHistory(readFileSync(new URL('../../'+old.file,import.meta.url),'utf8'));
    if(old.file.endsWith('/part_1.rs')){
      const result=readPortableLiteralOptionStyles(old.sourceText);
      const expected=old.file.includes('_preorder_')?8:20;
      assert.equal(result.proofs.flatMap(row=>row.literals).length,expected);
      assert.equal(formatProductRuleSource(result.source),current,'the complete actual generated module matches only the recorded spelling correction');
      let inverse=result.source;
      for(const action of result.actions.toReversed())inverse=inverse.replace(action.to,action.from);
      assert.equal(inverse,old.sourceText,'all complete original declarations and context survive the inverse');
      assert.equal(readPortableLiteralOptionStyles(current).source,current);
      assert.equal(current.includes('.unwrap_or_else('),false);
    }else if(old.file.endsWith('/concept_views.rs')){
      const before='assert!(!roots.is_empty());',after='assert_ne!(roots.as_slice(), &[] as &[Value]);';
      assert.equal(old.sourceText.split(before).length,2);assert.equal(current.split(after).length,2);
      assert.equal(current,old.sourceText.replace(before,after),'every original input and other view assertion remains byte exact');
    }else{
      const before="{check});",after="{check,sourceStyles:readPortableLiteralOptionStyles});";
      const added="\nimport {readPortableLiteralOptionStyles} from '../../../../js/scripts/portable-literal-option-style.mjs';\n";
      assert.equal(old.sourceText.split(before).length,2);
      assert.equal(current,old.sourceText.replace(before,after)+added,'the complete generator adds only its recorded style input');
    }
  }
});

import {restoreSourceTypeBindingHistory} from './support/source-type-binding-name-history.js';
