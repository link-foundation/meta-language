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
import {recordIssue195Observations} from './issue-195-observations.js';
const root=new URL('../../../',import.meta.url);
const digest=value=>createHash('sha256').update(value).digest('hex');
export const MODERN_ARRAY_REQUIREMENTS=Object.freeze([
 ['I195-PARITY-MODERN-JAVASCRIPT-ARRAY-BEHAVIOR',['originalArrayAndStringInputsRetained','utf16StringLengthParityExecuted','observableOutOfBoundsParityExecuted']],
 ['I195-PARITY-OBSOLETE-NATIVE-EXPECTATIONS',['obsoleteExpectationConflictRecorded','correctedExpectationsFollowModernJavaScript','originalCoverageAndGenuineRefusalsRetained']],
]);
export function validateModernArrayFixture(fixture){
 assert.equal(digest(readFileSync(new URL(MODERN_ARRAY_CONTRACT_FILE,root))),MODERN_ARRAY_CONTRACT_SHA256);
 assert.deepEqual(fixture,buildModernArrayParityFixture({historicalSource:fixture.historicalNative.source}));
 assert.deepEqual(fixture.cases.map(row=>row.id),MODERN_ARRAY_CASE_IDS);
 assert.equal(fixture.cases.filter(row=>row.origin==='original').length,9);
 assert.equal(fixture.cases.filter(row=>row.expected.outcome==='admitted').length,6);
 assert.equal(fixture.cases.filter(row=>row.expected.outcome==='refused').length,5);
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
  const checkedJavaScript=emitJavaScript(checkProgram(parseJavaScript(row.source))).text;
  const checkedJavaScriptStdout=execute(checkedJavaScript);assert.equal(checkedJavaScriptStdout,row.expected.stdout,row.id+': checked');
  for(const {target,descriptor}of targets){
   assert.equal(descriptor.diagnostic,null,row.id+': '+target);assert.equal(descriptor.contract.support,'semantic-translation');
   assert.equal(descriptor.contract.assumptions.includes(MODERN_ARRAY_IN_BOUNDS),false,row.id+': '+target);
  }
  admitted++;cases.push({kind:'case',id:row.id,sourceSha256:row.sourceSha256,originalStdout,checkedJavaScript,checkedJavaScriptStdout,targets});
 }
 assert.equal(admitted,6);assert.equal(refused,5);
 assert.equal(cases.find(row=>row.id==='original-string-length').originalStdout,'2\n');
 assert.equal(cases.find(row=>row.id==='supplementary-string-length').originalStdout,'4\n');
 assert.equal(cases.find(row=>row.id==='observable-array-absence').originalStdout,'9\n');
 // Execute tampering controls before recording any assertion, without changing premises.
 for(const change of [value=>{value.cases.pop();},value=>{value.cases[0].source+=' ';},value=>{value.cases[0].expected.stdout='wrong';},
  value=>{value.historicalNative.source+=' ';},value=>{value.contract.sha256='0'.repeat(64);}]){
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
