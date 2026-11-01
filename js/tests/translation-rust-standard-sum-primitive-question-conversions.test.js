// Deferred actual-source, graph and target obligations. No execution now.
import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import * as rules from '../src/translation/rust-standard-sum-source-rules.js';
import {parseRust} from '../src/translation/rust.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {encodeTypedProgramLinks,decodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {rustStandardSumPrimitiveQuestionDecisions as decisions,rustStandardSumPrimitiveQuestionOriginals as originals,rustStandardSumPrimitiveQuestionRefusals as refusals} from './fixtures/rust-standard-sum-primitive-question-conversions.mjs';
import {rustStandardSumSourceOriginals,rustStandardSumSourceRefusals,rustStandardSumSourcePreviousBoundaries} from './fixtures/rust-standard-sum-source-rules.mjs';
import {rustStandardSumOwnedPropagationOriginals,rustStandardSumOwnedPropagationRefusals,rustStandardSumOwnedPropagationPreviousBoundaries} from './fixtures/rust-standard-sum-owned-propagation.mjs';
const versions=['1.90.0','1.99.0'];
function observe(source){const lines=[];vm.runInNewContext(source,{console:{log:(...values)=>lines.push(values.map(String).join(' '))},process:{argv:[]}},{timeout:1000});return lines.length?lines.join('\n')+'\n':'';}
for(const [index,row]of decisions.entries())test('independent primitive residual source decision '+index,()=>{assert.equal(typeof rules[row.operation],'function');assert.deepEqual(rules[row.operation](...row.arguments),row.expected);});
for(const row of originals)for(const sourceVersion of versions)test(row.name+'/'+sourceVersion,()=>{
 assert.equal(row.sourceVersionRange,'>=1.90.0 <1.100.0');const checked=checkProgram(parseRust(row.source,{sourceVersion}));assert.equal(observe(emitJavaScript(checked).text),row.stdout);
 const packet=encodeTypedProgramLinks(checked);for(const graph of [packet.network,LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]){
  assert.deepEqual(decodeTypedProgramLinks(graph,packet.root),checked);
  for(const target of ['JavaScript','Rust','Lean','Rocq'])assert.deepEqual(emitTypedProgramLinks(graph,packet.root,target),emitTypedProgramLinks(packet.network,packet.root,target));
  assert.equal(observe(emitTypedProgramLinks(graph,packet.root,'JavaScript').code),row.stdout);
 }
});
for(const row of refusals)for(const sourceVersion of versions)test(row.name+'/'+sourceVersion,()=>assert.throws(()=>checkProgram(parseRust(row.source,{sourceVersion})),error=>row.originalCompilerOutcome==='accept'?error.kind==='unsupported':['unsupported','type','syntax'].includes(error.kind)));
test('exact formerly unsupported primitive error originals preserve source coordinate assertion and compiler acceptance history',()=>{
 for(const [history,positive,negative,collection,index]of [[rustStandardSumSourcePreviousBoundaries[0],rustStandardSumSourceOriginals,rustStandardSumSourceRefusals,'rustStandardSumSourceRefusals',4],[rustStandardSumOwnedPropagationPreviousBoundaries[0],rustStandardSumOwnedPropagationOriginals,rustStandardSumOwnedPropagationRefusals,'rustStandardSumOwnedPropagationRefusals',3]]){
  assert.equal(history.previousCollection,collection);assert.equal(history.previousIndex,index);assert.deepEqual(history.previousDiagnosticKinds,['syntax','type','unsupported']);assert.equal(history.originalCompilerOutcome,'accept');
  assert.equal(positive.filter(row=>row.source===history.row.source).length,1);assert.equal(negative.filter(row=>row.source===history.row.source).length,0);assert.equal(positive.find(row=>row.source===history.row.source).stdout,'');
  for(const sourceVersion of versions){assert.equal(history.sourceVersionRange,'>=1.90.0 <1.100.0');assert.equal(observe(emitJavaScript(checkProgram(parseRust(history.row.source,{sourceVersion}))).text),'');}
 }
});
