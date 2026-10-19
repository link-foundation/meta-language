import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {runInNewContext} from 'node:vm';
import test from 'node:test';
import {DecoratorSet} from '../src/decorators.js';
import {readProjectData,serializeProjectData} from '../scripts/project-data.mjs';
import {readStructuredDataLino} from '../src/structured-data-links.js';
import {tokenize} from '../src/translation/lexer.js';
import {restoreSourceTypeBindingHistory} from './support/source-type-binding-name-history.js';
import {checkOwnedOutcomeSourceTypes} from '../../experiments/grammar-outcome-network/native-transport/inverse/source-type-rules.mjs';

test('actual similar binding diagnostics retain complete source names inputs fixtures and the original executable oracle',()=>{
  const history=readProjectData(new URL('../../parity/naming/source-type-binding-name-source-history.lino',import.meta.url));
  assert.equal(history.capture.run,39309745847);assert.equal(history.capture.job,118269484456);
  assert.equal(history.capture.testedCommit,'8e153ff84594dd43513f322dad04e2bb4255a863');
  assert.equal(history.capture.log.sha256,'5766ecf8f699a11503c8d4a449bd873ca3a9ce3834493e7081eafe213d90a857');
  assert.deepEqual(history.capture.observedVersionRange,{minimumInclusive:'1.99.1',maximumExclusive:'1.99.2'});
  assert.ok(history.capture.diagnostics.some(line=>line.includes('mut type_:')));
  assert.ok(history.capture.diagnostics.some(line=>line.includes('let type_ =')));
  for(const source of history.sources){
    assert.equal(Buffer.byteLength(source.sourceText),source.bytes);
    assert.equal(createHash('sha256').update(source.sourceText).digest('hex'),source.sha256);
  }
  const original=history.sources.find(row=>row.file.endsWith('/source-type-rules.mjs'));
  const current=readFileSync(new URL('../../'+original.file,import.meta.url),'utf8');
  assert.equal(current,DecoratorSet.fromLino(history.transformation.recordedDecorator).decorate('importer',{format:'JavaScript',source:original.sourceText}).source);
  assert.equal(current.replaceAll(history.transformation.sourceNewName,history.transformation.sourceOldName),original.sourceText);
  assert.equal(history.transformation.sites.length,8);
  const fixtureFile='parity/fixtures/grammar-outcome-source-type-rules.lino';
  const fixture=readProjectData(new URL('../../'+fixtureFile,import.meta.url));
  const prior=history.sources.find(row=>row.file===fixtureFile),inverse=structuredClone(fixture);
  inverse.sourceSha256=createHash('sha256').update(original.sourceText).digest('hex');
  assert.deepEqual(inverse,readStructuredDataLino(prior.sourceText));
  assert.equal(serializeProjectData(inverse,fixtureFile),prior.sourceText,'every original manual shape and original parser projection remains byte exact');
  const oracle=runInNewContext(original.sourceText.replace('export function','function')+';checkOwnedOutcomeSourceTypes;');
  assert.equal(fixture.cases.length,33);assert.equal(fixture.projections.length,14);
  const schema=fixture.schema;
  for(const row of fixture.cases){
    const inputs=[row.types,row.selectors,row.offsets,row.children,schema.modes,schema.patterns,schema.patternOffsets,schema.patternTypes,schema.variantOwners];
    assert.equal(oracle(...inputs),row.expected,row.name);
    assert.equal(checkOwnedOutcomeSourceTypes(...inputs),row.expected,row.name);
  }
});

test('the complete native binding inverse preserves every preceding spelling assertion and refuses unrelated changes',()=>{
  const history=readProjectData(new URL('../../parity/naming/source-type-binding-name-source-history.lino',import.meta.url));
  const original=history.sources.find(row=>row.file.endsWith('/part_1.rs'));
  const current=readFileSync(new URL('../../'+original.file,import.meta.url),'utf8');
  assert.equal(restoreSourceTypeBindingHistory(current),original.sourceText);
  const tokens=tokenize(current,'Rust').tokens;
  assert.equal(tokens.some(token=>token.kind==='identifier'&&token.value===history.transformation.nativeOldName),false);
  assert.ok(tokens.some(token=>token.kind==='identifier'&&token.value===history.transformation.nativeNewName));
  assert.throws(()=>restoreSourceTypeBindingHistory(current.replace('-3f64','-99f64')));
  assert.throws(()=>restoreSourceTypeBindingHistory(current.replace('source_type_index','captured_index')));
  const previous=history.sources.find(row=>row.file==='js/tests/translation-constant-option-history.test.js');
  const before="const current=readFileSync(new URL('../../'+old.file,import.meta.url),'utf8');";
  const after="const current=restoreSourceTypeBindingHistory(readFileSync(new URL('../../'+old.file,import.meta.url),'utf8'));";
  assert.equal(readFileSync(new URL('../../'+previous.file,import.meta.url),'utf8'),previous.sourceText.replace(before,after)+"\nimport {restoreSourceTypeBindingHistory} from './support/source-type-binding-name-history.js';\n");
});
