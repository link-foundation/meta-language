// Deferred effect, source-coordinate and complete definition controls.
import assert from 'node:assert/strict';
import test from 'node:test';
import {format} from 'node:util';
import {runInNewContext} from 'node:vm';
import {readProjectData} from '../scripts/project-data.mjs';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {createSourceDefinitionProgramLinks} from '../src/source-definition-catalog.js';
import {encodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {partitionJavaScriptPatternInitializerPositions} from '../src/translation/binding-initialization-rules.js';
const fixture=readProjectData(new URL('../../parity/fixtures/binding-initialization.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.JavaScript);
function observe(source){const lines=[];let error=null;try{runInNewContext(source,{console:{log:(...values)=>lines.push(format(...values))}},{timeout:1000});}catch(thrown){error={name:thrown.name,message:thrown.message};}return {lines,error};}
for(const row of fixture.traces)test(row.name,()=>{const expected={lines:row.lines,error:row.error};assert.deepEqual(observe(row.source),expected);const program=checkProgram(parseJavaScript(row.source));assert.deepEqual(observe(emitJavaScript(program).text),expected);const packet=encodeTypedProgramLinks(program);for(const network of [LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))])assert.deepEqual(observe(emitTypedProgramLinks(network,packet.root,'JavaScript').code),expected);});
test('nameless source boundaries retain actual earlier and later names',()=>{assert.deepEqual(partitionJavaScriptPatternInitializerPositions([1,9],4,'punct','['),[1,1,9]);assert.deepEqual(partitionJavaScriptPatternInitializerPositions([],0,'punct','['),[0]);assert.throws(()=>partitionJavaScriptPatternInitializerPositions([1,9],4,'identifier','invented'),RangeError);assert.throws(()=>partitionJavaScriptPatternInitializerPositions([9,1],4,'punct','['),RangeError);});
test('complete source definition survives both codecs and every emission',()=>{const packet=createSourceDefinitionProgramLinks('meta-language/source-program/binding-initialization-program');for(const target of ['JavaScript','Rust','Lean','Rocq']){const expected=emitTypedProgramLinks(packet.network,packet.root,target);for(const network of [LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),expected);}});
