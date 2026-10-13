import { readStringMapOperation, readStringTestOperation } from './frontend-rules.js';
// Rust frontend for the portable core. It reads the subset of Rust the core
// can represent faithfully: modules with `use` of crate items, enums with
// unit and tuple variants (`Box<T>` fields are the recursive occurrences of
// a value type), free functions over machine integers, `bool` and `String`,
// expression bodies with `let`, `if`, `match` and `format!`, and a `main`
// made of `println!`, `let` and `assert!`/`assert_eq!` statements.
// Everything else is rejected with a precise obligation.

import { rustMacroMessage } from './aborts.js';
import { TranslationError, unsupported } from './diagnostics.js';
import { TokenCursor, describe, tokenize } from './lexer.js';
import { BOOL, FLOAT, STRING, UNIT, rustFixedType, tuple } from './types.js';
import { RustSourceOrigins } from './rust-source-origins.js';
import { acceptRustSourceBoxOwner, acceptRustSourceLifetimeBinding, acceptRustSourceLifetimeReference, acceptRustSourceLifetimeResult } from './rust-indirection-rules.js';
import { acceptRustTupleIndexSpelling } from './rust-product-rules.js';
import { checkProgram } from './check.js';

const ACCEPTED_DERIVES = new Set(['Clone', 'Copy', 'Debug', 'PartialEq', 'Eq', 'Hash', 'PartialOrd', 'Ord']);
const ACCEPTED_LINTS = new Set(['allow', 'warn', 'deny', 'expect', 'must_use', 'inline']);
const COMPARISON = { '==': 'eq', '!=': 'ne', '<': 'lt', '<=': 'le', '>': 'gt', '>=': 'ge' };
const ADDITIVE = { '+': 'add', '-': 'sub' };
const MULTIPLICATIVE = { '*': 'mul', '/': 'div', '%': 'rem' };
const RESERVED_ITEMS = new Set(['impl', 'trait', 'struct', 'union', 'static', 'type', 'extern', 'macro_rules', 'async', 'unsafe']);

export function parseRust(source) {
  const { tokens } = tokenize(source, 'Rust');
  const declarations = new RustParser(tokens);
  declarations.file();
  declarations.origins.resolveTypes((segments, path) => declarations.resolve(segments, path));
  const syntax = new RustParser(tokens, declarations.origins.catalog);
  syntax.origins.collecting = true;
  syntax.aliases = declarations.aliases;
  const surface = syntax.file();
  // Preserve syntax trees and the existing checker's diagnostic precedence
  // for programs that already cannot enter the canonical semantic core.
  try { checkProgram(surface); } catch (error) {
    if (!(error instanceof TranslationError)) throw error;
    return surface;
  }
  const parser = new RustParser(tokens, syntax.origins.catalog);
  parser.aliases = syntax.aliases;
  return parser.file();
}

class RustParser {
  constructor(tokens, catalog = null) {
    this.cursor = new TokenCursor([...tokens], 'Rust');
    // `use` aliases, per module path.
    this.aliases = new Map();
    this.modulePath = [];
    this.crateDeclarations = new Set();
    this.origins = new RustSourceOrigins(catalog);
    this.reservedNames = new Set(tokens.filter(token => token.kind === 'identifier').map(token => token.value));
    this.productBindingCount = 0;
    this.lifetimeBindings = new Set();
    this.lifetimeHeader = false;
  }

  fail(message, token = this.cursor.peek()) {
    return new TranslationError('syntax', `${message} but found ${describe(token)}`, token);
  }

  file() {
    const root = { items: [], main: null };
    this.innerAttributes();
    this.items(root, root.items, [], () => this.cursor.atEnd());
    if (!root.main) throw new TranslationError('syntax', 'a Rust program needs fn main', this.cursor.peek());
    return { language: 'Rust', items: root.items, main: root.main };
  }

  innerAttributes() {
    const c = this.cursor;
    while (c.is('#') && c.is('!', 1)) {
      const start = c.next();
      c.next();
      this.attributeBody(start);
    }
  }

  attributes() {
    const c = this.cursor;
    while (c.is('#') && c.is('[', 1)) {
      const start = c.next();
      this.attributeBody(start);
    }
  }

  /** `#[derive(Clone, …)]` and lint attributes do not change behaviour. */
  attributeBody(start) {
    const c = this.cursor;
    c.expect('[', 'attribute');
    const name = c.identifier('attribute').value;
    if (name === 'derive') {
      c.expect('(', 'derive');
      while (!c.is(')')) {
        const trait = c.identifier('derive');
        if (!ACCEPTED_DERIVES.has(trait.value)) throw unsupported(`derive(${trait.value})`, 'only derives that add no behaviour are portable', span(trait, trait));
        if (!c.eat(',')) break;
      }
      c.expect(')', 'derive');
    } else if (ACCEPTED_LINTS.has(name)) {
      let depth = 0;
      while (!c.atEnd() && !(depth === 0 && c.is(']'))) {
        if (c.is('(')) depth += 1;
        if (c.is(')')) depth -= 1;
        c.next();
      }
    } else {
      throw unsupported(`#[${name}]`, 'attributes that change compilation are outside the portable core', span(start, c.peek()));
    }
    c.expect(']', 'attribute');
  }

  items(root, items, path, done) {
    const c = this.cursor;
    while (!done()) {
      this.attributes();
      const start = c.peek();
      this.modulePath = path;
      this.visibility();
      if (c.is('mod')) {
        c.next();
        const name = c.identifier('mod').value;
        if (c.is(';')) throw unsupported('out-of-line module', 'the program must be a single file', span(start, c.peek()));
        c.expect('{', 'mod');
        const module = { k: 'module', name, items: [], span: span(start, c.peek()) };
        this.origins.declaration(name, path, { k: 'module' });
        items.push(module);
        this.items(root, module.items, [...path, name], () => c.is('}'));
        c.expect('}', 'mod');
        continue;
      }
      if (path.length === 0 && c.is('extern') && c.is('crate', 1) && c.is('alloc', 2) && c.is(';', 3)) {
        const key = [...path, 'alloc'].join('.');
        if (this.crateDeclarations.has(key)) throw unsupported('extern crate alloc', 'each source crate declaration must be unique in its module', span(start, start));
        this.crateDeclarations.add(key);
        for (let index = 0; index < 4; index += 1) c.next();
        this.origins.declaration('alloc', path, { k: 'externalCrate' });
        continue;
      }
      if (c.is('use')) {
        this.use(path);
        continue;
      }
      if (c.is('enum')) {
        items.push(this.enumItem(path));
        continue;
      }
      if (c.is('fn') || c.is('const')) {
        if (c.is('fn') && c.is('main', 1) && path.length === 0) {
          root.main = this.main();
          continue;
        }
        items.push(c.is('fn') ? this.fn(path) : this.constItem(path));
        continue;
      }
      if (start.kind === 'identifier' && RESERVED_ITEMS.has(start.value)) {
        throw unsupported(`Rust ${start.value} item`, `${start.value} items are outside the portable core`, span(start, start));
      }
      if (c.atEnd()) throw new TranslationError('syntax', `module ${path.at(-1)} is not closed`, start);
      throw this.fail('expected a Rust item');
    }
  }

  visibility() {
    const c = this.cursor;
    if (!c.eat('pub')) return;
    if (c.is('(')) {
      c.next();
      c.eat('in');
      while (!c.is(')')) c.next();
      c.expect(')', 'visibility');
    }
  }

  /**
   * `use super::Tree;`, `use crate::a::{b, c as d};`: each imported name is
   * an alias for its absolute path inside the importing module.
   */
  use(path) {
    const c = this.cursor;
    const start = c.next();
    const key = path.join('.');
    if (!this.aliases.has(key)) this.aliases.set(key, new Map());
    const aliases = this.aliases.get(key);
    const tree = (prefix) => {
      if (c.eat('{')) {
        while (!c.is('}')) {
          tree(prefix);
          if (!c.eat(',')) break;
        }
        c.expect('}', 'use');
        return;
      }
      if (c.is('*')) throw unsupported('glob import', 'name each imported item explicitly', span(start, c.peek()));
      const segment = c.identifier('use').value;
      const full = [...prefix, segment];
      if (c.eat('::')) {
        tree(full);
        return;
      }
      const standardBox = full.length > 1 && acceptRustSourceBoxOwner(full.join('::'), false);
      if (!['crate', 'super', 'self'].includes(full[0]) && !standardBox) {
        throw unsupported(`use ${full.join('::')}`, 'only items of this crate can be imported; the standard library is outside the portable core', span(start, c.peek()));
      }
      if (standardBox) this.origins.box(full, full, path, span(start, c.peek()));
      const alias = c.eat('as') ? c.identifier('use alias').value : segment;
      aliases.set(alias, standardBox ? full : absolute(full, path, span(start, c.peek())));
    };
    tree([]);
    c.expect(';', 'use');
  }

  /** A `use` alias is visible only in the module that declares it. */
  resolve(segments, path) {
    const aliases = this.aliases.get(path.join('.'));
    return aliases?.has(segments[0]) ? [...aliases.get(segments[0]), ...segments.slice(1)] : segments;
  }

  enumItem(path) {
    const c = this.cursor;
    const start = c.next();
    const name = c.identifier('enum').value;
    if (c.is('<')) throw unsupported('generic enum', 'type parameters are outside the portable core', span(start, c.peek()));
    c.expect('{', 'enum');
    const ctors = [];
    const owner = [...path, name].join('.');
    this.origins.declaration(name, path, { k: 'data' });
    while (!c.is('}')) {
      this.attributes();
      const variant = c.identifier('variant');
      const fields = [];
      const sourceFields = [];
      if (c.eat('(')) {
        while (!c.is(')')) {
          fields.push({ type: this.type() });
          sourceFields.push(this.origins.type);
          if (!c.eat(',')) break;
        }
        c.expect(')', 'variant');
      } else if (c.is('{')) {
        throw unsupported('struct variant', 'use a tuple variant; named fields are outside the Rust frontend', span(variant, c.peek()));
      } else if (c.is('=')) {
        throw unsupported('explicit discriminant', 'discriminant values are outside the portable core', span(variant, c.peek()));
      }
      ctors.push({ name: variant.value, fields });
      this.origins.declaration(variant.value, [...path, name], { k: 'ctor', owner, fields: sourceFields, result: { wrappers: [], type: { kind: 'named', path: ['crate', ...path, name] }, path: [] } });
      if (!c.eat(',')) break;
    }
    c.expect('}', 'enum');
    return { k: 'data', name, ctors, span: span(start, c.peek()) };
  }

  /**
   * Types: machine integers, `bool`, `String`, `&str`, `()`, crate types,
   * and `&T`/`Box<T>`, which denote the value `T` since every portable value
   * is immutable.
   */
  type() {
    const c = this.cursor;
    const token = c.peek();
    if (c.eat('&')) {
      let sourceLifetime = null;
      if (c.is('\'')) {
        c.next();
        const lifetime = c.identifier('lifetime');
        if (!acceptRustSourceLifetimeReference(lifetime.value, this.lifetimeBindings.has(lifetime.value))) throw unsupported('lifetime parameter', 'a reference needs a declared function lifetime, an elided placeholder, or \'static', span(token, lifetime));
        if (this.lifetimeBindings.has(lifetime.value) && !this.lifetimeHeader) throw unsupported('local lifetime annotation', 'named lifetimes in body annotations require an explicit region proof', span(token, lifetime));
        sourceLifetime = lifetime.value;
      }
      if (c.is('mut')) throw unsupported('mutable reference', 'mutation is outside the portable core', span(token, c.peek()));
      if (c.is('str')) {
        c.next();
        const result = this.origins.typed(STRING, this.modulePath, ['reference'], null, 'string slice');
        this.origins.type.sourceLifetimes = [sourceLifetime];
        return result;
      }
      const inner = this.type();
      const lifetimes = [sourceLifetime, ...(this.origins.type.sourceLifetimes ?? [])];
      const result = this.origins.typed(inner, this.origins.type.path, ['reference', ...this.origins.type.wrappers], this.origins.type.fields, this.origins.type.textSourceModel);
      this.origins.type.sourceLifetimes = lifetimes;
      return result;
    }
    if (c.eat('(')) {
      if (c.eat(')')) return this.origins.typed(UNIT, this.modulePath);
      const first = this.type();
      const fields = [this.origins.type];
      if (!c.eat(',')) {
        c.expect(')', 'parenthesised type');
        return first;
      }
      const elements = [first];
      while (!c.is(')')) {
        elements.push(this.type());
        fields.push(this.origins.type);
        if (!c.eat(',')) break;
      }
      c.expect(')', 'tuple type');
      const result = this.origins.typed(tuple(elements), this.modulePath, [], fields);
      if (!this.origins.immutableProduct(this.origins.type)) {
        throw unsupported('immutable Rust tuple product', 'every field requires a direct recursively immutable scalar Copy source value', span(token, c.peek()));
      }
      return result;
    }
    const segments = this.pathSegments('type');
    const name = segments.join('::');
    const resolved = this.resolve(segments, this.modulePath);
    if (acceptRustSourceBoxOwner(name, false) || acceptRustSourceBoxOwner(resolved.join('::'), false)) {
      this.origins.box(segments, resolved, this.modulePath, span(token, c.peek()));
      c.expect('<', 'Box');
      const inner = this.type();
      c.expect('>', 'Box');
      const lifetimes = this.origins.type.sourceLifetimes;
      const result = this.origins.typed(inner, this.origins.type.path, ['box', ...this.origins.type.wrappers], this.origins.type.fields, this.origins.type.textSourceModel);
      if (lifetimes) this.origins.type.sourceLifetimes = lifetimes;
      return result;
    }
    if (c.is('<')) throw unsupported(`type ${name}<…>`, 'generic types are outside the portable core', span(token, c.peek()));
    const fixed = segments.length === 1 ? rustFixedType(name) : undefined;
    if (fixed) return this.origins.typed(fixed, this.modulePath);
    if (name === 'bool') return this.origins.typed(BOOL, this.modulePath);
    if (name === 'String') return this.origins.typed(STRING, this.modulePath);
    if (name === 'f64') return this.origins.typed(FLOAT, this.modulePath);
    if (['str', 'char', 'f32', 'Vec', 'Option', 'Result', 'Rc', 'Arc', 'HashMap', 'Self'].includes(name)) {
      throw unsupported(`Rust type ${name}`, 'outside the portable core', span(token, token));
    }
    return this.origins.typed({ kind: 'named', path: this.resolve(segments, this.modulePath), span: span(token, token) }, this.modulePath);
  }

  pathSegments(context) {
    const c = this.cursor;
    const segments = [c.identifier(context).value];
    while (c.is('::') && c.peek(1).kind === 'identifier') {
      c.next();
      segments.push(c.next().value);
    }
    return segments;
  }

  fn(path) {
    const c = this.cursor;
    const start = c.next();
    const name = c.identifier('fn').value;
    const outerLifetimes = this.lifetimeBindings;
    const outerHeader = this.lifetimeHeader;
    this.lifetimeBindings = new Set();
    this.lifetimeHeader = true;
    try {
      if (c.is('<')) {
        const generic = c.next();
        while (!c.is('>')) {
          if (!c.eat("'")) throw unsupported('generic function', 'type parameters are outside the portable core', span(start, generic));
          const lifetime = c.identifier('lifetime');
          if (!acceptRustSourceLifetimeBinding(lifetime.value, this.lifetimeBindings.has(lifetime.value), c.is(':'))) throw unsupported('function lifetime binding', 'lifetime names must be unique and unconstrained; bounds need their own linked obligations', span(lifetime, c.peek()));
          this.lifetimeBindings.add(lifetime.value);
          if (!c.eat(',')) break;
        }
        c.expect('>', 'function lifetimes');
        if (!this.lifetimeBindings.size) throw unsupported('function lifetime binding', 'a lifetime parameter list must contain a named binding', span(generic, c.peek()));
      }
      c.expect('(', 'fn');
      const params = [];
      const sourceParams = [];
      while (!c.is(')')) {
        const paramToken = c.peek();
        if (c.is('mut')) throw unsupported('mutable parameter', 'mutation is outside the portable core', span(paramToken, paramToken));
        if (c.is('self') || c.is('&')) throw unsupported('method', 'methods are outside the Rust frontend; declare a free function', span(paramToken, paramToken));
        const paramName = c.identifier('parameter').value;
        c.expect(':', 'parameter');
        params.push({ name: paramName, type: this.type(), span: span(paramToken, c.peek()) });
        sourceParams.push(this.origins.type);
        if (!c.eat(',')) break;
      }
      c.expect(')', 'fn');
      if (!c.eat('->')) throw unsupported('function without a result', `${name} returns (); only value-returning functions are portable`, span(start, c.peek()));
      const ret = this.type();
      const sourceResult = this.origins.type;
      if (!acceptRustSourceLifetimeResult(this.lifetimeBindings.size > 0, sourceResult.wrappers.includes('reference'))) throw unsupported('function lifetime result', 'reference results with lifetime binders require an explicit return-lifetime instantiation and outlives proof', span(start, c.peek()));
      this.origins.declaration(name, path, { k: 'fn', lifetimeBindings: [...this.lifetimeBindings], parameters: sourceParams, result: sourceResult });
      if (c.is('where')) throw unsupported('where clause', 'trait bounds are outside the portable core', span(start, c.peek()));
      this.lifetimeHeader = false;
      const previous = this.origins.enter(new Map(params.map((param, index) => [param.name, sourceParams[index]])));
      const body = this.block(path);
      this.origins.argument(this.origins.origin(body, path), sourceResult, body.span);
      this.origins.scope = previous;
      return { k: 'fn', name, params, ret, body, span: span(start, c.peek()) };
    } finally {
      this.lifetimeBindings = outerLifetimes;
      this.lifetimeHeader = outerHeader;
    }
  }

  /** `const N: T = e;` is a function without parameters. */
  constItem(path) {
    const c = this.cursor;
    const start = c.next();
    const name = c.identifier('const').value;
    c.expect(':', 'const');
    const ret = this.type();
    const sourceResult = this.origins.type;
    this.origins.declaration(name, path, { k: 'fn', parameters: [], result: sourceResult });
    c.expect('=', 'const');
    const body = this.expr(path);
    this.origins.argument(this.origins.origin(body, path), sourceResult, body.span);
    c.expect(';', 'const');
    return { k: 'fn', name, params: [], ret, body, span: span(start, c.peek()) };
  }

  /**
   * A block `{ let x = e; …; tail }` is a chain of lets ending in its tail
   * expression. Statements with effects are not values of the portable core.
   */
  block(path) {
    const c = this.cursor;
    const open = c.expect('{', 'block');
    const previous = this.origins.enter();
    const lets = [];
    let tail = null;
    while (!c.is('}')) {
      const token = c.peek();
      if (c.is('let')) {
        lets.push(...this.letStatement(path));
        continue;
      }
      if (c.is('return')) throw unsupported('early return', 'write the result as the tail expression of the block', span(token, token));
      if (['while', 'loop', 'for'].includes(token.value) && token.kind === 'identifier') {
        throw unsupported(`${token.value} loop`, 'loops over mutable state are outside the portable core; use recursion', span(token, token));
      }
      const expr = this.expr(path, { statement: true });
      if (c.eat(';')) {
        if (expr.k === 'abort') {
          tail = expr;
          continue;
        }
        throw unsupported('expression statement', 'statements with effects are outside the portable core in function bodies', expr.span);
      }
      tail = expr;
      if (!c.is('}')) {
        if (['if', 'match', 'block'].includes(expr.k)) throw unsupported('expression statement', 'a block value must be its last expression', expr.span);
        throw this.fail('expected }');
      }
    }
    const close = c.expect('}', 'block');
    if (!tail) throw unsupported('block without a value', 'the block evaluates to (), which is not a portable result', span(open, close));
    const origin = this.origins.origin(tail, path);
    const result = lets.reduceRight((body, binding) => ({ k: 'let', ...binding, body, span: binding.span }), tail);
    this.origins.scope = previous;
    return this.origins.mark(result, origin);
  }

  letStatement(path) {
    const c = this.cursor;
    const start = c.next();
    if (c.is('mut')) throw unsupported('mutable binding', 'mutation is outside the portable core', span(start, c.peek()));
    const pattern = this.productBindingPattern();
    const names = new Set();
    const validateNames = binding => {
      if (binding.k === 'name') {
        if (names.has(binding.name)) throw unsupported('duplicate tuple binding', 'each name occurs only once in a destructuring pattern', binding.span);
        names.add(binding.name);
      }
      for (const item of binding.items ?? []) validateNames(item);
    };
    validateNames(pattern);
    const type = c.eat(':') ? this.type() : undefined;
    const sourceType = type ? this.origins.type : null;
    if (!c.eat('=')) throw unsupported('uninitialised binding', 'every binding needs its value', span(start, c.peek()));
    const value = this.expr(path);
    if (c.is('else')) throw unsupported('let-else', 'refutable bindings are outside the portable core', span(start, c.peek()));
    c.expect(';', 'let');
    const range = span(start, c.peek());
    if (pattern.k === 'name') {
      this.origins.bind(pattern.name, value, path, sourceType);
      return [{ name: pattern.name, type, value, span: range }];
    }
    const bindings = [];
    const name = this.freshProductBinding();
    this.origins.bind(name, value, path, sourceType);
    bindings.push({ name, type, value, span: range });
    this.lowerProductBinding(pattern, { k: 'name', path: [name], span: range }, path, bindings);
    return bindings;
  }

  freshProductBinding() {
    let name;
    do { name = `ml_tuple_value${this.productBindingCount++}`; } while (this.reservedNames.has(name));
    this.reservedNames.add(name);
    return name;
  }

  productBindingPattern() {
    const c = this.cursor;
    const token = c.peek();
    if (c.is('mut')) throw unsupported('mutable tuple binding', 'mutation is outside the immutable product model', span(token, token));
    if (c.is('ref') || c.is('&')) throw unsupported('reference tuple binding', 'product bindings must retain direct Copy values', span(token, token));
    if (c.eat('_')) return { k: 'wild', span: { start: token.start, end: token.end } };
    if (c.eat('(')) {
      if (c.eat(')')) return { k: 'unit', span: { start: token.start, end: c.tokens[c.index - 1].end } };
      const first = this.productBindingPattern();
      if (!c.eat(',')) {
        c.expect(')', 'parenthesised binding');
        return first;
      }
      const items = [first];
      while (!c.is(')')) {
        items.push(this.productBindingPattern());
        if (!c.eat(',')) break;
      }
      const close = c.expect(')', 'tuple binding');
      return { k: 'tuple', items, span: { start: token.start, end: close.end } };
    }
    const name = c.identifier('let');
    if (name.value === 'true' || name.value === 'false') throw unsupported('refutable product binding', 'only immutable names, ignored fields and anonymous tuple products are admitted', { start: name.start, end: name.end });
    if (c.is('::') || c.is('(') || c.is('{') || c.is('@')) throw unsupported('refutable product binding', 'only immutable names, ignored fields and anonymous tuple products are admitted', span(token, c.peek()));
    return { k: 'name', name: name.value, span: { start: name.start, end: name.end } };
  }

  lowerProductBinding(pattern, value, path, bindings) {
    if (pattern.k === 'wild') return;
    if (pattern.k === 'name') {
      this.origins.bind(pattern.name, value, path);
      bindings.push({ name: pattern.name, value, span: pattern.span });
      return;
    }
    const origin = this.origins.origin(value, path);
    if (pattern.k === 'unit') {
      if (!this.origins.collecting && (origin.type?.kind !== 'unit' || !this.origins.immutableProduct(origin))) {
        throw unsupported('unit product binding', 'the unit pattern requires a direct unit source value', pattern.span);
      }
      return;
    }
    this.origins.requireProduct(origin, pattern.span);
    if (!this.origins.collecting && pattern.items.length !== origin.fields.length) {
      throw new TranslationError('type', 'tuple binding has the wrong number of fields', pattern.span);
    }
    for (const [index, item] of pattern.items.entries()) {
      const projection = this.origins.project({ k: 'tupleIndex', object: value, index, span: item.span }, path, index, item.span);
      this.lowerProductBinding(item, projection, path, bindings);
    }
  }

  main() {
    const c = this.cursor;
    const start = c.next();
    c.expect('main', 'fn main');
    c.expect('(', 'fn main');
    c.expect(')', 'fn main');
    if (c.is('->')) throw unsupported('main with a result', 'main must return ()', span(start, c.peek()));
    this.modulePath = [];
    const previous = this.origins.enter(new Map());
    c.expect('{', 'fn main');
    const effects = [];
    while (!c.is('}')) {
      const token = c.peek();
      if (c.is('let')) {
        for (const binding of this.letStatement([])) effects.push({ k: 'let', name: binding.name, type: binding.type, value: binding.value, span: binding.span });
        continue;
      }
      if (token.kind === 'macro') {
        effects.push(this.mainMacro());
        c.expect(';', token.value);
        continue;
      }
      throw unsupported('main statement', 'main may only print, bind and assert', span(token, token));
    }
    c.expect('}', 'fn main');
    this.origins.scope = previous;
    return { effects, span: span(start, c.peek()) };
  }

  mainMacro() {
    const c = this.cursor;
    const token = c.next();
    const args = this.macroArguments(token, []);
    switch (token.value) {
      case 'println':
        if (!args.length) return { k: 'print', expr: { k: 'str', value: '' }, style: 'rust', span: span(token, c.peek()) };
        return { k: 'print', expr: this.format(args, token), style: 'rust', span: span(token, c.peek()) };
      case 'assert':
        if (args.length !== 1) throw unsupported('assert! message', 'custom assertion messages are not kept', span(token, c.peek()));
        return { k: 'assert', prop: propOf(args[0]), span: span(token, c.peek()) };
      case 'assert_eq':
      case 'assert_ne':
        if (args.length !== 2) throw unsupported(`${token.value}! message`, 'custom assertion messages are not kept', span(token, c.peek()));
        return { k: 'assert', prop: { p: token.value === 'assert_eq' ? 'eq' : 'ne', left: args[0], right: args[1] }, span: span(token, c.peek()) };
      default:
        throw unsupported(`${token.value}!`, 'main may only use println!, assert!, assert_eq! and assert_ne!', span(token, token));
    }
  }

  /** Macro arguments, parsed as expressions; the first one of a format macro is its literal. */
  macroArguments(token, path) {
    const c = this.cursor;
    const close = { '(': ')', '[': ']', '{': '}' }[c.peek().value];
    if (!close) throw this.fail(`expected ( after ${token.value}!`);
    c.next();
    const args = [];
    while (!c.is(close)) {
      args.push(this.expr(path));
      if (!c.eat(',')) break;
    }
    c.expect(close, `${token.value}!`);
    return args;
  }

  /**
   * `format!("{} and {name}", a)`: `{}` takes the next argument, `{name}` a
   * variable in scope, `{{`/`}}` are braces; every value is rendered with
   * Display, which for integers is its decimal text.
   */
  format(args, token) {
    const [template, ...values] = args;
    if (template?.k !== 'str') throw unsupported(`${token.value}!`, 'the format string must be a literal', span(token, token));
    const pieces = [];
    let text = '';
    let next = 0;
    const source = template.value;
    for (let index = 0; index < source.length; index += 1) {
      const char = source[index];
      if (char === '{' && source[index + 1] === '{') {
        text += '{';
        index += 1;
        continue;
      }
      if (char === '}' && source[index + 1] === '}') {
        text += '}';
        index += 1;
        continue;
      }
      if (char === '}') throw new TranslationError('syntax', 'unmatched } in format string', template.span);
      if (char !== '{') {
        text += char;
        continue;
      }
      const end = source.indexOf('}', index);
      if (end < 0) throw new TranslationError('syntax', 'unclosed { in format string', template.span);
      const spec = source.slice(index + 1, end);
      if (spec.includes(':')) throw unsupported(`format spec {${spec}}`, 'only plain {} Display formatting is portable', template.span);
      let value;
      if (spec === '') {
        value = values[next];
        next += 1;
        if (!value) throw new TranslationError('syntax', 'format string has more {} than arguments', template.span);
      } else if (/^\d+$/u.test(spec)) {
        value = values[Number(spec)];
        if (!value) throw new TranslationError('syntax', `format argument ${spec} is missing`, template.span);
        next = Math.max(next, Number(spec) + 1);
      } else if (/^[A-Za-z_][A-Za-z0-9_]*$/u.test(spec)) {
        value = { k: 'name', path: [spec], span: template.span };
      } else {
        throw unsupported(`format argument {${spec}}`, 'outside the portable core', template.span);
      }
      if (text) pieces.push({ k: 'str', value: text, span: template.span });
      text = '';
      pieces.push({ k: 'show', arg: value, style: 'rust', span: value.span });
      index = end;
    }
    if (next < values.length) throw new TranslationError('syntax', 'format string has fewer {} than arguments', template.span);
    if (text || !pieces.length) pieces.push({ k: 'str', value: text, span: template.span });
    return pieces.reduce((left, right) => ({ k: 'binary', op: 'concat', left, right, span: template.span }));
  }

  /** Expressions, by Rust precedence. */
  expr(path, options = {}) {
    return this.or(path, options);
  }

  or(path, options) {
    let left = this.and(path, options);
    while (this.cursor.is('||')) {
      const token = this.cursor.next();
      const right = this.and(path, {});
      left = { k: 'binary', op: 'or', left, right, span: joined(left, right, token) };
    }
    return left;
  }

  and(path, options) {
    let left = this.comparison(path, options);
    while (this.cursor.is('&&')) {
      const token = this.cursor.next();
      const right = this.comparison(path, {});
      left = { k: 'binary', op: 'and', left, right, span: joined(left, right, token) };
    }
    return left;
  }

  comparison(path, options) {
    const c = this.cursor;
    const left = this.additive(path, options);
    const token = c.peek();
    const shift = (token.value === '<' || token.value === '>') && c.is(token.value, 1) && c.peek(1).start === token.end;
    const blockStatement = options.statement && ['if', 'match'].includes(left.k);
    if (token.kind === 'punct' && (shift || ['|', '^', '&'].includes(token.value)) && !blockStatement) {
      throw unsupported(`operator ${shift ? token.value.repeat(2) : token.value}`, 'bitwise operators are outside the portable core', span(token, c.peek(shift ? 1 : 0)));
    }
    if (token.kind === 'punct' && COMPARISON[token.value]) {
      c.next();
      const right = this.additive(path, {});
      const after = c.peek();
      if (after.kind === 'punct' && COMPARISON[after.value]) throw this.fail('comparison operators cannot be chained', after);
      return { k: 'binary', op: COMPARISON[token.value], left, right, span: joined(left, right, token) };
    }
    return left;
  }

  additive(path, options) {
    let left = this.multiplicative(path, options);
    for (;;) {
      const token = this.cursor.peek();
      if (token.kind !== 'punct' || !ADDITIVE[token.value]) return left;
      if (options.statement && ['if', 'match', 'block'].includes(left.k)) return left;
      this.cursor.next();
      const right = this.multiplicative(path, {});
      left = { k: 'binary', op: ADDITIVE[token.value], left, right, span: joined(left, right, token) };
    }
  }

  multiplicative(path, options) {
    let left = this.cast(path, options);
    for (;;) {
      const token = this.cursor.peek();
      if (token.kind !== 'punct' || !MULTIPLICATIVE[token.value]) return left;
      if (options.statement && ['if', 'match', 'block'].includes(left.k)) return left;
      this.cursor.next();
      const right = this.cast(path, {});
      left = { k: 'binary', op: MULTIPLICATIVE[token.value], left, right, span: joined(left, right, token) };
    }
  }

  /** `e as T` converts between integer types when every value fits. */
  cast(path, options) {
    let value = this.unary(path, options);
    while (this.cursor.is('as')) {
      const token = this.cursor.next();
      const to = this.type();
      if (!this.origins.collecting && this.origins.type.wrappers.length) throw unsupported('source cast', 'numeric casts require a direct source and target value type', span(token, this.cursor.peek()));
      value = { k: 'cast', arg: value, to, flavor: 'exact', span: joined(value, { span: span(token, this.cursor.peek()) }, token) };
    }
    return value;
  }

  unary(path, options) {
    const c = this.cursor;
    const token = c.peek();
    if (c.eat('-')) {
      const arg = this.unary(path, {});
      return { k: 'unary', op: 'neg', arg, span: span(token, c.peek()) };
    }
    if (c.eat('!')) {
      const arg = this.unary(path, {});
      return { k: 'unary', op: 'not', arg, span: span(token, c.peek()) };
    }
    // Source evidence proves immutable indirection before projecting its value.
    if (c.is('&') || c.is('&&')) {
      c.next();
      if (c.is('mut')) throw unsupported('mutable reference', 'mutation is outside the portable core', span(token, c.peek()));
      const arg = this.unary(path, {});
      if (token.value === '&&') this.origins.wrap(arg, path, 'reference');
      return this.origins.wrap(arg, path, 'reference');
    }
    if (c.eat('*')) return this.origins.dereference(this.unary(path, {}), path, span(token, c.peek()));
    return this.postfix(path, options);
  }

  postfix(path, options) {
    const c = this.cursor;
    let value = this.primary(path, options);
    for (;;) {
      const token = c.peek();
      if (c.is('.')) {
        c.next();
        if (c.isKind('number')) {
          const number = this.tupleIndexToken();
          const index = Number(number.raw);
          if (!Number.isSafeInteger(index)) throw unsupported('tuple index', 'the positional index exceeds the exact supported integer range', { start: number.start, end: number.end });
          value = this.origins.project({ k: 'tupleIndex', object: value, index, span: { start: value.span?.start ?? token.start, end: number.end } }, path, index);
          continue;
        }
        const method = c.identifier('method');
        if (!c.is('(')) throw unsupported('field access', `field ${method.value} is outside the portable core`, span(token, method));
        c.next();
        const args = [];
        while (!c.is(')')) {
          args.push(this.expr(path));
          if (!c.eat(',')) break;
        }
        c.expect(')', method.value);
        value = this.method(value, method, args, span(token, c.peek()));
        continue;
      }
      if (c.is('?')) throw unsupported('? operator', 'Result and Option are outside the portable core', span(token, token));
      if (c.is('[')) throw unsupported('indexing', 'collections are outside the portable core', span(token, token));
      return value;
    }
  }

  tupleIndexToken() {
    const c = this.cursor;
    const token = c.peek();
    const parts = token.raw.split('.');
    if (parts.length === 2 && parts.every(acceptRustTupleIndexSpelling)) {
      const middle = token.start + parts[0].length;
      c.tokens.splice(c.index, 1,
        { kind: 'number', value: parts[0], suffix: '', raw: parts[0], start: token.start, end: middle },
        { kind: 'punct', value: '.', raw: '.', start: middle, end: middle + 1 },
        { kind: 'number', value: parts[1], suffix: '', raw: parts[1], start: middle + 1, end: token.end });
    }
    const number = c.next();
    if (!acceptRustTupleIndexSpelling(number.raw)) {
      throw unsupported('tuple index spelling', 'Rust tuple indices are decimal digits without leading zeros, separators or suffixes', { start: number.start, end: number.end });
    }
    return number;
  }

  method(receiver, method, args, range) {
    const expect = (count) => {
      if (args.length !== count) throw new TranslationError('type', `${method.value} expects ${count} arguments but got ${args.length}`, range);
    };
    const mapOperation = readStringMapOperation('Rust', method.value);
    if (mapOperation) {
      expect(0);
      return { k: 'stringMap', op: mapOperation, object: receiver, span: range };
    }
    switch (method.value) {
      case 'starts_with':
      case 'ends_with':
      case 'contains':
        expect(1);
        return { k: 'stringTest', op: readStringTestOperation('Rust', method.value), object: receiver, search: args[0], span: range };
      case 'to_lowercase':
      case 'to_uppercase':
        expect(0);
        return { k: 'stringMap', op: readStringMapOperation('Rust', method.value), object: receiver, span: range };
      case 'clone':
      case 'to_owned': {
        expect(0);
        const origin = this.origins.origin(receiver, this.modulePath);
        if (origin.wrappers[0] === 'reference') origin.wrappers.shift();
        if (method.value === 'to_owned' && origin.type?.kind === 'string') origin.textSourceModel = 'owned string';
        return this.origins.mark(receiver, origin);
      }
      case 'as_str':
        expect(0);
        return this.origins.mark(receiver, { ...this.origins.origin(receiver, this.modulePath), type: STRING, textSourceModel: 'string slice', path: [...this.modulePath], wrappers: ['reference'] });
      case 'to_string':
        expect(0);
        return { k: 'toString', arg: receiver, span: range };
      case 'div_euclid':
      case 'rem_euclid':
        expect(1);
        this.origins.direct(args[0], this.modulePath, args[0].span);
        return this.origins.mark({ k: 'binary', op: method.value === 'div_euclid' ? 'div' : 'rem', rounding: 'euclid', left: receiver, right: args[0], span: range }, { ...this.origins.origin(receiver, this.modulePath), wrappers: [] });
      default:
        throw unsupported(`method ${method.value}`, 'outside the portable core', range);
    }
  }

  primary(path, options) {
    const c = this.cursor;
    const token = c.peek();
    if (token.kind === 'number') {
      c.next();
      const node = { k: 'num', value: token.value, span: span(token, token) };
      if (token.suffix) {
        const type = token.suffix === 'f64' ? FLOAT : rustFixedType(token.suffix);
        if (!type) throw unsupported(`literal suffix ${token.suffix}`, 'portable numeric literals are integers and IEEE-754 binary64 values', span(token, token));
        node.type = type;
      } else if (token.floating) node.type = FLOAT;
      return node;
    }
    if (token.kind === 'string') {
      c.next();
      return this.origins.mark({ k: 'str', value: token.value, span: span(token, token) }, { type: STRING, textSourceModel: 'string slice', path: [...path], wrappers: ['reference'] });
    }
    if (token.kind === 'macro') {
      c.next();
      return this.expressionMacro(token, path);
    }
    if (c.is('\'')) throw unsupported('character literal', 'characters are outside the portable core', span(token, token));
    if (c.is('(')) {
      c.next();
      if (c.eat(')')) return { k: 'unit', span: span(token, token) };
      const inner = this.expr(path);
      if (!c.eat(',')) {
        c.expect(')', 'parenthesised expression');
        return inner;
      }
      const items = [inner];
      while (!c.is(')')) {
        items.push(this.expr(path));
        if (!c.eat(',')) break;
      }
      const close = c.expect(')', 'tuple expression');
      return this.origins.product({ k: 'tuple', items, frozen: true, productModel: 'immutable value product', span: { start: token.start, end: close.end } }, path);
    }
    if (c.is('{')) {
      const block = this.block(path);
      return this.origins.mark({ ...block, block: true }, this.origins.origin(block, path));
    }
    if (c.is('if')) return this.ifExpr(path);
    if (c.is('match')) return this.match(path);
    if (c.is('|') || c.is('||') || c.is('move')) throw unsupported('closure', 'higher-order values are outside the portable core', span(token, token));
    if (c.is('true') || c.is('false')) {
      c.next();
      return { k: 'bool', value: token.value === 'true', span: span(token, token) };
    }
    if (c.is('unsafe') || c.is('loop') || c.is('while') || c.is('for') || c.is('return') || c.is('break')) {
      throw unsupported(`${token.value} expression`, 'outside the portable core', span(token, token));
    }
    if (token.kind === 'identifier') {
      const segments = this.pathSegments('expression');
      const name = segments.join('::');
      if (c.is('::') && c.is('<', 1)) throw unsupported('turbofish', 'generic functions are outside the portable core', span(token, c.peek()));
      if (c.is('{') && /^[A-Z]/u.test(segments.at(-1)) && !options.noStruct) {
        throw unsupported('struct literal', 'named fields are outside the Rust frontend', span(token, c.peek()));
      }
      if (!c.is('(')) return { k: 'name', path: this.resolve(segments, path), span: span(token, token) };
      c.next();
      const args = [];
      while (!c.is(')')) {
        args.push(this.expr(path));
        if (!c.eat(',')) break;
      }
      c.expect(')', 'call');
      const range = span(token, c.peek());
      const resolvedOwner = segments.length > 1 ? this.resolve(segments.slice(0, -1), path) : [];
      if (segments.at(-1) === 'new' && (acceptRustSourceBoxOwner(segments.slice(0, -1).join('::'), false) || acceptRustSourceBoxOwner(resolvedOwner.join('::'), false))) {
        if (args.length !== 1) throw new TranslationError('type', 'Box::new expects 1 argument', range);
        this.origins.box(segments.slice(0, -1), resolvedOwner, path, range);
        return this.origins.wrap(args[0], path, 'box');
      }
      if (name === 'String::from') {
        if (args.length !== 1) throw new TranslationError('type', 'String::from expects 1 argument', range);
        return this.origins.conversion({ k: 'toString', arg: args[0], span: range }, path);
      }
      if (segments.length === 2 && segments[1] === 'from' && rustFixedType(segments[0])) {
        if (args.length !== 1) throw new TranslationError('type', `${name} expects 1 argument`, range);
        return { k: 'cast', arg: args[0], to: rustFixedType(segments[0]), flavor: 'exact', span: range };
      }
      if (segments.length === 2 && rustFixedType(segments[0])) {
        throw unsupported(`${name}`, 'outside the portable core', range);
      }
      if (['std', 'core', 'alloc'].includes(segments[0])) throw unsupported(name, 'the standard library is outside the portable core', range);
      this.origins.call(this.resolve(segments, path), args, path, range);
      if (!args.length) return { k: 'name', path: this.resolve(segments, path), span: range };
      return { k: 'app', fn: { k: 'name', path: this.resolve(segments, path), span: span(token, token) }, args, span: range };
    }
    throw this.fail('expected an expression');
  }

  expressionMacro(token, path) {
    const args = this.macroArguments(token, path);
    const range = span(token, this.cursor.peek());
    switch (token.value) {
      case 'format':
        return { ...this.format(args, token), span: range };
      case 'panic':
      case 'unreachable':
      case 'unimplemented':
      case 'todo': {
        if (args.length > 1) throw unsupported(`${token.value}! arguments`, 'only a literal message is portable', range);
        if (args.length && args[0].k !== 'str') throw unsupported(`${token.value}! message`, 'the message must be a literal', range);
        return { k: 'abort', message: rustMacroMessage(token.value, args.length ? panicText(args[0]) : undefined), span: range };
      }
      default:
        throw unsupported(`${token.value}!`, 'outside the portable core', range);
    }
  }

  ifExpr(path) {
    const c = this.cursor;
    const token = c.next();
    if (c.is('let')) throw unsupported('if let', 'write a match', span(token, c.peek()));
    const cond = this.expr(path, { noStruct: true });
    this.origins.direct(cond, path, cond.span);
    const then = this.block(path);
    if (!c.eat('else')) throw unsupported('if without else', 'the value of an if must be defined on both branches', span(token, c.peek()));
    const otherwise = c.is('if') ? this.ifExpr(path) : this.block(path);
    return { k: 'if', cond, then, else: otherwise, span: span(token, c.peek()) };
  }

  match(path) {
    const c = this.cursor;
    const token = c.next();
    const scrutinee = this.expr(path, { noStruct: true });
    c.expect('{', 'match');
    const rows = [];
    while (!c.is('}')) {
      const armStart = c.peek();
      const aliases = [];
      c.eat('|');
      const alternatives = [this.pattern(path, aliases)];
      while (c.eat('|')) alternatives.push(this.pattern(path, aliases));
      if (c.is('if')) throw unsupported('match guard', 'guards are outside the portable pattern language', span(armStart, c.peek()));
      c.expect('=>', 'match arm');
      const previous = this.origins.enter(this.origins.cases(alternatives, scrutinee, path));
      for (const alias of aliases) this.origins.bind(alias.name, alias.value, path);
      let body = this.expr(path);
      const bodyOrigin = this.origins.origin(body, path);
      this.origins.scope = previous;
      if (body.block || body.k === 'if' || body.k === 'match') c.eat(',');
      else if (!c.is('}')) c.expect(',', 'match arm');
      if (alternatives.length > 1 && aliases.length) {
        throw unsupported('binding in an or-pattern', 'bind the value in each alternative separately', span(armStart, c.peek()));
      }
      for (const alias of aliases.reverse()) body = { k: 'let', name: alias.name, value: alias.value, body, span: body.span };
      this.origins.mark(body, bodyOrigin);
      // `A | B => e` is one row per alternative with the same body.
      for (const pattern of alternatives) rows.push({ patterns: [pattern], body, span: span(armStart, c.peek()) });
    }
    c.expect('}', 'match');
    if (!rows.length) throw unsupported('empty match', 'uninhabited types are outside the portable core', span(token, c.peek()));
    const origins = rows.map(row => this.origins.origin(row.body, path));
    const result = { k: 'match', scrutinees: [scrutinee], rows, span: span(token, c.peek()) };
    return this.origins.mark(result, this.origins.join(origins));
  }

  /** Patterns: `_`, bindings, literals, `Enum::Variant(p, …)`, `&p` and `name @ p`. */
  pattern(path, aliases) {
    const c = this.cursor;
    const token = c.peek();
    if (c.eat('&')) return this.origins.referencePattern(this.pattern(path, aliases));
    if (c.eat('_')) return { k: 'wild' };
    if (c.is('ref') || c.is('mut')) throw unsupported(`${token.value} binding`, 'bindings are by value in the portable core', span(token, token));
    if (c.is('-') && c.isKind('number', 1)) {
      c.next();
      const number = c.next();
      return { k: 'numLit', value: number.value, negative: true, span: span(token, number) };
    }
    if (token.kind === 'number') {
      c.next();
      if (c.is('..') || c.is('..=')) throw unsupported('range pattern', 'ranges are outside the portable pattern language', span(token, c.peek()));
      return { k: 'numLit', value: token.value, span: span(token, token) };
    }
    if (token.kind === 'string') throw unsupported('string pattern', 'match on &str is outside the Rust frontend', span(token, token));
    if (c.is('true') || c.is('false')) {
      c.next();
      return { k: 'boolLit', value: token.value === 'true', span: span(token, token) };
    }
    if (c.is('(')) throw unsupported('tuple match pattern', 'tuple match coverage is not represented; use an immutable destructuring let', span(token, token));
    if (token.kind !== 'identifier') throw unsupported('pattern', `${describe(token)} is outside the portable pattern language`, span(token, token));
    const segments = this.pathSegments('pattern');
    if (c.eat('@')) {
      if (segments.length !== 1) throw this.fail('expected a binding before @', token);
      const inner = this.pattern(path, aliases);
      aliases.push({ name: segments[0], value: patternValue(inner, span(token, c.peek())) });
      return inner;
    }
    if (c.is('{')) throw unsupported('struct pattern', 'named fields are outside the Rust frontend', span(token, c.peek()));
    const resolved = this.resolve(segments, path);
    if (c.eat('(')) {
      const args = [];
      while (!c.is(')')) {
        if (c.is('..')) throw unsupported('rest pattern', 'name every field', span(c.peek(), c.peek()));
        args.push(this.pattern(path, aliases));
        if (!c.eat(',')) break;
      }
      c.expect(')', 'pattern');
      return { k: 'ctor', path: resolved, args, span: span(token, c.peek()) };
    }
    if (segments.length === 1 && resolved.length === 1) return { k: 'bindOrCtor', name: segments[0], span: span(token, token) };
    return { k: 'ctor', path: resolved, args: [], span: span(token, token) };
  }
}

/** An imported path from the crate root, so it means the same wherever it is used. */
function absolute(segments, path, range) {
  if (segments[0] === 'crate') return segments;
  if (segments[0] === 'self') return ['crate', ...path, ...segments.slice(1)];
  let up = 0;
  while (segments[up] === 'super') up += 1;
  if (up > path.length) throw new TranslationError('type', 'super beyond crate root', range);
  return ['crate', ...path.slice(0, path.length - up), ...segments.slice(up)];
}

function propOf(expr) {
  if (expr.k === 'binary' && ['eq', 'ne', 'lt', 'le', 'gt', 'ge'].includes(expr.op)) {
    return { p: expr.op, left: expr.left, right: expr.right };
  }
  if (expr.k === 'binary' && (expr.op === 'and' || expr.op === 'or')) return { p: expr.op, left: propOf(expr.left), right: propOf(expr.right) };
  if (expr.k === 'unary' && expr.op === 'not') return { p: 'not', arg: propOf(expr.arg) };
  return { p: 'bool', expr };
}

/** The value a pattern matched, for `name @ p`. */
function patternValue(pattern, range) {
  switch (pattern.k) {
    case 'bindOrCtor':
      return { k: 'name', path: [pattern.name], span: range };
    case 'numLit':
      return { k: 'num', value: pattern.value, negative: pattern.negative, span: range };
    case 'boolLit':
      return { k: 'bool', value: pattern.value, span: range };
    case 'ctor':
      if (!pattern.args.length) return { k: 'name', path: pattern.path, span: range };
      return { k: 'app', fn: { k: 'name', path: pattern.path, span: range }, args: pattern.args.map((arg) => patternValue(arg, range)), span: range };
    default:
      throw unsupported('@ binding', 'the aliased pattern must bind every field', range);
  }
}

function joined(left, right, token) {
  return { start: left.span?.start ?? token.start, end: right.span?.end ?? token.end };
}

function span(from, to) {
  return { start: from.start, end: Math.max(from.end, to && to.kind !== 'eof' ? to.start : from.end) };
}

/** The text a `panic!` message literal prints: `{{` and `}}` are braces; a lone brace captures a variable. */
function panicText(template) {
  let text = '';
  for (let index = 0; index < template.value.length; index += 1) {
    const char = template.value[index];
    if (char === '{' || char === '}') {
      if (template.value[index + 1] !== char) throw unsupported('formatted panic message', 'only a literal message is portable', template.span);
      index += 1;
    }
    text += char;
  }
  return text;
}
