// Source indirection evidence for the Rust value projection. The existing
// parser collects declarations before validating bodies, retaining its own
// type, path and pattern records rather than inferring a second typed program.
import { unsupported } from './diagnostics.js';
import { rustFixedType } from './types.js';
import { acceptRustImmutableIndirectionLayer, acceptRustSourceIndirectionArgument, acceptRustSourceBoxCoercionLayer, acceptRustSourceBoxOwner, acceptRustSourceBoxAvailability, readRustSourceExpressionOrigin } from './rust-indirection-rules.js';
import { acceptRustImmutableProductType } from './rust-product-rules.js';

const unknown = (uncertain = false) => ({ wrappers: [], type: null, path: [], fields: null, copyEvidence: false, proven: false, diverging: false, uncertain });
const value = (copyEvidence = false, type = null) => ({ ...unknown(), proven: true, copyEvidence, type });
const clone = origin => origin && ({ proven: true, diverging: false, uncertain: false, copyEvidence: false, fields: null, ...origin, wrappers: [...origin.wrappers], path: [...origin.path], fields: origin.fields?.map(clone) ?? null });

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

  typed(type, path, wrappers = [], fields = null) {
    fields ??= type.kind === 'tuple' ? type.elements.map(element => this.typeOrigin(element, path)) : null;
    this.type = { type, path: [...path], wrappers, fields, copyEvidence: acceptRustImmutableProductType(type.kind, fields?.every(field => this.immutableProduct(field)) ?? false, false), proven: true, diverging: false, uncertain: false };
    return type;
  }

  typeOrigin(type, path) {
    const fields = type.kind === 'tuple' ? type.elements.map(element => this.typeOrigin(element, path)) : null;
    return { ...value(acceptRustImmutableProductType(type.kind, fields?.every(field => this.immutableProduct(field)) ?? false, false), type), path: [...path], fields };
  }

  immutableProduct(origin) {
    return Boolean(origin?.proven && origin.copyEvidence && origin.wrappers.length === 0
      && (!origin.fields || origin.fields.every(field => this.immutableProduct(field))));
  }

  requireProduct(origin, span) {
    if (!this.collecting && (!origin.fields || !this.immutableProduct(origin))) {
      throw unsupported('immutable Rust tuple product', 'every field requires a direct recursively immutable scalar Copy source value', span);
    }
  }

  product(node, path, span = node.span) {
    const fields = node.items.map(item => this.origin(item, path));
    const origin = { ...value(fields.every(field => this.immutableProduct(field))), fields };
    this.requireProduct(origin, span);
    return this.mark(node, origin);
  }

  project(node, path, index, span = node.span) {
    const origin = this.origin(node.object, path);
    this.requireProduct(origin, span);
    return this.mark(node, origin.fields?.[index] ?? unknown());
  }

  declaration(name, path, entry) {
    if (this.collecting) {
      const key = [...path, name].join('.');
      const existing = this.catalog.get(key);
      if (existing && existing.k !== entry.k && (existing.k === 'externalCrate' || entry.k === 'externalCrate')) {
        throw unsupported('extern crate alloc', 'a source crate declaration cannot share its module name with another item');
      }
      this.catalog.set(key, entry);
    }
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
    if (!left || !right || left.proven !== right.proven || left.diverging !== right.diverging || left.uncertain !== right.uncertain || left.copyEvidence !== right.copyEvidence || JSON.stringify(left.wrappers) !== JSON.stringify(right.wrappers)) return false;
    if (Boolean(left.fields) !== Boolean(right.fields) || (left.fields && (left.fields.length !== right.fields.length || left.fields.some((field, index) => !this.same(field, right.fields[index]))))) return false;
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
    const owner = resolved.join('::');
    const source = this.lookup([segments[0]], path);
    const target = this.lookup([resolved[0]], path);
    const shadowed = Boolean(source && source.k !== 'externalCrate') || Boolean(target && target.k !== 'externalCrate');
    if (!acceptRustSourceBoxOwner(owner, shadowed)) {
      throw unsupported('Box owner', 'only the unshadowed standard immutable Box is portable', span);
    }
    if (!acceptRustSourceBoxAvailability(owner, target?.k === 'externalCrate')) {
      throw unsupported('Box source crate', 'alloc requires an actual extern crate alloc declaration', span);
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
      if (node.k === 'name' && node.path.length === 2 && !this.lookup([node.path[0]], path)
        && rustFixedType(node.path[0]) && ['MIN', 'MAX'].includes(node.path[1])) return value(true, rustFixedType(node.path[0]));
    }
    if (node.k === 'if') {
      return this.join([this.origin(node.then, path), this.origin(node.else, path)]);
    }
    const form = readRustSourceExpressionOrigin(node.k, node.op ?? '');
    if (form === 'value') return value(['num', 'bool', 'unit'].includes(node.k), node.type ?? (node.k === 'bool' ? { kind: 'bool' } : node.k === 'unit' ? { kind: 'unit' } : null));
    if (form === 'diverging') return { ...value(), diverging: true };
    if (form === 'derived-value') {
      const operands = [node.arg ?? node.object, ...(node.search ? [node.search] : [])].map(operand => this.origin(operand, path));
      if (operands.some(operand => !operand.proven)) return unknown(operands.some(operand => operand.uncertain));
      return value(node.k === 'stringTest', node.k === 'stringTest' ? { kind: 'bool' } : null);
    }
    if (['numeric-cast', 'primitive-operation', 'comparison', 'boolean-operation'].includes(form)) {
      const operands = (node.arg ? [node.arg] : [node.left, node.right]).map(operand => this.origin(operand, path));
      if (!this.collecting) {
        if (operands.some(operand => !operand.proven)) return unknown(operands.some(operand => operand.uncertain || operand.wrappers.length));
        if (form === 'comparison' && JSON.stringify(operands[0].wrappers) !== JSON.stringify(operands[1].wrappers)) return unknown(true);
        if (form === 'primitive-operation' && operands.some(operand => operand.wrappers.includes('box'))) return unknown(true);
        if (['numeric-cast', 'boolean-operation'].includes(form) && operands.some(operand => operand.wrappers.length > 0)) return unknown(true);
      }
      const copyEvidence = form !== 'primitive-operation' || (node.op !== 'concat' && operands.every(operand => operand.copyEvidence));
      return value(copyEvidence, node.to ?? (['comparison', 'boolean-operation'].includes(form) ? { kind: 'bool' } : null));
    }
    return unknown();
  }

  join(origins) {
    const returning = origins.filter(origin => !origin.diverging);
    if (!returning.length) return { ...value(), diverging: true };
    const first = returning[0];
    if (returning.every(origin => this.same(origin, first))) return clone(first);
    if (returning.every(origin => origin.proven && JSON.stringify(origin.wrappers) === JSON.stringify(first.wrappers))) {
      const fields = returning.every(origin => origin.fields?.length === first.fields?.length) && first.fields
        ? first.fields.map((_, index) => this.join(returning.map(origin => origin.fields[index]))) : null;
      return { ...value(returning.every(origin => origin.copyEvidence)), wrappers: [...first.wrappers], fields };
    }
    return unknown(returning.some(origin => origin.uncertain || origin.wrappers.length > 0));
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
    if (!this.collecting && (!origin.proven || !acceptRustImmutableIndirectionLayer(origin.wrappers[0] ?? ''))) {
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
    if (actualOrigin.diverging) return;
    if (!actualOrigin.proven && !actualOrigin.uncertain && !actualOrigin.wrappers.length && !expectedOrigin.wrappers.length) return;
    if (!actualOrigin.proven || !expectedOrigin.proven) throw unsupported('source indirection', 'the source ownership of this value cannot be proved', span);
    const actual = actualOrigin.wrappers;
    const want = expectedOrigin.wrappers;
    const indirected = actual.length > 0 || want.length > 0;
    const exact = JSON.stringify(actual) === JSON.stringify(want);
    const borrowed = actual[0] === 'reference' && want[0] === 'reference';
    const removed = actual.length - want.length;
    let initial = true;
    for (const layer of actual.slice(1, removed + 1)) {
      if (!acceptRustSourceBoxCoercionLayer(layer, initial)) initial = false;
    }
    const projected = actual.length ? [actual[0], ...actual.slice(removed + 1)] : [];
    const coerced = removed > 0 && initial && JSON.stringify(projected) === JSON.stringify(want);
    if (!acceptRustSourceIndirectionArgument(indirected, exact, borrowed, coerced)) {
      throw unsupported('source indirection', 'source ownership requires an explicit dereference or matching immutable reference or Box type', span);
    }
  }

  direct(node, path, span) {
    const expected = value();
    this.argument(this.origin(node, path), expected, span);
  }

  conversion(node, path) {
    const operand = this.origin(node.arg, path);
    const form = readRustSourceExpressionOrigin(node.k, 'from');
    if (!this.collecting && form === 'unboxed-conversion' && operand.wrappers.includes('box')) return this.mark(node, unknown(true));
    return this.mark(node, operand.proven ? value() : unknown(operand.uncertain));
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

  pattern(pattern, origin, path, result = new Map(), sourceSpan = pattern.span) {
    origin = clone(origin);
    for (let count = this.patternReferences.get(pattern) ?? 0; count > 0; count -= 1) {
      if (origin.wrappers[0] !== 'reference') {
        if (!this.collecting && origin.proven) throw unsupported('source pattern', 'a reference pattern requires a proven reference source type', sourceSpan);
        origin = unknown(true);
      }
      else origin.wrappers.shift();
    }
    if (!this.collecting && !['wild', 'bind', 'bindOrCtor'].includes(pattern.k) && origin.wrappers.includes('box')) {
      throw unsupported('source pattern', 'an owned Box requires an explicit dereference before a constructor or literal pattern', sourceSpan);
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
        this.pattern(pattern.args[index], field, path, result, pattern.args[index].span ?? sourceSpan);
      }
    }
    return result;
  }

  cases(patterns, scrutinee, path) {
    const maps = patterns.map(pattern => this.pattern(pattern, this.origin(scrutinee, path), path, new Map(), pattern.span ?? scrutinee.span));
    const scope = new Map(this.scope);
    const span = patterns.find(pattern => pattern.span)?.span ?? scrutinee.span;
    if (!this.collecting && maps.length > 1 && maps.some(map => map.size !== maps[0].size || [...maps[0].keys()].some(name => !map.has(name)))) {
      throw unsupported('or-pattern ownership', 'every alternative must bind the same names with the same source ownership', span);
    }
    for (const [name, origin] of maps[0]) {
      const candidates = maps.map(map => map.get(name));
      if (!this.collecting && maps.length > 1 && (maps.some(map => map.size !== maps[0].size)
        || candidates.some(candidate => !candidate || !candidate.proven || JSON.stringify(candidate.wrappers) !== JSON.stringify(origin.wrappers)))) {
        throw unsupported('or-pattern ownership', 'every alternative must bind the same names with the same source ownership', span);
      }
      scope.set(name, this.join(candidates.filter(Boolean)));
    }
    return scope;
  }
}
