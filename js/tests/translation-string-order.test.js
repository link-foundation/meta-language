import assert from 'node:assert/strict';
import {runInNewContext} from 'node:vm';
import test from 'node:test';
import {generateStringOrderRules} from '../scripts/string-order-generation.mjs';
import {parseJavaScript} from '../src/translation/javascript.js';
import {parseRust} from '../src/translation/rust.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {emitRust} from '../src/translation/emit-rust.js';
import {emitLean} from '../src/translation/emit-lean.js';
import {emitRocq} from '../src/translation/emit-rocq.js';
import {createTypedProgramLinks,decodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {recheckLinkedTypedProgram} from '../src/typed-program-checked-projection.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {selfTranslate} from '../src/self-translation.js';
import {readProjectData} from '../scripts/project-data.mjs';

const source=`/** @param {string} left @param {string} right @returns {boolean} */
function before(left,right){return left<right;}
/** @param {string} left @param {string} right @returns {boolean} */
function atMost(left,right){return left<=right;}
/** @param {string} left @param {string} right @returns {boolean} */
function after(left,right){return left>right;}
/** @param {string} left @param {string} right @returns {boolean} */
function atLeast(left,right){return left>=right;}`;
const names={lt:'before',le:'atMost',gt:'after',ge:'atLeast'};

test('all primitive string relations retain exact UTF16 order including every isolated unit and supplementary boundary',()=>{
  const {fixture}=generateStringOrderRules({check:true});assert.equal(fixture.units.length,27);assert.equal(fixture.cases.length,2916);
  const program=checkProgram(parseJavaScript(source));
  const operations=runInNewContext(emitJavaScript(program).text+';({before,atMost,after,atLeast});');
  const values=fixture.units.map(units=>String.fromCharCode(...units));
  for(const row of fixture.cases)assert.equal(operations[names[row.operator]](values[row.left],values[row.right]),row.expected,JSON.stringify(row));
  for(let unit=0;unit<=65535;unit++){
    const value=String.fromCharCode(unit),boundary=String.fromCharCode(0xd800);
    assert.equal(operations.before(value,boundary),unit<0xd800);
    assert.equal(operations.atMost(value,boundary),unit<=0xd800);
    assert.equal(operations.after(value,boundary),unit>0xd800);
    assert.equal(operations.atLeast(value,boundary),unit>=0xd800);
  }
  assert.equal(operations.before('😀','\ue000'),true,'UTF16 code unit order differs from Unicode scalar and UTF8 order');
  const native=emitRust(program);
  assert.ok(native.encodings.some(row=>row.id==='utf16-strings'));
  for(const method of ['lt','le','gt','ge'])assert.ok(native.text.includes('<crate::ml_string::Text as std::cmp::PartialOrd>::'+method+'('));
});

test('explicit string order models retain operand evaluation order and recheck through complete source free archives',()=>{
  const ordered="/** @returns {string} */function left(){console.log('left');return 'A';}/** @returns {string} */function right(){console.log('right');return 'B';}console.log(left()<right());";
  for(const text of [ordered,emitJavaScript(checkProgram(parseJavaScript(ordered))).text]){
    const lines=[];runInNewContext(text,{console:{log:value=>lines.push(String(value))}});assert.deepEqual(lines,['left','right','true']);
  }
  const packet=createTypedProgramLinks(source,'JavaScript'),native=emitRust(checkProgram(parseJavaScript(source))).text;
  for(const network of [LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]){
    const restored=decodeTypedProgramLinks(network,packet.root);recheckLinkedTypedProgram(restored,{wholeProgram:true});
    assert.ok(JSON.stringify(restored.items).includes('unicodeCodeUnitLessThan'));
    assert.equal(emitTypedProgramLinks(network,packet.root,'Rust').code,native);
    const operations=runInNewContext(emitTypedProgramLinks(network,packet.root,'JavaScript').code+';({before,atMost,after,atLeast});');
    assert.equal(operations.before('😀','\ue000'),true);assert.equal(operations.atMost('a','a'),true);assert.equal(operations.after('aa','a'),true);
  }
  for(const emit of [emitLean,emitRocq])assert.throws(()=>emit(checkProgram(parseJavaScript(source))),/UTF-16 operational string encoding for lexical comparison/u);
  const nativeSource='fn before(left: String,right: String)->bool {left<right}';
  assert.throws(()=>parseRust(nativeSource),/needs fn main/u,'retain the original incomplete program and its actual parser boundary');
  assert.throws(()=>checkProgram(parseRust(nativeSource+' fn main() {}')),/ordering on string/u,'native UTF8 String order remains a distinct unsupported source model');
  const malformed=parseJavaScript('/** @returns {boolean} */function before(){return true;}');
  malformed.items[0].body={k:'stringTest',op:'unknown',object:{k:'str',value:'a'},search:{k:'str',value:'b'}};
  assert.throws(()=>checkProgram(malformed),/no checked model/u);
});

test('the complete original integer ordering draft now translates without carried items and restores every source byte',()=>{
  const original=readProjectData(new URL('../../parity/fixtures/grammar-outcome-integer-decimal-ordering-source.lino',import.meta.url));
  const translated=selfTranslate(original.source,'JavaScript','Rust');
  assert.equal(translated.items.filter(row=>row.status==='carried').length,0);
  assert.equal(selfTranslate(translated.code,'Rust','JavaScript').code,original.source);
  const fixture=readProjectData(new URL('../../parity/fixtures/grammar-outcome-integer-decimal-rules.lino',import.meta.url));
  const operation=runInNewContext(emitJavaScript(checkProgram(parseJavaScript(original.source,{referenceSemantics:false}))).text+';acceptOwnedOutcomeIntegerDecimal;');
  for(const row of fixture.cases)assert.equal(operation(row.text,row.signed,row.width),row.expected,JSON.stringify(row));
});
