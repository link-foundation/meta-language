// Preserve original nullable-array observations through inference and both Links codecs.
import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {emitRust} from '../src/translation/emit-rust.js';
import {typeKey} from '../src/translation/types.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {encodeTypedProgramLinks,decodeTypedProgramLinks} from '../src/typed-program-links.js';
import {recheckLinkedTypedProgram} from '../src/typed-program-checked-projection.js';
const [major,minor]=process.versions.node.split('.').map(Number);
assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; supported >=22.15.0 <23 || >=24 <25');
const printed=source=>{const lines=[];vm.runInNewContext(source,{console:{log:value=>lines.push(String(value))}},{timeout:1000});return lines;};
const archives=packet=>[LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))];

test('the complete original CI nullable array fixture preserves every independent output through both codecs',()=>{
  // Keep every input from translation-structural-inference.test.js:142 intact.
  const source=`function numberFirst() { return [1, null, undefined, -0].join('|'); }
function absenceFirst() { return [null, undefined, 1, -0].join('|'); }
function spreadAbsence() { return [1, ...[null, undefined], -0].join('|'); }
function nullableBigints() { return [1n, null, 3n].join('|'); }
console.log(numberFirst());
console.log(absenceFirst());
console.log(spreadAbsence());
console.log(nullableBigints());`;
  const expected=['1|||0','||1|0','1|||0','1||3'];assert.deepEqual(printed(source),expected);
  const checked=checkProgram(parseJavaScript(source)),packet=encodeTypedProgramLinks(checked);
  assert.deepEqual(printed(emitJavaScript(checked).text),expected);
  for(const network of archives(packet)){
    const program=recheckLinkedTypedProgram(decodeTypedProgramLinks(network,packet.root),{wholeProgram:true});
    assert.deepEqual(printed(emitJavaScript(program).text),expected);
    assert.ok(emitRust(program,{arrayConsumption:true}).text.includes('fn main()'));
  }
});

test('known inline and named nullable array owners retain their complete element type before checking',()=>{
  for(const body of ['return [1,...[null,undefined],-0];',
    'const absent=[null,undefined];return [1,...absent,-0];']){
    const source='function kept(){'+body+'}console.log(kept().join("|"));';
    const checked=checkProgram(parseJavaScript(source));
    assert.equal(typeKey(checked.declarations.get('kept').ret),'array:option:both:float');
    assert.deepEqual(printed(emitJavaScript(checked).text),['1|||0']);
  }
});

test('actual string spreads still select code point iteration including isolated UTF16 units',()=>{
  const source='/** @param {string} value @returns {string[]} */function points(value){return [...value];}';
  const observation='\nconsole.log(JSON.stringify(points("A😀\\ud800Z")));';
  const expected=['["A","😀","\\ud800","Z"]'];
  assert.deepEqual(printed(source+observation),expected);
  const checked=checkProgram(parseJavaScript(source)),packet=encodeTypedProgramLinks(checked);
  assert.deepEqual(printed(emitJavaScript(checked).text+observation),expected);
  for(const network of archives(packet)){
    const program=recheckLinkedTypedProgram(decodeTypedProgramLinks(network,packet.root),{wholeProgram:true});
    assert.deepEqual(printed(emitJavaScript(program).text+observation),expected);
  }
});
