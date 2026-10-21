import assert from 'node:assert/strict';
import { test } from 'node:test';
import { runInNewContext } from 'node:vm';
import { selfTranslationSignatures } from '../src/self-translation.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { acceptJavaScriptExternalBinding, renderJavaScriptExternalCall } from '../src/translation/external-emission-rules.js';
import { encodeTypedProgramLinks, decodeTypedProgramLinks } from '../src/typed-program-links.js';
import { recheckLinkedTypedProgram } from '../src/typed-program-checked-projection.js';
import { LinkNetwork } from '../src/network.js';
import { encodeLinkNetworkBinary, decodeLinkNetworkBinary } from '../src/link-network-binary.js';

function versions() {
  const [major, minor] = process.versions.node.split('.').map(Number);
  assert.ok(major === 22 && minor >= 15 || major === 24,
    'uncovered Node ' + process.versions.node + '; supported >=22.15.0 <23 || >=24 <25');
}

function replayed(program) {
  const saved = encodeTypedProgramLinks(program);
  return [program, ...[LinkNetwork.fromLino(saved.network.toLino()),
    decodeLinkNetworkBinary(encodeLinkNetworkBinary(saved.network))].map(network =>
    recheckLinkedTypedProgram(decodeTypedProgramLinks(network, saved.root), { wholeProgram: true }))];
}

test('actual provider calls retain target bindings and survive both semantic archives', () => {
  versions();
  for (const name of ['increment', 'String', '_value', '$increment']) {
    const provider = '/** @param {number} value @returns {number} */export function increment(value){return value+1;}';
    const source = '/** @param {number} $value @returns {number} */function selected($value){return ' + name + '($value);}';
    const signatures = selfTranslationSignatures(provider, 'JavaScript');
    assert.equal(signatures.length, 1, name);
    // The complete provider supplies the signature; the actual import alias
    // supplies only its local spelling, as `increment as String` would.
    const externals = [{ ...signatures[0], name }];
    const providerScope = provider.replace('export ', '') + (name === 'increment' ? '' : '\nconst ' + name + '=increment;');
    const program = checkProgram(parseJavaScript(source, { externals, moduleBindingNames: [name] }));
    const original = providerScope + '\n' + source + '\nselected(7);';
    assert.equal(runInNewContext(original), 8, name);
    const rust = emitRust(program).text;
    for (const replay of replayed(program)) {
      const emitted = emitJavaScript(replay);
      const scope = runInNewContext(providerScope + '\n({' + name + '});');
      assert.equal(runInNewContext(emitted.text + '\nselected(7);', scope), 8, name);
      assert.equal(emitRust(replay).text, rust, name);
    }
  }
});

test('the original builtin-spelled provider retains its narrower signature admission', () => {
  versions();
  const provider = '/** @param {number} value @returns {number} */export function String(value){return value+1;}';
  assert.equal(runInNewContext(provider.replace('export ', '') + '\nString(7);'), 8);
  // Existing declaration-name validation refuses this source spelling. The
  // supported actual import alias above does not invent a provider signature.
  assert.deepEqual(selfTranslationSignatures(provider, 'JavaScript'), []);
});

test('an actual provider constant is read without invoking its value', () => {
  versions();
  const provider = 'export const limit=7;';
  const source = '/** @returns {number} */function selected(){return limit+1;}';
  const externals = selfTranslationSignatures(provider, 'JavaScript');
  assert.equal(externals.length, 1);
  assert.equal(externals[0].k, 'const');
  const program = checkProgram(parseJavaScript(source, { externals }));
  assert.equal(runInNewContext(provider.replace('export ', '') + '\n' + source + '\nselected();'), 8);
  for (const replay of replayed(program)) {
    const emitted = emitJavaScript(replay).text;
    assert.doesNotMatch(emitted, /limit\(\)/u);
    assert.equal(runInNewContext(emitted + '\nselected();', { limit: 7 }), 8);
  }
});

test('strict target binding policy keeps builtin spellings and refuses unsupported aliases', () => {
  versions();
  for (const name of ['String', 'NaN', 'Infinity', 'undefined', 'Array', '_value', '$increment', 'name2']) {
    assert.equal(acceptJavaScriptExternalBinding(name), true, name);
    assert.equal(renderJavaScriptExternalCall('fn', name, 'first(), second()'), name + '(first(), second())');
    assert.equal(renderJavaScriptExternalCall('const', name, ''), name);
  }
  for (const name of ['', '2name', 'a.b', 'a-b', 'class', 'await', 'eval', 'arguments', 'é']) {
    assert.equal(acceptJavaScriptExternalBinding(name), false, name);
    assert.equal(renderJavaScriptExternalCall('fn', name, ''), '', name);
  }
  assert.equal(renderJavaScriptExternalCall('const', 'limit', 'argument()'), '');
  assert.equal(renderJavaScriptExternalCall('unknown', 'limit', ''), '');
});
