import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import test from 'node:test';
import {buildIssue195Manifest,compareScopeBaseline,evaluateIssue195Acceptance} from '../scripts/issue-195-acceptance-lib.mjs';
import {generateCriticalDeliverySourceHistory} from '../../experiments/critical-delivery/generate-source-history.mjs';
import {verifyGitArtifactCertificate} from '../scripts/git-object-certificate.mjs';
import {readProjectData} from '../scripts/project-data.mjs';
import {recordIssue195Observations} from './support/issue-195-observations.js';
import {encodeStructuredDataLinks} from '../src/structured-data-links.js';
import {decodeStructuredDataLinks} from '../src/structured-data-graph.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';

const root=new URL('../../',import.meta.url),requirementId='I195-CONVERSATION-CRITICAL-DELIVERY';
const fixtureId='conversation-contract:recordConversationRequest',fixtureFile='parity/sources/record-conversation-requirements-2026-10-12.txt';

// Synthetic gate inputs test the gate itself; they are never published as
// observations of real grammars, translation programs or other requirements.
function criticalDeliveryUnitCase(actual) {
  const fixtureIds=[fixtureId,'link-foundation-behavior'],assertions=['first unit observation','second unit observation'];
  const commit='critical-delivery-unit-control';
  const row={id:'I195-CRITICAL-DELIVERY-UNIT-CONTROL',critical:true,fullScopeReleaseBlocker:true,area:'unit gate control',
    source:fixtureFile,expectedBehavior:'Every exact-commit assertion on every declared fixture is required.',
    scope:{language:'unit control',version:'1',edition:'unit',construct:'gate control',aliases:['original alias'],extensions:['.unit']},
    requiredRuntimes:['tooling'],implementationEntryPoints:{tooling:['js/scripts/issue-195-acceptance-lib.mjs']},
    verifications:Object.keys(actual.checkpoints).map(checkpoint=>({testId:'critical-delivery-unit-'+checkpoint,runtime:'tooling',kind:'positive',checkpoint,fixtureIds,assertions,evidenceArtifact:'unit-only.lino'}))};
  const manifest={checkpoints:structuredClone(actual.checkpoints),resultPolicy:structuredClone(actual.resultPolicy),
    fixtureCatalog:Object.fromEntries(fixtureIds.map(id=>{assert.ok(actual.fixtureCatalog[id]);return[id,actual.fixtureCatalog[id]];})),atomicRequirements:[row]};
  const document={schemaVersion:1,issue:195,commit,producer:'synthetic gate unit control only',generatedAt:new Date(0).toISOString(),
    results:row.verifications.map(cell=>({testId:cell.testId,outcome:'passed',kind:'positive',positiveEvidence:true,
      command:'unit gate control',toolchainVersions:{node:process.version},grammarVersions:{unit:'1'},evidenceArtifacts:['unit-only.lino'],assertionsPassed:assertions,
      fixtureDigests:Object.fromEntries(fixtureIds.map(id=>[id,manifest.fixtureCatalog[id].sha256])),
      executionRecords:assertions.flatMap(assertionId=>fixtureIds.map(id=>({testId:cell.testId,assertionId,fixtureId:id,
        fixtureDigest:manifest.fixtureCatalog[id].sha256,runtime:'tooling',commit,outcome:'passed',testName:'synthetic gate unit control only'})))}))};
  return{manifest,document,commit};
}

function checkCriticalDeliveryGateControls(actual) {
  const {manifest,document,commit}=criticalDeliveryUnitCase(actual);
  const evaluate=(documents,checkpoint)=>evaluateIssue195Acceptance(manifest,documents,{checkpoint,commit});
  assert.equal(evaluate([document],'all').passed,true,'the complete synthetic success control must pass before negative controls matter');
  for(const checkpoint of Object.keys(manifest.checkpoints)){
    const success=evaluate([document],checkpoint);assert.equal(success.passed,true);assert.equal(success.summary.verificationCells,1);
    assert.equal(evaluate([],checkpoint).passed,false,'an actual missing result blocks its applicable checkpoint');
    const cases=[
      value=>{value.commit='different exact commit';},
      value=>{value.results.find(row=>row.testId.endsWith(checkpoint)).outcome='unsupported';},
      value=>{value.results.find(row=>row.testId.endsWith(checkpoint)).executionRecords.pop();},
      value=>{value.results.find(row=>row.testId.endsWith(checkpoint)).fixtureDigests[fixtureId]='wrong source digest';},
      value=>{value.results.find(row=>row.testId.endsWith(checkpoint)).executionRecords[0].runtime='javascript';},
      value=>{value.results.find(row=>row.testId.endsWith(checkpoint)).executionRecords[0].commit='wrong original commit';},
      value=>{value.results.find(row=>row.testId.endsWith(checkpoint)).positiveEvidence=false;},
    ];
    for(const mutate of cases){const changed=structuredClone(document);mutate(changed);assert.equal(evaluate([changed],checkpoint).passed,false);}
    const declaration=structuredClone(manifest);declaration.atomicRequirements[0].status='complete';
    assert.equal(evaluateIssue195Acceptance(declaration,[],{checkpoint,commit}).passed,false,'a declared completion cannot replace executed observations');
  }
  assert.deepEqual(compareScopeBaseline(manifest,structuredClone(manifest)),[]);
  for(const mutate of [
    value=>value.atomicRequirements.pop(),value=>{value.atomicRequirements[0].critical=false;},
    value=>{value.atomicRequirements[0].fullScopeReleaseBlocker=false;},value=>{value.atomicRequirements[0].source='foreign source';},
    value=>{value.atomicRequirements[0].expectedBehavior='weaker behavior';},value=>{value.atomicRequirements[0].scope.construct='narrower construct';},
    value=>{value.atomicRequirements[0].scope.aliases=[];},value=>{value.atomicRequirements[0].scope.extensions=[];},
    value=>{value.atomicRequirements[0].requiredRuntimes=[];},value=>{value.atomicRequirements[0].verifications.pop();},
    value=>{value.atomicRequirements[0].verifications[0].assertions=[];},value=>{value.atomicRequirements[0].verifications[0].fixtureIds=[];},
    value=>{value.atomicRequirements[0].verifications[0].checkpoint='release-delivery';},
    value=>{value.checkpoints=Object.fromEntries(Object.entries(value.checkpoints).reverse());},
    value=>{value.resultPolicy.exactCommitRequired=false;},
  ]){const changed=structuredClone(manifest);mutate(changed);assert.ok(compareScopeBaseline(manifest,changed).length>0,'each independent scope or checkpoint reduction must fail');}
}

test('every inherited obligation remains critical while missing exact evidence and weakened scope block full delivery',async context=>{
  const manifest=await buildIssue195Manifest(new URL('../../',import.meta.url).pathname);
  assert.ok(manifest.atomicRequirements.length>=466);
  assert.equal(new Set(manifest.atomicRequirements.map(row=>row.id)).size,manifest.atomicRequirements.length);
  assert.deepEqual(Object.keys(manifest.checkpoints),['pre-merge','post-merge','release-delivery']);
  for(const row of manifest.atomicRequirements){
    assert.equal(row.critical,true,row.id);assert.equal(row.fullScopeReleaseBlocker,true,row.id);
    assert.ok(row.source&&row.expectedBehavior&&row.requiredRuntimes.length&&row.verifications.length,row.id);
    for(const cell of row.verifications){assert.ok(cell.assertions.length&&cell.fixtureIds.length);assert.ok(Object.hasOwn(manifest.checkpoints,cell.checkpoint));}
  }
  const row=manifest.atomicRequirements.find(row=>row.id===requirementId);assert.ok(row);
  assert.ok(row.implementationEntryPoints.tooling.includes('js/tests/issue-195-evidence-critical-delivery.test.js'));
  const request=readFileSync(new URL(fixtureFile,root));assert.equal(request.toString(),'Make sure it is recorded.');
  assert.equal(createHash('sha256').update(request).digest('hex'),manifest.fixtureCatalog[fixtureId].sha256);
  const sources=readProjectData(new URL('parity/issue-195-sources.lino',root));
  assert.ok(sources.requirementSources.some(source=>source.url===fixtureFile&&source.sha256===manifest.fixtureCatalog[fixtureId].sha256));
  checkCriticalDeliveryGateControls(manifest);
  const history=generateCriticalDeliverySourceHistory({check:true}),previous=history.sources[0],current=readFileSync(new URL(previous.file,root),'utf8');
  verifyGitArtifactCertificate(previous.certificate,Buffer.from(previous.sourceText),{revision:history.revision,path:previous.file});
  assert.equal(current.replace(history.addition,''),previous.sourceText);
  assert.throws(()=>verifyGitArtifactCertificate(previous.certificate,Buffer.from(previous.sourceText+'\n'),{revision:history.revision,path:previous.file}));
  const packet=encodeStructuredDataLinks(history),canonical=LinkNetwork.fromLino(packet.network.toLino());
  for(const graph of [canonical,decodeLinkNetworkBinary(encodeLinkNetworkBinary(canonical))]){
    const restored=decodeStructuredDataLinks(graph,packet.root);assert.deepEqual(restored,history);
    assert.equal(current.replace(restored.addition,''),restored.sources[0].sourceText);
  }
  recordIssue195Observations({requirementId,runtime:'tooling',suffix:'behavior',fixtureId,fixtureFile,
    assertions:['allRequirementsCritical','unverifiedRowsBlockFullScopeDelivery','inheritedScopeAndCheckpointsPreserved'],testName:context.name});
});
