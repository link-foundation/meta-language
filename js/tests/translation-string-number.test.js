// Future complete numeric controls. Do not run during the whole draft.
import assert from 'node:assert/strict';
import test from 'node:test';
import {runInNewContext} from 'node:vm';
import {readProjectData} from '../scripts/project-data.mjs';
import {convertJavaScriptStringNumber} from '../src/string-number.js';
import {convertJavaScriptStringNumberUnits,convertJavaScriptUtf8StringNumberUnits} from '../src/translation/string-number-rules.js';
import {createStringNumberProgramLinks} from '../src/string-number-program.js';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {encodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
const fixture=readProjectData(new URL('../../parity/fixtures/string-number.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);
assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node);
const observe=call=>{try{return{kind:'value',value:String(call())};}catch(error){return{kind:'abort',value:error.message};}};
test('Number conversion definitions and source behavior survive both codecs and all emitters',()=>{
  const program=checkProgram(parseJavaScript(fixture.source)),packet=encodeTypedProgramLinks(program),output=emitJavaScript(program);
  for(const actual of [packet,createStringNumberProgramLinks()])for(const target of ['JavaScript','Rust','Lean','Rocq']){
    const expected=emitTypedProgramLinks(actual.network,actual.root,target);
    for(const network of [LinkNetwork.fromLino(actual.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(actual.network))])assert.deepEqual(emitTypedProgramLinks(network,actual.root,target),expected);
  }
  const names=fixture.cases.map(row=>output.mappings.find(mapping=>mapping.kind==='function'&&mapping.source===row.entry).target);
  const functions=runInNewContext('"use strict";\n'+output.text.replace(/^export /gmu,'')+'\n['+names.join(',')+'];');
  for(const [index,row] of fixture.cases.entries()){
    const expected={kind:row.abort===null?'value':'abort',value:row.abort??row.value};
    assert.deepEqual(observe(()=>runInNewContext('"use strict";\n'+row.source.replace(/^export /gmu,'')+'\ncompute();',{}, {timeout:1000})),expected,row.name);
    assert.deepEqual(observe(()=>functions[index]()),expected,row.name);
  }
});
test('the exact numeric kernel agrees with independent Number for every transported UTF-16 input',()=>{
  for(const row of fixture.unitCases){
    const text=String.fromCharCode(...row.units),expected=Number(text);
    assert.equal(String(expected),row.value,row.name);
    assert.equal(Object.is(expected,-0),row.negativeZero,row.name);
    assert.ok(Object.is(convertJavaScriptStringNumber(text),expected),row.name);
    assert.ok(Object.is(convertJavaScriptStringNumberUnits(row.units),expected),row.name);
    if(row.bytes!==null)assert.ok(Object.is(convertJavaScriptUtf8StringNumberUnits(row.bytes),expected),row.name);
  }
  for(const units of [[-1],[65536],[1.5],[NaN],[Infinity]])assert.ok(Number.isNaN(convertJavaScriptStringNumberUnits(units)));
  for(const bytes of [[128],[192,128],[224,128,128],[237,160,128],[244,144,128,128],[240,159],[256]])assert.ok(Number.isNaN(convertJavaScriptUtf8StringNumberUnits(bytes)));
  assert.throws(()=>convertJavaScriptStringNumber(7),/actual string/u);
});
