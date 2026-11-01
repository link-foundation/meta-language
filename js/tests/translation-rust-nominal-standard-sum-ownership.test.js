// Deferred assertions only. This source draft has not been executed.
import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import * as rules from '../src/translation/rust-nominal-ownership-rules.js';
import {parseRust} from '../src/translation/rust.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {encodeTypedProgramLinks,decodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {rustNominalStandardSumOwnershipDecisions as decisions,rustNominalStandardSumOwnershipOriginals as originals,rustNominalStandardSumOwnershipRefusals as refusals} from './fixtures/rust-nominal-standard-sum-ownership.mjs';
import {rustStandardDefaultOriginals} from './fixtures/rust-standard-default-values.mjs';
const versions=['1.90.0','1.99.0'];
function observe(source){const lines=[];vm.runInNewContext(source,{console:{log:(...values)=>lines.push(values.map(String).join(' '))},process:{argv:[]}},{timeout:1000});return lines.length?lines.join('\n')+'\n':'';}
for(const[index,row]of decisions.entries())test('independent nominal field ownership decision '+index,()=>{assert.equal(typeof rules[row.operation],'function');assert.deepEqual(rules[row.operation](...row.arguments),row.expected);});
for(const row of originals)for(const sourceVersion of versions)test(row.name+'/'+sourceVersion,()=>{
 assert.equal(row.sourceVersionRange,'>=1.90.0 <1.100.0');const checked=checkProgram(parseRust(row.source,{sourceVersion}));assert.equal(observe(emitJavaScript(checked).text),row.stdout);
 const packet=encodeTypedProgramLinks(checked);for(const graph of [packet.network,LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]){
  assert.deepEqual(decodeTypedProgramLinks(graph,packet.root),checked);
  for(const target of ['JavaScript','Rust','Lean','Rocq'])assert.deepEqual(emitTypedProgramLinks(graph,packet.root,target),emitTypedProgramLinks(packet.network,packet.root,target));
  assert.equal(observe(emitTypedProgramLinks(graph,packet.root,'JavaScript').code),row.stdout);
 }
});
for(const row of refusals)for(const sourceVersion of versions)test(row.name+'/'+sourceVersion,()=>assert.throws(()=>checkProgram(parseRust(row.source,{sourceVersion})),error=>row.originalCompilerOutcome==='accept'?error.kind==='unsupported':['unsupported','type','syntax'].includes(error.kind)));
test('the existing NoDefault original keeps its exact source and default oracle',()=>{
 const rows=rustStandardDefaultOriginals.filter(row=>row.name==='empty Option and vector defaults require no element Default implementation');assert.equal(rows.length,1);
 assert.equal(rows[0].source,'#[derive(Clone,Copy)]struct NoDefault{value:u8}fn main(){let nested:Option<Option<NoDefault>>=None;let vectors:Option<Vec<NoDefault>>=None;println!("{}",nested.unwrap_or_default().is_none());println!("{}",vectors.unwrap_or_default().len());}');assert.equal(rows[0].stdout,'true\n0\n');
 for(const sourceVersion of versions)assert.equal(observe(emitJavaScript(checkProgram(parseRust(rows[0].source,{sourceVersion}))).text),rows[0].stdout);
});
