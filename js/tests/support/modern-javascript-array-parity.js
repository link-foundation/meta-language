// Execute the finite retained corpus; metadata alone cannot produce observations.
import assert from 'node:assert/strict';
import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {readProjectData} from '../../scripts/project-data.mjs';
import {buildModernArrayParityFixture,MODERN_ARRAY_CASE_IDS,MODERN_ARRAY_FIXTURE_FILE,MODERN_ARRAY_FIXTURE_ID,
 MODERN_ARRAY_CONTRACT_FILE,MODERN_ARRAY_CONTRACT_ID,MODERN_ARRAY_CONTRACT_SHA256,MODERN_ARRAY_IN_BOUNDS} from '../../scripts/generate-modern-javascript-array-parity-fixtures.mjs';
import {parseJavaScript} from '../../src/translation/javascript.js';
import {checkProgram} from '../../src/translation/check.js';
import {emitJavaScript} from '../../src/translation/emit-javascript.js';
import {translateProgram} from '../../src/program-translation.js';
import {structuredDataRecordToLino} from '../../src/structured-data-records.js';
import {selectVersionedAssertion} from '../../scripts/assertion-version-ranges.mjs';
import {emitRust} from '../../src/translation/emit-rust.js';
import {encodeTypedProgramLinks,decodeTypedProgramLinks} from '../../src/typed-program-links.js';
import {recheckLinkedTypedProgram} from '../../src/typed-program-checked-projection.js';
import {LinkNetwork} from '../../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../../src/link-network-binary.js';
import {recordIssue195Observations} from './issue-195-observations.js';
const root=new URL('../../../',import.meta.url);
const digest=value=>createHash('sha256').update(value).digest('hex');
export const MODERN_ARRAY_REQUIREMENTS=Object.freeze([
 ['I195-PARITY-MODERN-JAVASCRIPT-ARRAY-BEHAVIOR',['originalArrayAndStringInputsRetained','utf16StringLengthParityExecuted','observableOutOfBoundsParityExecuted']],
 ['I195-PARITY-OBSOLETE-NATIVE-EXPECTATIONS',['obsoleteExpectationConflictRecorded','correctedExpectationsFollowModernJavaScript','originalCoverageAndGenuineRefusalsRetained']],
]);
export function assertModernArrayIdentityTranslation(source,previousMessage){
 const active=JSON.parse(readFileSync(new URL('js/package.json',root),'utf8')).version;
 const selected=selectVersionedAssertion({id:'retained original array owner identity',subject:'meta-language npm',comparison:{kind:'release',precision:3},expectations:[{id:'source-qualified reference implementation',minimum:'1.0.0',maximum:'1.0.1',includeMinimum:true,includeMaximum:false,expected:true,evidence:'reference policy source31c61031a2b2bc4c856f85687f8195bb60f51edc823344711962966df39aae1e; original independent Node owner identity'}]},active);
 assert.equal(selected.expected,true);
 const[major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; >=22.15 <23 || >=24 <25 required');
 const policy=readProjectData(new URL('js/src/translation/data/reference-identity-rules.lino',root));
 assert.equal(policy.model,'same-domain structural reference identity equality 1');
 assert.equal(policy.sourceClosure.sourceSha256,'31c61031a2b2bc4c856f85687f8195bb60f51edc823344711962966df39aae1e');
 assert.equal(digest(readFileSync(new URL(policy.sourcePath,root))),policy.sourceClosure.sourceSha256);
 assert.equal(execute(source),'true\n','independent original owner identity');
 assert.throws(()=>checkProgram(parseJavaScript(source,{referenceSemantics:false})),error=>error.message===previousMessage,'the complete original value-mode refusal remains independent');
 const descriptor=translateProgram(source,'JavaScript','Rust');assert.equal(descriptor.diagnostic,null);assert.equal(descriptor.contract.support,'semantic-translation');
 const checked=checkProgram(parseJavaScript(source)),saved=encodeTypedProgramLinks(checked),originalRust=emitRust(checked).text;
 for(const program of[checked,...['text','binary'].map(codec=>recheckLinkedTypedProgram(decodeTypedProgramLinks(codec==='text'?LinkNetwork.fromLino(saved.network.toLino()):decodeLinkNetworkBinary(encodeLinkNetworkBinary(saved.network)),saved.root),{wholeProgram:true}))]){
  assert.equal(execute(emitJavaScript(program).text),'true\n');assert.equal(emitRust(program).text,originalRust);
 }
}
export function validateModernArrayFixture(fixture){
 assert.equal(digest(readFileSync(new URL(MODERN_ARRAY_CONTRACT_FILE,root))),MODERN_ARRAY_CONTRACT_SHA256);
 assert.deepEqual(fixture,buildModernArrayParityFixture({historicalSource:fixture.historicalNative.source}));
 assert.deepEqual(fixture.cases.map(row=>row.id),MODERN_ARRAY_CASE_IDS);
 assert.equal(fixture.cases.filter(row=>row.origin==='original').length,9);
 assert.equal(fixture.cases.filter(row=>row.expected.outcome==='admitted').length,7);
 assert.equal(fixture.cases.filter(row=>row.expected.outcome==='refused').length,4);
 return fixture;
}
function execute(source){
 return execFileSync(process.execPath,['--input-type=module','-e',source],{encoding:'utf8',timeout:10000,maxBuffer:1024*1024});
}
export function runModernArrayParityAssertions({fixture=readProjectData(new URL(MODERN_ARRAY_FIXTURE_FILE,root)),record=false,testName='retained modern JavaScript array contract executes every original input'}={}){
 validateModernArrayFixture(fixture);
 const cases=[];let admitted=0,refused=0;
 for(const row of fixture.cases){
  assert.equal(digest(row.source),row.sourceSha256);
  const targets=row.targets.map(target=>({target,descriptor:translateProgram(row.source,'JavaScript',target)}));
  if(row.expected.outcome==='refused'){
   assert.equal(targets.length,1);const diagnostic=targets[0].descriptor.diagnostic;assert.ok(diagnostic);
   if(row.expected.match==='exact')assert.equal(diagnostic.message,row.expected.diagnostic,row.id);
   else assert.ok(diagnostic.message.startsWith(row.expected.diagnostic),row.id+': '+diagnostic.message);
   refused++;cases.push({kind:'case',id:row.id,sourceSha256:row.sourceSha256,targets});continue;
  }
  const originalStdout=execute(row.source);assert.equal(originalStdout,row.expected.stdout,row.id+': original');
  if(row.id==='original-array-identity-refusal')assertModernArrayIdentityTranslation(row.source,fixture.identityCorrection.previous.row.expected.diagnostic);
  const checkedJavaScript=emitJavaScript(checkProgram(parseJavaScript(row.source))).text;
  const checkedJavaScriptStdout=execute(checkedJavaScript);assert.equal(checkedJavaScriptStdout,row.expected.stdout,row.id+': checked');
  for(const {target,descriptor}of targets){
   assert.equal(descriptor.diagnostic,null,row.id+': '+target);assert.equal(descriptor.contract.support,'semantic-translation');
   assert.equal(descriptor.contract.assumptions.includes(MODERN_ARRAY_IN_BOUNDS),false,row.id+': '+target);
  }
  admitted++;cases.push({kind:'case',id:row.id,sourceSha256:row.sourceSha256,originalStdout,checkedJavaScript,checkedJavaScriptStdout,targets});
 }
 assert.equal(admitted,7);assert.equal(refused,4);
 assert.equal(fixture.identityCorrection.previous.admitted,6);assert.equal(fixture.identityCorrection.previous.refused,5);
 assert.equal(cases.find(row=>row.id==='original-array-identity-refusal').originalStdout,'true\n');
 assert.equal(cases.find(row=>row.id==='original-string-length').originalStdout,'2\n');
 assert.equal(cases.find(row=>row.id==='supplementary-string-length').originalStdout,'4\n');
 assert.equal(cases.find(row=>row.id==='observable-array-absence').originalStdout,'9\n');
 // Execute tampering controls before recording any assertion, without changing premises.
 for(const change of [value=>{value.cases.pop();},value=>{value.cases[0].source+=' ';},value=>{value.cases[0].expected.stdout='wrong';},
  value=>{value.historicalNative.source+=' ';},value=>{value.contract.sha256='0'.repeat(64);},
  value=>{value.identityCorrection.previous.row.expected.diagnostic+=' ';},value=>{value.identityCorrection.previous.fixture.sha256='0'.repeat(64);},value=>{value.identityCorrection.sourceEpochs[0].sha256='0'.repeat(64);}]){
  const altered=structuredClone(fixture);change(altered);assert.throws(()=>validateModernArrayFixture(altered));
 }
 if(record)for(const [fixtureId,fixtureFile]of [[MODERN_ARRAY_CONTRACT_ID,MODERN_ARRAY_CONTRACT_FILE],[MODERN_ARRAY_FIXTURE_ID,MODERN_ARRAY_FIXTURE_FILE]])
  for(const [requirementId,assertions]of MODERN_ARRAY_REQUIREMENTS)recordIssue195Observations({requirementId,suffix:'behavior',fixtureId,fixtureFile,assertions,testName});
 return {metadata:{kind:'metadata',schemaVersion:1,fixtureDigest:digest(readFileSync(new URL(MODERN_ARRAY_FIXTURE_FILE,root))),nodeVersion:process.versions.node,admitted,refused},cases};
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
 assert.deepEqual(process.argv.slice(2),['--probe'],'the CLI is a fresh assertion probe and never records suite observations');
 const result=runModernArrayParityAssertions({record:false});
 process.stdout.write(structuredDataRecordToLino(result.metadata));
 for(const row of result.cases)process.stdout.write(structuredDataRecordToLino(row));
}
