import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import vm from 'node:vm';
import { checkProgram } from '../src/translation/check.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { encodeTypedProgramLinks, decodeTypedProgramLinks, emitTypedProgramLinks } from '../src/typed-program-links.js';
import { LinkNetwork } from '../src/network.js';
import { encodeLinkNetworkBinary, decodeLinkNetworkBinary } from '../src/link-network-binary.js';
import { selfTranslationSignatures } from '../src/self-translation.js';
import { restoreCurrentNativeHostSource } from '../scripts/current-native-host-source-history.mjs';

test('checked external calls and reads retain their owned signatures through both Links codecs', () => {
  for (const name of ['increment', 'String', '_value', '$increment', 'limit']) {
    const constant = name === 'limit';
    const provider = constant ? 'export const limit=7;'
      : '/** @param {number} value @returns {number} */export function increment(value){return value+1;}';
    const expression = constant ? 'limit+1' : `${name}(7)`;
    const source = `/** @returns {number} */function selected(){return ${expression};}`;
    const scope = provider.replace('export ', '') + (['increment', 'limit'].includes(name) ? '' : `\nconst ${name}=increment;`);
    const externals = selfTranslationSignatures(provider, 'JavaScript').map(row => ({ ...row, name }));
    const checked = checkProgram(parseJavaScript(source, { externals, moduleBindingNames: [name] }));
    const packet = encodeTypedProgramLinks(checked);
    assert.equal(vm.runInNewContext(`${scope}\n${source}\nselected();`, {}, { timeout: 1000 }), 8, name);
    for (const network of [
      LinkNetwork.fromLino(packet.network.toLino()),
      decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network)),
    ]) {
      const relations = network.fieldRelations();
      const external = relations.find(({ label }) => label === 'declaration reference');
      assert.ok(external, name);
      assert.ok(relations.some(({ parent, label, child }) => parent.equals(external.child)
        && label === 'name' && network.link(child).metadata().definition === name), name);
      assert.deepEqual(decodeTypedProgramLinks(network, packet.root), checked, name);
      const emitted = emitTypedProgramLinks(network, packet.root, 'JavaScript').code;
      assert.equal(vm.runInNewContext(`${scope}\n${emitted}\nselected();`, {}, { timeout: 1000 }), 8, name);
    }
  }
});

test('external Links references reject an incompatible owned signature', () => {
  const source = '/** @returns {number} */function selected(){return value(7);}';
  const externals = [
    { k: 'fn', name: 'value', params: [{ name: 'value', type: { kind: 'float' } }], ret: { kind: 'float' } },
    { k: 'fn', name: 'incompatible', params: [{ name: 'value', type: { kind: 'float' } }], ret: { kind: 'bool' } },
  ];
  const checked = checkProgram(parseJavaScript(source, { externals, moduleBindingNames: ['value', 'incompatible'] }));
  const packet = encodeTypedProgramLinks(checked);
  for (const network of [
    LinkNetwork.fromLino(packet.network.toLino()),
    decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network)),
  ]) {
    const relations = network.fieldRelations();
    const external = relations.find(({ label }) => label === 'declaration reference');
    const incompatible = relations.find(({ label, child }) => label === 'name'
      && network.link(child).metadata().definition === 'incompatible').parent;
    external.link.setReferences([external.parent, external.link.references()[1], incompatible]);
    assert.throws(() => decodeTypedProgramLinks(network, packet.root), /inconsistent call return type/u);
  }
});

test('an owned declaration shadows a same-named external signature', () => {
  const source = '/** @param {number} input @returns {number} */function value(input){return input+1;}\n'
    + '/** @returns {number} */function selected(){return value(7);}';
  const externals = [{ k: 'const', name: 'value', type: { kind: 'bool' }, literal: true }];
  const checked = checkProgram(parseJavaScript(source, { externals, moduleBindingNames: ['value', 'selected'] }));
  const packet = encodeTypedProgramLinks(checked);
  for (const network of [
    LinkNetwork.fromLino(packet.network.toLino()),
    decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network)),
  ]) {
    const relations = network.fieldRelations();
    const reference = relations.find(({ label }) => label === 'declaration reference');
    assert.ok(relations.some(({ parent, label, child }) => parent.equals(reference.child)
      && label === 'full name' && network.link(child).metadata().definition === 'value'));
    assert.deepEqual(decodeTypedProgramLinks(network, packet.root), checked);
    const emitted = emitTypedProgramLinks(network, packet.root, 'JavaScript').code;
    assert.equal(vm.runInNewContext(`${emitted}\nselected();`, {}, { timeout: 1000 }), 8);
  }
});

test('historical codec readers recover only the recorded predecessor sources', () => {
  for (const [path, previousHash] of [
    ['js/src/typed-program-links-validation.js', '6c2ea9bcb6266ef98af1b86708561dd8fd991e002f46c22dbfa047d90de39051'],
    ['rust/src/typed_program_links/checked.rs', 'ea6b9054929ad83be10af922b26238f5b2f19066c26b435773f99218306dcd10'],
  ]) {
    const source = readFileSync(new URL(`../../${path}`, import.meta.url), 'utf8');
    const previous = restoreCurrentNativeHostSource(source, path);
    assert.equal(createHash('sha256').update(previous).digest('hex'), previousHash, path);
    const changed = `${source}\n`;
    assert.equal(restoreCurrentNativeHostSource(changed, path), changed, path);
    assert.equal(restoreCurrentNativeHostSource(source, 'unregistered.rs'), source, path);
  }
});
