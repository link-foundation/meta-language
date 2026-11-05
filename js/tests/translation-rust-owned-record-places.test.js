// Deferred source contracts; no executed evidence is recorded here.
import test from 'node:test';import assert from 'node:assert/strict';import vm from 'node:vm';
import {parseRust} from '../src/translation/rust.js';import {checkProgram} from '../src/translation/check.js';import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {createTypedProgramLinks,decodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';import {LinkNetwork} from '../src/network.js';import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import * as rules from '../src/translation/rust-owned-record-place-rules.js';import {rustOwnedRecordFieldState} from '../src/translation/rust-owned-record-updates.js';
import {rustOwnedRecordPlaceDecisions as decisions,rustOwnedRecordPlaceOriginals as originals,rustOwnedRecordPlaceRefusals as refusals} from './fixtures/rust-owned-record-places.mjs';
const versions=['1.90.0','1.99.0'],targets=['JavaScript','Rust','Lean','Rocq'];
const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===24||major===22&&minor>=15,'uncovered nested record place Node '+process.versions.node);
function stdout(code){const lines=[];vm.runInNewContext(code,{console:{log:(...values)=>lines.push(values.map(String).join(' '))},process:{argv:[]}}, {timeout:1000});return lines.length?lines.join('\n')+'\n':'';}
for(const [index,row]of decisions.entries())test('record place syntax decision '+index+' '+row.operation,()=>assert.deepEqual(rules[row.operation](...row.arguments),row.expected));
test('all independent nested place inputs stay represented',()=>{assert.equal(decisions.length,12);assert.equal(originals.length,8);assert.equal(refusals.length,5);});
test('an absent nested field inherits only an actually available ancestor',()=>{
 const identity='record field record field 7 0 1';
 assert.equal(rustOwnedRecordFieldState(new Map([['7','available']]),identity),'available');
 assert.equal(rustOwnedRecordFieldState(new Map([['7','available'],['record field 7 0','moved']]),identity),'absent');
 assert.equal(rustOwnedRecordFieldState(new Map([['7','available'],[identity,'moved']]),identity),'moved');
 assert.equal(rustOwnedRecordFieldState(new Map([['70','available']]),identity),'absent');
});
for(const row of originals)for(const sourceVersion of versions)test('actual nested record place '+row.name+'/'+sourceVersion,()=>{
 assert.equal(row.sourceVersionRange,'>=1.90.0 <1.100.0');const concrete=LinkNetwork.parse(row.source,'Rust'),text=concrete.toLino(),binary=encodeLinkNetworkBinary(concrete);
 for(const graph of [concrete,LinkNetwork.fromLino(text),decodeLinkNetworkBinary(binary)])assert.equal(graph.reconstructText(),row.source);
 const checked=checkProgram(parseRust(row.source,{sourceVersion}));assert.equal(stdout(emitJavaScript(checked).text),row.stdout);
 const packet=createTypedProgramLinks(row.source,'Rust',{rustSourceVersion:sourceVersion,provenance:{fixture:row.name}}),typedText=packet.network.toLino(),typedBinary=encodeLinkNetworkBinary(packet.network),baseline=Object.fromEntries(targets.map(target=>[target,emitTypedProgramLinks(packet.network,packet.root,target)]));
 for(const graph of [packet.network,LinkNetwork.fromLino(typedText),decodeLinkNetworkBinary(typedBinary)]){assert.equal(graph.toLino(),typedText);const restored=decodeTypedProgramLinks(graph,packet.root);assert.deepEqual(restored.items,checked.items);assert.deepEqual(restored.main,checked.main);for(const target of targets){const emitted=emitTypedProgramLinks(graph,packet.root,target);assert.deepEqual(emitted,baseline[target]);assert.ok(emitted.code.length>0);if(target==='JavaScript')assert.equal(stdout(emitted.code),row.stdout);}assert.deepEqual(encodeLinkNetworkBinary(graph),typedBinary);}
});
for(const row of refusals)for(const sourceVersion of versions)test('actual record place refusal '+row.name+'/'+sourceVersion,()=>{assert.equal(row.originalCompilerOutcome,'reject');for(const read of [()=>checkProgram(parseRust(row.source,{sourceVersion})),()=>createTypedProgramLinks(row.source,'Rust',{rustSourceVersion:sourceVersion})])assert.throws(read,error=>{assert.ok(['syntax','type','unsupported'].includes(error.kind));assert.ok(error.reason.length>0);return true;});});
