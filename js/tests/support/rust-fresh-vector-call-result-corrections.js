// Stronger same-input controls for the now represented source call contract.
import assert from 'node:assert/strict';
import {runInNewContext} from 'node:vm';
import {readProjectData} from '../../scripts/project-data.mjs';
import {parseRust} from '../../src/translation/rust.js';
import {checkProgram} from '../../src/translation/check.js';
import {emitJavaScript} from '../../src/translation/emit-javascript.js';
import {encodeTypedProgramLinks,emitTypedProgramLinks} from '../../src/typed-program-links.js';
import {LinkNetwork} from '../../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../../src/link-network-binary.js';
export function assertRustFreshVectorCallResultCorrection(entry){
 const fixture=readProjectData(new URL('../../../parity/fixtures/rust-fresh-vector-call-results.lino',import.meta.url)),correction=fixture.corrections.find(row=>row.original.name===entry.name);assert.ok(correction);assert.deepEqual(entry,correction.original);
 const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.JavaScript);
 const program=checkProgram(parseRust(entry.source));assert.deepEqual(program.declarations.get(correction.functionName).ret,correction.resultType);
 const javascript=emitJavaScript(program),target=javascript.mappings.find(row=>row.kind==='function'&&row.source===correction.functionName).target,operation=runInNewContext(javascript.text.replace(/^export /gmu,'')+'\n'+target+';');
 const first=operation(),second=operation();assert.notStrictEqual(first,second);assert.deepEqual(Array.from(first,String),correction.values);assert.deepEqual(Array.from(second,String),correction.values);
 const packet=encodeTypedProgramLinks(program);for(const target of ['JavaScript','Rust','Lean','Rocq']){const expected=emitTypedProgramLinks(packet.network,packet.root,target);for(const network of [LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),expected);}
}
