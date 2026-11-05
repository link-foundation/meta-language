// Future controls only. Never execute during the complete requirements draft.
import assert from 'node:assert/strict';
import test from 'node:test';
import {runInNewContext} from 'node:vm';
import {readProjectData} from '../scripts/project-data.mjs';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {mapUnicodeCaseUtf16Columns} from '../src/translation/unicode-case-rules.js';
import {readUnicodeCaseLowerRanges,readUnicodeCaseLowerIndexes,readUnicodeCaseLowerValues,readUnicodeCaseUpperRanges,readUnicodeCaseUpperIndexes,readUnicodeCaseUpperValues,readUnicodeCaseCasedRanges,readUnicodeCaseIgnorableRanges} from '../src/translation/unicode-case-table-declarations.js';
import {createUnicodeCaseProgramLinks} from '../src/unicode-case-program.js';
import {encodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
function requireProfile(){assert.equal(process.versions.node.split('.')[0],'24','uncovered Unicode casing Node range');assert.equal(process.versions.unicode,'16.0','uncovered Unicode casing profile');}
test('original casing values preserve context expansions units contracts and complete archives',()=>{
 requireProfile();
 const fixture=readProjectData(new URL('../../parity/fixtures/unicode-case-values.lino',import.meta.url));
 const program=checkProgram(parseJavaScript(fixture.source)),packet=encodeTypedProgramLinks(program),output=emitJavaScript(program);
 const names=fixture.cases.map(row=>output.mappings.find(mapping=>mapping.kind==='function'&&mapping.source===row.entry).target);
 const calls=runInNewContext('"use strict";\n'+output.text.replace(/^export /gmu,'')+'\n['+names.join(',')+'];');
 for(const [index,row] of fixture.cases.entries()){
  assert.equal(String(runInNewContext('"use strict";\n'+row.source.replace(/^export /gmu,'')+'\ncompute();')),row.value,row.name+' original');
  assert.equal(String(calls[index]()),row.value,row.name+' emitted');
 }
 for(const actual of [packet,createUnicodeCaseProgramLinks()])for(const target of ['JavaScript','Rust','Lean','Rocq']){
  const expected=emitTypedProgramLinks(actual.network,actual.root,target);
  for(const network of [LinkNetwork.fromLino(actual.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(actual.network))])assert.deepEqual(emitTypedProgramLinks(network,actual.root,target),expected);
 }
 assert.ok(output.encodings.some(row=>row.id==='unicode-default-case-16'));
});
test('every Unicode code point agrees with the independently version-selected Node default casing',()=>{
 assert.equal(process.env.CI,'true','the exhaustive Unicode source oracle runs only in CI');
 requireProfile();
 const units=text=>Array.from({length:text.length},(_,index)=>text.charCodeAt(index));
 const cased=readUnicodeCaseCasedRanges(),ignorable=readUnicodeCaseIgnorableRanges();
 const lower=[readUnicodeCaseLowerRanges(),readUnicodeCaseLowerIndexes(),readUnicodeCaseLowerValues()];
 const upper=[readUnicodeCaseUpperRanges(),readUnicodeCaseUpperIndexes(),readUnicodeCaseUpperValues()];
 for(let point=0;point<=1114111;point+=1){
  const text=String.fromCodePoint(point),input=units(text);
  assert.deepEqual(mapUnicodeCaseUtf16Columns(input,false,...lower,cased,ignorable),units(text.toLowerCase()),'lower '+point);
  assert.deepEqual(mapUnicodeCaseUtf16Columns(input,true,...upper,cased,ignorable),units(text.toUpperCase()),'upper '+point);
 }
});
