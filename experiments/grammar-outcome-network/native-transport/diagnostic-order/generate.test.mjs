// Source, value and publication controls; native execution belongs in CI.
import test from 'node:test';
import {restoreCurrentCiRepairSource} from '../../../../js/scripts/current-ci-repair-source-history.mjs';
import {restoreGlobalGrammarContextSource} from '../../../global-grammar-context-reuse/current-source.mjs';
import {readOutcomeInventory} from '../wire/generate.mjs';
import {authorWireSourceViewEdits,authorBridgeSourceViewEdits,authorRunnerPlanSourceViewEdits,authorRunnerOverlaySourceViewEdits,appendWirePublicationEdits} from './publication-edits.mjs';
import {captureOutcomeSourceViews,finalizeOutcomeSourceViews,checkOutcomeSourceViews,applyFrozenOutcomeProducerCorrection} from './source-view.mjs';
import assert from 'node:assert/strict';
import {readFileSync,mkdirSync,mkdtempSync,writeFileSync,rmSync} from 'node:fs';
import {dirname,join} from 'node:path';
import {tmpdir} from 'node:os';
import {fileURLToPath} from 'node:url';
import {captureDiagnosticOrderDecorator,deriveDiagnosticOrderDecorator,
  applyDiagnosticOrderDecorator,readJavaScriptDiagnosticOrders,
  captureDiagnosticDependencyProfile,buildDiagnosticOrderFixtures,
  javascriptPath,nativePath} from './generate.mjs';
import {compileGrammar} from '../../../../js/src/grammar.js';
import {parseGrammarLinks} from '../../../../js/src/grammar-links.js';
import {grammarSelectionFixture} from '../../../../js/tests/support/grammar-selection-cases.js';
import {serializeProjectData,readProjectData} from '../../../../js/scripts/project-data.mjs';
import {readStructuredDataLino} from '../../../../js/src/structured-data-links.js';

const repository=process.env.META_LANGUAGE_DIAGNOSTIC_ORDER_REPOSITORY??fileURLToPath(new URL('../../../../',import.meta.url));
let captured;
const current=()=>captured??=captureDiagnosticOrderDecorator();
const original=path=>current().decorator.inputs.find(input=>input.path===path).source;

test('actual complete CST source correspondence retains all nine JS call contexts and the unmatched native guard',()=>{
  const {decorator,corrected}=current();
  assert.equal(decorator.matches.length,9);assert.equal(decorator.edits.length,2);assert.equal(decorator.unmatchedNative.length,1);
  assert.equal(Object.keys(decorator.baselinePins).length,13);
  assert.equal(decorator.matches.filter(row=>row.changed).length,2);
  for(const match of decorator.matches) {
    assert.ok(match.javascript.owner.span.end>match.javascript.owner.span.start);
    assert.ok(match.javascript.contexts.length>0);
    assert.ok(match.native.owner.span.end>match.native.owner.span.start);
    assert.ok(match.native.contexts.length>0);
    assert.deepEqual([...match.javascript.keys].sort(),[...match.native.keys].sort());
    // Every native value expression and its complete key:value source slice is
    // retained. Only the order of those existing slices may change.
    for(const pair of match.native.pairs)assert.ok(original(nativePath).includes(pair.source));
  }
  const changed=decorator.matches.filter(row=>row.changed);
  assert.deepEqual(changed.map(row=>row.javascript.reason).sort(),['unknownGrammar','unsupportedEmbeddingMapping']);
  assert.deepEqual(changed.find(row=>row.javascript.reason==='unknownGrammar').javascript.keys,['language','start','end','reason']);
  assert.deepEqual(changed.find(row=>row.javascript.reason==='unsupportedEmbeddingMapping').javascript.keys,['language','start','end','reason','mapping']);
  assert.ok(corrected.includes(decorator.unmatchedNative[0].source),'unmatched guard remains byte-exact');
  assert.equal(Buffer.byteLength(corrected),Buffer.byteLength(original(nativePath)));
  const normal=serializeProjectData(decorator,'decorator.lino');
  assert.deepEqual(readStructuredDataLino(normal),decorator,'shared Links policy retains the complete source/context archive');
});

test('one generated decorator supplies separate production and frozen overlay receipts and exact whole-source inverses',()=>{
  const {decorator,corrected}=current();
  for(const role of ['production','frozen CI overlay']) {
    const forward=applyDiagnosticOrderDecorator(original(nativePath),decorator,{role});
    assert.equal(forward.source,corrected);assert.equal(forward.receipt.role,role);assert.equal(forward.receipt.inverseVerified,true);
    assert.deepEqual(forward.receipt.preservedBaselinePins,decorator.baselinePins);
    const inverse=applyDiagnosticOrderDecorator(forward.source,decorator,{role,direction:'inverse'});
    assert.equal(inverse.source,original(nativePath));assert.equal(inverse.receipt.inverseVerified,true);
  }
});

test('whole source, conditional contexts, spread bindings, keys and policy tampering refuse',()=>{
  const {decorator}=current(),javascript=original(javascriptPath),native=original(nativePath);
  for(const changed of [
    javascript.replace("if (!program) {","if (program) {"),
    javascript.replace('const diagnostic =','let diagnostic ='),
    javascript.replace('{ ...diagnostic, reason }','{ reason, ...diagnostic }')
  ]) {assert.notEqual(changed,javascript);assert.throws(()=>deriveDiagnosticOrderDecorator(changed,native),/authenticated/u);}
  assert.throws(()=>deriveDiagnosticOrderDecorator(javascript,native+'\n'),/authenticated/u);
  for(const change of [
    value=>{value.edits[0].after=value.edits[0].before;},
    value=>{value.matches[0].javascript.contexts=[];},
    value=>{value.unmatchedNative=[];},
    value=>{value.baselinePins[nativePath]='0'.repeat(64);},
    value=>{value.inputs[0].source+='\n';}
  ]) {const value=structuredClone(decorator);change(value);assert.throws(()=>applyDiagnosticOrderDecorator(native,value,{role:'production'}));}
  assert.throws(()=>applyDiagnosticOrderDecorator(native,decorator,{role:'unqualified'}));
  assert.throws(()=>applyDiagnosticOrderDecorator(native+'\n',decorator,{role:'production'}));
  assert.throws(()=>applyDiagnosticOrderDecorator(native,decorator,{role:'production',direction:'inverse'}));
});

test('finite CST spread reader refuses changed local mutability, duplicate keys and computed properties',()=>{
  const javascript=original(javascriptPath);
  for(const changed of [
    javascript.replace('const diagnostic =','let diagnostic ='),
    javascript.replace('const diagnostic = { language: candidate.name, start, end: region.end };',
      "const diagnostic = { language: candidate.name, start, end: region.end, reason: 'extra' };"),
    javascript.replace('{ ...diagnostic, reason }',"{ ...diagnostic, ['reason']: reason }"),
    javascript.replace('{ ...diagnostic, reason }','{ ...unknownDiagnostic, reason }')
  ]) {assert.notEqual(changed,javascript);assert.throws(()=>readJavaScriptDiagnosticOrders(changed));}
});

test('actual JavaScript unknown and unsupported candidate diagnostics keep their complete original ordered values',()=>{
  const nativeBefore=readFileSync(repository+'/'+nativePath,'utf8'),javascriptBefore=readFileSync(repository+'/'+javascriptPath,'utf8');
  const approved=current();
  assert.ok(nativeBefore===original(nativePath)||restoreGlobalGrammarContextSource(nativePath,nativeBefore).predecessor===approved.corrected,
    'the actual current source is the exact authenticated original or corrected producer');
  const fixture=grammarSelectionFixture();
  const linked=new Map(fixture.grammars.map(row=>[row.name,parseGrammarLinks(row.listing)]));
  for(const [sourceCase,reason]of [[fixture.unknown,'unknownGrammar'],[fixture.escaped,'unsupportedEmbeddingMapping']]) {
    const outcome=compileGrammar(parseGrammarLinks(sourceCase.listing),{resolveGrammar:name=>linked.get(name)}).parseTree(sourceCase.source);
    assert.equal(outcome.ok,false);assert.equal(outcome.rejection.reason,reason);
    assert.ok(outcome.rejection.diagnostics.length>0);
    const descriptor=current().decorator.matches.filter(row=>row.javascript.reason===reason
      &&row.javascript.keys.includes('language')&&row.changed);
    assert.equal(descriptor.length,1);
    // The retained authored body is repeat0 over balanced braces or one
    // non-brace scalar. For the unchanged single π input its complete regions
    // are empty and the full scalar, so diagnostics retain BOTH occurrences.
    assert.equal(sourceCase.source,'π');
    assert.match(sourceCase.listing,/\(rule body silent \(repeat0 /u);
    assert.deepEqual(outcome.rejection.diagnostics.map(value=>value.end),[0,Buffer.byteLength(sourceCase.source)]);
    const language=reason==='unknownGrammar'?'missing':'first';
    const expected=[0,Buffer.byteLength(sourceCase.source)].map(end=>{
      const values={language,start:0,end,reason,...(reason==='unsupportedEmbeddingMapping'?{mapping:'quoted-escape'}:{})};
      return Object.fromEntries(descriptor[0].javascript.keys.map(key=>[key,values[key]]));
    });
    assert.deepEqual(outcome.rejection.diagnostics,expected,'complete independently expected diagnostic list for both original regions');
    for(const diagnostic of outcome.rejection.diagnostics) {
      assert.deepEqual(Object.keys(diagnostic),descriptor[0].javascript.keys,'actual public producer preserves the derived field order');
      assert.equal(diagnostic.reason,reason);assert.equal(typeof diagnostic.language,'string');
      assert.equal(diagnostic.language,language);assert.equal(diagnostic.start,0);
      if(reason==='unsupportedEmbeddingMapping')assert.equal(diagnostic.mapping,'quoted-escape');
    }
  }
  // Reading and constructing these controls must not change original inputs.
  assert.equal(restoreGlobalGrammarContextSource(javascriptPath,javascriptBefore).predecessor,original(javascriptPath));
  assert.equal(readFileSync(repository+'/'+javascriptPath,'utf8'),javascriptBefore);
  assert.equal(readFileSync(repository+'/'+nativePath,'utf8'),nativeBefore);
});

test('original and current owners are separately authenticated and all selected current CST types/imports remain exact',()=>{
  const {decorator,corrected}=current(),inventory=readOutcomeInventory(repository);
  const scratch=mkdtempSync(join(tmpdir(),'meta-language-diagnostic-view-'));
  try {
    let total=0;
    for(const file of Object.keys(decorator.baselinePins)) {
      const bytes=[javascriptPath,nativePath].includes(file)?Buffer.from(decorator.inputs.find(input=>input.path===file).source):readFileSync(join(repository,file));
      total+=bytes.length;assert.ok(total<=2*1024*1024,'small selected source closure only');
      mkdirSync(dirname(join(scratch,file)),{recursive:true});writeFileSync(join(scratch,file),bytes);
    }
    const baseline=captureOutcomeSourceViews(scratch,decorator);
    assert.equal(baseline.adaptations.length,0);
    assert.equal(readFileSync(join(scratch,nativePath),'utf8'),original(nativePath),
      'the baseline fixture always uses retained original bytes even after production changes');
    const historical={...structuredClone(inventory),sourceViews:finalizeOutcomeSourceViews(baseline,inventory.inputs,inventory.declarations)};
    assert.deepEqual(checkOutcomeSourceViews(historical,decorator),historical.sourceViews);
    writeFileSync(join(scratch,nativePath),corrected);
    assert.deepEqual(captureDiagnosticOrderDecorator(scratch,{archive:decorator}),current(),
      'regeneration after production correction still authenticates the original archive rather than repinning');
    const capture=captureOutcomeSourceViews(scratch,decorator);
    assert.equal(capture.adaptations.length,1);assert.deepEqual(capture.pins,baseline.pins);
    const actual={...structuredClone(inventory),sourceViews:finalizeOutcomeSourceViews(capture,inventory.inputs,inventory.declarations)};
    assert.deepEqual(checkOutcomeSourceViews(actual,decorator),actual.sourceViews);
    const adaptation=actual.sourceViews.adaptations[0];
    assert.equal(adaptation.currentSource,corrected);assert.equal(adaptation.historicalSource,capture.historical.get(nativePath).toString());
    assert.ok(adaptation.currentSourceProof.selectedDeclarations.length>0);
    assert.notEqual(actual.sourceViews.currentPins.find(pin=>pin.file===nativePath).sha256,actual.pins.find(pin=>pin.file===nativePath).sha256);
    for(const change of [
      value=>{value.sourceViews.currentPins.find(pin=>pin.file===nativePath).sha256=value.pins.find(pin=>pin.file===nativePath).sha256;},
      value=>{value.sourceViews.adaptations[0].currentSourceProof.selectedDeclarations=[];},
      value=>{value.sourceViews.adaptations[0].sourceView='actual current bytes';},
      value=>{value.sourceViews.adaptations[0].historicalSource+='\n';},
      value=>{value.inputs.find(input=>input.file===nativePath).imports[0].source+=' changed';},
      value=>{value.declarations.find(declaration=>declaration.file===nativePath).span.start++;},
      value=>{value.declarations.find(declaration=>declaration.file===nativePath).source+=' changed';}
    ]){const value=structuredClone(actual);change(value);assert.throws(()=>checkOutcomeSourceViews(value,decorator));}
    const returned=checkOutcomeSourceViews(actual,decorator);
    returned.adaptations[0].currentSourceProof.selectedDeclarations[0].sha256='0'.repeat(64);
    assert.deepEqual(checkOutcomeSourceViews(actual,decorator),actual.sourceViews,
      'caller-owned returned proofs cannot mutate the private syntax cache');
    assert.throws(()=>checkOutcomeSourceViews({...actual,sourceViews:returned},decorator),
      'a cached syntax tree never authenticates a changed returned proof');
    const overlay=applyFrozenOutcomeProducerCorrection(capture.historical.get(nativePath).toString(),decorator);
    assert.equal(overlay.original,adaptation.historicalSource);assert.equal(overlay.source,corrected);assert.equal(overlay.receipt.role,'frozen CI overlay');
    writeFileSync(join(scratch,nativePath),corrected+'\n');assert.throws(()=>captureOutcomeSourceViews(scratch,decorator));
  } finally {rmSync(scratch,{recursive:true,force:true});}
});

test('all permanent source changes are authored as exact inverse publication additions',()=>{
  const files=[
    ['wire/generate.mjs',authorWireSourceViewEdits],
    ['bridge/generate.mjs',authorBridgeSourceViewEdits],
    ['runner/plan.mjs',authorRunnerPlanSourceViewEdits],
    ['runner/run.mjs',source=>authorRunnerOverlayReceiptSourceViewEdits(source,
      readProjectData(join(repository,'parity/naming/outcome-compiler-profile-source-history.lino')).sources.find(row=>row.file.endsWith('/runner/run.mjs')).sourceText)]
  ];
  for(const [path,author]of files) {
    const file='experiments/grammar-outcome-network/native-transport/'+path;
    const original=restoreCurrentCiRepairSource(readFileSync(join(repository,file),'utf8'),file);
    const result=author(original);assert.notEqual(result.source,result.original);
    let inverse=result.source;
    for(const {from,to,count}of [...result.edits].reverse()) {
      assert.equal(count,1);assert.equal(inverse.split(to).length,2);inverse=inverse.replace(to,from);
    }
    assert.equal(inverse,result.original,'whole previous source is retained exactly');
    // Exercise the installed state before installation too. Reversing the exact
    // source recipe recovers the retained predecessor, so no current module is
    // accidentally passed through the addition a second time.
    const installed=author(result.source);
    assert.equal(installed.alreadyApplied,true);
    assert.equal(installed.source,result.source);
    assert.equal(installed.original,result.original);
    assert.deepEqual(installed.edits,result.edits);
    const replay=author(installed.original);
    assert.equal(replay.alreadyApplied,false);
    assert.equal(replay.source,installed.source);
    if(path.startsWith('wire/')||path.startsWith('bridge/')) {
      const archiveFile='experiments/grammar-outcome-network/native-transport/wire/original-authoring.lino';
      const archive=readStructuredDataLino(restoreCurrentCiRepairSource(readFileSync(join(repository,archiveFile),'utf8'),archiveFile));
      const record=archive.records.find(record=>record.path===path);assert.ok(record,'actual retained normal Links source archive');
      const appended=appendWirePublicationEdits(record,author);
      assert.equal(appended.source,result.source);
      const repeated=appendWirePublicationEdits(appended.record,author);
      assert.deepEqual(repeated,appended,'installed archive preserves one exact suffix and every historical edit');
      let historical=appended.source;
      for(const {from,to,count}of [...appended.record.edits].reverse()) {
        assert.equal(historical.split(to).length-1,count);historical=historical.split(to).join(from);
      }
      assert.equal(historical,record.original,'complete original author plus every old/new publication change has an exact inverse');
    }
  }
});

test('dependency assertions select only the actual exact locked patch and full source profile',()=>{
  const profile=captureDiagnosticDependencyProfile(repository);
  const parts=profile.version.split('.').map(Number);
  assert.deepEqual(profile.coveredRange,{minimumInclusive:profile.version,maximumExclusive:parts[0]+'.'+parts[1]+'.'+(parts[2]+1)});
  assert.ok(profile.declaration.features.includes('preserve_order'));
  assert.deepEqual(profile.requiredFeatures,['preserve_order']);
  for(const owner of [profile.lock,profile.manifest]) {
    const bytes=readFileSync(join(repository,owner.path));
    assert.equal(bytes.subarray(owner.selected.span.start,owner.selected.span.end).toString(),owner.selected.source);
  }
  assert.ok(profile.lock.selected.source.includes('name = "serde_json"\nversion = "'+profile.version+'"'));
  assert.equal(buildDiagnosticOrderFixtures(repository,{archive:current().decorator}).dependencyProfile.version,profile.version);
  const scratch=mkdtempSync(join(tmpdir(),'meta-language-diagnostic-dependency-'));
  try {
    mkdirSync(join(scratch,'rust'));
    writeFileSync(join(scratch,'rust/Cargo.lock'),readFileSync(join(repository,'rust/Cargo.lock')));
    const manifest=readFileSync(join(repository,'rust/Cargo.toml'),'utf8');
    writeFileSync(join(scratch,'rust/Cargo.toml'),manifest.replace('"preserve_order"','"uncovered_order_profile"'));
    assert.throws(()=>captureDiagnosticDependencyProfile(scratch),/uncovered/u);
  } finally {rmSync(scratch,{recursive:true,force:true});}
});

import {authorRunnerOverlayReceiptSourceViewEdits} from './publication-edits.mjs';
