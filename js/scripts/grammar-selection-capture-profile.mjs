// Test-source corrections retain the complete authenticated historical capture.
// The existing shared decorator interpreter performs both directions.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {DecoratorSet} from '../src/decorators.js';
import {readProjectData,readProjectFileBytes} from './project-data.mjs';

export const GRAMMAR_SELECTION_CAPTURE_PROFILE='parity/fixtures/grammar-selection-capture-source-profile.lino';
export const GRAMMAR_SELECTION_HISTORICAL_FIXTURE='parity/fixtures/grammar-selection-json-string.lino';
export const GRAMMAR_SELECTION_HISTORICAL_SHA256='ee8105fd4aa66df19feb1abd20d1012f696abb712cbd339d5e2816f60fcfaba9';
const root=new URL('../../',import.meta.url);
const digest=bytes=>createHash('sha256').update(bytes).digest('hex');
let retainedProfile;
export function originalGrammarSelectionCapture(relative,bytes,profile){
  profile??=retainedProfile??=readProjectData(new URL(GRAMMAR_SELECTION_CAPTURE_PROFILE,root));
  assert.equal(profile.schemaVersion,1);
  assert.equal(profile.historicalFixture.path,GRAMMAR_SELECTION_HISTORICAL_FIXTURE);
  assert.equal(profile.historicalFixture.sha256,GRAMMAR_SELECTION_HISTORICAL_SHA256);
  const fixtureBytes=readProjectFileBytes(new URL(GRAMMAR_SELECTION_HISTORICAL_FIXTURE,root));
  assert.equal(digest(fixtureBytes),GRAMMAR_SELECTION_HISTORICAL_SHA256,'the complete original mapping fixture remains immutable');
  const originals=profile.sources.filter(source=>source.path===relative);
  assert.ok(originals.length<=1,'each corrected source has one complete profile');
  if(originals.length===0)return bytes;
  const original=originals[0];
  const capture=readProjectData(new URL(GRAMMAR_SELECTION_HISTORICAL_FIXTURE,root)).captures.find(source=>source.path===relative);
  assert.ok(capture);
  assert.equal(Buffer.byteLength(original.source),capture.bytes);
  assert.equal(digest(original.source),capture.sha256);
  assert.equal(bytes.length,original.current.bytes,relative+': current source byte count');
  assert.equal(digest(bytes),original.current.sha256,relative+': current source hash');
  const current=new TextDecoder('utf-8',{fatal:true}).decode(bytes);
  assert.deepEqual(DecoratorSet.fromLino(profile.forwardDecorators).decorate('emitter',{capture:relative,source:original.source}),
    {capture:relative,source:current},relative+': complete forward correction');
  assert.deepEqual(DecoratorSet.fromLino(profile.reverseDecorators).decorate('emitter',{capture:relative,source:current}),
    {capture:relative,source:original.source},relative+': complete reverse correction');
  return Buffer.from(original.source);
}
