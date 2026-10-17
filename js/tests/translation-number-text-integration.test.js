import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { inspect } from 'node:util';
import vm from 'node:vm';
import test from 'node:test';
import { createNumberTextIntegrationArchive } from '../../experiments/issue-199-number-text-integration-archive.mjs';
import {
  renderBinaryDoublePrecisionBits, renderJavaScriptNumberBits,
  renderNodeConsoleNumberBits, renderRustDefaultNumberBits,
} from '../../experiments/issue-199-number-text-integration-rules.mjs';
import { decimalDraftValues } from '../../experiments/issue-199-check-decimal-draft.mjs';
import { formatBinaryDoublePrecisionValue } from '../../experiments/issue-199-binary-double-precision-format.mjs';
import { LinkNetwork } from '../src/network.js';
import {
  createTypedProgramLinks, decodeTypedProgramLinks, emitTypedProgramLinks,
  TYPED_PROGRAM_LINKS_SCHEMA as schema,
} from '../src/typed-program-links.js';

const models = ['ecmascript-number-text', 'node-console-number-text', 'rust-default-binary-double-precision-text'];
const names = ['renderJavaScriptNumberBits', 'renderNodeConsoleNumberBits', 'renderRustDefaultNumberBits'];
const entries = [renderJavaScriptNumberBits, renderNodeConsoleNumberBits, renderRustDefaultNumberBits];
const inputs = decimalDraftValues();
const view = new DataView(new ArrayBuffer(8));
const bitsOf = value => { view.setFloat64(0, value, false); return view.getBigUint64(0, false); };
const expected = (value, model) => model === models[0] ? String(value)
  : model === models[1] ? inspect(value) : formatBinaryDoublePrecisionValue(value, model);
let archivedText, archivedRoot;

function reloadAlgorithm() {
  if (archivedText === undefined) {
    const archive = createNumberTextIntegrationArchive();
    archivedText = archive.network.toLino();
    archivedRoot = archive.root;
    assert.ok(Buffer.byteLength(archivedText) < 1024 * 1024, 'the complete closed integration draft stays bounded');
  }
  return { network: LinkNetwork.fromLino(archivedText), root: archivedRoot };
}

function execute(archive) {
  const context = vm.createContext({});
  vm.runInContext(emitTypedProgramLinks(archive.network, archive.root, 'JavaScript').code, context, { timeout: 1000 });
  return context;
}

function field(network, parent, name) {
  const relation = network.fieldRelations().find(item => item.parent.equals(parent)
    && item.label === (schema.fields[name] ?? name));
  assert.ok(relation, `missing ${name} relation`);
  return relation.child;
}

test('portable bit extraction preserves every existing decimal pattern and all three source text models', () => {
  assert.equal(inputs.length, 92);
  for (const value of inputs) {
    const bits = bitsOf(value);
    for (let index = 0; index < models.length; index += 1) {
      assert.equal(renderBinaryDoublePrecisionBits(bits, models[index]), expected(value, models[index]), `${bits.toString(16)} ${models[index]}`);
      assert.equal(entries[index](bits), expected(value, models[index]));
    }
  }
  assert.deepEqual(entries.map(entry => entry(0x8000000000000000n)), ['0', '-0', '-0']);
  assert.deepEqual(entries.map(entry => entry(0x7ff0000000000000n)), ['Infinity', 'Infinity', 'inf']);
  assert.deepEqual(entries.map(entry => entry(0xfff0000000000000n)), ['-Infinity', '-Infinity', '-inf']);
  assert.equal(renderJavaScriptNumberBits(bitsOf(1 + 2 ** -17)), '1.0000076293945312');
  assert.equal(renderRustDefaultNumberBits(bitsOf(1 + 2 ** -17)), '1.0000076293945313');
});

test('reloaded typed links own the complete extraction and decimal algorithms before emitting both targets', () => {
  const archive = reloadAlgorithm();
  const program = decodeTypedProgramLinks(archive.network, archive.root);
  assert.equal(program.declarations.size, 26, 'the original 22 declarations plus four bit-pattern entry points');
  assert.equal(program.externals, undefined, 'the linked algorithm is closed over its actual declarations');
  assert.equal(program.imports, undefined, 'the actual relative import was resolved before serialization');
  for (const name of [...names, 'renderBinaryDoublePrecisionBits', 'renderFiniteBinaryDoublePrecision']) {
    assert.ok(program.declarations.has(name), name);
  }
  for (const name of names) {
    const body = program.declarations.get(name).body;
    assert.equal(body.k, 'call');
    assert.equal(body.fn, 'renderBinaryDoublePrecisionBits');
    assert.equal(body.args[1].type.kind, 'string', 'the selected source model is ordinary editable linked data');
  }
  assert.ok(archive.network.fieldRelations().filter(({ label }) => label === schema.references.declaration).length > 3);
  const sharedSource = readFileSync(new URL('../../experiments/issue-199-decimal-number-rules.mjs', import.meta.url), 'utf8');
  const entrySource = readFileSync(new URL('../../experiments/issue-199-number-text-integration-rules.mjs', import.meta.url), 'utf8');
  assert.ok(archive.network.links().every(link => ![sharedSource, entrySource].includes(link.metadata().definition)), 'source files are absent from the semantic graph');
  const rust = emitTypedProgramLinks(archive.network, archive.root, 'Rust').code;
  for (const name of ['render_binary_double_precision_bits', 'render_java_script_number_bits',
    'render_node_console_number_bits', 'render_rust_default_number_bits']) assert.match(rust, new RegExp(`pub fn ${name}\\(`, 'u'));
  assert.doesNotMatch(rust, /format!\("\{(?::e|:\.800e)\}"/u, 'decimal selection no longer calls the native float formatter');
  const context = execute(archive);
  for (const value of inputs) {
    for (let index = 0; index < models.length; index += 1) {
      assert.equal(context[names[index]](bitsOf(value)), expected(value, models[index]), `${bitsOf(value).toString(16)} ${models[index]}`);
    }
  }
});

test('editing a source-free model argument changes one consumer without changing shared decimal decisions', () => {
  const archive = reloadAlgorithm();
  const declaration = archive.network.fieldRelations().find(relation => relation.label === (schema.fields.fullName ?? 'full name')
    && archive.network.link(relation.child).metadata().definition === names[0])?.parent;
  assert.ok(declaration);
  const argumentsId = field(archive.network, field(archive.network, declaration, 'body'), 'args');
  const literal = archive.network.link(argumentsId).references()[1];
  const value = field(archive.network, literal, 'value');
  const link = archive.network.link(value);
  assert.equal(link.metadata().definition, models[0]);
  link.setMetadata(link.metadata().withDefinition(models[2]));
  const sourceFree = { network: LinkNetwork.fromLino(archive.network.toLino()), root: archive.root };
  const context = execute(sourceFree);
  for (const input of inputs) {
    const bits = bitsOf(input);
    assert.equal(context[names[0]](bits), expected(input, models[2]));
    assert.equal(context[names[1]](bits), expected(input, models[1]));
  }
  assert.equal(context[names[0]](0x8000000000000000n), '-0');
  assert.equal(context[names[0]](bitsOf(1e21)), '1000000000000000000000');
});

test('the closed draft retains invalid bit domains and NaN payload decisions explicitly', () => {
  const context = execute(reloadAlgorithm());
  for (const entry of [renderBinaryDoublePrecisionBits, context.renderBinaryDoublePrecisionBits]) {
    for (const bits of [-1n, 18446744073709551616n]) assert.equal(entry(bits, models[0]), '');
    assert.equal(entry(0n, 'undeclared-source-format'), '');
    for (const bits of [0x7ff0000000000001n, 0x7ff8000000000000n, 0xfff8000000000001n]) {
      for (const model of models) assert.equal(entry(bits, model), 'NaN');
    }
  }
});

test('a bit-pattern integration draft preserves the existing production Rust float admission boundary', () => {
  assert.throws(() => createTypedProgramLinks('pub fn text(value: f64) -> String { value.to_string() } fn main() {}', 'Rust'), /Rust Float display/u);
  assert.throws(() => createTypedProgramLinks('fn main() { println!("{}", -0.0f64); }', 'Rust'), /Rust Float display/u);
});
