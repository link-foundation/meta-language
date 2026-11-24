// Future independent primitive-source producer; never run while drafting.
import assert from 'node:assert/strict';
import {readProjectData} from '../../js/scripts/project-data.mjs';
import {structuredDataToLino} from '../../js/src/structured-data-links.js';
import {parseJavaScript} from '../../js/src/translation/javascript.js';
import {checkProgram} from '../../js/src/translation/check.js';
import {emitJavaScript} from '../../js/src/translation/emit-javascript.js';
import {emitRust} from '../../js/src/translation/emit-rust.js';
import {encodeTypedProgramLinks,emitTypedProgramLinks} from '../../js/src/typed-program-links.js';
import {runInNewContext} from 'node:vm';
const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node);
const fixture=readProjectData(new URL('../../parity/fixtures/string-array-copy-sort.lino',import.meta.url));
const program=checkProgram(parseJavaScript(fixture.source)),packet=encodeTypedProgramLinks(program),javascript=emitJavaScript(program),rust=emitRust(program);
const original=new Function(fixture.source.replace(/^export /gmu,'')+';return sortStrings;')();
const translated=runInNewContext(javascript.text.replace(/^export /gmu,'')+'\n'+javascript.mappings.find(row=>row.source==='sortStrings').target);
const units=value=>Array.from({length:value.length},(_,index)=>value.charCodeAt(index));
for(const row of fixture.cases){const input=row.units.map(value=>String.fromCharCode(...value)),before=input.map(units),expected=row.expected;for(const operation of [original,translated]){const result=operation(input);assert.notStrictEqual(result,input);assert.deepEqual(Array.from(result,units),expected);assert.deepEqual(input.map(units),before);}}
const emissions={};for(const target of ['JavaScript','Rust'])emissions[target]=emitTypedProgramLinks(packet.network,packet.root,target);
for(const target of ['Lean','Rocq'])assert.throws(()=>emitTypedProgramLinks(packet.network,packet.root,target),error=>error.kind==='unsupported'&&error.reason===fixture.formalRefusalReason);
const header=rust.text.split('\n').find(line=>line.startsWith('#![allow('));assert.ok(header);
process.stdout.write(structuredDataToLino({source:fixture.source,cases:fixture.cases,refusals:fixture.refusals,refusalReason:fixture.refusalReason,formalRefusalReason:fixture.formalRefusalReason,links:packet.network.toLino(),root:packet.root.asU64(),emissions,nativeTarget:rust.mappings.find(row=>row.source==='sortStrings').target,nativeLibrary:[header,...rust.preludes,...rust.definitions].join('\n\n')}));
