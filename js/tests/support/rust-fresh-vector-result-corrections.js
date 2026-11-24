// Actual positive controls replacing obsolete current-default refusals.
import assert from 'node:assert/strict';
import {runInNewContext} from 'node:vm';
import {readProjectData} from '../../scripts/project-data.mjs';
import {parseRust} from '../../src/translation/rust.js';
import {checkProgram} from '../../src/translation/check.js';
import {emitJavaScript} from '../../src/translation/emit-javascript.js';
import {encodeTypedProgramLinks,emitTypedProgramLinks} from '../../src/typed-program-links.js';
import {LinkNetwork} from '../../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../../src/link-network-binary.js';
export function assertRustFreshVectorResultCorrection(entry){
 const fixture=readProjectData(new URL('../../../parity/fixtures/rust-fresh-vector-results.lino',import.meta.url)),correction=fixture.corrections.find(row=>row.id===entry.id);assert.ok(correction,'the source correction is explicit');
 const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.JavaScript);
 assert.equal(entry.id,correction.id);assert.equal(entry.source,correction.source);assert.deepEqual({kind:entry.kind,reason:entry.reason,span:entry.span},correction.previousExpectation);
 const program=checkProgram(parseRust(entry.source)),declaration=program.declarations.get(correction.functionName);assert.equal(declaration.k,'fn');assert.deepEqual(declaration.ret,correction.resultType);
 const javascript=emitJavaScript(program),target=javascript.mappings.find(row=>row.kind==='function'&&row.source===correction.functionName).target;
 const operation=runInNewContext(javascript.text.replace(/^export /gmu,'')+'\n'+target+';');
 for(const row of correction.calls){const args=row.arguments.map(value=>Array.isArray(value)?Object.freeze([...value]):value),first=operation(...args),second=operation(...args);if(correction.resultType.kind==='array'){assert.notStrictEqual(first,second);assert.deepEqual(Array.from(first,String),row.value);assert.deepEqual(Array.from(second,String),row.value);}else{assert.equal(String(first),row.value);assert.equal(String(second),row.value);}}
 const packet=encodeTypedProgramLinks(program);for(const target of ['JavaScript','Rust','Lean','Rocq']){
  const expected=emitTypedProgramLinks(packet.network,packet.root,target);
  for(const network of [LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),expected);
 }
}
