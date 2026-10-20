import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';
import {DecoratorSet} from '../src/decorators.js';
import {readPortableIntegerCharacterSourceView} from '../scripts/portable-integer-character-source-style.mjs';
import {removeFamilyLibraryDivergentElse} from '../scripts/portable-conditional-source-style.mjs';
import {acceptOwnedOutcomeIntegerDecimal} from '../../experiments/grammar-outcome-network/native-transport/inverse/integer-decimal-rules.mjs';
import {acceptOwnedOutcomeIntegerProjection} from '../../experiments/grammar-outcome-network/native-transport/inverse/integer-projection-rules.mjs';
import {readOwnedOutcomeIntegerProjectionSource} from '../../experiments/grammar-outcome-network/native-transport/inverse/generate-integer-projections.mjs';
import {generateRelativeCharacterSelfTranslation} from '../../experiments/grammar-outcome-network/native-transport/inverse/generate-relative-character-self-translation.mjs';
import {selectVersionedAssertion} from '../scripts/assertion-version-ranges.mjs';
import {generateNativeTestLintCorrections} from '../../experiments/grammar-outcome-network/native-transport/inverse/generate-native-test-lint-corrections.mjs';
import {restoreNativeTestLintSource} from '../scripts/native-test-lint-source-history.mjs';
import {verifyGitArtifactCertificate} from '../scripts/git-object-certificate.mjs';

const root=new URL('../../',import.meta.url);
let integerView;
async function integerSourceViewModule() {
  if(!integerView){const closure=readOwnedOutcomeIntegerProjectionSource();integerView={closure,module:await import('data:text/javascript;base64,'+Buffer.from(closure.loweredSource).toString('base64'))};}
  return integerView;
}

test('all original integer source bytes and every reviewed character operand survive recorded source lowering',async()=>{
  const {closure}=await integerSourceViewModule();assert.equal(closure.sourceViews.length,2);
  assert.deepEqual(closure.sourceViews.map(row=>row.edits.length),[4,3]);
  for(const row of closure.sourceViews){
    assert.equal(readFileSync(new URL(row.file,root),'utf8'),row.original.sourceText);
    let current=row.lowered.sourceText;
    for(const edit of row.edits){assert.equal(current.slice(edit.start,edit.start+edit.to.length),edit.to);current=current.slice(0,edit.start)+edit.from+current.slice(edit.start+edit.to.length);}
    assert.equal(current,row.original.sourceText);
    assert.equal(DecoratorSet.fromLino(row.decorators).decorate('emitter',{format:'JavaScript',source:current}).source,row.lowered.sourceText);
    assert.deepEqual(readPortableIntegerCharacterSourceView(row.file,current).record,row);
    for(const altered of [current+'\n',current.replace('charAt(0)','charAt(-1)'),current.replace('index+=1','index-=1')])
      assert.throws(()=>readPortableIntegerCharacterSourceView(row.file,altered),/unchanged/u);
  }
  assert.throws(()=>readPortableIntegerCharacterSourceView('unknown-source',''),/covered/u);
});

test('both complete lowered algorithms retain every original independent BigInt case and all one-unit UTF16 inputs',async()=>{
  const {module:lowered}=await integerSourceViewModule();
  const decimal=readProjectData(new URL('parity/fixtures/grammar-outcome-integer-decimal-rules.lino',root));
  assert.equal(decimal.cases.length,3660);
  for(const row of decimal.cases){
    assert.equal(lowered.acceptOwnedOutcomeIntegerDecimal(row.text,row.signed,row.width),row.expected,JSON.stringify(row));
    assert.equal(acceptOwnedOutcomeIntegerDecimal(row.text,row.signed,row.width),row.expected,JSON.stringify(row));
  }
  const projection=readProjectData(new URL('parity/fixtures/grammar-outcome-integer-projections.lino',root));assert.equal(projection.cases.length,3628);
  for(const row of projection.cases){const projected=Buffer.from(row.projectedBits,'hex').readDoubleBE();
    assert.equal(lowered.acceptOwnedOutcomeIntegerProjection(row.text,row.signed,row.width,projected),row.expected,JSON.stringify(row));
    assert.equal(acceptOwnedOutcomeIntegerProjection(row.text,row.signed,row.width,projected),row.expected,JSON.stringify(row));
  }
  for(let unit=0;unit<=65535;unit++)for(const text of [String.fromCharCode(unit),'1'+String.fromCharCode(unit),'-'+String.fromCharCode(unit),'0'+String.fromCharCode(unit)]){
    for(const signed of [false,true])assert.equal(lowered.acceptOwnedOutcomeIntegerDecimal(text,signed,64),acceptOwnedOutcomeIntegerDecimal(text,signed,64),'every original code unit including unpaired surrogates');
  }
});

test('match-arm divergence is explicit opt-in and every possible normal or guarded arm retains its else',()=>{
  const source='pub fn f(flag: bool, value: Step) -> usize { if flag { match value { Step::First => { return 1; }, Step::Second => { { return 2; } } } } else { 0 } }';
  assert.equal(removeFamilyLibraryDivergentElse(source),source);
  const result=removeFamilyLibraryDivergentElse(source,{divergentMatches:true});
  assert.equal(result,source.replace('else','    '));assert.equal(result.length,source.length);
  for(const changed of [source.replace('return 2;','2'),source.replace('Step::Second =>','Step::Second if flag =>'),
    source.replace('return 2;','break \'outer;'),source.replace('match value','match evaluate()')])
    assert.equal(removeFamilyLibraryDivergentElse(changed,{divergentMatches:true}),changed);
});

test('corrected relative-character decisions select the active bounded source profile and preserve every item',()=>{
  const fixture=generateRelativeCharacterSelfTranslation({check:true});
  assert.deepEqual(fixture.statuses,['carried','translated','carried','translated','carried','carried','carried','carried']);
  assert.deepEqual(selectVersionedAssertion(fixture.versionPolicy,fixture.package.version).expected,fixture.statuses);
  assert.throws(()=>selectVersionedAssertion(fixture.versionPolicy,fixture.versionPolicy.expectations[0].maximum),/uncovered/u);
});

test('every native test lint correction retains complete authenticated original controls and refuses unrelated current edits',()=>{
  const history=generateNativeTestLintCorrections({check:true});assert.equal(history.sources.length,4);
  for(const row of history.sources){
    verifyGitArtifactCertificate(row.certificate,Buffer.from(row.sourceText),{revision:row.commit,path:row.file});
    const current=readFileSync(new URL(row.file,root),'utf8');assert.equal(restoreNativeTestLintSource(row.file,current),row.sourceText);
    assert.throws(()=>restoreNativeTestLintSource(row.file,current+'\n'),/complete actual current/u);
  }
});
