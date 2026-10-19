import assert from 'node:assert/strict';
import {runInNewContext} from 'node:vm';
import test from 'node:test';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {emitRust} from '../src/translation/emit-rust.js';
import {createTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {selfTranslate} from '../src/self-translation.js';
import {generateNumberPredicateRules} from '../scripts/number-predicate-generation.mjs';

const observe=source=>{const lines=[];runInNewContext(source,{console:{log:(...values)=>lines.push(values.map(String).join(' '))}});return lines;};

test('all Number predicates preserve known nonnumeric values nullable operands and missing arguments without coercion',()=>{
  const {fixture}=generateNumberPredicateRules({check:true});assert.equal(fixture.cases.length,80);
  const values=["'1'","'NaN'",'true','false','null','undefined','1n','[1,2]','{value:1}', 'new Map([[1,2]])','new Set([1,2])','() => 1','0','-0','1','1.5','NaN','Infinity','9007199254740992'];
  const calls=['isInteger','isSafeInteger','isFinite','isNaN'].flatMap(op=>values.map(value=>'console.log(Number.'+op+'('+value+'));').concat('console.log(Number.'+op+'());'));
  const source=calls.join('\n');
  assert.equal(source,fixture.cases.map(row=>'console.log(Number.'+row.operation+'('+row.expression+'));').join('\n'));
  assert.deepEqual(observe(source),fixture.cases.map(row=>String(row.expected)));
  const checked=checkProgram(parseJavaScript(source));assert.deepEqual(observe(emitJavaScript(checked).text),observe(source));
  const nullable="/** @param {string | null | undefined} value @returns {boolean} */function check(value){return Number.isNaN(value);}console.log(check('NaN'),check(null),check(undefined));";
  assert.deepEqual(observe(emitJavaScript(checkProgram(parseJavaScript(nullable))).text),observe(nullable));
  const inferred='function check(value){return Number.isInteger(value);}console.log(check(3));';
  assert.deepEqual(observe(emitJavaScript(checkProgram(parseJavaScript(inferred))).text),observe(inferred));
});

test('ignored Number predicate arguments retain actual ordered effects complete archives and source bytes',()=>{
  const source="/** @returns {number} */function number(){console.log('number');return 2;}/** @returns {string} */function text(){console.log('text');return 'NaN';}/** @returns {boolean} */function extra(){console.log('extra');return true;}console.log(Number.isInteger(number(),text(),extra()));console.log(Number.isNaN(text(),number(),extra()));";
  const checked=checkProgram(parseJavaScript(source)),expected=observe(source),native=emitRust(checked).text;
  assert.deepEqual(observe(emitJavaScript(checked).text),expected);
  const packet=createTypedProgramLinks(source,'JavaScript');
  for(const network of [LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]){
    assert.deepEqual(observe(emitTypedProgramLinks(network,packet.root,'JavaScript').code),expected);
    assert.equal(emitTypedProgramLinks(network,packet.root,'Rust').code,native);
  }
  const original=selfTranslate(source,'JavaScript','Rust');
  assert.equal(original.items.filter(row=>row.status==='carried'&&row.reason==='top-level statement').length,2,'retain both original top-level statements and the documented module translation boundary');
  assert.equal(original.items.filter(row=>row.status==='carried'&&row.reason==='comment the target cannot hold').length,3,'retain all three original separately carried documentation comments');
  assert.equal(selfTranslate(original.code,'Rust','JavaScript').code,source);
  const complete=source.replace("console.log(Number.isInteger(number(),text(),extra()));console.log(Number.isNaN(text(),number(),extra()));","function main(){console.log(Number.isInteger(number(),text(),extra()));console.log(Number.isNaN(text(),number(),extra()));}");
  const translated=selfTranslate(complete,'JavaScript','Rust');assert.equal(translated.items.filter(row=>row.status==='carried').length,0);
  assert.equal(selfTranslate(translated.code,'Rust','JavaScript').code,complete);
  assert.throws(()=>checkProgram(parseJavaScript('console.log(Number.isInteger(...[1]));')),/spread argument/u);
});
