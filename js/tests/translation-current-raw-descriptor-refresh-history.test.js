import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {LinkNetwork} from '../src/network.js';
import {encodeStructuredDataLinks,readStructuredDataLino} from '../src/structured-data-links.js';
import {decodeStructuredDataLinks} from '../src/structured-data-graph.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {readProjectData,serializeProjectData} from '../scripts/project-data.mjs';
import {readCurrentOwnCodebaseRawDescriptorPolicy,readCurrentOwnCodebaseRawDescriptorPacket,
  verifyCurrentOwnCodebaseRawDescriptorPacket,verifyCurrentOwnCodebaseRawDescriptorRefresh,
  deriveCurrentOwnCodebaseRawDescriptorOutputs} from '../scripts/current-own-codebase-raw-source-descriptors.mjs';
import {generateCurrentOwnCodebaseRawSourceDescriptors} from '../scripts/generate-current-own-codebase-raw-source-descriptors.mjs';
import {CURRENT_RAW_PREDECESSOR_ARCHIVE,CURRENT_RAW_PREDECESSOR_PIN,
  pinCurrentRawPredecessorSource,readCurrentRawPredecessorHistory,readCurrentRawPredecessorPacket,
  readCurrentRawPredecessorOutputs,verifyCurrentRawPredecessorHistory,
  acceptCurrentRawPredecessorOutput,acceptCurrentRawPredecessorPacketSource} from '../scripts/current-own-codebase-raw-source-history.mjs';
import {generateCurrentRawPredecessorHistory} from '../scripts/generate-current-own-codebase-raw-source-history.mjs';
const root=new URL('../../',import.meta.url),read=file=>readFileSync(new URL(file,root));
const fields=['completeRust','owners','declarationNames','totals','assumptions','encodings','mappings','preludeHashes','replays','archive'];

test('current raw version-conservation controls select a covered active Node range',()=>{
  const[major,minor]=process.versions.node.split('.').map(Number);
  assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; >=22.15 <23 || >=24 <25 required');
});

test('the complete8d current packet and its106 native definitions remain separately available with exact source aliases',()=>{
  const history=readCurrentRawPredecessorHistory(),old=readCurrentRawPredecessorPacket(),current=readCurrentOwnCodebaseRawDescriptorPacket();
  assert.deepEqual(pinCurrentRawPredecessorSource(read(CURRENT_RAW_PREDECESSOR_ARCHIVE)),CURRENT_RAW_PREDECESSOR_PIN);
  assert.equal(old.schemaVersion,1);assert.equal(current.schemaVersion,old.schemaVersion);
  assert.equal(old.sources[0].current.bytes,136201);
  assert.equal(old.sources[0].current.sha256,'face5521fb69545cd5c06ff2eebed38878ccba559e69d1454cb7cc352efc43ed');
  assert.equal(old.edge.changes.length,10);assert.equal(current.edge.changes.length,10);
  const outputs=readCurrentRawPredecessorOutputs();assert.equal(outputs.length,3);
  const prior=readStructuredDataLino(outputs[0].source),actual=readProjectData(new URL(history.outputs[0].file,root));
  assert.equal(prior.definitionCount,106);assert.equal(prior.owners.length,106);
  assert.equal(actual.definitionCount,106);assert.equal(actual.owners.length,106);
  for(const field of fields)assert.deepEqual(actual[field],prior[field],field+' is conserved as its original complete definition');
  for(const[packet,artifact]of [[old,prior],[current,actual]]){
    const proof=artifact.predecessor.proof.producerApplicability;
    assert.equal(proof.sources.find(row=>row.file===packet.sources[1].file)===proof.sourceHistory.current,packet.inferenceDescriptorAlias);
    assert.deepEqual(packet.edge,deriveCurrentOwnCodebaseRawDescriptorOutputs(packet).edge);
  }
  assert.equal(serializeProjectData(old,readCurrentOwnCodebaseRawDescriptorPolicy().output),read(CURRENT_RAW_PREDECESSOR_ARCHIVE).toString('utf8'));
  const detached=readCurrentRawPredecessorPacket();detached.edge.changes.reverse();
  assert.deepEqual(readCurrentRawPredecessorPacket().edge,old.edge);
});

test('normal transition admits only the three complete certified prior output bytes and never an unknown path or mutation',()=>{
  for(const row of readCurrentRawPredecessorOutputs()){
    assert.equal(acceptCurrentRawPredecessorOutput(row.source,row.file),true);
    assert.equal(acceptCurrentRawPredecessorOutput(row.source+'\n',row.file),false);
    assert.equal(acceptCurrentRawPredecessorOutput(row.source,'unregistered.lino'),false);
  }
  const source=read(CURRENT_RAW_PREDECESSOR_ARCHIVE).toString('utf8');
  assert.equal(acceptCurrentRawPredecessorPacketSource(source),true);
  assert.equal(acceptCurrentRawPredecessorPacketSource(source+'\n'),false);
});

test('new active provenance uses actual physical sources while the old current revision keeps its own identity',()=>{
  const current=readCurrentOwnCodebaseRawDescriptorPacket(),old=readCurrentRawPredecessorPacket();
  assert.notEqual(current.sources[0].current.sourceText,old.sources[0].current.sourceText,'the actual SCC frontend is not assigned the old8d definition');
  for(const row of current.sources){
    assert.equal(row.current.sourceText,read(row.file).toString('utf8'));
    assert.deepEqual(pinCurrentRawPredecessorSource(read(row.file)),{bytes:row.current.bytes,sha256:row.current.sha256});
  }
  verifyCurrentOwnCodebaseRawDescriptorRefresh(current);
  // Public readers retain their exact installed packet digest. The separate
  // historical packet cannot be substituted, even though all its proofs pass.
  assert.throws(()=>verifyCurrentOwnCodebaseRawDescriptorPacket(old),assert.AssertionError);
  const changed=structuredClone(current);changed.sources[0].current.sourceText+='\n';
  assert.throws(()=>verifyCurrentOwnCodebaseRawDescriptorRefresh(changed),assert.AssertionError);
  const boundary=structuredClone(current);boundary.boundary+='changed scope';
  assert.throws(()=>verifyCurrentOwnCodebaseRawDescriptorRefresh(boundary),assert.AssertionError);
});

test('forged archive membership output revisions coherent pins and source-owner substitutions cannot authenticate',()=>{
  const mutations=[
    history=>{history.revision='0'.repeat(40);},history=>{history.archive.path='unregistered.lino';},
    history=>{history.archive.sha256='0'.repeat(64);},history=>{history.archive.certificate.path='unregistered.lino';},
    history=>{history.outputs.reverse();},history=>{history.sources.reverse();},
    history=>{history.outputs[0].certificate.revision='0'.repeat(40);},
    history=>{history.outputs[1].bytes+=1;history.outputs[1].sha256='0'.repeat(64);},
    history=>{history.sources[0].certificate.path=history.sources[1].file;},
    history=>{history.additionalOutputs=['unregistered.lino'];},
  ];
  const archive=read(CURRENT_RAW_PREDECESSOR_ARCHIVE);
  for(const[index,mutate]of mutations.entries()){
    const history=readCurrentRawPredecessorHistory();mutate(history);
    assert.throws(()=>verifyCurrentRawPredecessorHistory(history,archive),assert.AssertionError,'tamper '+index);
  }
  assert.throws(()=>verifyCurrentRawPredecessorHistory(readCurrentRawPredecessorHistory(),Buffer.concat([archive,Buffer.from('\n')])),assert.AssertionError);
});

test('compact historical memberships survive both Links codecs without duplicating the large archived packet',()=>{
  const history=readCurrentRawPredecessorHistory(),saved=encodeStructuredDataLinks(history);
  const canonical=LinkNetwork.fromLino(saved.network.toLino());
  for(const network of [canonical,decodeLinkNetworkBinary(encodeLinkNetworkBinary(canonical))]){
    const restored=decodeStructuredDataLinks(network,saved.root);assert.deepEqual(restored,history);
    assert.deepEqual(verifyCurrentRawPredecessorHistory(restored).packet.edge,readCurrentRawPredecessorPacket().edge);
  }
});

test('both normal authors remain repeatable without overwriting any prior packet receipt native definition or runtime source',()=>{
  const policy=readCurrentOwnCodebaseRawDescriptorPolicy();
  const paths=[policy.output,CURRENT_RAW_PREDECESSOR_ARCHIVE,policy.artifactFile,policy.registerFile,policy.loanFile,...policy.sourcePaths];
  const before=paths.map(read);
  assert.deepEqual(generateCurrentRawPredecessorHistory({check:true}),readCurrentRawPredecessorHistory());
  assert.deepEqual(generateCurrentOwnCodebaseRawSourceDescriptors({check:true}),readCurrentOwnCodebaseRawDescriptorPacket());
  paths.forEach((file,index)=>assert.deepEqual(read(file),before[index],file));
});
