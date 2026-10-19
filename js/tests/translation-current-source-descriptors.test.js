import assert from 'node:assert/strict';
import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {isDeepStrictEqual} from 'node:util';
import test from 'node:test';
import {readProjectData,serializeProjectData} from '../scripts/project-data.mjs';
import {readHistoricalTranslationSourceDescriptorRegister,projectHistoricalTranslationSourceDescriptorArtifact}
  from '../scripts/current-translation-source-descriptors.mjs';
import {readOriginalLinkRawEmissionProducerSources} from '../../experiments/original-link-complete-table-projection/raw-emission-profiles.mjs';
import {verifyOriginalLinkRawEmissionProducerApplicability} from '../../experiments/original-link-complete-table-projection/raw-emission-producer-applicability.mjs';

const root=new URL('../../',import.meta.url);
const policy=readProjectData(new URL('parity/self-translation/current-translation-source-descriptors.lino',root));
const hash=bytes=>createHash('sha256').update(bytes).digest('hex');

test('current descriptor provenance restores the entire prior artifact and register without changing emitted meaning',()=>{
  const artifact=readProjectData(new URL(policy.edge.current.file,root));
  assert.equal(policy.edge.changes.length,13);
  const previous=projectHistoricalTranslationSourceDescriptorArtifact(artifact);
  const expected=execFileSync('git',['show',policy.edge.previous.revision+':'+policy.edge.previous.file],{cwd:root,maxBuffer:4*1024*1024});
  assert.equal(serializeProjectData(previous,policy.edge.previous.file),expected.toString('utf8'));
  for(const field of ['completeRust','owners','declarationNames','totals','assumptions','encodings','mappings','preludeHashes','replays','archive']){
    assert.ok(isDeepStrictEqual(artifact[field],previous[field]),'complete original artifact field retained: '+field);
  }
  assert.equal(artifact.definitionCount,106);
  for(const row of artifact.decisionSources){
    const bytes=readFileSync(new URL(row.file,root));assert.equal(bytes.length,row.bytes);assert.equal(hash(bytes),row.sha256);
  }
  const register=readHistoricalTranslationSourceDescriptorRegister();
  const registerFile='parity/self-translation/complete-declaration-receipts.lino';
  assert.equal(serializeProjectData(register,registerFile),execFileSync('git',['show',policy.edge.previous.revision+':'+registerFile],{cwd:root,encoding:'utf8',maxBuffer:2*1024*1024}));
  for(const mutate of [value=>{value.completeRust+='\n';},value=>{value.owners.reverse();},
    value=>{value.decisionSources[2].sha256='0'.repeat(64);},value=>{delete value.predecessor.proof.producerApplicability.sourceHistory;}]){
    const changed=structuredClone(artifact);mutate(changed);
    assert.throws(()=>projectHistoricalTranslationSourceDescriptorArtifact(changed),/complete registered artifact/u);
  }
});

test('causal source applicability retains actual current inference bytes and its complete checked predecessor',()=>{
  const sources=readOriginalLinkRawEmissionProducerSources();
  const proof=verifyOriginalLinkRawEmissionProducerApplicability(sources);
  const stored=readProjectData(new URL(policy.edge.current.file,root)).predecessor.proof.producerApplicability;
  assert.ok(isDeepStrictEqual(proof,stored),'the recorded complete provenance equals actual source verification');
  assert.equal(proof.sources.length,14);assert.equal(proof.sourceHistory.revision,'81107a5945651f17c962e4bb81fb560f96c65ccd');
  assert.equal(proof.sourceHistory.current.bytes,sources[proof.sourceHistory.current.file].length);
  assert.equal(proof.sourceHistory.current.sha256,hash(sources[proof.sourceHistory.current.file]));
  assert.equal(proof.sourceHistory.current,proof.sources.find(row=>row.file===proof.sourceHistory.current.file));
  const changed={...sources,[proof.sourceHistory.current.file]:Buffer.concat([sources[proof.sourceHistory.current.file],Buffer.from('\n')])};
  assert.throws(()=>verifyOriginalLinkRawEmissionProducerApplicability(changed),assert.AssertionError);
});
