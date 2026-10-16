import test from 'node:test';
import assert from 'node:assert/strict';
import { readProjectData } from '../scripts/project-data.mjs';
import { LinkNetwork } from '../src/network.js';
import { createTypedProgramLinks, decodeTypedProgramLinks, emitTypedProgramLinks, TYPED_PROGRAM_LINKS_SCHEMA } from '../src/typed-program-links.js';
import { sameType } from '../src/translation/types.js';
import { readRustBorrowedCallTargetDiagnostic } from '../src/translation/rust-borrowed-call-interface-rules.js';

const policy = readProjectData(new URL('../src/translation/data/rust-borrowed-call-interface.lino', import.meta.url));
const model = policy.model;
const argumentDiagnostic = 'the linked program has inconsistent call argument type';
const targetDiagnostic = 'Rust borrowed call interface: every caller must target an owned pure leaf without a static borrowed input';

test('the shared selected-call diagnostic gives actual argument mismatches priority over a nonleaf target', () => {
  assert.deepEqual(policy.targetDiagnostics, [['mismatch', '', argumentDiagnostic], ['matched', 'call', targetDiagnostic]]);
  for (const target of ['call', 'var', 'lit', '']) {
    assert.equal(readRustBorrowedCallTargetDiagnostic(model, 'call', target, 'mismatch'), argumentDiagnostic);
    assert.equal(readRustBorrowedCallTargetDiagnostic(model, 'call', target, 'matched'), target === 'call' ? targetDiagnostic : '');
    for (const status of ['', 'unknown', 'wrong arity']) assert.equal(readRustBorrowedCallTargetDiagnostic(model, 'call', target, status), '');
    for (const selection of ['', 'unknown', 'primitive borrowed Rust function header 1']) {
      for (const status of ['mismatch', 'matched']) assert.equal(readRustBorrowedCallTargetDiagnostic(selection, 'call', target, status), '');
    }
    for (const kind of ['', 'var', 'lit']) assert.equal(readRustBorrowedCallTargetDiagnostic(model, kind, target, 'mismatch'), '');
  }
});

test('actual source-free declaration retargets retain the existing JavaScript diagnostic precedence', () => {
  const source = 'fn leaf(value:&f64)->f64{*value}fn caller(value:f64)->f64{leaf(&value)}fn other(value:&bool)->f64{if *value{1.0}else{2.0}}fn main(){}';
  const packet = createTypedProgramLinks(source, 'Rust', { rustSourceSignatureModel: 'Rust source function signature 1' });
  const original = decodeTypedProgramLinks(packet.network, packet.root);
  const call = original.declarations.get('caller').body;
  const field = (network, parent, key) => network.fieldRelations().find(relation => relation.parent.equals(parent) && relation.label === (TYPED_PROGRAM_LINKS_SCHEMA.fields[key] ?? key));
  for (const name of ['caller', 'other']) {
    const target = original.declarations.get(name);
    const status = call.args.every((argument, index) => sameType(argument.type, target.params[index].type)) ? 'matched' : 'mismatch';
    const diagnostic = readRustBorrowedCallTargetDiagnostic(model, call.k, target.body.k, status);
    assert.equal(diagnostic, name === 'caller' ? targetDiagnostic : argumentDiagnostic);
    const network = LinkNetwork.fromLino(packet.network.toLino());
    const body = network.link(packet.root).references()[0];
    const directory = field(network, body, 'declarations').child;
    const declarations = new Map(network.link(directory).references().map(id => {
      const [key, value] = network.link(id).references();
      return [network.link(key).metadata().definition, value];
    }));
    const expression = field(network, declarations.get('caller'), 'body').child;
    const reference = field(network, expression, TYPED_PROGRAM_LINKS_SCHEMA.references.declaration).link;
    const references = reference.references();
    references[2] = declarations.get(name);
    reference.setReferences(references);
    const restored = LinkNetwork.fromLino(network.toLino());
    assert.throws(() => emitTypedProgramLinks(restored, packet.root, 'Rust', { rustSourceHeaderEmissionModel: model }), error => {
      assert.equal(error.message, diagnostic);
      return true;
    });
  }
  const target = original.declarations.get('leaf');
  assert.equal(readRustBorrowedCallTargetDiagnostic(model, call.k, target.body.k, 'matched'), '');
  assert.ok(emitTypedProgramLinks(packet.network, packet.root, 'Rust', { rustSourceHeaderEmissionModel: model }).code);
  assert.ok(emitTypedProgramLinks(packet.network, packet.root, 'Rust').code);
});

test('both complete caller orders report argument mismatch before nonleaf targets while malformed containers stay structural', () => {
  const leaf = 'fn leaf(value:&f64)->f64{*value}';
  const first = 'fn first(value:f64)->f64{leaf(&value)}';
  const second = 'fn second(value:f64)->f64{leaf(&value)}';
  const other = 'fn other(value:&bool)->f64{if *value{1.0}else{2.0}}';
  for (const callers of [[first, second], [second, first]]) {
    const source = [leaf, ...callers, other, 'fn main(){}'].join('');
    const packet = createTypedProgramLinks(source, 'Rust', { rustSourceSignatureModel: 'Rust source function signature 1' });
    const program = decodeTypedProgramLinks(packet.network, packet.root);
    const network = LinkNetwork.fromLino(packet.network.toLino());
    const field = (parent, key) => network.fieldRelations().find(relation => relation.parent.equals(parent) && relation.label === (TYPED_PROGRAM_LINKS_SCHEMA.fields[key] ?? key));
    const body = network.link(packet.root).references()[0];
    const declarations = new Map(network.link(field(body, 'declarations').child).references().map(id => {
      const [key, value] = network.link(id).references();
      return [network.link(key).metadata().definition, value];
    }));
    const diagnostics = [];
    for (const [caller, targetName] of [['first', 'first'], ['second', 'other']]) {
      const expression = field(declarations.get(caller), 'body').child;
      const reference = field(expression, TYPED_PROGRAM_LINKS_SCHEMA.references.declaration).link;
      const references = reference.references();
      references[2] = declarations.get(targetName);
      reference.setReferences(references);
      const call = program.declarations.get(caller).body, target = program.declarations.get(targetName);
      const status = call.args.every((argument, index) => sameType(argument.type, target.params[index].type)) ? 'matched' : 'mismatch';
      diagnostics.push(readRustBorrowedCallTargetDiagnostic(model, call.k, target.body.k, status));
    }
    assert.deepEqual(diagnostics, [targetDiagnostic, argumentDiagnostic]);
    assert.equal(policy.targetDiagnostics.find(row => diagnostics.includes(row[2]))[2], argumentDiagnostic);
    assert.throws(() => emitTypedProgramLinks(LinkNetwork.fromLino(network.toLino()), packet.root, 'Rust', { rustSourceHeaderEmissionModel: model }), error => {
      assert.equal(error.message, argumentDiagnostic);
      return true;
    });
    const argumentsList = field(field(declarations.get('second'), 'body').child, 'args').child;
    network.insertFieldRelation(argumentsList, 'opaque container fact', body);
    assert.throws(() => emitTypedProgramLinks(LinkNetwork.fromLino(network.toLino()), packet.root, 'Rust', { rustSourceHeaderEmissionModel: model }), /an ordered program container contains named fields/u);
  }
});
