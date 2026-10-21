// Complete original source fixtures. No body substitution or fake signatures.
import assert from 'node:assert/strict';
import { test } from 'node:test';
import { runInNewContext } from 'node:vm';
import { selfTranslate, selfTranslationSignatures } from '../src/self-translation.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { acceptJavaScriptStandardBuiltinOwner } from '../src/translation/javascript-record-parameter-rules.js';
import { encodeTypedProgramLinks, decodeTypedProgramLinks } from '../src/typed-program-links.js';
import { recheckLinkedTypedProgram } from '../src/typed-program-checked-projection.js';
import { LinkNetwork } from '../src/network.js';
import { encodeLinkNetworkBinary, decodeLinkNetworkBinary } from '../src/link-network-binary.js';
import { declarationSpecialGlobalInventory, readDeclarationSpecialGlobalSources,
  readDeclarationModuleOwnerFixtures } from '../scripts/declaration-module-owner-fixtures.mjs';

const inventory = declarationSpecialGlobalInventory;
const specialSources = readDeclarationSpecialGlobalSources();

function archives(network) {
  return [LinkNetwork.fromLino(network.toLino()), decodeLinkNetworkBinary(encodeLinkNetworkBinary(network))];
}
function versions() {
  const [major, minor] = process.versions.node.split('.').map(Number);
  assert.ok(major === 22 && minor >= 15 || major === 24,
    'uncovered Node ' + process.versions.node + '; supported >=22.15.0 <23 || >=24 <25');
}

test('shared complete owner fixtures preserve independent original behavior and source bytes', async () => {
  versions();
  let serial = 0;
  for (const fixture of readDeclarationModuleOwnerFixtures()) {
    if (fixture.sourceMode === 'Script') {
      if (Object.hasOwn(fixture, 'sourceProbeExpected')) {
        assert.equal(runInNewContext(fixture.source + '\n' + fixture.sourceProbe + ';'), fixture.sourceProbeExpected, fixture.id);
      } else {
        assert.throws(() => runInNewContext(fixture.source + '\n' + fixture.sourceProbe + ';'), error => error.name === fixture.sourceProbeError, fixture.id);
      }
    } else {
      // Execute the complete actual ES module, retaining restricted-global
      // shadowing semantics for names such as NaN/Infinity/undefined.
      const observation = Object.hasOwn(fixture, 'sourceProbeExpected')
        ? `\nexport const observation=(${fixture.sourceProbe});\n`
        : `\nexport const observation=(()=>{try{${fixture.sourceProbe};return '';}catch(error){return error.name;}})();\n`;
      const module = await import('data:text/javascript;charset=utf-8,' + encodeURIComponent(fixture.source + observation + `\n// fixture ${serial++}\n`));
      assert.equal(module.observation, fixture.sourceProbeExpected ?? fixture.sourceProbeError, fixture.id);
    }
    const translated = selfTranslate(fixture.source, fixture.language, 'Rust');
    assert.deepEqual(translated.items.filter(item => ['function_declaration', 'export_statement'].includes(item.term)).map(item => item.status), fixture.functionStatuses, fixture.id);
    assert.deepEqual(selfTranslationSignatures(fixture.source, fixture.language).map(signature => signature.name), fixture.signatures, fixture.id);
    assert.equal(selfTranslate(translated.code, 'Rust', fixture.language).code, fixture.source, fixture.id);
  }
});

test('the existing shared owner decision retains old cases and covers actual special-global sites', () => {
  versions();
  for (const name of inventory) {
    assert.equal(acceptJavaScriptStandardBuiltinOwner(name, false), true, name);
    assert.equal(acceptJavaScriptStandardBuiltinOwner(name, true), false, name);
  }
  assert.equal(acceptJavaScriptStandardBuiltinOwner('Unknown', false), false);
});

test('actual known outer names and unproved scope cannot select a special intrinsic', () => {
  versions();
  for (const [name, source] of specialSources) {
    assert.throws(() => checkProgram(parseJavaScript(source, { moduleBindingNames: [name] })), undefined, name + ' actual unavailable owner');
    assert.throws(() => checkProgram(parseJavaScript(source, { moduleScopeKnown: false })), undefined, name + ' unknown scope');
    // The original source and default unshadowed route remain meaningful.
    if (name === 'Object') {
      // The original product validator requires actual callers for a returned
      // immutable tuple. Retain that input and its independent refusal, then
      // add a real caller instead of claiming the isolated profile is broader.
      assert.throws(() => checkProgram(parseJavaScript(source)), /closed set of actual callers/u);
      assert.doesNotThrow(() => checkProgram(parseJavaScript(source + '\nfunction consume(){return selected()[0];}\n')));
    } else assert.doesNotThrow(() => checkProgram(parseJavaScript(source)), name + ' original standalone profile');
  }
});

test('unknown source scopes preserve independent parameter arithmetic and shadowed local values', () => {
  versions();
  const source = '/** @param {number} Map @returns {number} */function selected(Map){return Map+1;}';
  assert.equal(runInNewContext(source + '\nselected(7);'), 8);
  const checked = checkProgram(parseJavaScript(source, { moduleBindingNames: ['Map'], moduleScopeKnown: false }));
  const saved = encodeTypedProgramLinks(checked);
  const rust = emitRust(checked).text;
  for (const network of archives(saved.network)) {
    const replay = recheckLinkedTypedProgram(decodeTypedProgramLinks(network, saved.root), { wholeProgram: true });
    assert.equal(runInNewContext(emitJavaScript(replay).text + '\nselected(7);'), 8);
    assert.equal(emitRust(replay).text, rust);
  }
});

test('a known actual String signature is called, never normalized to the standard conversion', () => {
  versions();
  const source = '/** @returns {number} */function selected(){return String(7);}';
  const provider = '/** @param {number} value @returns {number} */export function increment(value){return value+1;}';
  const providerSignatures = selfTranslationSignatures(provider, 'JavaScript');
  assert.equal(providerSignatures.length, 1);
  // This is the actual signature of an original portable provider, renamed
  // exactly as the actual named import `increment as String` would bind it.
  const external = { ...providerSignatures[0], name: 'String' };
  const context = { externals: [external], moduleBindingNames: ['String'], moduleScopeKnown: false };
  const checked = checkProgram(parseJavaScript(source, context));
  assert.equal(checked.declarations.get('selected').body.k, 'call');
  assert.equal(checked.declarations.get('selected').body.fn, 'String');
  const independent = { String: value => value + 1 };
  assert.equal(runInNewContext(source + '\nselected();', independent), 8);
  const saved = encodeTypedProgramLinks(checked);
  for (const network of archives(saved.network)) {
    const replay = recheckLinkedTypedProgram(decodeTypedProgramLinks(network, saved.root), { wholeProgram: true });
    assert.equal(runInNewContext(emitJavaScript(replay).text + '\nselected();', { String: value => value + 1 }), 8);
    assert.equal(emitRust(replay).text, emitRust(checked).text);
  }
});

test('a carried actual empty collection class cannot disappear during ordinary or retry parsing', () => {
  versions();
  for (const name of ['Map', 'Set']) {
    const source = `class ${name} { /* inert actual declaration */ }\nfunction selected(){return new ${name}();}\nfunction independent(value){return value+1;}\n`;
    assert.equal(runInNewContext(source + `\nselected().constructor===${name};`), true);
    const translated = selfTranslate(source, 'JavaScript', 'Rust');
    const functions = translated.items.filter(item => item.term === 'function_declaration');
    assert.deepEqual(functions.map(item => item.status), ['carried', 'translated']);
    assert.doesNotMatch(translated.code, /pub fn selected\(/u);
    assert.equal(selfTranslate(translated.code, 'Rust', 'JavaScript').code, source);
  }
});

test('an unscanned original var constructor is not claimed as a complete outer-name inventory', () => {
  versions();
  const source = 'var Map = class {};\nfunction selected(){return new Map();}\nfunction independent(value){return value+1;}\n';
  assert.equal(runInNewContext(source + '\nselected().constructor===Map;'), true);
  assert.equal(runInNewContext(source + '\nselected() instanceof Map;'), true);
  const translated = selfTranslate(source, 'JavaScript', 'Rust');
  assert.deepEqual(translated.items.filter(item => item.term === 'function_declaration').map(item => item.status), ['carried', 'translated']);
  assert.doesNotMatch(translated.code, /pub fn selected\(/u);
  assert.equal(selfTranslate(translated.code, 'Rust', 'JavaScript').code, source);
});

test('actual class-owned numeric names cannot become false primitive literals through a retry', () => {
  versions();
  const source = 'class NaN {}\nexport const value=NaN;\nexport function selected(){return value;}\n';
  const translated = selfTranslate(source, 'JavaScript', 'Rust');
  assert.doesNotMatch(translated.code, /pub const value:|pub fn selected\(/u);
  assert.deepEqual(selfTranslationSignatures(source, 'JavaScript'), []);
  assert.equal(selfTranslate(translated.code, 'Rust', 'JavaScript').code, source);
});

test('the exact existing count/limit alongside empty-class signature oracle remains', () => {
  versions();
  const source = '/** @returns {number} */\nexport function count() { return 2; }\nexport const limit = 3;\nexport class Unsupported {}\n';
  assert.deepEqual(selfTranslationSignatures(source, 'JavaScript'), [
    { k: 'fn', name: 'count', params: [], ret: { kind: 'float' } },
    { k: 'const', name: 'limit', type: { kind: 'float' }, literal: true },
  ]);
});

test('malformed outer-name transports refuse instead of accepting absent facts', () => {
  versions();
  const sparse = [null]; delete sparse[0];
  for (const moduleBindingNames of [sparse, [undefined], [null], [''], 'Map']) {
    assert.throws(() => parseJavaScript('function selected(value){return value+1;}', { moduleBindingNames }), /module binding context/u);
  }
  assert.throws(() => parseJavaScript('function selected(value){return value+1;}', { moduleScopeKnown: 'false' }), /module binding context/u);
});
