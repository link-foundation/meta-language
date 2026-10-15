import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import { readProjectData } from '../../js/scripts/project-data.mjs';
import { LinkNetwork, createTypedProgramLinks, encodeLinkNetworkBinary, decodeLinkNetworkBinary } from '../../js/src/index.js';
import { tokenize } from '../../js/src/translation/lexer.js';
import { TYPED_PROGRAM_LINKS_SCHEMA as schema } from '../../js/src/typed-program-links.js';
import { checkProgram } from '../../js/src/translation/check.js';
import { parseJavaScript } from '../../js/src/translation/javascript.js';
import { rustRawBindingIdentifierRules, rustRawBindingIdentifierFixtures } from './generate-rules.mjs';
import {
  RUST_RAW_BINDING_IDENTIFIER_POLICY as policy, createRustRawBindingIdentifiers,
  createRustRawBindingIdentifierLinks, decodeRustRawBindingIdentifierLinks, encodeRustRawBindingIdentifierLinks,
  checkRustRawBindingIdentifiers, emitRustRawBindingIdentifiers, reconstructRustRawBindingIdentifierSource,
} from './model.mjs';
import { validName } from './syntax.mjs';

const fixture = readProjectData(new URL('./data/fixtures.lino', import.meta.url));
const reloads = packet => [LinkNetwork.fromLino(packet.network.toLino()), decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))].map(network => ({ network, root: packet.root }));
const chosen = packet => decodeRustRawBindingIdentifierLinks(packet.network, packet.root);
const load = index => chosen(reloads(createRustRawBindingIdentifierLinks(fixture.cases[index].source))[0]);
function execute(program, proof, name, args) {
  const context = {};
  vm.runInNewContext(emitRustRawBindingIdentifiers(program, proof, 'JavaScript').text, context, { timeout: 1000 });
  const fn = context[name]; assert.equal(typeof fn, 'function', name);
  return fn(...args.map(value => typeof value === 'string' ? BigInt(value) : value));
}
function refusal(program, proof) {
  assert.throws(() => checkRustRawBindingIdentifiers(program, proof));
  assert.throws(() => encodeRustRawBindingIdentifierLinks(program, proof));
  assert.throws(() => emitRustRawBindingIdentifiers(program, proof, 'Rust'));
}
function graphIdentity(packet, { program, proof }) {
  const fields = packet.network.fieldRelations();
  const label = name => schema.fields[name] ?? name.replace(/[A-Z]/gu, letter => ' ' + letter.toLowerCase());
  const field = (parent, name) => fields.find(row => row.parent.equals(parent) && row.label === label(name));
  const body = packet.network.link(packet.root).references()[0], mapping = field(body, 'declarations');
  const actualOwners = new Set();
  for (const pair of packet.network.link(mapping.child).references()) {
    const declaration = packet.network.link(pair).references()[1]; actualOwners.add(declaration.asU64().toString());
    const parameters = field(declaration, 'params');
    for (const parameter of packet.network.link(parameters.child).references()) actualOwners.add(parameter.asU64().toString());
  }
  for (const alias of proof.aliases) {
    const role = alias.kind === 'declaration' ? 'declaration' : 'binding';
    const start = alias.token.span.start;
    const tokenId = fields.filter(row => row.label === role).find(row => {
      const span = field(row.parent, 'span'), offset = span && field(span.child, 'start');
      return offset && packet.network.link(offset.child)?.metadata().definition === String(start);
    });
    assert.ok(tokenId, 'actual source token has a saved owned relation');
    assert.ok(actualOwners.has(tokenId.child.asU64().toString()), 'the token points to an actual declared core owner LinkId');
    const inverse = fields.find(row => row.label === label('checkedOwner') && row.child.equals(tokenId.child));
    assert.ok(inverse, 'the alias points to the exact saved checked owner LinkId');
    const ownerName = fields.find(row => row.parent.equals(tokenId.child) && row.label === 'name');
    assert.equal(packet.network.link(ownerName.child).metadata().definition, alias.checkedOwner.name);
    if (alias.kind === 'declaration') assert.equal(program.declarations.get(alias.checkedOwner.fullName), alias.checkedOwner);
  }
  for (const expression of fields.filter(row => row.label === label('checkedExpression'))) {
    const inverse = field(expression.parent, 'alias'), owner = field(inverse.child, 'checkedOwner');
    const reference = fields.find(row => row.parent.equals(expression.child) && [schema.references.binding, schema.references.declaration].includes(row.label));
    assert.ok(reference, 'the actual checked expression has its core binding or declaration relation');
    assert.ok(reference.child.equals(owner.child), 'the occurrence inverse and checked expression share exactly one actual core owner LinkId');
  }
}

test('the exact opt-in ASCII identity selectors share the existing character concept and forbid all five raw exclusions', () => {
  assert.deepEqual(fixture, rustRawBindingIdentifierFixtures);
  checkProgram(parseJavaScript(rustRawBindingIdentifierRules()));
  for (const keyword of policy.ordinaryKeywords) {
    assert.equal(validName(keyword, false), false, keyword);
    assert.equal(validName(keyword, true), !policy.forbiddenNames.includes(keyword), keyword);
  }
  for (const name of ['_', 'crate', 'self', 'super', 'Self', '', 'é', 'a\u0301', '😀', 'a#b', 'a b', '0name']) assert.equal(validName(name, true), false, name);
  for (const name of ['foo', 'sourceFunction0', 'sourceParameter0_0', '_value', 'ml_value', 'crate_value', 'main']) assert.equal(validName(name, false), true, name);
});

test('all complete original sources retain actual owners, real LinkIds and Unicode byte replay through text and binary', () => {
  for (const entry of fixture.cases) {
    const source = '\ufeff// Ω😀 source offset\r\n/* ordinary /* nested */ comment */\n' + entry.source;
    const packet = createRustRawBindingIdentifierLinks(source);
    for (const saved of reloads(packet)) {
      const { program, proof } = chosen(saved);
      graphIdentity(saved, { program, proof });
      assert.equal(reconstructRustRawBindingIdentifierSource(program, proof), source);
      assert.deepEqual(Buffer.from(emitRustRawBindingIdentifiers(program, proof, 'Rust').text), Buffer.from(source));
      assert.equal(proof.worker, program.main);
      for (const alias of proof.aliases) {
        assert.ok(proof.tokens.includes(alias.token));
        if (alias.kind === 'parameter') assert.ok(program.items.some(item => item.params.includes(alias.checkedOwner)));
      }
      for (const occurrence of proof.occurrences) {
        assert.ok(proof.tokens.includes(occurrence.token)); assert.ok(proof.aliases.includes(occurrence.alias));
        assert.equal(occurrence.token[occurrence.alias.kind === 'parameter' ? 'binding' : 'declaration'], occurrence.alias.checkedOwner);
      }
      assert.equal(execute(program, proof, entry.name, entry.arguments), typeof entry.value === 'string' ? BigInt(entry.value) : entry.value);
      const cst = LinkNetwork.parse(source, 'Rust');
      for (const archived of [LinkNetwork.fromLino(cst.toLino()), decodeLinkNetworkBinary(encodeLinkNetworkBinary(cst))]) assert.deepEqual(Buffer.from(archived.reconstructText()), Buffer.from(source));
      const emitted = emitRustRawBindingIdentifiers(program, proof, 'Rust').text;
      const again = createRustRawBindingIdentifiers(emitted);
      assert.equal(execute(again.program, again.proof, entry.name, entry.arguments), typeof entry.value === 'string' ? BigInt(entry.value) : entry.value);
    }
  }
});

test('actual source-free parameter retargets and keyword renames produce fresh complete source without name-only authority', () => {
  let { program, proof } = load(5);
  const target = program.items[0].params[1], use = proof.occurrences[0];
  use.token.binding = target; use.alias = proof.aliases.find(alias => alias.checkedOwner === target); use.checkedExpression.name = target.name;
  assert.equal(execute(program, proof, 'choose', ['13', '17']), 17n);
  let saved = reloads(encodeRustRawBindingIdentifierLinks(program, proof))[1];
  ({ program, proof } = chosen(saved));
  const original = reconstructRustRawBindingIdentifierSource(program, proof), edited = emitRustRawBindingIdentifiers(program, proof, 'Rust').text;
  assert.ok(original.includes('{r#match}')); assert.ok(edited.includes('{r#type}'));
  const fresh = createRustRawBindingIdentifiers(edited);
  assert.equal(execute(fresh.program, fresh.proof, 'choose', ['13', '17']), 17n);
  const binder = program.items[0].params[1]; binder.name = 'loop'; proof.occurrences[0].checkedExpression.name = 'loop';
  saved = reloads(encodeRustRawBindingIdentifierLinks(program, proof))[0];
  ({ program, proof } = chosen(saved));
  const renamed = emitRustRawBindingIdentifiers(program, proof, 'Rust').text;
  assert.ok(renamed.includes('r#loop:i32')); assert.ok(renamed.includes('{r#loop}'));
  const reimported = createRustRawBindingIdentifiers(renamed);
  assert.equal(execute(reimported.program, reimported.proof, 'choose', ['13', '17']), 17n);
});

test('actual source-free declaration retarget and keyword rename update both owned call links and complete source', () => {
  const source = 'pub fn r#match(value:i32)->i32{r#type(value)}pub fn r#type(value:i32)->i32{value}pub fn alternate(value:i32)->i32{19i32}fn main(){}';
  let { program, proof } = chosen(reloads(createRustRawBindingIdentifierLinks(source))[1]);
  const target = program.declarations.get('alternate'), call = proof.occurrences.find(row => row.alias.kind === 'declaration');
  call.token.declaration = target; call.alias = proof.aliases.find(row => row.checkedOwner === target); call.checkedExpression.fn = target.fullName;
  assert.equal(execute(program, proof, 'match', ['3']), 19n);
  target.name = 'loop'; target.fullName = 'loop'; call.checkedExpression.fn = 'loop';
  program.declarations = new Map(program.items.map(item => [item.fullName, item]));
  ({ program, proof } = chosen(reloads(encodeRustRawBindingIdentifierLinks(program, proof))[0]));
  const text = emitRustRawBindingIdentifiers(program, proof, 'Rust').text;
  assert.ok(text.includes('pub fn r#loop(')); assert.ok(text.includes('{r#loop(value)}'));
  const replay = createRustRawBindingIdentifiers(text);
  assert.equal(execute(replay.program, replay.proof, 'match', ['3']), 19n);
});

test('duplicate canonical owners, excluded raw identifiers, invalid contiguity, shadowed calls and wider grammar refuse', () => {
  for (const source of fixture.refusals) assert.throws(() => createRustRawBindingIdentifierLinks(source), source);
  for (const changed of ['r #type', 'r#\ntype', 'r/**/#type', 'r#/**/type', 'r#é', 'r#😀']) {
    const source = fixture.cases[0].source.replaceAll('r#type', changed);
    assert.throws(() => createRustRawBindingIdentifiers(source), changed);
  }
  for (const source of ['/// hidden attribute\n' + fixture.cases[0].source, fixture.cases[0].source + 'fn hidden(){}', 'use other::name;' + fixture.cases[0].source]) assert.throws(() => createRustRawBindingIdentifiers(source));
  assert.throws(() => createRustRawBindingIdentifiers(fixture.cases[0].source, { model: 'unknown' }));
});

test('counterfeit aliases, expression and binding owners, keyword injection and incoherent edits fail on every explicit boundary', () => {
  const mutations = [
    value => { value.proof.aliases[0].value = value.proof.aliases[1].value; },
    value => { value.proof.aliases[0].checkedOwner = structuredClone(value.proof.aliases[0].checkedOwner); },
    value => { value.proof.occurrences[0].alias = structuredClone(value.proof.occurrences[0].alias); },
    value => { value.proof.occurrences[0].checkedExpression = structuredClone(value.proof.occurrences[0].checkedExpression); },
    value => { value.proof.occurrences[0].token.binding = structuredClone(value.proof.occurrences[0].token.binding); },
    value => { value.proof.occurrences[0].checkedExpression.name = 'type'; },
    value => { value.proof.aliases.pop(); },
    value => { value.proof.occurrences.pop(); },
    value => { value.proof.tokens[0].value = 'if'; },
    value => { value.program.items[0].params[1].name = value.program.items[0].params[0].name; },
    value => { value.proof.worker = structuredClone(value.program.main); },
    value => { value.proof.tokens.find(token => token.raw === '#').span.start--; },
  ];
  for (const mutate of mutations) { const value = load(5); mutate(value); refusal(value.program, value.proof); }
  const literal = createRustRawBindingIdentifiers('pub fn value(input:bool)->bool{true}fn main(){}');
  const token = literal.proof.tokens.find(row => row.raw === 'true'); delete token.value; token.binding = literal.program.items[0].params[0];
  refusal(literal.program, literal.proof);
  let invoked = 0;
  for (const target of [value => value.proof.aliases[0], value => value.proof.occurrences[0], value => value.program.items[0].params[0]]) {
    const value = load(5); Object.defineProperty(target(value), 'host', { get() { invoked++; return 1; }, enumerable: true });
    refusal(value.program, value.proof);
  }
  assert.equal(invoked, 0);
});

test('default production raw identifier and namespace refusal paths are unchanged by this explicit experiment', () => {
  for (const entry of fixture.cases) {
    assert.throws(() => createTypedProgramLinks(entry.source, 'Rust'));
    const raw = tokenize('r#type', 'Rust').tokens;
    assert.deepEqual(raw.slice(0, 3).map(token => [token.kind, token.raw]), [['identifier', 'r'], ['punct', '#'], ['identifier', 'type']]);
  }
});

test('the experimental linked policy, fixtures and whole checked selectors regenerate without native compilation', () => {
  execFileSync(process.execPath, ['experiments/rust-raw-binding-identifiers/generate-rules.mjs', '--check', '--javascript-only'], { cwd: fileURLToPath(new URL('../../', import.meta.url)), stdio: 'pipe' });
});
