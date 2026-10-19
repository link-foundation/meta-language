// Source coverage for the reviewed instruction compaction; no alternate contract.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {readProjectData} from './project-data.mjs';
import {structuredDataToLino} from '../src/structured-data-links.js';
import {verifyGitArtifactCertificate} from './git-object-certificate.mjs';

export const CONTRIBUTOR_INSTRUCTION_COVERAGE_FILE='parity/fixtures/contributor-instruction-coverage.lino';
const originalRevision='f0ae00412047227ccfb4826c30608841d16ea7a6';
const compactRevision='e0d407019196e5ea72e85c7b10afa09407108066';
const policyHash='8dc1dc22e72f1f0f0c04eb52aaab3ba117ffde1e5848c935d46d5908af729f29';
const requestHash='5d4a7d12bfb3d97937ec7c2ab2a5d60729d27a65e7ca53ec0cd62381402d21d2';
const root=new URL('../../',import.meta.url);
const digest=value=>createHash('sha256').update(value).digest('hex');
const normalize=value=>value.replace(/[\t\r\n ]+/gu,' ').trim();

/** Count explicit ASCII-whitespace words, excluding markup-only tokens. */
export function countContributorInstructionWords(source) {
  assert.equal(typeof source,'string');
  assert.ok(Buffer.byteLength(source)<=32*1024,'bounded contributor instruction source');
  assert.equal(Buffer.from(source).toString('utf8'),source);
  return source.split(/[\t\r\n ]+/u).filter(word=>/[A-Za-z0-9]/u.test(word)).length;
}

/** Preserve every complete paragraph/bullet and its original heading context. */
export function readContributorInstructionUnits(source) {
  countContributorInstructionWords(source);
  const units=[],headings=[],context=[];
  let at=0,current;
  const finish=()=>{if(current){units.push(current);current=undefined;}};
  for(const match of source.matchAll(/[^\n]*\n|[^\n]+$/gu)){
    const text=match[0],start=at;at+=Buffer.byteLength(text);
    const heading=/^(#+) ([^\r\n]+)[\r\n]*$/u.exec(text);
    if(/^[\t\r\n ]*$/u.test(text)){finish();continue;}
    if(heading){
      finish();const level=heading[1].length;
      context.length=level-1;context[level-1]=heading[2];
      headings.push({level,text:heading[2]});continue;
    }
    const bullet=text.startsWith('- ');
    if(bullet)finish();
    if(!current)current={kind:bullet?'bullet':'paragraph',section:context.join(' / '),start,end:at,text};
    else{current.end=at;current.text+=text;}
  }
  finish();assert.equal(at,Buffer.byteLength(source));
  return{units,headings};
}

/** Verify actual files against complete reviewed sources without Git/network IO. */
export function verifyContributorInstructionCoverage({agents,claude,request,versionRequest},policy) {
  for(const value of [agents,claude,request,versionRequest])countContributorInstructionWords(value);
  const serialized=structuredDataToLino(policy);
  assert.ok(Buffer.byteLength(serialized)<=256*1024,'bounded reviewed instruction policy');
  assert.equal(digest(serialized),policyHash,'the complete independently reviewed clause correspondence stays exact');
  assert.equal(policy.schemaVersion,1);assert.equal(policy.model,'reviewed contributor instruction source coverage 1');
  assert.equal(digest(request),requestHash);assert.equal(request,policy.request.source);
  assert.equal(versionRequest,policy.versionRequest.source);assert.equal(digest(versionRequest),policy.versionRequest.sha256);
  const captures=[['originalAgents',originalRevision,'AGENTS.md'],['compactAgents',compactRevision,'AGENTS.md'],['sharedReference',compactRevision,'CLAUDE.md']];
  for(const [name,revision,path]of captures){
    const source=policy.sources[name];
    verifyGitArtifactCertificate(source.certificate,Buffer.from(source.text),{revision,path,
      limits:{maxBlobBytes:32*1024,maxTreeBytes:256*1024,maxTreeEntries:1000,maxCommitBytes:64*1024,maxPathDepth:8}});
  }
  assert.equal(policy.sources.originalAgents.certificate.trees[0].entries.some(entry=>entry.name==='CLAUDE.md'),false,
    'the complete original Git root authenticates the unavailable preceding reference file');
  const original=readContributorInstructionUnits(policy.sources.originalAgents.text);
  const reviewed=readContributorInstructionUnits(policy.sources.compactAgents.text);
  const actual=readContributorInstructionUnits(agents);
  assert.equal(original.units.length,28);assert.equal(reviewed.units.length,20);
  assert.equal(policy.coverage.length,original.units.length);
  assert.equal(policy.currentUnits.length,reviewed.units.length);
  assert.deepEqual(actual.headings,reviewed.headings,'the reviewed contract and local-resource contexts remain authoritative');
  assert.equal(actual.units.length,reviewed.units.length,'no competing instruction block is added or omitted');
  const currentIds=new Set(),covered=new Set();
  for(const [index,unit]of policy.currentUnits.entries()){
    assert.equal(currentIds.has(unit.id),false);currentIds.add(unit.id);
    assert.deepEqual(unit.source,reviewed.units[index]);
    assert.equal(actual.units.filter(value=>value.kind===unit.source.kind&&value.section===unit.source.section
      &&normalize(value.text)===normalize(unit.source.text)).length,1,'every complete reviewed instruction remains: '+unit.id);
  }
  for(const [index,row]of policy.coverage.entries()){
    assert.equal(row.originalIndex,index);assert.deepEqual(row.source,original.units[index]);
    assert.ok(row.coveredBy.length>0);
    for(const id of row.coveredBy){assert.ok(currentIds.has(id));covered.add(id);}
    if(row.authorizedCorrection!==undefined)assert.equal(row.authorizedCorrection,requestHash);
  }
  for(const addition of policy.additions){assert.ok(currentIds.has(addition.unit));assert.equal(addition.authority,policy.versionRequest.sha256);covered.add(addition.unit);}
  assert.deepEqual([...covered].sort(),[...currentIds].sort(),'every current rule has an explicit original or newly requested source');
  assert.equal(normalize(claude),normalize(policy.sources.sharedReference.text),'the single reference preserves one authoritative contributor contract');
  assert.equal(normalize(claude),'Follow [AGENTS.md](AGENTS.md) for all repository instructions.');
  const words={original:countContributorInstructionWords(policy.sources.originalAgents.text),
    agents:countContributorInstructionWords(agents),claude:countContributorInstructionWords(claude)};
  assert.ok(words.agents<=countContributorInstructionWords(policy.sources.compactAgents.text));
  assert.ok(words.claude<=countContributorInstructionWords(policy.sources.sharedReference.text));
  assert.ok(words.agents+words.claude<words.original,'the whole combined contract is shorter than the actual preceding single file');
  return{words,originalUnits:original.units.length,reviewedUnits:reviewed.units.length,coveredOriginalUnits:policy.coverage.length,referenceTarget:'AGENTS.md'};
}

export function checkContributorInstructionCoverage() {
  return verifyContributorInstructionCoverage({agents:readFileSync(new URL('AGENTS.md',root),'utf8'),
    claude:readFileSync(new URL('CLAUDE.md',root),'utf8'),
    request:readFileSync(new URL('parity/sources/compact-contributor-instructions-and-assertion-corrections-2026-10-16.txt',root),'utf8'),
    versionRequest:readFileSync(new URL('parity/sources/version-bounded-assertions-2026-10-16.txt',root),'utf8')},
  readProjectData(new URL(CONTRIBUTOR_INSTRUCTION_COVERAGE_FILE,root)));
}
