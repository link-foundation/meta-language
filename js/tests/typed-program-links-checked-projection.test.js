import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import { checkProgram } from '../src/translation/check.js';
import { parseRust } from '../src/translation/rust.js';
import { encodeTypedProgramLinks, decodeTypedProgramLinks, emitTypedProgramLinks, createTypedProgramLinks } from '../src/typed-program-links.js';
import { projectLinkedProgramExpression } from '../src/typed-program-expression-projection.js';
import { recheckLinkedTypedProgram } from '../src/typed-program-checked-projection.js';

import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { parseJavaScript } from '../src/translation/javascript.js';
import { LinkNetwork } from '../src/network.js';
import { encodeLinkNetworkBinary, decodeLinkNetworkBinary } from '../src/link-network-binary.js';
import { serializeProjectData } from '../scripts/project-data.mjs';
import { COLLECTION_SIZE_PROJECTION_SCHEMA_POLICY, COLLECTION_SIZE_PROJECTION_SCHEMA_ENTRIES,
  COLLECTION_SIZE_PROJECTION_FIXTURE_INPUTS, deriveCollectionSizeProjectionSchema,
  deriveCollectionSizeProjectionFixtureSource, deriveCollectionSizeProjectionValidationSource, readCollectionSizeProjectionSchemaPacket,
  verifyCollectionSizeProjectionSchemaPacket, restoreCollectionSizeProjectionSchema } from '../scripts/collection-size-projection-schema.mjs';
import { generateCollectionSizeProjectionSchema } from '../scripts/generate-collection-size-projection-schema.mjs';

const floating = { kind: 'float' };
const boolean = { kind: 'bool' };
function productProgram() {
  return checkProgram({ language: 'JavaScript', items: [{ k: 'fn', name: 'make', params: [],
    ret: { kind: 'tuple', elements: [floating, boolean] },
    body: { k: 'tuple', frozen: true, productModel: 'immutable value product', items: [{ k: 'num', value: '5', type: floating }, { k: 'bool', value: true }] },
  }], main: { effects: [{ k: 'print', style: 'js-console', expr: { k: 'tupleIndex', index: 0,
    object: { k: 'app', fn: { k: 'name', path: ['make'] }, args: [] },
  } }] } });
}

test('whole linked product programs recheck with ordered elements and immutable descriptors', () => {
  const program = productProgram();
  const archive = encodeTypedProgramLinks(program);
  assert.equal(decodeTypedProgramLinks(archive.network, archive.root).declarations.get('make').body.frozen, true);
  const code = emitTypedProgramLinks(archive.network, archive.root, 'JavaScript').code;
  const output = [];
  const descriptor = vm.runInNewContext(`${code}\n[Object.isFrozen(make()), make().length, make()[1]]`,
    { console: { log: value => output.push(value) } }, { timeout: 1000 });
  assert.deepEqual(output, ['5']);
  assert.deepEqual(Array.from(descriptor), [true, 2, true]);
  const product = program.declarations.get('make').body;
  product.frozen = false;
  assert.throws(() => encodeTypedProgramLinks(program), /product model.*contradicts/u);
  delete product.productModel;
  const mutable = encodeTypedProgramLinks(program);
  const changed = emitTypedProgramLinks(mutable.network, mutable.root, 'JavaScript').code;
  assert.equal(vm.runInNewContext(`${changed}\nObject.isFrozen(make())`, { console: { log() {} } }, { timeout: 1000 }), false);
});

test('tuple field, layout, facet and positional edits are checked before emission', () => {
  for (const mutate of [
    program => { program.declarations.get('make').body.items.pop(); },
    program => { program.declarations.get('make').body.type = { kind: 'tuple', elements: [boolean, floating] }; },
    program => { program.declarations.get('make').body.frozen = 'true'; },
    program => { program.declarations.get('make').body.productModel = 'mutable value product'; },
    program => { program.declarations.get('make').body.productModel = null; },
    program => { program.declarations.get('make').body.productModel = ''; },
    program => { program.declarations.get('make').body.extraProjection = true; },
    program => { program.declarations.get('make').body.type = { ...program.declarations.get('make').ret, frozen: true }; },
    program => { program.main.effects[0].expr.arg.index = 2; },
    program => { program.main.effects[0].expr.arg.index = Number.MAX_SAFE_INTEGER + 1; },
    program => { program.main.effects[0].expr.arg.index = '0'; },
    program => { program.main.effects[0].expr.arg.object.type = { kind: 'array', element: floating }; },
  ]) {
    const program = productProgram(); mutate(program);
    assert.throws(() => encodeTypedProgramLinks(program), /linked|tuple|positional/u);
  }
  const tuple = productProgram().declarations.get('make').body;
  assert.throws(() => projectLinkedProgramExpression({ ...tuple, extraProjection: true }), /unrecognized projection field/u);
});

test('complete IR comparison preserves span roles and rejects forged program semantics', () => {
  const spanned = productProgram();
  spanned.declarations.get('make').body.span = { start: 4, end: 20 };
  const archive = encodeTypedProgramLinks(spanned);
  assert.deepEqual(decodeTypedProgramLinks(archive.network, archive.root).declarations.get('make').body.span, { start: 4, end: 20 });
  for (const mutate of [
    program => { program.declarations.get('make').recursive = true; },
    program => { program.declarations.get('make').decreasing = 0; },
    program => { program.declarations.get('make').body.items[0].unexpectedPolicy = 'ignored'; },
    program => { program.main.effects[0].expr.arg.unexpectedPolicy = 'ignored'; },
    program => { program.span = null; },
  ]) {
    const program = productProgram(); mutate(program);
    assert.throws(() => encodeTypedProgramLinks(program), /projection|inconsistent types, bindings or operation semantics/u);
  }
});

test('the same recheck retains actual data declarations and match scopes across the whole program', () => {
  const base = checkProgram(parseRust('enum Choice{Empty,Value(i32)} fn read(item:Choice)->i32{match item{Choice::Empty=>0,Choice::Value(value)=>value}} fn main(){println!("{}",read(Choice::Value(7)));}'));
  const product = productProgram().declarations.get('make');
  base.items.push(product); base.declarations.set(product.fullName, product);
  assert.equal(recheckLinkedTypedProgram(base), base);
  const archive = encodeTypedProgramLinks(base);
  const output = [];
  vm.runInNewContext(emitTypedProgramLinks(archive.network, archive.root, 'JavaScript').code,
    { console: { log: value => output.push(value) } }, { timeout: 1000 });
  assert.deepEqual(output, ['7']);
  base.declarations.get('read').body.cases[1].body.type = { kind: 'fixed', bits: 64, signed: true };
  assert.throws(() => encodeTypedProgramLinks(base), /inconsistent variable type/u);
});

test('new explicit product programs refuse uncovered projections while legacy collections retain behavior', () => {
  const program = productProgram();
  const literal = program.declarations.get('make').body.items[0];
  program.declarations.get('make').body.items[0] = { k: 'collectionRead', op: 'get', type: floating,
    object: { k: 'var', name: 'unbound', type: { kind: 'map', key: floating, value: floating } }, key: literal,
  };
  assert.throws(() => recheckLinkedTypedProgram(program), /collectionRead has no surface projection/u);
  const legacy = createTypedProgramLinks('const values=new Map([[1,2]]);console.log(values.get(1));', 'JavaScript');
  const output = [];
  vm.runInNewContext(emitTypedProgramLinks(legacy.network, legacy.root, 'JavaScript').code,
    { console: { log: value => output.push(value) } }, { timeout: 1000 });
  assert.deepEqual(output, ['2']);
});

test('the mechanical recheck rejects structural cycles and excessively deep program records', () => {
  const cyclic = productProgram();
  cyclic.declarations.get('make').body.items.push(cyclic.declarations.get('make').body);
  assert.throws(() => recheckLinkedTypedProgram(cyclic), /structural cycle/u);
  const deep = productProgram();
  let value = {};
  for (let index = 0; index < 260; index += 1) value = { value };
  deep.metadata = value;
  assert.throws(() => recheckLinkedTypedProgram(deep), /resource limit/u);
});

test('tuple position zero is canonical while floating literal negative zero survives', () => {
  const program = productProgram();
  program.main.effects[0].expr.arg.index = -0;
  const archive = encodeTypedProgramLinks(program);
  assert.equal(Object.is(program.main.effects[0].expr.arg.index, -0), false);
  const index = archive.network.fieldRelations().find(relation => relation.label === 'index').child;
  const literal = archive.network.fieldRelations().find(relation => relation.label === 'value'
    && archive.network.link(relation.child).metadata().definition === '5').child;
  const scalar = archive.network.link(literal);
  scalar.setMetadata(scalar.metadata().withDefinition('-0'));
  for (const spelling of ['-0', '0.0', '0e0']) {
    const position = archive.network.link(index);
    position.setMetadata(position.metadata().withDefinition(spelling));
    const decoded = decodeTypedProgramLinks(archive.network, archive.root);
    assert.equal(Object.is(decoded.main.effects[0].expr.arg.index, -0), false);
    assert.equal(decoded.declarations.get('make').body.items[0].value, '-0');
    const output = [];
    vm.runInNewContext(emitTypedProgramLinks(archive.network, archive.root, 'JavaScript').code,
      { console: { log: value => output.push(value) } }, { timeout: 1000 });
    assert.deepEqual(output, ['-0']);
  }
});


function observeCollectionConsole(code) {
  const output = [];
  vm.runInNewContext(code, { console: { log: (...values) => output.push(values.map(String).join(' ')) } }, { timeout: 1000 });
  return output;
}

// This adds a checked projection for an existing owner operation. It changes no
// default whole-program policy and makes no claim about unmodeled Map/Set APIs.
test('actual Map and Set size owners recheck completely through both source-free codecs', () => {
  for (const [id, language, source, expected] of COLLECTION_SIZE_PROJECTION_FIXTURE_INPUTS) {
    assert.equal(language, 'JavaScript');
    assert.deepEqual(observeCollectionConsole(source), expected, id + ' independent native JavaScript result');
    const checked = checkProgram(parseJavaScript(source));
    const count = checked.declarations.get('count');
    assert.equal(count.body.k, 'collectionSize');
    assert.deepEqual(count.body.type, floating);
    assert.ok(['map', 'set'].includes(count.params[0].type.kind));
    assert.deepEqual(count.body.object.type, count.params[0].type);
    assert.equal(count.body.object.name, count.params[0].name);
    const surfaceSize = projectLinkedProgramExpression(count.body);
    assert.equal(surfaceSize.k, 'field');
    assert.equal(surfaceSize.field, 'size');
    assert.equal(surfaceSize.object.k, 'name');
    assert.deepEqual(surfaceSize.object.path, [count.params[0].name]);
    assert.equal(recheckLinkedTypedProgram(checked, { wholeProgram: true }), checked);
    const archive = encodeTypedProgramLinks(checked);
    const javascript = emitTypedProgramLinks(archive.network, archive.root, 'JavaScript').code;
    const rust = emitTypedProgramLinks(archive.network, archive.root, 'Rust').code;
    assert.deepEqual(observeCollectionConsole(javascript), expected);
    assert.match(rust, /fn count/u);
    for (const network of [LinkNetwork.fromLino(archive.network.toLino()), decodeLinkNetworkBinary(encodeLinkNetworkBinary(archive.network))]) {
      const restored = decodeTypedProgramLinks(network, archive.root);
      assert.deepEqual(restored, checked, id + ' complete checked IR, including types and binding owners');
      assert.equal(restored.items[0], restored.declarations.get('count'));
      assert.equal(recheckLinkedTypedProgram(restored, { wholeProgram: true }), restored);
      assert.equal(emitTypedProgramLinks(network, archive.root, 'Rust').code, rust);
      assert.equal(emitTypedProgramLinks(network, archive.root, 'JavaScript').code, javascript);
      assert.deepEqual(observeCollectionConsole(emitTypedProgramLinks(network, archive.root, 'JavaScript').code), expected);
      assert.ok(network.links().every(link => !link.metadata().definition?.includes(source)), 'semantic graph does not carry source text');
    }
    for (const mutate of [
      program => { program.declarations.get('count').body.type = { kind: 'bool' }; },
      program => { program.declarations.get('count').body.object.type = { kind: 'array', element: floating }; },
      program => { program.declarations.get('count').body.object.name = 'unbound'; },
      program => { program.declarations.get('count').body.unrecognizedProjection = true; },
      program => { program.declarations.get('count').body.op = 'arbitrary collection operation'; },
      program => { program.declarations.get('count').params[0].type = { kind: 'string' }; },
    ]) {
      const changed = structuredClone(checked); mutate(changed);
      assert.throws(() => recheckLinkedTypedProgram(changed, { wholeProgram: true }), undefined, id + ' complete owner/type/projection refusal');
      for (const reload of [network => LinkNetwork.fromLino(network.toLino()), network => decodeLinkNetworkBinary(encodeLinkNetworkBinary(network))]) {
        assert.throws(() => {
          const edited = encodeTypedProgramLinks(changed);
          recheckLinkedTypedProgram(decodeTypedProgramLinks(reload(edited.network), edited.root), { wholeProgram: true });
        }, undefined, id + ' altered semantics also refuse through either codec');
      }
    }
  }
});

test('collection size shared schema successor retains every original byte owner and fixture input', () => {
  const packet = generateCollectionSizeProjectionSchema({ check: true });
  const proof = verifyCollectionSizeProjectionSchemaPacket(packet);
  const pin = value => ({ bytes: Buffer.byteLength(value), sha256: createHash('sha256').update(value).digest('hex') });
  assert.deepEqual(pin(proof.predecessor), COLLECTION_SIZE_PROJECTION_SCHEMA_POLICY.predecessor);
  assert.equal(deriveCollectionSizeProjectionSchema(proof.predecessor), proof.current);
  assert.deepEqual(packet.extensions, COLLECTION_SIZE_PROJECTION_SCHEMA_ENTRIES);
  for (const file of COLLECTION_SIZE_PROJECTION_SCHEMA_POLICY.files) {
    const actual = readFileSync(new URL('../../' + file, import.meta.url));
    assert.equal(actual.toString('utf8'), proof.current);
    assert.equal(restoreCollectionSizeProjectionSchema(actual, file), proof.predecessor);
    assert.equal(restoreCollectionSizeProjectionSchema(Buffer.from(proof.predecessor), file), proof.predecessor);
    assert.throws(() => restoreCollectionSizeProjectionSchema(Buffer.concat([actual, Buffer.from('\n')]), file), undefined, 'unknown complete source refuses');
  }
  for (const owner of packet.owners) {
    const current = owner.file === 'js/scripts/generate-typed-program-links-fixtures.mjs'
      ? deriveCollectionSizeProjectionFixtureSource(owner.source)
      : owner.file === 'js/src/typed-program-links-validation.js' ? deriveCollectionSizeProjectionValidationSource(owner.source) : owner.source;
    assert.deepEqual(pin(current), owner.current);
    assert.equal(readFileSync(new URL('../../' + owner.file, import.meta.url), 'utf8'), current);
  }
  const original = packet.owners.find(owner => owner.file === 'js/scripts/generate-typed-program-links-fixtures.mjs');
  const fixtureSource = readFileSync(new URL('../../' + original.file, import.meta.url), 'utf8');
  assert.equal(deriveCollectionSizeProjectionFixtureSource(original.source), fixtureSource);
  assert.throws(() => deriveCollectionSizeProjectionFixtureSource(fixtureSource), undefined, 'duplicate new fixtures refuse');
  assert.throws(() => restoreCollectionSizeProjectionSchema(Buffer.from(proof.current), 'unknown/schema.lino'));
  for (const mutate of [
    changed => { changed.extensions.expressionProjections.collectionSize.fields[1][2] = 'length'; },
    changed => { changed.extensions.expressionFields.collectionSize.push('op'); },
    changed => { changed.current.bytes += 1; },
    changed => { changed.predecessor.source += '\n'; Object.assign(changed.predecessor, pin(changed.predecessor.source)); },
    changed => { changed.predecessor.certificates[0].certificate.blob.sha256 = '0'.repeat(64); },
    changed => { changed.owners[0].source += '\n'; Object.assign(changed.owners[0], pin(changed.owners[0].source)); changed.owners[0].current = pin(changed.owners[0].source); },
    changed => { changed.owners[3].current.sha256 = '0'.repeat(64); },
    changed => { changed.owners[4].current.sha256 = '0'.repeat(64); },
    changed => { changed.unrecognizedSourceAuthority = true; },
  ]) {
    const changed = structuredClone(packet); mutate(changed);
    assert.throws(() => verifyCollectionSizeProjectionSchemaPacket(changed));
  }
  packet.extensions.expressionFields.collectionSize.push('ignored');
  assert.deepEqual(readCollectionSizeProjectionSchemaPacket().extensions, COLLECTION_SIZE_PROJECTION_SCHEMA_ENTRIES, 'returned packet edits cannot alter the source authority');
  assert.ok(serializeProjectData(COLLECTION_SIZE_PROJECTION_SCHEMA_ENTRIES, 'control.lino').includes('collectionSize'));
});


// Inject fields directly into a valid graph: encode validation cannot intercept
// these malformed cases. No explicit whole-program option is requested here.
test('default collection size decoding refuses undeclared semantic fields through both codecs', () => {
  for (const [id, language, source] of COLLECTION_SIZE_PROJECTION_FIXTURE_INPUTS) {
    const packet = createTypedProgramLinks(source, language);
    for (const label of ['unrecognized projection policy', 'operation']) {
      const edited = LinkNetwork.fromLino(packet.network.toLino());
      const kinds = edited.fieldRelations().filter(relation => relation.label === 'construct kind'
        && edited.link(relation.child).metadata().definition === 'collectionSize');
      assert.equal(kinds.length, 1, id + ' actual size owner is unique');
      edited.insertFieldRelation(kinds[0].parent, label, kinds[0].child);
      for (const network of [LinkNetwork.fromLino(edited.toLino()), decodeLinkNetworkBinary(encodeLinkNetworkBinary(edited))]) {
        assert.throws(() => decodeTypedProgramLinks(network, packet.root),
          /a linked expression contains an unrecognized semantic field/u, id + ' default decode refuses ' + label);
        for (const target of ['JavaScript', 'Rust']) {
          assert.throws(() => emitTypedProgramLinks(network, packet.root, target),
            /a linked expression contains an unrecognized semantic field/u, id + ' default ' + target + ' emission refuses ' + label);
        }
      }
    }
  }
});
