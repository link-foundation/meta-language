// Source indirection evidence for the Rust value projection. The existing
// parser collects declarations before validating bodies, retaining its own
// type, path and pattern records rather than inferring a second typed program.
import { unsupported } from './diagnostics.js';
import { acceptRustImmutableIndirectionLayer, acceptRustSourceIndirectionArgument, acceptRustSourceBoxOwner } from './rust-indirection-rules.js';

const unknown = () => ({ wrappers: [], type: null, path: [] });
const clone = origin => origin && ({ ...origin, wrappers: [...origin.wrappers], path: [...origin.path] });

export class RustSourceOrigins {
  constructor(catalog = null) {
    this.collecting = catalog === null;
    this.catalog = catalog ?? new Map();
    this.scope = new Map();
    this.expressions = new WeakMap();
    this.patternReferences = new WeakMap();
    this.type = unknown();
  }

  enter(scope = this.scope) {
    const previous = this.scope;
    this.scope = new Map(scope);
    return previous;
  }

  typed(type, path, wrappers = []) {
    this.type = { type, path: [...path], wrappers };
    return type;
  }

  declaration(name, path, entry) {
    if (this.collecting) this.catalog.set([...path, name].join('.'), entry);
  }

  resolveTypes(resolve) {
    for (const entry of this.catalog.values()) {
      const origins = entry.k === 'fn' ? [...entry.parameters, entry.result]
        : entry.k === 'ctor' ? [...entry.fields, entry.result] : [];
      for (const origin of origins) {
        if (origin.type?.kind === 'named') origin.type = { ...origin.type, path: resolve(origin.type.path, origin.path) };
      }
    }
  }

  lookup(segments, path) {
    let names = [...segments];
    let start = [...path];
    let absolute = false;
    if (names[0] === 'crate') { names.shift(); start = []; absolute = true; }
    else if (names[0] === 'self') { names.shift(); absolute = true; }
    else if (names[0] === 'super') {
      while (names[0] === 'super') { names.shift(); start.pop(); }
      absolute = true;
    }
    for (let depth = start.length; depth >= 0; depth -= 1) {
      const fullName = [...start.slice(0, depth), ...names].join('.');
      const found = this.catalog.get(fullName);
      if (found) return { ...found, fullName };
      if (absolute) break;
    }
    return null;
  }

  same(left, right) {
    if (!left || !right || JSON.stringify(left.wrappers) !== JSON.stringify(right.wrappers)) return false;
    if (left.type?.kind === 'named' && right.type?.kind === 'named') {
      const leftOwner = this.lookup(left.type.path, left.path);
      const rightOwner = this.lookup(right.type.path, right.path);
      if (leftOwner && rightOwner) return leftOwner.fullName === rightOwner.fullName;
      return JSON.stringify(left.type.path) === JSON.stringify(right.type.path) && JSON.stringify(left.path) === JSON.stringify(right.path);
    }
    return JSON.stringify(left.type) === JSON.stringify(right.type);
  }

  box(segments, resolved, path, span) {
    if (this.collecting) return;
    const owner = segments.join('::');
    const shadowed = JSON.stringify(segments) !== JSON.stringify(resolved)
      || Boolean(this.lookup([segments[0]], path));
    if (!acceptRustSourceBoxOwner(owner, shadowed)) {
      throw unsupported('Box owner', 'only the unshadowed standard immutable Box is portable', span);
    }
  }

  origin(node, path) {
    if (!node) return unknown();
    if (this.expressions.has(node)) return clone(this.expressions.get(node));
    if (node.k === 'name' && node.path.length === 1 && this.scope.has(node.path[0])) {
      return clone(this.scope.get(node.path[0]));
    }
    if (node.k === 'name' || (node.k === 'app' && node.fn.k === 'name')) {
      const entry = this.lookup(node.k === 'name' ? node.path : node.fn.path, path);
      if (entry?.k === 'fn') return clone(entry.result);
      if (entry?.k === 'ctor') return clone(entry.result);
    }
    if (node.k === 'if') {
      const left = this.origin(node.then, path); const right = this.origin(node.else, path);
      return this.same(left, right) ? left : unknown();
    }
    return unknown();
  }

  mark(node, origin) {
    this.expressions.set(node, clone(origin));
    return node;
  }

  wrap(node, path, layer) {
    const origin = this.origin(node, path);
    origin.wrappers.unshift(layer);
    return this.mark(node, origin);
  }

  dereference(node, path, span) {
    const origin = this.origin(node, path);
    if (!this.collecting && !acceptRustImmutableIndirectionLayer(origin.wrappers[0] ?? '')) {
      throw unsupported('dereference', 'the operand must have a proven immutable reference or standard Box source type', span);
    }
    origin.wrappers.shift();
    return this.mark(node, origin);
  }

  bind(name, node, path, declared = null) {
    const actual = this.origin(node, path);
    if (declared) this.argument(actual, declared, node.span);
    this.scope.set(name, clone(declared ?? actual));
  }

  argument(actualOrigin, expectedOrigin, span) {
    if (this.collecting) return;
    const actual = actualOrigin.wrappers;
    const want = expectedOrigin.wrappers;
    const indirected = actual.length > 0 || want.length > 0;
    const exact = JSON.stringify(actual) === JSON.stringify(want);
    const borrowed = actual[0] === 'reference' && want[0] === 'reference';
    const coerced = JSON.stringify(actual.filter(layer => layer !== 'box')) === JSON.stringify(want);
    if (!acceptRustSourceIndirectionArgument(indirected, exact, borrowed, coerced)) {
      throw unsupported('source indirection', 'source ownership requires an explicit dereference or matching immutable reference or Box type', span);
    }
  }

  // Keep Box ownership contracts until source admission has checked a call;
  // ordinary value type unification remains exclusively in the checker.
  call(segments, args, path, span) {
    if (this.collecting) return;
    const entry = this.lookup(segments, path);
    const expected = entry?.k === 'fn' ? entry.parameters : entry?.k === 'ctor' ? entry.fields : null;
    if (!expected || args.length !== expected.length) return;
    for (let index = 0; index < args.length; index += 1) {
      // Immutable reference coercions may dereference Box behind a reference;
      // an owned Box never coerces into its owned inner value.
      this.argument(this.origin(args[index], path), expected[index], span);
    }
  }

  referencePattern(pattern) {
    this.patternReferences.set(pattern, (this.patternReferences.get(pattern) ?? 0) + 1);
    return pattern;
  }

  pattern(pattern, origin, path, result = new Map()) {
    origin = clone(origin);
    for (let count = this.patternReferences.get(pattern) ?? 0; count > 0; count -= 1) {
      if (origin.wrappers[0] !== 'reference') origin = unknown();
      else origin.wrappers.shift();
    }
    if (pattern.k === 'bindOrCtor' || pattern.k === 'bind') {
      if (!this.lookup([pattern.name], path)?.fields) result.set(pattern.name, origin);
    } else if (pattern.k === 'ctor') {
      const entry = this.lookup(pattern.path, path);
      const typeEntry = origin.type?.kind === 'named' ? this.lookup(origin.type.path, origin.path) : null;
      const borrowed = origin.wrappers.length > 0 && origin.wrappers.every(layer => layer === 'reference');
      const admissible = entry?.k === 'ctor' && typeEntry?.k === 'data' && entry.owner === typeEntry.fullName
        && (origin.wrappers.length === 0 || borrowed);
      for (let index = 0; index < pattern.args.length; index += 1) {
        let field = admissible && entry.fields[index] ? clone(entry.fields[index]) : unknown();
        if (borrowed && admissible) field.wrappers.unshift('reference');
        this.pattern(pattern.args[index], field, path, result);
      }
    }
    return result;
  }

  cases(patterns, scrutinee, path) {
    const maps = patterns.map(pattern => this.pattern(pattern, this.origin(scrutinee, path), path));
    const scope = new Map(this.scope);
    for (const [name, origin] of maps[0]) {
      scope.set(name, maps.every(map => this.same(origin, map.get(name))) ? origin : unknown());
    }
    return scope;
  }
}
