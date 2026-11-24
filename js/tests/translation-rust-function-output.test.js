// Future source controls; no execution during the complete draft.
import assert from 'node:assert/strict';
import test from 'node:test';
import {runInNewContext} from 'node:vm';
import {readProjectData} from '../scripts/project-data.mjs';
import {parseRust} from '../src/translation/rust.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {createRustFormattingProgramLinks} from '../src/rust-formatting-program.js';
import {encodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
const [major,minor]=process.versions.node.split('.').map(Number);
assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node);
const fixture=readProjectData(new URL('../../parity/fixtures/rust-function-output.lino',import.meta.url));
test('function print traces preserve capture order source names and actual formatting grammar',()=>{
 const program=checkProgram(parseRust(fixture.source)),packet=encodeTypedProgramLinks(program),output=emitJavaScript(program),lines=[];
 runInNewContext(output.text,{console:{log:(...values)=>lines.push(values.map(String).join(' '))}},{timeout:1000});
 assert.deepEqual(lines,fixture.lines);
 for(const row of fixture.cases)assert.ok(fixture.source.includes(row.source),row.name+' exact source');
 for(const actual of [packet,createRustFormattingProgramLinks()])for(const target of ['JavaScript','Rust','Lean','Rocq']){
  const expected=emitTypedProgramLinks(actual.network,actual.root,target);
  for(const network of [LinkNetwork.fromLino(actual.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(actual.network))])assert.deepEqual(emitTypedProgramLinks(network,actual.root,target),expected);
 }
});
for(const row of fixture.refusals)test(row.name,()=>assert.throws(()=>checkProgram(parseRust(row.source)),error=>error.kind===row.kind&&error.reason===row.reason));
