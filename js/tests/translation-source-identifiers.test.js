// Future complete controls. Never run during the whole requirements draft.
import assert from 'node:assert/strict';
import test from 'node:test';
import { runInNewContext } from 'node:vm';
import { readProjectData } from '../scripts/project-data.mjs';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { createJavaScriptRecordParameterProgramLinks } from '../src/javascript-record-parameter-program.js';
import { createSourceIdentifierProgramLinks } from '../src/source-identifier-program.js';
import { encodeTypedProgramLinks, emitTypedProgramLinks } from '../src/typed-program-links.js';
import { LinkNetwork } from '../src/network.js';
import { encodeLinkNetworkBinary, decodeLinkNetworkBinary } from '../src/link-network-binary.js';
import { readSourceIdentifierSupply } from '../src/translation/source-identifiers.js';
import { readSourceIdentifierPrefixOrdinal, readSourceIdentifierPrefix, renderSourceTargetIdentifier, renderSourceGeneratedIdentifier, acceptJavaScriptSourceBindingName } from '../src/translation/source-identifier-rules.js';
const fixture = readProjectData(new URL('../../parity/fixtures/source-identifiers.lino', import.meta.url));
const [major, minor] = process.versions.node.split('.').map(Number);
assert.ok(major === 22 && minor >= 15 || major === 24, 'uncovered Node ' + process.versions.node);
const observe = call => { try { return { kind: 'value', value: String(call()) }; } catch (error) { return { kind: 'abort', value: error.message }; } };
test('source identifiers remain capture free across checked graphs codecs four targets and native values', () => {
  const program = checkProgram(parseJavaScript(fixture.source)), packet = encodeTypedProgramLinks(program), js = emitJavaScript(program);
  for (const actual of [packet, createSourceIdentifierProgramLinks(), createJavaScriptRecordParameterProgramLinks()]) for (const target of ['JavaScript', 'Rust', 'Lean', 'Rocq']) {
    const expected = emitTypedProgramLinks(actual.network, actual.root, target);
    for (const network of [LinkNetwork.fromLino(actual.network.toLino()), decodeLinkNetworkBinary(encodeLinkNetworkBinary(actual.network))])
      assert.deepEqual(emitTypedProgramLinks(network, actual.root, target), expected);
  }
  const names = fixture.cases.map(row => js.mappings.find(mapping => mapping.kind === 'function' && mapping.source === row.entry).target);
  const functions = runInNewContext('"use strict";\n' + js.text.replace(/^export /gmu, '') + '\n[' + names.join(',') + '];');
  for (const [index, row] of fixture.cases.entries()) {
    const expected = { kind: row.abort === null ? 'value' : 'abort', value: row.abort ?? row.value };
    assert.deepEqual(observe(() => runInNewContext('"use strict";\n' + row.source.replace(/^export /gmu, '') + '\ncompute();', {}, { timeout: 1000 })), expected, row.name);
    assert.deepEqual(observe(() => functions[index]()), expected, row.name);
  }
  assert.ok(emitRust(program).text.includes('source_ml_reference_array1'));
  for (const row of fixture.historicalAssertions) {
    const source = row.source + '\nconsole.log(' + (row.source.includes('function probe') ? 'probe(19)' : 'f(17)') + ');';
    const observed = [];
    const context = {console:{log:value=>observed.push(String(value))}};
    runInNewContext(source, context, {timeout:1000});
    const expected = [...observed];observed.length=0;
    runInNewContext(emitJavaScript(checkProgram(parseJavaScript(source))).text, context, {timeout:1000});
    assert.deepEqual(observed, expected, row.message + ': retained input, corrected ECMAScript expectation');
  }
});
test('the shared namespace rules retain canonical ordinals gaps validation and target aliases', () => {
  for (const [text, expected] of [['ml_source1_a',1],['ml_source12_',12],['ml_source01_a',-1],['ml_source0_a',-1],['ml_source1',-1],['ml_source1x',-1],['ml_source9007199254740991_a',9007199254740991],['ml_source9007199254740992_a',-1]])
    assert.equal(readSourceIdentifierPrefixOrdinal(text),expected,text);
  assert.equal(readSourceIdentifierSupply(['ordinary']), 'ml_');
  assert.equal(readSourceIdentifierSupply(['ml_value','ml_source1_a','ml_source3_a']), 'ml_source2_');
  assert.equal(readSourceIdentifierSupply(['ml_source1_a','ml_source2_a','ml_source2_b']), 'ml_source3_');
  for (const ordinals of [[0],[-1],[1,0],[2,1],[2,NaN],[Infinity],[9007199254740992]])
    assert.throws(()=>readSourceIdentifierPrefix(true,ordinals),/sorted positive integer/u);
  assert.equal(renderSourceTargetIdentifier('ml_array'),'source_ml_array');
  assert.equal(renderSourceTargetIdentifier('source_ml_array'),'source_ml_array');
  assert.equal(renderSourceGeneratedIdentifier('ml_source2_','ml_array_test',3),'ml_source2_array_test3');
  for (const ordinal of [-1,0.5,NaN,Infinity,9007199254740992])assert.throws(()=>renderSourceGeneratedIdentifier('ml_','a',ordinal),/exact range/u);
});
test('ordinary source pattern names and closed historical names keep distinct profiles',()=>{
  const reserved=['class','arguments','eval','await','yield'];
  assert.equal(acceptJavaScriptSourceBindingName('identifier','ml_hidden',false,reserved),true);
  for (const word of reserved)assert.equal(acceptJavaScriptSourceBindingName('identifier',word,false,reserved),false);
  assert.equal(acceptJavaScriptSourceBindingName('identifier','ml_hidden',true,reserved),false);
  assert.equal(acceptJavaScriptSourceBindingName('string','ml_hidden',false,reserved),false);
});
