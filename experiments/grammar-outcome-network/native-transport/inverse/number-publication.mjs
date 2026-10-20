// A retained-log contract. Authentic CI origin and compiler units are separate.
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {readStructuredDataLino} from '../../../../js/src/structured-data-links.js';
import {readStructuredDataRecords} from '../../../../js/src/structured-data-records.js';
import {verifyHarnessExecution,readHarnessOutcomes} from '../../../../js/scripts/rust-test-shards.mjs';

const root=new URL('../../../../',import.meta.url);
const marker='meta-language original native number controls: ';
const testName='grammar_outcome_original_numbers::original_borrowed_number_predicates_payloads_and_wire_positions_match_every_independent_control';
const pin=value=>({bytes:Buffer.byteLength(value),sha256:createHash('sha256').update(value).digest('hex')});
const sources={test:'rust/tests/unit/grammar_outcome_original_numbers.rs',model:'rust/tests/unit/native_json_number_observation_model.rs',
  visitors:'rust/tests/unit/native_json_number_observation_visitors.rs',publisher:'rust/tests/unit/native_json_number_publication.rs',
  fixture:'parity/fixtures/original-native-number-observations.lino',lock:'rust/Cargo.lock'};

/** Join original primitive facts to complete successful physical execution logs. */
export function readNativeNumberControlPublication(standardError,standardOutput,receiptSource,identitySource) {
  for(const source of [standardError,standardOutput,receiptSource,identitySource])assert.ok(typeof source==='string'&&source.isWellFormed()&&Buffer.byteLength(source)<=16*1024*1024,'bounded complete UTF-8 execution text required');
  const identity=readStructuredDataLino(identitySource,{nodeLimit:10000,byteLimit:131072,depthLimit:32});
  assert.deepEqual(Object.keys(identity).sort(),['attempt','commit','platform','profile','run','suite','tree']);
  for(const key of ['commit','tree'])assert.match(identity[key],/^[0-9a-f]{40}$/u);
  for(const key of ['run','attempt'])assert.match(identity[key],/^[1-9][0-9]{0,23}$/u);
  assert.equal(identity.profile,'evidence');assert.equal(identity.suite,'all');assert.equal(identity.platform,'ubuntu-latest');
  const receipt=readStructuredDataLino(receiptSource,{nodeLimit:100000,byteLimit:16*1024*1024,depthLimit:64});
  assert.equal(receipt.schemaVersion,1);assert.equal(receipt.auxiliary,undefined);assert.match(receipt.binary,/^unit-[0-9a-f]+$/u);
  assert.deepEqual(receipt.identity,identity);assert.deepEqual(receipt.arguments,['--nocapture']);verifyHarnessExecution(receipt);
  assert.equal(receipt.stderrDigest,pin(standardError).sha256);assert.equal(receipt.stdoutDigest,pin(standardOutput).sha256);
  assert.deepEqual(readHarnessOutcomes(standardOutput,receipt.selected),receipt.outcomes,'the complete actual standard output must supply every recorded native outcome');
  assert.equal(receipt.selected.filter(row=>row.name===testName&&row.kind==='test').length,1);
  assert.equal(receipt.outcomes.filter(row=>row.name===testName&&row.outcome==='ok').length,1);
  const frames=standardError.split(/\r?\n/u).filter(line=>line.includes(marker));assert.equal(frames.length,1,'one complete original number publication required');
  assert.ok(frames[0].startsWith(marker),'an original observation frame must occupy its own diagnostic line');
  const records=readStructuredDataRecords(frames[0].slice(marker.length)+'\n',{nodeLimit:10000,byteLimit:131072,depthLimit:32});
  assert.equal(records.length,1);const report=records[0];
  assert.deepEqual(Object.keys(report).sort(),['attempt','commit','observations','profile','run','schemaVersion','sources','targetUsizeBits','testName']);
  assert.equal(report.schemaVersion,1);assert.equal(report.profile,'original native number controls');assert.equal(report.testName,testName);
  for(const key of ['commit','run','attempt'])assert.equal(report[key],identity[key]);
  assert.ok([32,64].includes(report.targetUsizeBits),'uncovered actual native pointer width');
  assert.deepEqual(Object.keys(report.sources).sort(),Object.keys(sources).sort());
  for(const[key,file]of Object.entries(sources))assert.deepEqual(report.sources[key],pin(readFileSync(new URL(file,root))),file);
  const fixture=readStructuredDataLino(readFileSync(new URL(sources.fixture,root),'utf8'));
  assert.deepEqual(report.observations,fixture.observations,'all independent original number predicates payloads and wire occurrences must match');
  return{schemaVersion:1,identity,sourcePins:report.sources,targetUsizeBits:report.targetUsizeBits,observations:report.observations,
    execution:{binary:receipt.binary,shard:receipt.shard,count:receipt.count,testName,receipt:pin(receiptSource),standardError:pin(standardError),standardOutput:pin(standardOutput)},
    boundary:'Complete supplied retained logs and successful physical execution receipt agree at the exact recorded commit and current sources. This contract does not authenticate caller-supplied text as live CI, select actual compiler feature units, reconstruct the complete native model or establish public CST ownership. The existing complete physical union and CI origin gates remain required.'};
}
