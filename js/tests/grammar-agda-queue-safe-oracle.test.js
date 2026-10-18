import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {readAgdaScannerPatchHunks,prepareAgdaQueueSafeScanner,AGDA_QUEUE_GROWTH_CONTROL} from '../scripts/agda-queue-safe-oracle.mjs';
import {buildAgdaCurrentOracleFixture,readAgdaCurrentOracleFixture,selectAgdaOracleAssertion,
  selectAgdaReleasedOracleAssertion,AGDA_HISTORICAL_FIXTURE_FILE,
  AGDA_CURRENT_ORACLE_PROFILE_FILE,AGDA_CURRENT_NATIVE_FIXTURE_FILE} from '../scripts/agda-oracle-profiles.mjs';
import {assertionVersionPolicy} from '../scripts/generate-assertion-version-policy.mjs';
import {readStructuredDataLino,structuredDataToLino} from '../src/structured-data-links.js';
import {transformNativeSource} from '../scripts/native-grammar-transforms.mjs';
import {compileGrammar} from '../src/grammar.js';
import {parseGrammarLinks} from '../src/grammar-links.js';
import {nativeGrammarText} from '../src/native-grammar-parser.js';
import {nativeRows} from '../scripts/native-grammar-rows.mjs';

const patch=readFileSync(new URL('../../rust/vendor/tree-sitter-agda-layout/layout-state.patch',import.meta.url),'utf8');
const hunks=readAgdaScannerPatchHunks(patch);
const digest=value=>createHash('sha256').update(value).digest('hex');
// Project-authored assembly of the existing patch contexts, never an upstream
// grammar download, scanner execution or corpus experiment.
const original=hunks.map((hunk,index)=>'/* context '+index+' */\n'+hunk.before).join('\n');
const full=hunks.reduce((source,hunk)=>source.replace(hunk.before,()=>hunk.after),original);
const manifest={originalScannerSha256:digest(original),scannerSha256:digest(full),files:{'layout-state.patch':digest(patch)}};

test('Agda queue-only source derivation preserves the complete original serialization and full patched successor',()=>{
  const queue=prepareAgdaQueueSafeScanner(original,patch,manifest);
  assert.equal(queue,original.replace(hunks[0].before,()=>hunks[0].after));
  assert.equal(queue.replace(hunks[0].after,()=>hunks[0].before),original);
  for(const hunk of hunks.slice(1))assert.ok(queue.includes(hunk.before));
  assert.equal(hunks.slice(1).reduce((source,hunk)=>source.replace(hunk.before,()=>hunk.after),queue),full);
  assert.match(AGDA_QUEUE_GROWTH_CONTROL,/token_queue queue = token_queue_new\(\)/u);
  assert.match(AGDA_QUEUE_GROWTH_CONTROL,/QUEUE_POP\(queue\);\n    QUEUE_PUSH\(queue, DEDENT\);\n    QUEUE_PUSH\(queue, NEWLINE\);/u);
  assert.match(AGDA_QUEUE_GROWTH_CONTROL,/free\(queue.data\)/u);
});

test('Agda queue-only source derivation rejects changed missing duplicated or unrelated source edits',()=>{
  for(const source of [original+'/* unrelated edit */\n',original.replace(hunks[0].before,''),original+hunks[0].before])
    assert.throws(()=>prepareAgdaQueueSafeScanner(source,patch,manifest),/original scanner remains authenticated/u);
  assert.throws(()=>prepareAgdaQueueSafeScanner(original,patch+'\n',manifest),/complete existing patch remains authenticated/u);
  const duplicate=original+hunks[0].before;
  assert.throws(()=>prepareAgdaQueueSafeScanner(duplicate,patch,{...manifest,originalScannerSha256:digest(duplicate)}),/hunk occurs once/u);
  const missing=original.replace(hunks[0].before,'');
  assert.throws(()=>prepareAgdaQueueSafeScanner(missing,patch,{...manifest,originalScannerSha256:digest(missing)}),/hunk occurs once/u);
  const wrong={...manifest,scannerSha256:'0'.repeat(64)};
  assert.throws(()=>prepareAgdaQueueSafeScanner(original,patch,wrong),/full successor/u);
  assert.throws(()=>readAgdaScannerPatchHunks(patch.replace(/^@@ .* @@.*\n[\s\S]*?(?=^@@)/mu,'')),/three original hunks/u);
});

test('Agda current profile retains every historical byte and adds strict clean-row obligations for all four sources',()=>{
  const bytes=readFileSync(new URL('../../'+AGDA_HISTORICAL_FIXTURE_FILE,import.meta.url));
  const original=readStructuredDataLino(bytes.toString('utf8'));
  const nativeBytes=readFileSync(new URL('../../'+AGDA_CURRENT_NATIVE_FIXTURE_FILE,import.meta.url));
  const native=readStructuredDataLino(nativeBytes.toString('utf8'));
  const current=readAgdaCurrentOracleFixture();
  assert.deepEqual(readStructuredDataLino(readFileSync(new URL('../../'+AGDA_CURRENT_ORACLE_PROFILE_FILE,import.meta.url),'utf8')),current);
  assert.deepEqual(current.matches.slice(0,original.matches.length),original.matches);
  assert.deepEqual(current.matches.slice(original.matches.length),native.divergences.map(({source,rows})=>({source,rows})));
  assert.deepEqual(native.matches,original.matches);
  assert.deepEqual(current.rejections,native.rejections);
  assert.deepEqual(current.oracleProfile.nativeFixture,{file:AGDA_CURRENT_NATIVE_FIXTURE_FILE,bytes:nativeBytes.length,sha256:digest(nativeBytes)});
  assert.deepEqual(current.oracleProfile.historicalRejections,original.rejections);
  assert.deepEqual(current.divergences,[]);
  assert.deepEqual(current.oracleProfile.historicalDivergences,original.divergences);
  assert.match(current.oracleProfile.expectationBasis,/Authored current obligations/u);
  const inverse=structuredClone(current);
  inverse.matches.splice(original.matches.length);
  inverse.divergences=inverse.oracleProfile.historicalDivergences;
  inverse.rejections=inverse.oracleProfile.historicalRejections;
  delete inverse.oracleProfile;
  assert.deepEqual(inverse,original);
  assert.deepEqual(Buffer.from(structuredDataToLino(inverse)),bytes);
  assert.deepEqual(readFileSync(new URL('../../'+AGDA_HISTORICAL_FIXTURE_FILE,import.meta.url)),bytes);
  assert.throws(()=>buildAgdaCurrentOracleFixture(Buffer.concat([bytes,Buffer.from('\n')]),current.oracleProfile,nativeBytes),/historical fixture/u);
  const changed=Buffer.from(bytes);changed[0]^=1;
  assert.throws(()=>buildAgdaCurrentOracleFixture(changed,current.oracleProfile,nativeBytes),/historical fixture/u);
});

test('Agda oracle assertions select only exact registered scanner profiles and reject future versions or altered source pins',()=>{
  const policy=assertionVersionPolicy();
  const registry={policy,sha256:'fixture registry'};
  const current=selectAgdaOracleAssertion({registry});
  assert.equal(current.activeVersion,'1.3.3');
  assert.equal(current.minimum,current.maximum);
  assert.equal(current.includeMinimum,true);assert.equal(current.includeMaximum,true);
  assert.equal(selectAgdaReleasedOracleAssertion({registry}).expected.profile,'released-wasm');
  for(const mutate of [
    row=>{row.expectations[0].expected.scannerSha256='0'.repeat(64);},
    row=>{row.expectations[0].expected.patchSha256='0'.repeat(64);},
    row=>{row.expectations[0].minimum='1.3.4';row.expectations[0].maximum='1.3.4';},
  ]){
    const changed=structuredClone(policy);mutate(changed.assertions.find(row=>row.id==='agda-corrected-layout-oracle'));
    assert.throws(()=>selectAgdaOracleAssertion({registry:{policy:changed,sha256:'changed'}}));
  }
  for(const mutate of [
    row=>{row.expected.wasmSha256='0'.repeat(64);},
    row=>{row.minimum='1.3.4';row.maximum='1.3.4';},
  ]){
    const changed=structuredClone(policy);mutate(changed.assertions.find(row=>row.id==='agda-released-wasm-oracle').expectations[0]);
    assert.throws(()=>selectAgdaReleasedOracleAssertion({registry:{policy:changed,sha256:'changed'}}));
  }
});

test('Agda optional universe arguments respect declared layout boundaries and the independently captured complete CI tree',()=>{
  const source={name:'optional_boundary',rules:{document:{type:'SEQ',members:[{type:'STRING',value:'Set'},{type:'CHOICE',members:[{type:'SYMBOL',name:'atom'},{type:'BLANK'}]}]},atom:{type:'PATTERN',value:'[a-z]+'}},externals:[{type:'SYMBOL',name:'_newline'},{type:'SYMBOL',name:'_dedent'}]};
  const original=structuredClone(source),decision={family:'prefer-optional-symbol',rule:'document',symbol:'atom',excludedRules:['_newline','_dedent']};
  const transformed=transformNativeSource(source,[decision]);
  assert.deepEqual(transformed.rules.document.members[1],{type:'NATIVE_ORDERED_CHOICE',members:[{type:'NATIVE_PREFIX_EXCLUSION',content:{type:'SYMBOL',name:'atom'},prefixes:{type:'CHOICE',members:[{type:'SYMBOL',name:'_newline'},{type:'SYMBOL',name:'_dedent'}]}},{type:'BLANK'}]});
  assert.deepEqual(source,original);
  assert.deepEqual(transformed.externals,source.externals);
  assert.throws(()=>transformNativeSource(source,[{...decision,excludedRules:['_unknown']}]),/declared rules or external tokens/u);
  const counterfeit=structuredClone(source);counterfeit.externals[0]={type:'STRING',name:'_newline',value:'\n'};
  assert.throws(()=>transformNativeSource(counterfeit,[decision]),/declared rules or external tokens/u);
  const witness=readStructuredDataLino(readFileSync(new URL('../../parity/fixtures/agda-current-oracle-universe-ci.lino',import.meta.url),'utf8'));
  const historical=readStructuredDataLino(readFileSync(new URL('../../'+AGDA_HISTORICAL_FIXTURE_FILE,import.meta.url),'utf8'));
  const current=readAgdaCurrentOracleFixture();
  assert.deepEqual(witness.historicalRows,historical.divergences[0].rows);
  assert.equal(witness.source,historical.divergences[0].source);
  assert.deepEqual(witness.oracleSource,current.oracleProfile.source);
  assert.equal(witness.nativeExecutionObserved,false);
  assert.deepEqual(current.matches[historical.matches.length],{source:witness.source,rows:witness.oracleRows});
  const outcome=compileGrammar(parseGrammarLinks(nativeGrammarText('native-agda'))).parseTree(witness.source);
  assert.equal(outcome.ok,true);assert.deepEqual(outcome.ambiguities,[]);
  assert.deepEqual(nativeRows(outcome.tree,witness.source,current),witness.oracleRows);
  assert.equal(witness.oracleRows.filter(row=>row[2]==='SetN').length,2);
  assert.equal(witness.oracleRows.filter(row=>row[2]==='Set').length,2);
});
