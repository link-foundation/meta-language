import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {CRITICAL_CONVERSATION_REQUIREMENTS} from '../scripts/issue-195-conversation-requirements.mjs';
import {readProjectData} from '../scripts/project-data.mjs';
import {recordIssue195Observations} from './support/issue-195-observations.js';

const trackingId='I195-GRAMMAR-INFERENCE-SEPARATE-ISSUE';
const issuePath='parity/sources/grammar-inference-issue-224.txt';
const requestPath='parity/sources/dynamic-knowledge-and-grammar-inference-2026-10-12.txt';
const issueDigest='74ca63f308a8b2c692a340f74d8a3bf4592f33575d69feb11da6ce1f3356d933';
const implementationIds=[
  'EXISTING-WORK-AUDIT','RESEARCH-SELECTION','INDEPENDENT-EVALUATION','COMPARATIVE-BENCHMARKS',
  'CI-RESOURCE-POLICY','PUBLIC-API-CLI','NATIVE-OUTPUT','GENERIC-UNION','FORMAL-CONCEPT-INTEGRATION',
  'DETERMINISTIC-REFINEMENT','SOURCE-FREE-SNAPSHOTS','DYNAMIC-TRANSLATION',
].map(suffix=>'I195-GRAMMAR-INFERENCE-'+suffix);

function assertTrackedInferenceScope(body,registration,requirements,vision) {
  assert.equal(Buffer.byteLength(body),5722);
  assert.equal(createHash('sha256').update(body).digest('hex'),issueDigest);
  assert.deepEqual({key:registration.key,id:registration.id,kind:registration.kind,url:registration.url,
    createdAt:registration.createdAt,updatedAt:registration.updatedAt,bytes:registration.bytes,sha256:registration.sha256},
  {key:'grammarInferenceIssue',id:224,kind:'issue',url:'https://github.com/link-foundation/meta-language/issues/224',
    createdAt:'2026-10-12T05:03:01Z',updatedAt:'2026-10-12T05:03:01Z',bytes:5722,sha256:issueDigest});
  for(const revision of ['2607.08959v4','2601.12385v1','2509.26616v2','2507.13117v1'])
    assert.ok(body.includes('https://arxiv.org/abs/'+revision),'each recorded primary research revision remains pinned');
  const acceptance=body.split('\n').filter(line=>line.startsWith('- [ ] '));
  const prefixes=[
    'Provide library and CLI discovery APIs in JavaScript and Rust.',
    'Produce executable native Links Notation grammars, lossless CST structure,',
    'Discover lexical classes, recursive structure and semantic relations generically.',
    'Copy discovered concepts into the single global concept space.',
    'Support deterministic seeded runs, counterexample-guided refinement,',
    'Demonstrate ordinary parsing and Rust/JavaScript translation',
    'Run holdout tests, adversarial rejected examples and real project corpora',
    'Benchmark existing inference and selected recent methods',
    'Use JavaScript for development and targeted local validation.',
    'Add atomic critical requirements, public API parity mappings and executed observations',
  ];
  assert.equal(acceptance.length,10);
  for(let at=0;at<prefixes.length;at++)assert.ok(acceptance[at].startsWith('- [ ] '+prefixes[at]));
  assert.ok(body.includes('alongside issue #199 and PR #201'));
  assert.ok(body.includes('docs/vision.md'));
  assert.ok(vision.includes('https://github.com/link-foundation/meta-language/issues/224'));
  const implementations=requirements.filter(row=>row.sourceKey==='grammarInferenceIssue');
  assert.deepEqual(implementations.map(row=>row.id),implementationIds);
  for(const row of implementations){
    assert.notEqual(row.id,trackingId);
    assert.ok(row.assertions.length>0);
    assert.equal(row.fixture,issuePath,'all implementation obligations retain their complete issue source');
  }
  const tracking=requirements.find(row=>row.id===trackingId);assert.ok(tracking);
  assert.match(tracking.expectedBehavior,/Creating the issue does not satisfy inference implementation/u);
  assert.equal(tracking.sourceKey,'dynamicKnowledgeRequest');
  assert.equal(tracking.fixture,requestPath);
  assert.deepEqual(tracking.additionalFixtures,[{id:'conversation-contract:grammarInferenceIssue',path:issuePath}]);
  assert.ok(tracking.tooling.includes('js/tests/issue-195-evidence-inference-tracking.test.js'));
  assert.ok(body.includes('These are methods to compare, not evidence that any reported result already holds in meta-language.'));
  assert.ok(body.includes('no competing architecture document, declaration-only completion or issue-closing directive until all required evidence is delivered.'));
}

test('separate inference tracking retains the actual issue, all research and every independent implementation obligation',context=>{
  const request=readFileSync(new URL('../../'+requestPath,import.meta.url));
  assert.equal(createHash('sha256').update(request).digest('hex'),
    '2d18d9d8ca59821082208375584d793fb3220a963bb46d9ef914557870b27612');
  const body=readFileSync(new URL('../../'+issuePath,import.meta.url),'utf8');
  const register=readProjectData(new URL('../../parity/issue-195-sources.lino',import.meta.url));
  const registration=register.requirementSources.find(source=>source.key==='grammarInferenceIssue');assert.ok(registration);
  const vision=readFileSync(new URL('../../docs/vision.md',import.meta.url),'utf8');
  assertTrackedInferenceScope(body,registration,CRITICAL_CONVERSATION_REQUIREMENTS,vision);
  assert.throws(()=>assertTrackedInferenceScope(body.replace('- [ ] Provide','- [x] Provide'),registration,CRITICAL_CONVERSATION_REQUIREMENTS,vision));
  assert.throws(()=>assertTrackedInferenceScope(body,{...registration,id:225},CRITICAL_CONVERSATION_REQUIREMENTS,vision));
  assert.throws(()=>assertTrackedInferenceScope(body,registration,CRITICAL_CONVERSATION_REQUIREMENTS.filter(row=>row.id!==implementationIds[0]),vision));
  assert.throws(()=>assertTrackedInferenceScope(body,registration,CRITICAL_CONVERSATION_REQUIREMENTS,vision.replaceAll('/issues/224','/issues/225')));
  for(const [fixtureId,fixtureFile]of [
    ['conversation-contract:dynamicKnowledgeRequest',requestPath],
    ['conversation-contract:grammarInferenceIssue',issuePath],
  ])recordIssue195Observations({requirementId:trackingId,runtime:'tooling',suffix:'behavior',fixtureId,fixtureFile,
    assertions:['separateInferenceIssueCreated','modernResearchAndAcceptanceRecorded','issueDoesNotSubstituteImplementation'],
    testName:context.name});
});
