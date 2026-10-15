import assert from 'node:assert/strict';
import test, { after } from 'node:test';
import vm from 'node:vm';
import { readFileSync } from 'node:fs';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import { readProjectData } from '../../js/scripts/project-data.mjs';
import { LinkNetwork, encodeLinkNetworkBinary, decodeLinkNetworkBinary } from '../../js/src/index.js';
import * as published from '../../js/src/typed-program-links.js';
import { tokenize } from '../../js/src/translation/lexer.js';
import { createRustRawBindingCertificateOverlay } from './runtime.mjs';
import { RAW_BINDING_CERTIFICATE_POLICY as policy, readRustRawBindingCertificate, checkRustRawBindingCertificateInvocation } from './certificate.mjs';
import { rawBindingCertificateOverlayRecords } from './generate-overlay.mjs';

const overlay = await createRustRawBindingCertificateOverlay();
after(() => overlay.dispose());
const api = overlay.api;
const cases = readProjectData(new URL('../rust-raw-binding-identifiers/data/fixtures.lino', import.meta.url)).cases;
const source = entry => '\ufeff// Ω😀 complete original source\r\n/* nested /* ordinary */ trivia */\n' + entry.source;
const options = { [policy.sourceOption]: policy.sourceModel };
const create = (entry = cases[0]) => api.createTypedProgramLinks(source(entry), 'Rust', options);
const reloads = packet => [LinkNetwork.fromLino(packet.network.toLino()), decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))].map(network => ({ network, root: packet.root }));
const decode = packet => api.decodeTypedProgramLinks(packet.network, packet.root);
const label = name => published.TYPED_PROGRAM_LINKS_SCHEMA.fields[name] ?? name.replace(/[A-Z]/gu, letter => ' ' + letter.toLowerCase());
const field = (network, parent, name) => {
  const found = network.fieldRelations().filter(row => row.parent.equals(parent) && row.label === label(name));
  assert.equal(found.length, 1, name); return found[0];
};
function replaceChild(relation, child) { relation.link.setReferences([relation.parent, relation.link.references()[1], child]); }
function cloneRecord(network, id) {
  const clone = network.insertLink([], network.link(id).metadata());
  for (const relation of network.fieldRelations().filter(row => row.parent.equals(id))) network.insertFieldRelation(clone, relation.label, relation.child);
  return clone;
}
function execute(program, name, args) {
  const declaration = program.declarations.get(name);
  checkRustRawBindingCertificateInvocation(program, declaration, args);
  const emitted = overlay.javascript.emitJavaScript(program);
  assert.ok(emitted.assumptions.some(row => row.id === policy.inputAssumption.id));
  assert.ok(emitted.encodings.some(row => row.id === policy.encoding.id));
  assert.equal(emitted.definitions.length, program.items.length);
  const context = {};
  vm.runInNewContext(emitted.text, context, { timeout: 1000 });
  const target = emitted.mappings.find(row => row.kind === 'function' && row.source === declaration.fullName)?.target ?? name;
  assert.equal(typeof context[target], 'function'); return context[target](...args);
}
function refused(program) {
  assert.throws(() => readRustRawBindingCertificate(program));
  assert.throws(() => api.encodeTypedProgramLinks(program));
  assert.throws(() => overlay.validation.validateLinkedTypedProgram(program));
  assert.throws(() => overlay.projection.recheckLinkedTypedProgram(program));
  assert.throws(() => overlay.projection.recheckLinkedTypedProgram(program, { wholeProgram: true }));
  assert.throws(() => overlay.javascript.emitJavaScript(program));
  assert.throws(() => overlay.rust.emitRust(program));
}

test('actual decorated public factory, text and binary archives preserve full source and execute bounded primitive observations', () => {
  for (const entry of cases) for (const packet of reloads(create(entry))) {
    const program = decode(packet), selected = readRustRawBindingCertificate(program);
    assert.equal(selected.facet.worker, program.main);
    assert.equal(program.main[policy.contextField].sourceExtent, selected.facet.span);
    assert.equal(overlay.projection.recheckLinkedTypedProgram(program, { wholeProgram: true }), program);
    assert.deepEqual(Buffer.from(api.emitTypedProgramLinks(packet.network, packet.root, 'Rust').code), Buffer.from(source(entry)));
    const rust = overlay.rust.emitRust(program);
    assert.equal(rust.definitions.length, program.items.length);
    for (const definition of rust.definitions) assert.match(definition, /^(?:pub )?fn /u);
    for (const mapping of rust.mappings) assert.ok(rust.text.includes('fn ' + mapping.target + '('), 'each Rust target is the actual emitted callable identifier spelling');
    if (entry === cases[0]) assert.equal(rust.mappings[0].target, 'r#match');
    const values = entry.arguments.map(value => typeof value === 'string' ? BigInt(value) : value);
    assert.equal(execute(program, entry.name, values), typeof entry.value === 'string' ? BigInt(entry.value) : entry.value);
    const reimported = api.createTypedProgramLinks(rust.text, 'Rust', options);
    assert.equal(execute(decode(reimported), entry.name, values), typeof entry.value === 'string' ? BigInt(entry.value) : entry.value);
    assert.equal(Object.hasOwn(program, 'source'), false);
    assert.equal(Object.hasOwn(program, 'provenance'), false);
  }
});

test('saved program facts preserve actual main, source extent, Fn, Param, expression and ordered parameter-array LinkIds', () => {
  for (const packet of reloads(create(cases[2]))) {
    const { network, root } = packet, program = decode(packet), facet = program[policy.facetField];
    const body = network.link(root).references()[0], fact = field(network, body, policy.facetField).child;
    const main = field(network, body, 'main').child;
    assert.ok(field(network, fact, 'worker').child.equals(main));
    const context = field(network, main, policy.contextField).child;
    assert.ok(field(network, context, 'sourceExtent').child.equals(field(network, fact, 'span').child));
    const declarations = new Set(), parameters = new Set();
    for (const pair of network.link(field(network, body, 'declarations').child).references()) {
      const declaration = network.link(pair).references()[1]; declarations.add(declaration.asU64());
      for (const parameter of network.link(field(network, declaration, 'params').child).references()) parameters.add(parameter.asU64());
    }
    const aliases = field(network, fact, 'aliases').child;
    for (const id of network.link(aliases).references()) {
      const checkedOwner = field(network, id, 'checkedOwner').child;
      const alias = facet.aliases.find(row => row.value === network.link(field(network, id, 'value').child).metadata().definition);
      const token = field(network, id, 'token').child;
      const role = alias.kind === 'declaration' ? 'declaration' : 'binding';
      assert.ok(field(network, token, role).child.equals(checkedOwner));
      if (alias.kind === 'declaration') assert.ok(declarations.has(checkedOwner.asU64()));
      else assert.ok(parameters.has(checkedOwner.asU64()));
    }
    const occurrences = field(network, fact, 'occurrences').child;
    for (const id of network.link(occurrences).references()) {
      const inverse = field(network, id, 'alias').child, owner = field(network, inverse, 'checkedOwner').child;
      const expression = field(network, id, 'checkedExpression').child;
      const reference = network.fieldRelations().find(row => row.parent.equals(expression) && Object.values(published.TYPED_PROGRAM_LINKS_SCHEMA.references).includes(row.label));
      assert.ok(reference?.child.equals(owner));
    }
    for (const id of network.link(field(network, fact, 'inputAssumptions').child).references()) {
      const declaration = field(network, id, 'declaration').child;
      assert.ok(declarations.has(declaration.asU64()));
      assert.ok(field(network, id, 'parameterOwners').child.equals(field(network, declaration, 'params').child));
    }
    for (const premise of facet.inputAssumptions) assert.equal(premise.parameterOwners, premise.declaration.params);
  }
});

test('coherent source-free owner edits survive both archives and fresh complete Rust reimport', () => {
  for (const packet of reloads(create(cases[5]))) {
    let program = decode(packet), facet = program[policy.facetField];
    const target = program.items[0].params[1], occurrence = facet.occurrences[0];
    occurrence.token.binding = target;
    occurrence.alias = facet.aliases.find(alias => alias.checkedOwner === target);
    occurrence.checkedExpression.name = target.name;
    target.name = 'loop'; occurrence.checkedExpression.name = 'loop';
    const saved = reloads(api.encodeTypedProgramLinks(program))[1];
    program = decode(saved); facet = program[policy.facetField];
    assert.equal(execute(program, 'choose', [13n, 17n]), 17n);
    const text = api.emitTypedProgramLinks(saved.network, saved.root, 'Rust').code;
    assert.match(text, /r#loop:i32/u); assert.match(text, /\{r#loop\}/u);
    assert.ok(facet.tokens.some(token => token.raw === 'match'), 'original owned spelling survives the semantic edit');
    const fresh = decode(api.createTypedProgramLinks(text, 'Rust', options));
    assert.equal(execute(fresh, 'choose', [13n, 17n]), 17n);
    const declaration = program.items[0]; declaration.name = 'loop'; declaration.fullName = 'loop';
    program.declarations = new Map([[declaration.fullName, declaration]]);
    const renamed = overlay.rust.emitRust(program);
    assert.equal(renamed.mappings[0].source, 'loop'); assert.equal(renamed.mappings[0].target, 'r#loop');
    assert.ok(renamed.text.includes('fn r#loop('));
    const rebound = api.createTypedProgramLinks(renamed.text, 'Rust', options);
    assert.equal(execute(decode(rebound), 'loop', [13n, 17n]), 17n);
  }
});

test('stripped, misplaced, null and equal-value cloned certificate facts refuse at every actual object boundary', () => {
  const attacks = [
    p => { delete p[policy.facetField]; },
    p => { delete p.main[policy.contextField]; },
    p => { p[policy.facetField] = null; },
    p => { p.main[policy.contextField] = null; },
    p => { p[policy.facetField].worker = structuredClone(p.main); },
    p => { p.main[policy.contextField].sourceExtent = { ...p[policy.facetField].span }; },
    p => { p.main[policy.contextField].model = 'unknown'; },
    p => { p.items[0][policy.contextField] = p.main[policy.contextField]; delete p.main[policy.contextField]; },
    p => { p.items[0][policy.facetField] = p[policy.facetField]; delete p[policy.facetField]; },
    p => { p[policy.facetField].aliases[0].checkedOwner = structuredClone(p.items[0]); },
    p => { p[policy.facetField].occurrences[0].checkedExpression = structuredClone(p[policy.facetField].occurrences[0].checkedExpression); },
    p => { p[policy.facetField].inputAssumptions[0].parameterOwners = [...p.items[0].params]; },
    p => { p[policy.facetField].occurrences[0].checkedExpression.name = 'injected'; },
    p => { p[policy.facetField].tokens.find(token => token.raw === '#').span.start--; },
  ];
  for (const attack of attacks) { const program = decode(create(cases[5])); attack(program); refused(program); }
  for (const substitute of [p => Object.assign(function host() {}, p), p => Object.create(p)]) {
    const original = decode(create(cases[5]));
    original[policy.facetField].aliases[0].checkedOwner = structuredClone(original.items[0]);
    refused(substitute(original));
  }
  for (const substitute of [main => Object.assign(function host() {}, main), main => Object.create(main)]) {
    const program = decode(create(cases[5])); program.main = substitute(program.main); delete program[policy.facetField];
    refused(program);
  }
  for (const role of [policy.facetField, policy.contextField]) {
    const program = decode(create(cases[5]));
    const value = role === policy.facetField ? program[role] : program.main[role];
    delete program[policy.facetField]; delete program.main[policy.contextField];
    program.hidden = Object.assign(function host() {}, { [role]: value });
    refused(program);
    const inherited = decode(create(cases[5]));
    const inheritedValue = role === policy.facetField ? inherited[role] : inherited.main[role];
    delete inherited[policy.facetField]; delete inherited.main[policy.contextField];
    inherited.hidden = Object.create({ [role]: inheritedValue });
    refused(inherited);
  }
  const nullPrototype = Object.assign(Object.create(null), decode(create()));
  assert.ok(readRustRawBindingCertificate(nullPrototype));
  assert.equal(overlay.rust.emitRust(nullPrototype).text, source(cases[0]));
});

test('genuine saved relation stripping and same-value LinkId substitutions fail before archived emission', () => {
  const attacks = [
    (n, body, fact, main) => field(n, body, policy.facetField).link.setReferences([]),
    (n, body, fact, main) => field(n, main, policy.contextField).link.setReferences([]),
    (n, body, fact, main) => replaceChild(field(n, fact, 'worker'), cloneRecord(n, main)),
    (n, body, fact, main) => {
      const context = field(n, main, policy.contextField).child;
      replaceChild(field(n, context, 'sourceExtent'), cloneRecord(n, field(n, fact, 'span').child));
    },
    (n, body, fact) => {
      const id = n.link(field(n, fact, 'inputAssumptions').child).references()[0], parameters = field(n, id, 'parameterOwners');
      const copy = n.insertLink(n.link(parameters.child).references(), n.link(parameters.child).metadata());
      replaceChild(parameters, copy);
    },
    (n, body, fact) => {
      const id = n.link(field(n, fact, 'aliases').child).references()[0], owner = field(n, id, 'checkedOwner');
      replaceChild(owner, cloneRecord(n, owner.child));
    },
    (n, body, fact) => {
      const id = n.link(field(n, fact, 'occurrences').child).references()[0], expression = field(n, id, 'checkedExpression');
      replaceChild(expression, cloneRecord(n, expression.child));
    },
  ];
  for (const attack of attacks) for (const packet of reloads(create(cases[5]))) {
    const { network: n, root } = packet, body = n.link(root).references()[0], fact = field(n, body, policy.facetField).child, main = field(n, body, 'main').child;
    attack(n, body, fact, main);
    for (const saved of reloads(packet)) {
      assert.throws(() => decode(saved));
      assert.throws(() => api.emitTypedProgramLinks(saved.network, saved.root, 'Rust'));
      assert.throws(() => api.emitTypedProgramLinks(saved.network, saved.root, 'JavaScript'));
    }
  }
});

test('owned invocation premises, source/target profile refusals and accessor guards preserve their exact domains', () => {
  const program = decode(create(cases[5])), declaration = program.items[0];
  for (const args of [[-2147483648n, 2147483647n], [13n, 17n]]) assert.equal(checkRustRawBindingCertificateInvocation(program, declaration, args).parameterOwners, declaration.params);
  for (const args of [[1, 2], [1n], [1n, 2n, 3n], [-2147483649n, 1n], [2147483648n, 1n], [0.5, 1n], [true, 1n]]) assert.throws(() => checkRustRawBindingCertificateInvocation(program, declaration, args));
  assert.throws(() => checkRustRawBindingCertificateInvocation(program, structuredClone(declaration), [1n, 2n]));
  let invoked = 0;
  const args = [1n, 2n]; Object.defineProperty(args, '0', { get() { invoked++; return 1n; } });
  assert.throws(() => checkRustRawBindingCertificateInvocation(program, declaration, args));
  for (const target of [p => p, p => p.main, p => p[policy.facetField], p => p[policy.facetField].aliases[0]]) {
    const p = decode(create()); Object.defineProperty(target(p), 'host', { enumerable: true, get() { invoked++; return 1; } }); refused(p);
  }
  for (const role of [policy.facetField, 'main']) {
    const p = decode(create()), value = p[role];
    if (role === 'main') delete p[policy.facetField];
    Object.defineProperty(p, role, { enumerable: true, get() { invoked++; return value; } }); refused(p);
  }
  const misplaced = decode(create());
  delete misplaced[policy.facetField]; delete misplaced.main[policy.contextField];
  misplaced.hidden = function host() {};
  Object.defineProperty(misplaced.hidden, policy.facetField, { enumerable: true, get() { invoked++; return null; } });
  refused(misplaced);
  for (const role of [policy.facetField, policy.contextField]) {
    const inherited = decode(create()), prototype = {};
    delete inherited[policy.facetField]; delete inherited.main[policy.contextField];
    Object.defineProperty(prototype, role, { enumerable: true, get() { invoked++; return null; } });
    inherited.hidden = Object.create(prototype); refused(inherited);
  }
  assert.equal(invoked, 0);
  for (const name of policy.sourceOptions) assert.throws(() => api.createTypedProgramLinks(source(cases[0]), 'Rust', { ...options, [name]: '' }));
  for (const extra of [{ utf16Strings: true }, { context: {} }, { [policy.sourceOption]: 'unknown' }]) assert.throws(() => api.createTypedProgramLinks(source(cases[0]), 'Rust', { ...options, ...extra }));
  assert.throws(() => api.createTypedProgramLinks('function main(){}', 'JavaScript', options));
  for (const value of ['unknown', '', false, null, 0]) for (const [target, key] of [['Rust', 'rustSourceHeaderEmissionModel'], ['JavaScript', 'javaScriptScopedBindingEmissionModel']]) {
    const packet = create(); assert.throws(() => api.emitTypedProgramLinks(packet.network, packet.root, target, { [key]: value }));
  }
});

test('the published factory and default lexer retain every protected raw-name refusal and ordinary behavior', () => {
  for (const entry of cases) {
    assert.throws(() => published.createTypedProgramLinks(entry.source, 'Rust'));
    assert.throws(() => published.createTypedProgramLinks(entry.source, 'Rust', options));
    assert.throws(() => api.createTypedProgramLinks(entry.source, 'Rust'));
  }
  assert.deepEqual(tokenize('r#type', 'Rust').tokens.map(token => token.raw), ['r', '#', 'type', '']);
  const ordinary = 'pub fn identity(value:i32)->i32{value}fn main(){}';
  const old = published.createTypedProgramLinks(ordinary, 'Rust'), current = api.createTypedProgramLinks(ordinary, 'Rust');
  assert.equal(current.network.toLino(), old.network.toLino());
  assert.deepEqual(api.emitTypedProgramLinks(current.network, current.root, 'JavaScript'), published.emitTypedProgramLinks(old.network, old.root, 'JavaScript'));
  const program = decode(current);
  const expected = [overlay.rust.emitRust(program), overlay.javascript.emitJavaScript(program)];
  Object.defineProperty(program, 'inferredExpressionTypes', { value: new WeakMap() });
  const nullRoot = Object.defineProperties(Object.create(null), Object.getOwnPropertyDescriptors(program));
  for (const input of [program, nullRoot]) {
    assert.equal(readRustRawBindingCertificate(input), null);
    assert.equal(Object.getOwnPropertyDescriptor(input, 'inferredExpressionTypes').enumerable, false);
    assert.deepEqual([overlay.rust.emitRust(input), overlay.javascript.emitJavaScript(input)], expected);
  }
});

test('shared data, checked module hashes and actual translated predicates regenerate while the native identity gap remains explicit', () => {
  assert.deepEqual(overlay.receipts, rawBindingCertificateOverlayRecords().map(({ module, sourceSha256, overlaySha256 }) => ({ module, sourceSha256, overlaySha256 })));
  const cwd = fileURLToPath(new URL('../../', import.meta.url));
  for (const generator of ['generate-rules.mjs', 'generate-overlay.mjs']) execFileSync(process.execPath, ['experiments/rust-raw-binding-certificate-adapter/' + generator, '--check'], { cwd, stdio: 'pipe' });
  const native = readFileSync(new URL('../../' + policy.nativeBoundary.programSource, import.meta.url), 'utf8');
  assert.doesNotMatch(native, /source_raw_binding_identifiers|source_raw_binding_identifier_context/u);
  assert.equal(policy.nativeBoundary.requiredRelations.length, 5);
  assert.match(policy.nativeBoundary.gap, /original-LinkId/u);
  assert.match(readFileSync(new URL('./data/rules.rs', import.meta.url), 'utf8'), /pub const fn accept_rust_raw_binding_certificate_owners/u);
});
