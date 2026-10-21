import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {restoreGlobalGrammarContextSource} from '../../experiments/global-grammar-context-reuse/current-source.mjs';
import {buildGlobalGrammarContextSourceProfiles,checkGlobalGrammarContextSourceProfiles} from '../../experiments/global-grammar-context-reuse/source-profiles.mjs';
import {readProjectData} from '../scripts/project-data.mjs';
import {structuredDataToLino,encodeStructuredDataLinks} from '../src/structured-data-links.js';
import {decodeStructuredDataLinks} from '../src/structured-data-graph.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
const root=new URL('../../',import.meta.url);

test('candidate current context profiles preserve every frozen pin actual native owner and complete diagnostic call',()=>{
  const packet=buildGlobalGrammarContextSourceProfiles();
  assert.deepEqual(packet,readProjectData(new URL('../../parity/self-translation/global-grammar-context-source-profiles.lino',import.meta.url)));
  assert.equal(Object.keys(packet.historicalPins).length,13);assert.equal(packet.sources.length,2);
  const [javascript,native]=packet.sources;assert.equal(javascript.proof.length,9);
  assert.ok(native.proof.imports.length>0);assert.ok(native.proof.declarations.length>0);
  assert.notEqual(javascript.proposed.sha256,javascript.historical.sha256);
  assert.notEqual(native.proposed.sha256,native.productionPredecessor.sha256);
  assert.equal(packet.frozenOverlay.output.sha256,native.productionPredecessor.sha256);
  assert.equal(packet.frozenOverlay.original.sha256,native.historical.sha256);
  assert.equal(packet.frozenOverlay.receipt.role,'frozen CI overlay');
  for(const row of packet.sources)assert.equal(Object.hasOwn(row,'observations'),false);
  for(const row of native.proof.declarations){
    const bytes=Buffer.from(restoreGlobalGrammarContextSource(native.file,readFileSync(new URL(native.file,root),'utf8')).predecessor);assert.equal(bytes.subarray(row.start,row.end).toString(),row.source);
  }
  assert.deepEqual(checkGlobalGrammarContextSourceProfiles(packet),packet);
});

test('candidate context source proofs survive exact Links text and binary reload without shared caller objects',()=>{
  const packet=buildGlobalGrammarContextSourceProfiles(),encoded=encodeStructuredDataLinks(packet);
  const text=structuredDataToLino(packet);assert.equal(typeof text,'string');
  const network=LinkNetwork.fromLino(text);
  for(const graph of [network,decodeLinkNetworkBinary(encodeLinkNetworkBinary(network))]){
    const value=decodeStructuredDataLinks(graph,encoded.root);assert.deepEqual(value,packet);
    assert.deepEqual(checkGlobalGrammarContextSourceProfiles(value),packet);
  }
  const returned=checkGlobalGrammarContextSourceProfiles(packet);returned.sources[1].proof.declarations[0].source+='changed';
  assert.deepEqual(buildGlobalGrammarContextSourceProfiles(),packet);
  assert.deepEqual(checkGlobalGrammarContextSourceProfiles(packet),packet);
  assert.throws(()=>checkGlobalGrammarContextSourceProfiles(returned));
});

test('mixed frozen current and proposed identities changed owners missing sources and altered inverse order refuse',()=>{
  const packet=buildGlobalGrammarContextSourceProfiles();
  for(const mutate of [
    value=>{value.historicalPins[value.sources[0].file]='0'.repeat(64);},
    value=>{value.sources[1].proposed.sha256=value.sources[1].productionPredecessor.sha256;},
    value=>{value.sources[0].proposed.bytes++;},
    value=>{value.frozenOverlay.output.sha256=value.sources[1].proposed.sha256;},
    value=>{value.frozenOverlay.receipt.role='production';},
    value=>{value.sources[1].proof.imports[0].source+='changed';},
    value=>{value.sources[1].proof.declarations[0].source+='changed';},
    value=>{value.sources[1].proof.declarations[0].start++;},
    value=>{value.sources[0].proof[0].source+='changed';},
    value=>{value.sources[0].proof[0].currentCall.sha256='0'.repeat(64);},
    value=>{value.sources[1].inverseSteps.reverse();},
    value=>{value.sources.pop();},
    value=>{value.proposal.sha256='0'.repeat(64);},
  ]){
    const value=structuredClone(packet);mutate(value);assert.throws(()=>checkGlobalGrammarContextSourceProfiles(value));
  }
});
