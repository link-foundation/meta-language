import assert from 'node:assert/strict';
import {execFileSync} from 'node:child_process';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';
import {CONTRIBUTOR_INSTRUCTION_COVERAGE_FILE,checkContributorInstructionCoverage,
  verifyContributorInstructionCoverage} from '../scripts/contributor-instruction-coverage.mjs';
import {generateContributorInstructionCoverage} from '../scripts/generate-contributor-instruction-coverage.mjs';
import {readStructuredDataLino,structuredDataToLino,encodeStructuredDataLinks,decodeStructuredDataLinks} from '../src/structured-data-links.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {LinkNetwork} from '../src/network.js';
import {recordIssue195Observations} from './support/issue-195-observations.js';

const root=new URL('../../',import.meta.url);
const input=()=>({agents:readFileSync(new URL('AGENTS.md',root),'utf8'),claude:readFileSync(new URL('CLAUDE.md',root),'utf8'),
  request:readFileSync(new URL('parity/sources/compact-contributor-instructions-and-assertion-corrections-2026-10-16.txt',root),'utf8'),
  versionRequest:readFileSync(new URL('parity/sources/version-bounded-assertions-2026-10-16.txt',root),'utf8')});
const policy=()=>readProjectData(new URL(CONTRIBUTOR_INSTRUCTION_COVERAGE_FILE,root));

test('the actual shortened contributor files preserve every original instruction and one shared authority',context=>{
  const result=checkContributorInstructionCoverage();
  assert.deepEqual(result,{words:{original:927,agents:476,claude:6},originalUnits:28,reviewedUnits:20,
    coveredOriginalUnits:28,referenceTarget:'AGENTS.md'});
  const reviewed=policy();
  assert.equal(reviewed.coverage.filter(row=>row.source.kind==='bullet').length,25);
  assert.equal(reviewed.coverage.filter(row=>row.source.kind==='paragraph').length,3);
  assert.equal(reviewed.coverage.filter(row=>row.authorizedCorrection!==undefined).length,1);
  recordIssue195Observations({requirementId:'I195-CONTRIBUTOR-INSTRUCTIONS-CONCISE',runtime:'tooling',suffix:'behavior',
    fixtureId:'conversation-contract:compactContributorInstructionsRequest',
    fixtureFile:'parity/sources/compact-contributor-instructions-and-assertion-corrections-2026-10-16.txt',
    assertions:['contributorInstructionsReducedWithoutLostObligations','sharedInstructionsRemainAuthoritative'],testName:context.name});
});

test('deleting any whole obligation or replacing the shared authority cannot satisfy instruction conservation',()=>{
  const original=input(),reviewed=policy();
  for(const unit of reviewed.currentUnits){
    const agents=original.agents.replace(unit.source.text,'');assert.notEqual(agents,original.agents,unit.id);
    assert.throws(()=>verifyContributorInstructionCoverage({...original,agents},reviewed),assert.AssertionError,unit.id);
  }
  for(const claude of ['',original.agents,original.claude.replace('AGENTS.md','OTHER.md'),original.claude+'Use another contract.\n']){
    assert.throws(()=>verifyContributorInstructionCoverage({...original,claude},reviewed),assert.AssertionError);
  }
  assert.throws(()=>verifyContributorInstructionCoverage({...original,agents:original.agents.replace('## Contract','## Optional contract')},reviewed));
  assert.throws(()=>verifyContributorInstructionCoverage({...original,request:original.request.replace('wrong','all')},reviewed));
});

test('reviewed clause order and whitespace remain editable inside their unchanged contract context',()=>{
  const original=input(),reviewed=policy(),grammar=reviewed.currentUnits.find(row=>row.id==='grammar generation').source.text;
  const decorators=reviewed.currentUnits.find(row=>row.id==='shared decorators').source.text;
  const agents=original.agents.replace(grammar,'CONTRIBUTOR_RULE_SWAP\n').replace(decorators,grammar).replace('CONTRIBUTOR_RULE_SWAP\n',decorators);
  assert.notEqual(agents,original.agents);
  assert.deepEqual(verifyContributorInstructionCoverage({...original,agents},reviewed),checkContributorInstructionCoverage());
  assert.deepEqual(verifyContributorInstructionCoverage({...original,agents:original.agents.replaceAll('  ','    '),
    claude:original.claude.replaceAll(' ','  ')},reviewed),checkContributorInstructionCoverage());
});

test('instruction source capture is reproducible and forged authorities or omitted coverage refuse',()=>{
  generateContributorInstructionCoverage({check:true});
  const original=input(),reviewed=policy();
  for(const mutate of [value=>{value.coverage.pop();},value=>{value.coverage[3].authorizedCorrection='0'.repeat(64);},
    value=>{value.currentUnits.pop();},value=>{value.sources.originalAgents.text+='extra\n';},
    value=>{value.sources.compactAgents.certificate.blob.sha256='0'.repeat(64);},
    value=>{value.sources.sharedReference.certificate.revision='f'.repeat(40);}]){
    const changed=structuredClone(reviewed);mutate(changed);
    assert.throws(()=>verifyContributorInstructionCoverage(original,changed),assert.AssertionError);
  }
});

test('instruction coverage survives text and binary Links reload with original files processes and network unavailable',()=>{
  const original=input(),reviewed=policy(),encoded=encodeStructuredDataLinks(reviewed);
  const network=new LinkNetwork();
  for(const link of encoded.network.links())network.insertLinkWithOptionalId(link.id(),link.references(),link.metadata());
  for(const value of [readStructuredDataLino(structuredDataToLino(reviewed)),
    decodeStructuredDataLinks(decodeLinkNetworkBinary(encodeLinkNetworkBinary(network)),encoded.root)]){
    assert.deepEqual(verifyContributorInstructionCoverage(original,value),checkContributorInstructionCoverage());
  }
  const validator=new URL('../scripts/contributor-instruction-coverage.mjs',import.meta.url).href;
  const codec=new URL('../src/structured-data-links.js',import.meta.url).href;
  const script=`import fs from 'node:fs';import cp from 'node:child_process';import net from 'node:net';import http from 'node:http';import https from 'node:https';
import{syncBuiltinESMExports}from'node:module';import{verifyContributorInstructionCoverage}from${JSON.stringify(validator)};
import{readStructuredDataLino,structuredDataToLino}from${JSON.stringify(codec)};
let source='';for await(const chunk of process.stdin)source+=chunk;const packet=readStructuredDataLino(source);
const refuse=()=>{throw new Error('Original source files, Git, processes and network are unavailable');};
for(const[owner,names]of[[fs,['readFileSync','existsSync','statSync','openSync','readSync']],[cp,['execFileSync','spawnSync','execSync','spawn','exec','execFile']],[net,['connect','createConnection']],[http,['request','get']],[https,['request','get']]])for(const name of names)owner[name]=refuse;
globalThis.fetch=refuse;syncBuiltinESMExports();const result=verifyContributorInstructionCoverage(packet.input,packet.policy);process.stdout.write(structuredDataToLino(result));`;
  const output=execFileSync(process.execPath,['--max-old-space-size=256','--input-type=module','-e',script],{
    input:structuredDataToLino({input:original,policy:reviewed}),encoding:'utf8',maxBuffer:1024*1024,timeout:15000});
  assert.deepEqual(readStructuredDataLino(output),checkContributorInstructionCoverage());
});
