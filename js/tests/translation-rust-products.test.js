import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { test } from 'node:test';
import { runInNewContext } from 'node:vm';

import { translateProgram } from '../src/program-translation.js';
import { checkProgram } from '../src/translation/check.js';
import { parseRust } from '../src/translation/rust.js';
import { RustSourceOrigins } from '../src/translation/rust-source-origins.js';
import { acceptRustImmutableProductType, acceptRustProductOrigin, acceptRustTupleIndexSpelling } from '../src/translation/rust-product-rules.js';
import { readProjectData } from '../scripts/project-data.mjs';

const checkRust = source => checkProgram(parseRust(source));

test('shared product admission keeps selected owned origins distinct from immutable Copy evidence', () => {
  const fixture = readProjectData(new URL('../../experiments/owned-tuple-call-fixture.lino', import.meta.url));
  const pair = { kind: 'tuple', elements: [{ kind: 'string' }, { kind: 'fixed', bits: 32, signed: true }] };
  const origins = new RustSourceOrigins(new Map(), '', fixture.model);
  const fields = pair.elements.map(type => origins.typeOrigin(type, []));
  origins.typed(pair, [], [], fields);
  const origin = origins.type;
  assert.equal(origin.fields, fields);
  assert.equal(origins.immutableProduct(origin), false);
  assert.equal(origin.copyEvidence, false);
  assert.equal(fields[0].copyEvidence, false);
  assert.equal(origins.ownedLocalProduct(origin), true);
  assert.equal(acceptRustProductOrigin(origins.immutableProduct(origin), origins.ownedLocalProduct(origin)), true);
  origins.requireProduct(origin);

  const ordinary = new RustSourceOrigins(new Map());
  assert.equal(acceptRustProductOrigin(ordinary.immutableProduct(origin), ordinary.ownedLocalProduct(origin)), false);
  assert.throws(() => ordinary.requireProduct(origin), /immutable Rust tuple product/u);
  for (const change of [
    value => { value.wrappers = ['reference']; },
    value => { value.fields[0].wrappers = ['reference']; },
    value => { value.fields[0].textSourceModel = 'string slice'; },
    value => { value.fields[0].proven = false; },
    value => { value.type.elements[1].signed = false; },
  ]) {
    const altered = structuredClone(origin);
    change(altered);
    assert.equal(acceptRustProductOrigin(origins.immutableProduct(altered), origins.ownedLocalProduct(altered)), false);
    assert.throws(() => origins.requireProduct(altered), /immutable Rust tuple product/u);
  }
  const immutable = ordinary.typeOrigin({ kind: 'tuple', elements: [{ kind: 'bool' }, { kind: 'fixed', bits: 32, signed: true }] }, []);
  assert.equal(ordinary.immutableProduct(immutable), true);
  assert.equal(ordinary.ownedLocalProduct(immutable), false);
  assert.equal(acceptRustProductOrigin(ordinary.immutableProduct(immutable), ordinary.ownedLocalProduct(immutable)), true);
});

test('selected owned tuple headers retain actual field owners before target-specific diagnostics', () => {
  const fixture = readProjectData(new URL('../../experiments/owned-tuple-call-fixture.lino', import.meta.url));
  const source = fixture.cases[0].source;
  const surface = parseRust(source, { localValueStateModel: fixture.model });
  const result = surface.items[0].sourceSignature.result;
  assert.deepEqual(result.type, { kind: 'tuple', elements: [{ kind: 'string' }, { kind: 'fixed', bits: 32, signed: true }] });
  assert.deepEqual(result.owner.arguments.map(owner => [owner.kind, owner.path]), [
    ['library', ['std', 'string', 'String']], ['primitive', ['i32']],
  ]);
  assert.throws(() => parseRust(source), error => {
    const start = source.indexOf('(String,i32)');
    assert.equal(error.reason, 'immutable Rust tuple product: every field requires a direct recursively immutable scalar Copy source value');
    assert.deepEqual(error.span, { start, end: start + '(String,i32)'.length });
    return true;
  });
  const refusal = fixture.refusals.find(entry => entry.name === 'unknown target');
  assert.throws(() => checkProgram(parseRust(refusal.source, { localValueStateModel: fixture.model })), error => {
    assert.equal(error.kind, refusal.kind);
    assert.equal(error.reason, refusal.reason);
    assert.deepEqual(error.span, refusal.span);
    return true;
  });
});

test('Rust Copy products retain frozen construction, positional reads and immutable destructuring', () => {
  const source = `fn make() -> (u32, bool) { (5, true) }
fn choose(pair: (u32, bool)) -> u32 {
  let (value, flag) = pair;
  if flag { value + 2 } else { value }
}
fn main() {
  let pair = make();
  println!("{}", choose(pair));
  println!("{}", pair.0);
}`;
  const parsed = parseRust(source);
  assert.equal(parsed.items[0].body.k, 'tuple');
  assert.equal(parsed.items[0].body.frozen, true);
  assert.equal(parsed.items[0].body.productModel, 'immutable value product');
  const checked = checkProgram(parsed);
  assert.equal(checked.declarations.get('make').body.frozen, true);
  const translated = translateProgram(source, 'Rust', 'JavaScript');
  assert.equal(translated.diagnostic, null);
  const lines = [];
  const context = { console: { log: value => lines.push(String(value)) } };
  runInNewContext(translated.code, context);
  assert.deepEqual(lines, ['7', '5']);
  assert.equal(Object.isFrozen(context.make()), true);
  assert.equal(context.choose(context.make()), 7n);
});

test('nested tuple projections contextually split decimal tokens with exact source spans', () => {
  const source = 'fn main() { let pair = ((1u32, true), ()); println!("{}", pair.0.1); }';
  const parsed = parseRust(source);
  const projection = parsed.main.effects[1].expr.arg;
  assert.equal(projection.k, 'tupleIndex');
  assert.equal(projection.index, 1);
  assert.equal(projection.object.index, 0);
  assert.equal(source.slice(projection.span.start, projection.span.end), 'pair.0.1');
  assert.equal(source.slice(projection.object.span.start, projection.object.span.end), 'pair.0');
  checkProgram(parsed);
  const grouped = parseRust('fn value() -> f64 { (0.1) } fn main() {}');
  assert.equal(grouped.items[0].body.k, 'num');
  assert.equal(grouped.items[0].body.value, '0.1');
});

test('unit, singleton product and grouped scalar types and expressions stay distinct', () => {
  const parsed = parseRust(`fn unit() -> () { () }
fn one() -> (f64,) { (5.0,) }
fn scalar() -> (f64) { (5.0) }
fn main() { let () = unit(); let (value,) = one(); let (grouped) = scalar(); }`);
  assert.equal(parsed.items[0].ret.kind, 'unit');
  assert.equal(parsed.items[0].body.k, 'unit');
  assert.equal(parsed.items[1].ret.kind, 'tuple');
  assert.equal(parsed.items[1].ret.elements.length, 1);
  assert.equal(parsed.items[1].body.k, 'tuple');
  assert.equal(parsed.items[2].ret.kind, 'float');
  assert.equal(parsed.items[2].body.k, 'num');
  checkProgram(parsed);
});

test('nested destructuring evaluates its initializer once and preserves binder spans with file-wide hygiene', () => {
  const source = `fn make() -> ((u32, bool), ()) { ((5, true), ()) }
fn main() {
  let ((value, flag), ()) = make();
  let ml_tuple_value0 = 9u32;
  let ml_tuple_value1 = 10u32;
  println!("{}", value);
}`;
  const parsed = parseRust(source);
  const effects = parsed.main.effects;
  assert.equal(effects[0].name, 'ml_tuple_value2');
  assert.deepEqual(effects[0].value.path, ['make']);
  assert.equal(effects[1].name, 'value');
  assert.equal(effects[2].name, 'flag');
  assert.equal(source.slice(effects[1].span.start, effects[1].span.end), 'value');
  assert.equal(source.slice(effects[2].span.start, effects[2].span.end), 'flag');
  assert.equal(effects[1].value.object.object.path[0], effects[0].name);
  assert.equal(effects[2].value.object.object.path[0], effects[0].name);
  checkProgram(parsed);
});

test('Rust product admission retains syntactic numeric Copy evidence without inventing an integer width', () => {
  const origins = new RustSourceOrigins(new Map());
  const number = origins.origin({ k: 'num', value: '7' }, []);
  assert.equal(number.copyEvidence, true);
  assert.equal(number.type, null);
  const wrapped = { k: 'num', value: '7' };
  origins.wrap(wrapped, [], 'reference');
  assert.equal(origins.origin(wrapped, []).copyEvidence, true);
  assert.equal(origins.immutableProduct(origins.origin(wrapped, [])), false);
  const product = { k: 'tuple', items: [{ k: 'num', value: '7' }, { k: 'bool', value: true }], frozen: true };
  origins.product(product, []);
  assert.equal(origins.origin(product, []).type, null);
  assert.equal(origins.origin(product, []).fields[0].type, null);
  const concatenation = origins.origin({ k: 'binary', op: 'concat', left: { k: 'str', value: 'x' }, right: { k: 'str', value: 'y' } }, []);
  assert.equal(concatenation.copyEvidence, false);
});

test('pointer fields, owned strings, datatypes and nominal products retain precise refusals', () => {
  const cases = [
    ['fn take(pair: (&u32, bool)) -> bool { pair.1 } fn main() {}', /immutable Rust tuple product/u],
    ['fn take(pair: (Box<u32>, bool)) -> bool { pair.1 } fn main() {}', /immutable Rust tuple product/u],
    ['fn take(pair: (String, bool)) -> bool { pair.1 } fn main() {}', /immutable Rust tuple product/u],
    ['enum Value { Item } fn take(pair: (Value, bool)) -> bool { pair.1 } fn main() {}', /immutable Rust tuple product/u],
    ['fn main() { let pointer = &1u32; let pair = (pointer, true); }', /immutable Rust tuple product/u],
    ['fn main() { let pair = (Box::new(1u32), true); }', /immutable Rust tuple product/u],
    ['fn main() { let pair = (String::from("x"), true); }', /immutable tuple product/u],
    ['struct Pair(u32, bool); fn main() {}', /Rust struct item/u],
  ];
  for (const [source, message] of cases) assert.throws(() => checkRust(source), error => error.kind === 'unsupported' && message.test(error.message), source);
});

test('tuple binding refuses mutation, references, refutable coverage and duplicate names', () => {
  const cases = [
    ['fn main() { let (mut value, flag) = (1u32, true); }', /mutable tuple binding/u],
    ['fn main() { let (ref value, flag) = (1u32, true); }', /reference tuple binding/u],
    ['fn main() { let (&value, flag) = (1u32, true); }', /reference tuple binding/u],
    ['fn main() { let (value, value) = (1u32, 2u32); }', /duplicate tuple binding/u],
    ['fn main() { let (value, true) = (1u32, true); }', /refutable product binding/u],
    ['fn main() { let (value,) = (1u32, true); }', /tuple binding has the wrong number of fields/u],
    ['fn main() { let () = 1u32; }', /unit product binding/u],
    ['fn main() { let pair = (1u32, true); let value = match pair { (value, _) => value }; }', /tuple match pattern/u],
  ];
  for (const [source, message] of cases) assert.throws(() => checkRust(source), error => message.test(error.message), source);
});

test('tuple index spelling rejects separators, leading zeros, suffixes and non-decimal forms', () => {
  for (const spelling of ['00', '01', '0_0', '0u32', '1f64', '1e0', '0.01']) {
    const source = `fn main() { let pair = (1u32, true); let value = pair.${spelling}; }`;
    assert.throws(() => checkRust(source), error => error.kind === 'unsupported' && /tuple index spelling/u.test(error.message), spelling);
  }
  assert.throws(() => checkRust('fn main() { let pair = (1u32, true); let value = pair.9007199254740992; }'), /positional index exceeds/u);
  assert.throws(() => checkRust('fn main() { let pair = (1u32, true); let value = pair.2; }'), /tuple index out of range/u);
});

test('shared source product rules separate scalar Copy evidence and decimal index spellings', () => {
  for (const kind of ['bool', 'float', 'fixed', 'unit']) assert.equal(acceptRustImmutableProductType(kind, false, false), true, kind);
  for (const kind of ['string', 'data', 'named', 'array', 'record', 'int', 'nat']) assert.equal(acceptRustImmutableProductType(kind, true, false), false, kind);
  assert.equal(acceptRustImmutableProductType('tuple', true, false), true);
  assert.equal(acceptRustImmutableProductType('tuple', false, false), false);
  assert.equal(acceptRustImmutableProductType('tuple', true, true), false);
  for (const text of ['0', '1', '9', '10', '1234567890', '9007199254740991']) assert.equal(acceptRustTupleIndexSpelling(text), true, text);
  for (const text of ['', '00', '01', '-1', '1_0', '1u8', '1.0', '١', 'é', '💬', '\ud800', '0é']) assert.equal(acceptRustTupleIndexSpelling(text), false, text);
  const generated = readFileSync(new URL('../../rust/src/translation/rust/product_rules.rs', import.meta.url), 'utf8');
  assert.match(generated, /Generated from rust-product-rules\.js by the checked translator/u);
  assert.match(generated, /pub fn accept_rust_tuple_index_spelling\(text: &str\)/u);
  assert.match(generated, /text\.starts_with\('0'\)/u);
  assert.doesNotMatch(generated, /text\.starts_with\("0"\)/u);
});
