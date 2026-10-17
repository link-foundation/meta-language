import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import test from 'node:test';
import vm from 'node:vm';
import { LinkNetwork } from '../src/network.js';
import { encodeLinkNetworkBinary, decodeLinkNetworkBinary } from '../src/link-network-binary.js';
import { createTypedProgramLinks, decodeTypedProgramLinks, encodeTypedProgramLinks, emitTypedProgramLinks } from '../src/typed-program-links.js';
import { projectLinkedProgramExpression } from '../src/typed-program-expression-projection.js';
import { recheckLinkedTypedProgram } from '../src/typed-program-checked-projection.js';
import { acceptProgramDerivedNumberOperand } from '../src/typed-program-number-projection-rules.js';
import { readLinkedProgramNumberOperand } from '../src/typed-program-number-projection.js';
import { readProgramNumberProjectionSourceModule, PROGRAM_NUMBER_PROJECTION_SOURCE } from '../scripts/generate-program-number-projection-rules.mjs';
import { readProjectData } from '../scripts/project-data.mjs';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';

const source = PROGRAM_NUMBER_PROJECTION_SOURCE;
const forms = packet => [LinkNetwork.fromLino(packet.network.toLino()), decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))];
function observe(code) {
  const output = [];
  vm.runInNewContext(code, { console: { log: value => output.push(String(value)) } }, { timeout: 1000 });
  return output;
}
function arithmetic(program) {
  const seen = new WeakSet();
  function visit(value) {
    if (value === null || typeof value !== 'object' || seen.has(value)) return null;
    seen.add(value);
    if (value.k === 'binary' && value.left?.k === 'nullishNumber') return value;
    for (const child of value instanceof Map ? value.values() : Object.values(value)) {
      const found = visit(child);
      if (found) return found;
    }
    return null;
  }
  const value = visit(program);
  assert.ok(value, 'the actual checker produced an optional-number arithmetic operand');
  return value;
}

test('whole checked arithmetic rederives its actual optional-number operand after text and binary reload', () => {
  assert.deepEqual(observe(source), ['7']);
  const packet = createTypedProgramLinks(source, 'JavaScript');
  for (const network of forms(packet)) {
    const program = decodeTypedProgramLinks(network, packet.root), before = structuredClone(program);
    assert.equal(arithmetic(program).left.arg.type.kind, 'option');
    for (let repeat = 0; repeat < 2; repeat += 1) {
      assert.equal(recheckLinkedTypedProgram(program, { wholeProgram: true }), program);
      assert.deepEqual(program, before);
    }
    assert.deepEqual(observe(emitTypedProgramLinks(network, packet.root, 'JavaScript').code), ['7']);
    const operation = arithmetic(program);
    assert.equal(operation.right.value, '2');
    operation.right.value = '7';
    assert.equal(recheckLinkedTypedProgram(program, { wholeProgram: true }), program);
    const edited = encodeTypedProgramLinks(program);
    for (const saved of forms(edited)) {
      recheckLinkedTypedProgram(decodeTypedProgramLinks(saved, edited.root), { wholeProgram: true });
      assert.deepEqual(observe(emitTypedProgramLinks(saved, edited.root, 'JavaScript').code), ['12']);
    }
  }
});

test('a derived numeric wrapper cannot become a root or hide changed operands, types or ownership fields', () => {
  const packet = createTypedProgramLinks(source, 'JavaScript');
  for (const [index, change] of [
    (program, operation) => { program.declarations.get('probe').body = operation.left; },
    (_program, operation) => { operation.left.type = { kind: 'bool' }; },
    (_program, operation) => { operation.left.borrowEscape = true; },
    (_program, operation) => { operation.left.arg = { k: 'lit', type: { kind: 'float' }, value: '5' }; },
    (_program, operation) => { operation.left.arg.type = { kind: 'option', element: { kind: 'string' }, absence: 'undefined' }; },
    (_program, operation) => { operation.op = 'concat'; },
  ].entries()) {
    const program = decodeTypedProgramLinks(LinkNetwork.fromLino(packet.network.toLino()), packet.root);
    const operation = arithmetic(program);
    change(program, operation);
    // The shared numberValue projection can reconstruct the wrapper, but this
    // original root edit removes the Let that binds values. Preserve that
    // source-scope refusal rather than treating every root wrapper as valid.
    if (index === 0) {
      assert.throws(() => recheckLinkedTypedProgram(program, { wholeProgram: true }), /^TranslationError: unknown name values at 106\.\.112$/u);
      continue;
    }
    if (index === 5) {
      assert.throws(() => recheckLinkedTypedProgram(program, { wholeProgram: true }), /^TranslationError: expected string but found float at 106\.\.117$/u);
      continue;
    }
    assert.throws(() => recheckLinkedTypedProgram(program, { wholeProgram: true }), /derived number operand|no surface projection|inconsistent types, bindings or operation semantics/u);
  }
  const program = decodeTypedProgramLinks(packet.network, packet.root);
  const wrapper = arithmetic(program).left;
  const projected = projectLinkedProgramExpression(wrapper);
  assert.equal(projected.k, 'numberValue');
  assert.equal(projected.arg.k, 'index');
  assert.equal(projected.arg.optional, true);
  for (const operation of ['add', 'sub', 'mul', 'div', 'rem']) for (const field of ['left', 'right']) {
    assert.equal(acceptProgramDerivedNumberOperand('binary', operation, field), true);
  }
  for (const [kind, operation, field] of [['binary', 'concat', 'left'], ['binary', 'add', 'arg'], ['unary', 'neg', 'arg'], ['nullishNumber', '', 'arg']]) {
    assert.equal(acceptProgramDerivedNumberOperand(kind, operation, field), false);
    assert.strictEqual(readLinkedProgramNumberOperand({ k: kind, op: operation }, field, wrapper), wrapper);
  }
  const bound = createTypedProgramLinks('/** @param {number[]} values @returns {number} */ function probe(values){return values[0]+2;}console.log(probe([5]));', 'JavaScript');
  for (const network of forms(bound)) {
    const program = decodeTypedProgramLinks(network, bound.root);
    assert.equal(program.declarations.get('probe').params[0].name, 'values');
    program.declarations.get('probe').body = arithmetic(program).left;
    assert.strictEqual(recheckLinkedTypedProgram(program, { wholeProgram: true }), program);
    assert.deepEqual(observe(emitJavaScript(program).text), ['5']);
    const edited = encodeTypedProgramLinks(program);
    for (const saved of forms(edited)) {
      const restored = decodeTypedProgramLinks(saved, edited.root);
      assert.deepEqual(restored, program);
      assert.strictEqual(recheckLinkedTypedProgram(restored, { wholeProgram: true }), restored);
      assert.deepEqual(observe(emitTypedProgramLinks(saved, edited.root, 'JavaScript').code), ['5']);
    }
  }
});

test('the complete portable numeric projection module checks and reloads without extracting declarations', () => {
  const source = readProgramNumberProjectionSourceModule();
  const declarations = [...source.matchAll(/^export function (\w+)\(/gmu)].map(match => match[1]);
  assert.deepEqual(declarations, ['acceptProgramDerivedNumberOperand']);
  const checked = checkProgram(parseJavaScript(source, { referenceSemantics: false }));
  assert.deepEqual([...checked.declarations.keys()], declarations);
  const packet = encodeTypedProgramLinks(checked);
  for (const program of [checked, ...forms(packet).map(network => decodeTypedProgramLinks(network, packet.root))]) {
    assert.equal(recheckLinkedTypedProgram(program, { wholeProgram: true }), program);
    const context = vm.createContext({});
    vm.runInContext(emitJavaScript(program).text, context, { timeout: 1000 });
    assert.equal(typeof context.acceptProgramDerivedNumberOperand, 'function');
    for (const operation of ['add', 'sub', 'mul', 'div', 'rem']) for (const field of ['left', 'right']) {
      assert.equal(context.acceptProgramDerivedNumberOperand('binary', operation, field), true);
    }
    for (const [kind, operation, field] of [['binary', 'concat', 'left'], ['binary', 'add', 'arg'], ['unary', 'neg', 'arg']]) {
      assert.equal(context.acceptProgramDerivedNumberOperand(kind, operation, field), false);
    }
  }
});

test('the shared native control retains its actual JavaScript source archive and executable value edit', () => {
  const shared = readProjectData(new URL('../../parity/fixtures/program-number-projection.lino', import.meta.url));
  assert.equal(shared.source, source);
  assert.equal(shared.sourceSha256, createHash('sha256').update(source).digest('hex'));
  assert.deepEqual(observe(source), shared.output);
  const network = LinkNetwork.fromLino(shared.links);
  const program = decodeTypedProgramLinks(network, shared.root);
  assert.equal(recheckLinkedTypedProgram(program, { wholeProgram: true }), program);
  assert.deepEqual(observe(emitTypedProgramLinks(network, shared.root, 'JavaScript').code), shared.output);
  const operation = arithmetic(program);
  assert.equal(operation.right.value, shared.edit.original);
  operation.right.value = shared.edit.replacement;
  recheckLinkedTypedProgram(program, { wholeProgram: true });
  const edited = encodeTypedProgramLinks(program);
  assert.deepEqual(observe(emitTypedProgramLinks(edited.network, edited.root, 'JavaScript').code), shared.editedOutput);
});
