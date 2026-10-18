import assert from 'node:assert/strict';
import test from 'node:test';
import {readFileSync} from 'node:fs';
import {DecoratorSet} from '../src/decorators.js';
import {readProjectData} from '../scripts/project-data.mjs';
import {generateGrammarSelectionCaptureProfile} from '../scripts/generate-grammar-selection-capture-profile.mjs';
import {generateGrammarSelectionMappingFixtures} from '../scripts/generate-grammar-selection-mapping-fixtures.mjs';
import {GRAMMAR_SELECTION_CAPTURE_PROFILE,originalGrammarSelectionCapture} from '../scripts/grammar-selection-capture-profile.mjs';

const root=new URL('../../',import.meta.url);
test('mapping captures retain every original fixture byte through exact reversible corrections',()=>{
  const profile=generateGrammarSelectionCaptureProfile({check:true});
  generateGrammarSelectionMappingFixtures({check:true});
  assert.equal(profile.sources.length,4);
  for(const source of profile.sources){
    const bytes=readFileSync(new URL(source.path,root));
    assert.deepEqual(originalGrammarSelectionCapture(source.path,bytes),Buffer.from(source.source));
    assert.throws(()=>originalGrammarSelectionCapture(source.path,Buffer.concat([bytes,Buffer.from('\n')])));
    const changed=Buffer.from(bytes);changed[0]^=1;
    assert.throws(()=>originalGrammarSelectionCapture(source.path,changed));
  }
});
test('mapping capture profiles reject altered history and incomplete forward or reverse proofs',()=>{
  const profile=readProjectData(new URL(GRAMMAR_SELECTION_CAPTURE_PROFILE,root));
  const source=profile.sources[0],bytes=readFileSync(new URL(source.path,root));
  for(const key of ['forwardDecorators','reverseDecorators']){
    const changed=structuredClone(profile);changed[key]=DecoratorSet.empty().toLino();
    assert.throws(()=>originalGrammarSelectionCapture(source.path,bytes,changed));
  }
  const changed=structuredClone(profile);changed.sources[0].source+='\n';
  assert.throws(()=>originalGrammarSelectionCapture(source.path,bytes,changed));
  const wrongFixture=structuredClone(profile);wrongFixture.historicalFixture.sha256='0'.repeat(64);
  assert.throws(()=>originalGrammarSelectionCapture(source.path,bytes,wrongFixture));
  const duplicate=structuredClone(profile);duplicate.sources.push(duplicate.sources[0]);
  assert.throws(()=>originalGrammarSelectionCapture(source.path,bytes,duplicate));
  const unrelated=Buffer.from('untouched source bytes');
  assert.equal(originalGrammarSelectionCapture('unrelated',unrelated,profile),unrelated);
});
