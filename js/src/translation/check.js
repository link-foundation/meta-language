// Resolves names, checks types, and fixes the exact operator semantics of a
// surface program produced by one of the language frontends. The result is
// the portable-core program the emitters consume: every expression carries
// its type and every operator carries the source language's semantics, so a
// target can never silently substitute its own (for example, Lean's Euclidean
// `Int` division for JavaScript's truncating BigInt division).

import { typeError, unsupported } from './diagnostics.js';
import { normaliseProof } from './proof.js';
import {
  BOOL, INT, NAT, STRING, UNIT, data, fixedBounds, isNatural, isNumeric, sameType, typeKey,
} from './types.js';

const COMPARISONS = new Set(['eq', 'ne', 'lt', 'le', 'gt', 'ge']);
const ARITHMETIC = new Set(['add', 'sub', 'mul', 'div', 'rem']);

export function checkProgram(surface) {
  const checker = new Checker(surface.language);
  return checker.program(surface);
}

class Checker {
  constructor(language) {
    this.language = language;
    this.items = new Map();
    this.fresh = 0;
    this.modules = new Map([['', { path: [], names: new Map(), modules: new Map() }]]);
  }

  program(surface) {
    this.declare(surface.items, []);
    const items = this.checkItems(surface.items, []);
    const main = surface.main ? this.checkMain(surface.main) : null;
    const program = {
      schemaVersion: 1,
      sourceLanguage: this.language,
      items,
      main,
      declarations: this.items,
    };
    for (const entry of this.items.values()) {
      if (entry.k === 'fn') entry.body = reuseSuccessors(entry.body, new Map());
    }
    analyseRecursion(program);
    return program;
  }

  declare(items, path) {
    const scope = this.modules.get(path.join('.'));
    for (const item of items) {
      const fullName = [...path, item.name].join('.');
      if (item.k !== 'module' && scope.names.has(item.name)) {
        throw typeError(`duplicate declaration ${fullName}`, item.span);
      }
      if (/^ml_/u.test(item.name)) {
        throw unsupported('reserved identifier', `${item.name} uses the translator's reserved ml_ prefix`, item.span);
      }
      if (item.k === 'module') {
        // Modules live in their own namespace, so Lean's `namespace Tree` may accompany `inductive Tree`.
        if (!scope.modules.has(item.name)) {
          scope.modules.set(item.name, { k: 'module', fullName });
          this.modules.set(fullName, { path: [...path, item.name], names: new Map(), modules: new Map() });
        }
        this.declare(item.items, [...path, item.name]);
        continue;
      }
      const entry = { ...item, fullName, modulePath: path };
      scope.names.set(item.name, entry);
      this.items.set(fullName, entry);
      if (item.k === 'data') {
        for (const ctor of item.ctors) {
          if (scope.names.has(`${item.name}.${ctor.name}`)) throw typeError(`duplicate constructor ${ctor.name}`, item.span);
          const ctorEntry = { k: 'ctor', data: fullName, name: ctor.name, ctor };
          scope.names.set(`${item.name}.${ctor.name}`, ctorEntry);
          if (this.language === 'Rocq') {
            if (scope.names.has(ctor.name)) throw typeError(`duplicate declaration ${ctor.name}`, item.span);
            scope.names.set(ctor.name, ctorEntry);
          }
        }
      }
    }
  }

  checkItems(items, path) {
    // Data layouts and signatures are resolved program-wide before any body,
    // so declarations may refer to each other in any order.
    this.walk(items, path, (item, itemPath, entry) => {
      if (item.k !== 'data') return;
      entry.ctors = item.ctors.map((ctor) => ({
        name: ctor.name,
        fields: ctor.fields.map((field, index) => ({
          name: field.name ?? `field${index}`,
          type: this.resolveType(field.type, itemPath, item.span),
        })),
      }));
    });
    this.walk(items, path, (item, itemPath, entry) => {
      if (item.k !== 'fn') return;
      entry.params = item.params.map((param) => ({
        name: param.name,
        type: this.resolveType(param.type, itemPath, item.span),
        guard: param.guard ?? null,
      }));
      entry.ret = this.resolveType(item.ret, itemPath, item.span);
    });
    this.walk(items, path, (item, itemPath, entry) => {
      if (item.k === 'fn') {
        const env = new Map(entry.params.map((param) => [param.name, param.type]));
        entry.body = coerce(this.expr(item.body, env, itemPath, entry.ret), entry.ret, this.language, item.span);
      } else if (item.k === 'theorem') {
        entry.binders = item.binders.map((binder) => ({ name: binder.name, type: this.resolveType(binder.type, itemPath, item.span) }));
        let prop = this.prop(item.prop, new Map(entry.binders.map((binder) => [binder.name, binder.type])), itemPath);
        // `theorem t : ∀ n, P n` and `theorem t (n) : P n` state the same
        // proposition; hoisting leading binders gives every target one shape.
        while (prop.p === 'forall') {
          entry.binders.push(...prop.binders);
          prop = prop.body;
        }
        entry.prop = prop;
        entry.proof = normaliseProof(item.proof, entry, {
          lookup: (name) => this.lookup(name.split('.'), itemPath, item.span),
          dataEntry: (name) => this.items.get(name),
        });
      } else if (item.k !== 'data') {
        throw typeError(`unknown item ${item.k}`, item.span);
      }
    });
    const build = (list, listPath) => list.map((item) => (item.k === 'module'
      ? { k: 'module', name: item.name, fullName: [...listPath, item.name].join('.'), items: build(item.items, [...listPath, item.name]), span: item.span }
      : this.items.get([...listPath, item.name].join('.'))));
    return build(items, path);
  }

  walk(items, path, visit) {
    for (const item of items) {
      if (item.k === 'module') this.walk(item.items, [...path, item.name], visit);
      else visit(item, path, this.items.get([...path, item.name].join('.')));
    }
  }

  resolveType(type, path, span) {
    if (!type) throw typeError('missing type annotation', span);
    if (type.kind !== 'named') return type;
    const entry = this.lookup(type.path, path, span);
    if (!entry || entry.k !== 'data') throw typeError(`unknown type ${type.path.join('.')}`, type.span ?? span);
    return data(entry.fullName);
  }

  lookup(segments, path, span) {
    let names = [...segments];
    let start = path;
    if (names[0] === 'crate') {
      names = names.slice(1);
      start = [];
    } else if (names[0] === 'self') {
      names = names.slice(1);
    } else {
      let up = 0;
      while (names[0] === 'super') {
        names = names.slice(1);
        up += 1;
      }
      if (up > path.length) throw typeError('super beyond crate root', span);
      if (up > 0) start = path.slice(0, path.length - up);
    }
    for (let depth = start.length; depth >= 0; depth -= 1) {
      const found = this.lookupFrom(start.slice(0, depth), names);
      if (found) return found;
      if (segments[0] === 'crate' || segments[0] === 'self' || segments[0] === 'super') break;
    }
    return undefined;
  }

  lookupFrom(modulePath, names) {
    let scope = this.modules.get(modulePath.join('.'));
    for (let index = 0; index < names.length - 1; index += 1) {
      const rest = names.slice(index).join('.');
      if (index === names.length - 2 && scope.names.has(rest)) return scope.names.get(rest);
      const module = scope.modules.get(names[index]);
      if (!module) return undefined;
      scope = this.modules.get(module.fullName);
    }
    const last = names.at(-1);
    if (scope.names.has(last)) return scope.names.get(last);
    // Inside `namespace T`, the constructors of a sibling `inductive T` are in scope.
    if (scope.path.length > 0) {
      const parent = this.modules.get(scope.path.slice(0, -1).join('.'));
      const sibling = parent.names.get(scope.path.at(-1));
      if (sibling?.k === 'data') return parent.names.get(`${sibling.name}.${last}`);
    }
    return undefined;
  }

  checkMain(main) {
    const env = new Map();
    const effects = [];
    for (const effect of main.effects) {
      if (effect.k === 'print') {
        effects.push({ k: 'print', expr: this.show(effect.expr, env, [], effect.style), span: effect.span });
        continue;
      }
      if (effect.k === 'let') {
        const expected = effect.type ? this.resolveType(effect.type, [], effect.span) : undefined;
        let value = this.expr(effect.value, env, [], expected);
        if (expected) value = coerce(value, expected, this.language, effect.span);
        env.set(effect.name, value.type);
        effects.push({ k: 'let', name: effect.name, value, span: effect.span });
        continue;
      }
      if (effect.k === 'assert') {
        effects.push({ k: 'assert', prop: this.prop(effect.prop, env, []), span: effect.span });
        continue;
      }
      throw typeError(`unknown effect ${effect.k}`, effect.span);
    }
    return { effects, span: main.span };
  }

  show(expr, env, path, style) {
    const value = this.expr(expr, env, path, undefined);
    if (value.type.kind === 'string') return value;
    if (value.type.kind === 'data' || value.type.kind === 'unit') {
      throw unsupported('output of structured values', `printing a ${typeKey(value.type)} value has no portable textual form`, expr.span);
    }
    const text = { k: 'toString', arg: value, type: STRING };
    if (style === 'js-console' && (value.type.kind === 'int' || value.type.kind === 'nat')) {
      // Node's console.log prints BigInt values with their `n` suffix.
      return { k: 'binary', op: 'concat', left: text, right: literal(STRING, 'n'), type: STRING };
    }
    return text;
  }

  prop(prop, env, path) {
    switch (prop.p) {
      case 'forall': {
        const binders = prop.binders.map((binder) => ({ name: binder.name, type: this.resolveType(binder.type, path, prop.span) }));
        const inner = new Map(env);
        for (const binder of binders) inner.set(binder.name, binder.type);
        return { p: 'forall', binders, body: this.prop(prop.body, inner, path) };
      }
      case 'and':
      case 'or':
      case 'implies':
        return { p: prop.p, left: this.prop(prop.left, env, path), right: this.prop(prop.right, env, path) };
      case 'not':
        return { p: 'not', arg: this.prop(prop.arg, env, path) };
      case 'bool':
        return { p: 'bool', expr: coerce(this.expr(prop.expr, env, path, BOOL), BOOL, this.language, prop.span) };
      default: {
        if (!COMPARISONS.has(prop.p)) throw typeError(`unknown proposition ${prop.p}`, prop.span);
        const [left, right] = this.operands(prop.left, prop.right, env, path, prop.span);
        if (prop.p !== 'eq' && prop.p !== 'ne' && !isNumeric(left.type)) {
          throw typeError(`ordering on ${typeKey(left.type)}`, prop.span);
        }
        // Equality propositions over data are structural; executable targets get a generated equality.
        return { p: prop.p, left, right, domain: left.type };
      }
    }
  }

  operands(leftSurface, rightSurface, env, path, span) {
    let left = this.expr(leftSurface, env, path, undefined, true);
    let right = this.expr(rightSurface, env, path, left.type.kind === 'literal' ? undefined : left.type, true);
    if (left.type.kind === 'literal') left = this.expr(leftSurface, env, path, right.type);
    if (left.type.kind === 'literal') {
      left = this.expr(leftSurface, env, path, this.defaultNumber());
      right = this.expr(rightSurface, env, path, left.type);
    }
    return unify(left, right, this.language, span);
  }

  defaultNumber() {
    return { JavaScript: INT, Rust: { kind: 'fixed', bits: 32, signed: true }, Lean: NAT, Rocq: NAT }[this.language];
  }

  expr(node, env, path, expected, allowLiteral = false) {
    const result = this.exprInner(node, env, path, expected, allowLiteral);
    if (node.span && !result.span) result.span = node.span;
    return result;
  }

  exprInner(node, env, path, expected, allowLiteral) {
    switch (node.k) {
      case 'num': {
        const type = node.type ?? (expected && isNumeric(expected) ? expected : undefined);
        if (!type) {
          if (allowLiteral) return { k: 'lit', type: { kind: 'literal' }, value: node.value };
          return this.expr(node, env, path, this.defaultNumber());
        }
        if (!isNumeric(type)) throw typeError(`numeric literal where ${typeKey(type)} is expected`, node.span);
        const value = BigInt(node.value) * (node.negative ? -1n : 1n);
        if (isNatural(type) && value < 0n) throw typeError('negative literal for a natural type', node.span);
        if (type.kind === 'fixed') {
          const { min, max } = fixedBounds(type);
          if (value < min || value > max) throw typeError(`literal out of range for ${typeKey(type)}`, node.span);
        }
        return literal(type, value.toString());
      }
      case 'bool':
        return literal(BOOL, node.value);
      case 'str':
        return literal(STRING, node.value);
      case 'unit':
        return { k: 'unit', type: UNIT };
      case 'name': {
        if (node.path.length === 1 && env.has(node.path[0])) {
          return { k: 'var', name: node.path[0], type: env.get(node.path[0]) };
        }
        return this.application(node, [], env, path, expected, node.span);
      }
      case 'dotCtor':
      case 'app':
        return this.application(node.k === 'app' ? node.fn : node, node.args ?? [], env, path, expected, node.span);
      case 'field': {
        throw unsupported('field access', `field ${node.field} is only portable inside a constructor match`, node.span);
      }
      case 'unary': {
        if (node.op === 'not') {
          return { k: 'unary', op: 'not', arg: coerce(this.expr(node.arg, env, path, BOOL), BOOL, this.language, node.span), type: BOOL };
        }
        if (node.op === 'neg') {
          if (node.arg.k === 'num') return this.expr({ ...node.arg, negative: !node.arg.negative, span: node.span }, env, path, expected, allowLiteral);
          const arg = this.expr(node.arg, env, path, expected && isNumeric(expected) ? expected : undefined);
          if (arg.type.kind === 'nat' || (arg.type.kind === 'fixed' && !arg.type.signed)) {
            throw typeError(`negation of ${typeKey(arg.type)}`, node.span);
          }
          if (arg.type.kind !== 'int' && arg.type.kind !== 'fixed') throw typeError(`negation of ${typeKey(arg.type)}`, node.span);
          return { k: 'unary', op: 'neg', arg, type: arg.type, semantics: arg.type.kind === 'fixed' ? 'checked' : 'exact' };
        }
        throw typeError(`unknown unary ${node.op}`, node.span);
      }
      case 'binary':
        return this.binary(node, env, path, expected, allowLiteral);
      case 'if': {
        const cond = coerce(this.expr(node.cond, env, path, BOOL), BOOL, this.language, node.span);
        let then = this.expr(node.then, env, path, expected, allowLiteral);
        let otherwise = this.expr(node.else, env, path, then.type.kind === 'literal' ? expected : then.type, allowLiteral);
        if (then.type.kind === 'literal' && otherwise.type.kind !== 'literal') then = this.expr(node.then, env, path, otherwise.type);
        if (then.type.kind === 'literal') {
          then = this.expr(node.then, env, path, this.defaultNumber());
          otherwise = this.expr(node.else, env, path, then.type);
        }
        [then, otherwise] = unifyBranches(then, otherwise, expected, this.language, node.span);
        return { k: 'if', cond, then, else: otherwise, type: then.type };
      }
      case 'let': {
        const declared = node.type ? this.resolveType(node.type, path, node.span) : undefined;
        let value = this.expr(node.value, env, path, declared);
        if (declared) value = coerce(value, declared, this.language, node.span);
        const inner = new Map(env);
        inner.set(node.name, value.type);
        const body = this.expr(node.body, inner, path, expected, allowLiteral);
        return { k: 'let', name: node.name, value, body, type: body.type };
      }
      case 'match':
        return this.expr(this.compileMatch(node, env, path), env, path, expected, allowLiteral);
      case 'match1':
        return this.match(node, env, path, expected);
      case 'toString': {
        const arg = this.expr(node.arg, env, path, undefined);
        if (arg.type.kind === 'string') return arg;
        if (!isNumeric(arg.type) && arg.type.kind !== 'bool') throw typeError(`toString of ${typeKey(arg.type)}`, node.span);
        return { k: 'toString', arg, type: STRING };
      }
      case 'show':
        return this.show(node.arg, env, path, node.style);
      case 'cast': {
        const to = this.resolveType(node.to, path, node.span);
        const arg = this.expr(node.arg, env, path, node.from);
        return castTo(arg, to, node.flavor, node.span);
      }
      case 'abort': {
        if (!expected) throw typeError('abort needs a known result type', node.span);
        return { k: 'abort', message: node.message, type: expected };
      }
      case 'ctorObject':
        return this.ctorObject(node, env, path, expected);
      default:
        throw typeError(`unknown expression ${node.k}`, node.span);
    }
  }

  binary(node, env, path, expected, allowLiteral) {
    const { op } = node;
    if (op === 'and' || op === 'or') {
      const left = coerce(this.expr(node.left, env, path, BOOL), BOOL, this.language, node.span);
      const right = coerce(this.expr(node.right, env, path, BOOL), BOOL, this.language, node.span);
      return { k: 'binary', op, left, right, type: BOOL };
    }
    if (op === 'concat') {
      const left = coerce(this.expr(node.left, env, path, STRING), STRING, this.language, node.span);
      const right = coerce(this.expr(node.right, env, path, STRING), STRING, this.language, node.span);
      return { k: 'binary', op, left, right, type: STRING };
    }
    if (COMPARISONS.has(op)) {
      const [left, right] = this.operands(node.left, node.right, env, path, node.span);
      if (left.type.kind === 'data' || left.type.kind === 'unit') {
        throw unsupported('structural equality of data values', 'comparisons of data values are not in the portable core', node.span);
      }
      if (op !== 'eq' && op !== 'ne' && !isNumeric(left.type)) throw typeError(`ordering on ${typeKey(left.type)}`, node.span);
      return { k: 'binary', op, left, right, type: BOOL, domain: left.type };
    }
    if (!ARITHMETIC.has(op)) throw typeError(`unknown operator ${op}`, node.span);
    const numericExpected = expected && isNumeric(expected) ? expected : undefined;
    let left = this.expr(node.left, env, path, numericExpected, true);
    let right = this.expr(node.right, env, path, left.type.kind === 'literal' ? numericExpected : left.type, true);
    if (left.type.kind === 'literal') left = this.expr(node.left, env, path, right.type.kind === 'literal' ? numericExpected : right.type, true);
    if (left.type.kind === 'literal' && right.type.kind === 'literal') {
      if (allowLiteral) return { k: 'lit', type: { kind: 'literal' }, value: '0', pending: node };
      left = this.expr(node.left, env, path, this.defaultNumber());
      right = this.expr(node.right, env, path, left.type);
    }
    if (right.type.kind === 'literal') right = this.expr(node.right, env, path, left.type);
    if (left.type.kind === 'literal') left = this.expr(node.left, env, path, right.type);
    [left, right] = unify(left, right, this.language, node.span);
    if (!isNumeric(left.type)) throw typeError(`arithmetic on ${typeKey(left.type)}`, node.span);
    const semantics = arithmeticSemantics(this.language, op, left.type, node);
    return { k: 'binary', op, left, right, type: left.type, domain: left.type, ...semantics };
  }

  application(head, args, env, path, expected, span) {
    if (head.k === 'dotCtor') {
      if (!expected || expected.kind !== 'data') throw typeError(`cannot infer the type of .${head.name}`, span);
      const dataEntry = this.items.get(expected.name);
      const ctor = dataEntry.ctors.find((candidate) => candidate.name === head.name);
      if (!ctor) throw typeError(`${expected.name} has no constructor ${head.name}`, span);
      return this.construct(dataEntry, ctor, args, env, path, span);
    }
    if (head.k !== 'name') {
      throw unsupported('higher-order application', 'only named functions and constructors can be applied', span);
    }
    if (head.path.length === 1 && env.has(head.path[0])) {
      throw unsupported('higher-order application', `local ${head.path[0]} is not a function in the portable core`, span);
    }
    const entry = this.lookup(head.path, path, span);
    if (!entry) throw typeError(`unknown name ${head.path.join('.')}`, span);
    if (entry.k === 'ctor') {
      const dataEntry = this.items.get(entry.data);
      const ctor = dataEntry.ctors.find((candidate) => candidate.name === entry.name);
      return this.construct(dataEntry, ctor, args, env, path, span);
    }
    if (entry.k !== 'fn') throw typeError(`${head.path.join('.')} is not a function`, span);
    if (!entry.params) throw typeError(`${entry.fullName} is used before its signature is known`, span);
    if (args.length !== entry.params.length) {
      if (args.length < entry.params.length) {
        throw unsupported('partial application', `${entry.fullName} expects ${entry.params.length} arguments`, span);
      }
      throw typeError(`${entry.fullName} expects ${entry.params.length} arguments but got ${args.length}`, span);
    }
    const checkedArgs = args.map((arg, index) => {
      const param = entry.params[index];
      const value = this.expr(arg, env, path, param.type);
      return coerce(value, param.type, this.language, arg.span ?? span, param.guard ? 'checked' : undefined);
    });
    return { k: 'call', fn: entry.fullName, args: checkedArgs, type: entry.ret };
  }

  construct(dataEntry, ctor, args, env, path, span) {
    if (args.length !== ctor.fields.length) {
      throw typeError(`${dataEntry.fullName}.${ctor.name} expects ${ctor.fields.length} fields but got ${args.length}`, span);
    }
    const checkedArgs = args.map((arg, index) => coerce(
      this.expr(arg, env, path, ctor.fields[index].type),
      ctor.fields[index].type,
      this.language,
      arg.span ?? span,
    ));
    return { k: 'ctor', data: dataEntry.fullName, ctor: ctor.name, args: checkedArgs, type: data(dataEntry.fullName) };
  }

  ctorObject(node, env, path, expected) {
    const candidates = [];
    for (const entry of this.items.values()) {
      if (entry.k !== 'data' || !entry.ctors) continue;
      if (expected && expected.kind === 'data' && entry.fullName !== expected.name) continue;
      const ctor = entry.ctors.find((candidate) => candidate.name === node.tag);
      if (ctor) candidates.push([entry, ctor]);
    }
    if (candidates.length !== 1) {
      throw typeError(candidates.length === 0 ? `no data type has tag ${node.tag}` : `tag ${node.tag} is ambiguous`, node.span);
    }
    const [entry, ctor] = candidates[0];
    const names = Object.keys(node.fields);
    const expectedNames = ctor.fields.map((field) => field.name);
    if (names.length !== expectedNames.length || names.some((name) => !expectedNames.includes(name))) {
      throw typeError(`${node.tag} expects fields ${expectedNames.join(', ')}`, node.span);
    }
    return this.construct(entry, ctor, expectedNames.map((name) => node.fields[name]), env, path, node.span);
  }

  /**
   * Compiles a surface match (several scrutinees, nested and literal
   * patterns) into single-level constructor matches and literal if-chains.
   * Bind patterns name the constructor fields directly so structural
   * recursion stays visible to the recursion analysis.
   */
  compileMatch(node, env, path) {
    const lets = [];
    const columns = node.scrutinees.map((scrutinee) => {
      const checked = this.expr(scrutinee, env, path, undefined);
      if (scrutinee.k === 'name' && scrutinee.path.length === 1 && env.has(scrutinee.path[0])) {
        return { name: scrutinee.path[0], type: checked.type };
      }
      const name = this.freshName('ml_s');
      lets.push({ name, value: scrutinee, type: checked.type });
      return { name, type: checked.type };
    });
    const rows = node.rows.map((row) => ({ patterns: row.patterns, body: row.body, binds: [], span: row.span }));
    let body = this.compileRows(columns, rows, node.span);
    for (const binding of lets.reverse()) {
      body = { k: 'let', name: binding.name, value: binding.value, body, span: node.span };
    }
    return body;
  }

  freshName(prefix) {
    this.fresh += 1;
    return `${prefix}${this.fresh}`;
  }

  compileRows(columns, rows, span) {
    if (!rows.length) throw typeError('non-exhaustive match', span);
    const normalised = rows.map((row) => ({
      ...row,
      patterns: row.patterns.map((pattern, index) => this.normalisePattern(pattern, columns[index].type, span)),
    }));
    const [first] = normalised;
    const refutable = first.patterns.findIndex((pattern) => pattern.k !== 'wild' && pattern.k !== 'bind');
    if (refutable < 0) {
      const binds = [...first.binds];
      first.patterns.forEach((pattern, index) => {
        if (pattern.k === 'bind') binds.push([pattern.name, columns[index].name]);
      });
      return binds.reduceRight((body, [name, variable]) => (name === variable
        ? body
        : { k: 'let', name, value: { k: 'name', path: [variable] }, body, span }), first.body);
    }
    const column = columns[refutable];
    const pivot = first.patterns[refutable];
    const rest = (patterns) => patterns.filter((_, index) => index !== refutable);
    const bindColumn = (row) => {
      const pattern = row.patterns[refutable];
      return pattern.k === 'bind' ? [...row.binds, [pattern.name, column.name]] : row.binds;
    };
    if (pivot.k === 'lit') {
      // Literal patterns on integers, booleans and strings become equality tests.
      const matching = normalised.filter((row) => ['wild', 'bind'].includes(row.patterns[refutable].k)
        || (row.patterns[refutable].k === 'lit' && row.patterns[refutable].key === pivot.key));
      const others = normalised.filter((row) => row.patterns[refutable].k !== 'lit' || row.patterns[refutable].key !== pivot.key);
      const remaining = columns.filter((_, index) => index !== refutable);
      const then = this.compileRows(remaining, matching.map((row) => ({ ...row, patterns: rest(row.patterns), binds: bindColumn(row) })), span);
      if (column.type.kind === 'bool' && others.every((row) => row.patterns[refutable].k === 'lit')) {
        const exhaustive = new Set(normalised.map((row) => row.patterns[refutable].key)).size === 2;
        if (!exhaustive) throw typeError('non-exhaustive match on bool', span);
      }
      const otherwise = this.compileRows(columns, others, span);
      return {
        k: 'if',
        cond: { k: 'binary', op: 'eq', left: { k: 'name', path: [column.name] }, right: pivot.value },
        then,
        else: otherwise,
        span,
      };
    }
    const ctors = pivot.k === 'nat'
      ? [{ name: 'zero', fields: [] }, { name: 'succ', fields: [{ type: column.type }] }]
      : this.items.get(column.type.name).ctors;
    const cases = ctors.map((ctor) => {
      const fieldNames = ctor.fields.map((_, fieldIndex) => {
        for (const row of normalised) {
          const pattern = row.patterns[refutable];
          if (pattern.ctor === ctor.name && pattern.args[fieldIndex].k === 'bind') return pattern.args[fieldIndex].name;
        }
        return this.freshName('ml_f');
      });
      const fieldColumns = ctor.fields.map((field, fieldIndex) => ({ name: fieldNames[fieldIndex], type: field.type }));
      const specialised = [];
      for (const row of normalised) {
        const pattern = row.patterns[refutable];
        if (pattern.k === 'wild' || pattern.k === 'bind') {
          specialised.push({
            ...row,
            patterns: [...fieldColumns.map(() => ({ k: 'wild' })), ...rest(row.patterns)],
            binds: bindColumn(row),
          });
        } else if (pattern.ctor === ctor.name) {
          specialised.push({ ...row, patterns: [...pattern.args, ...rest(row.patterns)] });
        }
      }
      const body = this.compileRows([...fieldColumns, ...columns.filter((_, index) => index !== refutable)], specialised, span);
      const casePattern = pivot.k === 'nat'
        ? (ctor.name === 'zero' ? { k: 'natZero' } : { k: 'natSucc', name: fieldNames[0] })
        : { k: 'ctor', path: [ctor.name], binds: fieldNames };
      return { pattern: casePattern, body };
    });
    return { k: 'match1', scrutinee: { k: 'name', path: [column.name] }, cases, span };
  }

  /** Resolves a surface pattern against the column type: constructors, naturals, literals, binders. */
  normalisePattern(pattern, type, span) {
    const natural = type.kind === 'nat';
    switch (pattern.k) {
      case 'wild':
      case 'bind':
      case 'nat':
      case 'data':
      case 'lit':
        return pattern;
      case 'bindOrCtor': {
        if (type.kind === 'data') {
          const ctor = this.items.get(type.name).ctors.find((candidate) => candidate.name === pattern.name);
          if (ctor) {
            if (ctor.fields.length) throw typeError(`${pattern.name} pattern needs ${ctor.fields.length} fields`, pattern.span ?? span);
            return { k: 'data', ctor: ctor.name, args: [] };
          }
        }
        if (natural && this.language === 'Rocq' && pattern.name === 'O') return { k: 'nat', ctor: 'zero', args: [] };
        if (type.kind === 'bool' && (pattern.name === 'true' || pattern.name === 'false')) {
          return this.literalPattern({ k: 'bool', value: pattern.name === 'true' }, type, span);
        }
        return { k: 'bind', name: pattern.name };
      }
      case 'numLit': {
        if (natural) {
          let result = { k: 'nat', ctor: 'zero', args: [] };
          for (let count = 0; count < pattern.value; count += 1) result = { k: 'nat', ctor: 'succ', args: [result] };
          return result;
        }
        return this.literalPattern({ k: 'num', value: String(pattern.value), negative: pattern.negative }, type, span);
      }
      case 'boolLit':
        return this.literalPattern({ k: 'bool', value: pattern.value }, type, span);
      case 'strLit':
        return this.literalPattern({ k: 'str', value: pattern.value }, type, span);
      case 'natAdd': {
        if (!natural) throw unsupported('n + k pattern', `only natural-number scrutinees support n + k patterns, not ${typeKey(type)}`, pattern.span ?? span);
        let result = this.normalisePattern(pattern.inner, type, span);
        for (let count = 0; count < pattern.add; count += 1) result = { k: 'nat', ctor: 'succ', args: [result] };
        return result;
      }
      case 'ctor': {
        const name = pattern.path.at(-1);
        if (natural && pattern.args.length === 1 && (name === 'succ' || name === 'S')) {
          return { k: 'nat', ctor: 'succ', args: [this.normalisePattern(pattern.args[0], type, span)] };
        }
        if (natural && pattern.args.length === 0 && (name === 'zero' || name === 'O')) return { k: 'nat', ctor: 'zero', args: [] };
        if (type.kind !== 'data') throw typeError(`constructor pattern ${name} on ${typeKey(type)}`, pattern.span ?? span);
        const dataEntry = this.items.get(type.name);
        if (pattern.path.length > 1) {
          const owner = this.lookup(pattern.path, [], pattern.span ?? span) ?? this.lookup(pattern.path, dataEntry.modulePath, pattern.span ?? span);
          if (!owner || owner.k !== 'ctor' || owner.data !== type.name) throw typeError(`${pattern.path.join('.')} is not a constructor of ${type.name}`, pattern.span ?? span);
        }
        const ctor = dataEntry.ctors.find((candidate) => candidate.name === name);
        if (!ctor) throw typeError(`${type.name} has no constructor ${name}`, pattern.span ?? span);
        if (pattern.args.length !== ctor.fields.length) {
          throw typeError(`${name} pattern has ${pattern.args.length} of ${ctor.fields.length} fields`, pattern.span ?? span);
        }
        return {
          k: 'data',
          ctor: name,
          args: pattern.args.map((arg, index) => this.normalisePattern(arg, ctor.fields[index].type, span)),
        };
      }
      default:
        throw unsupported('pattern', `${pattern.k} patterns are outside the portable core`, pattern.span ?? span);
    }
  }

  literalPattern(value, type, span) {
    const checked = this.expr(value, new Map(), [], type);
    if (!sameType(checked.type, type)) throw typeError(`literal pattern of type ${typeKey(checked.type)} on ${typeKey(type)}`, span);
    return { k: 'lit', value, key: `${typeKey(type)}:${checked.value}` };
  }

  match(node, env, path, expected) {
    const scrutinee = this.expr(node.scrutinee, env, path, undefined);
    const type = scrutinee.type;
    const cases = [];
    let resultType = expected;
    const pending = [];
    for (const kase of node.cases) {
      const inner = new Map(env);
      const pattern = this.pattern(kase.pattern, type, inner, path, kase.span ?? node.span);
      pending.push({ pattern, body: kase.body, env: inner, span: kase.span });
    }
    for (const kase of pending) {
      const body = this.expr(kase.body, kase.env, path, resultType, true);
      if (body.type.kind !== 'literal' && !resultType) resultType = body.type;
      cases.push({ pattern: kase.pattern, body, surface: kase.body, env: kase.env });
    }
    if (!resultType) resultType = this.defaultNumber();
    const finalCases = cases.map((kase) => {
      let body = kase.body.type.kind === 'literal' ? this.expr(kase.surface, kase.env, path, resultType) : kase.body;
      body = coerce(body, resultType, this.language, kase.surface.span ?? node.span);
      return { pattern: kase.pattern, body };
    });
    checkExhaustive(type, finalCases.map((kase) => kase.pattern), this.items, node.span);
    return { k: 'match', scrutinee, cases: finalCases, type: resultType };
  }

  pattern(pattern, type, env, path, span) {
    if (pattern.k === 'wild') return { k: 'wild' };
    if (pattern.k === 'bind') {
      env.set(pattern.name, type);
      return { k: 'bind', name: pattern.name };
    }
    if (pattern.k === 'natZero' || pattern.k === 'natSucc') {
      if (!isNatural(type) || type.kind === 'fixed') throw typeError(`natural-number pattern on ${typeKey(type)}`, span);
      if (pattern.k === 'natSucc') {
        env.set(pattern.name, type);
        return { k: 'natSucc', name: pattern.name };
      }
      return { k: 'natZero' };
    }
    if (pattern.k === 'ctor') {
      if (type.kind !== 'data') throw typeError(`constructor pattern on ${typeKey(type)}`, span);
      const dataEntry = this.items.get(type.name);
      const ctorName = pattern.path.at(-1);
      const ctor = dataEntry.ctors.find((candidate) => candidate.name === ctorName);
      if (!ctor) throw typeError(`${type.name} has no constructor ${ctorName}`, span);
      if (pattern.binds.length !== ctor.fields.length) {
        throw typeError(`${ctorName} pattern binds ${pattern.binds.length} of ${ctor.fields.length} fields`, span);
      }
      const binds = pattern.binds.map((bind, index) => {
        if (bind === '_' || bind === null) return null;
        env.set(bind, ctor.fields[index].type);
        return bind;
      });
      return { k: 'ctor', data: type.name, ctor: ctorName, binds };
    }
    throw unsupported('pattern', `${pattern.k} patterns are not in the portable core`, span);
  }
}

function literal(type, value) {
  return { k: 'lit', type, value };
}

/**
 * Inside `match v with | succ k => …`, the value `k + 1` is `v` itself.
 * Rewriting it to `v` keeps recursion such as `fib (n + 1) + fib n`
 * structural for Lean and Rocq without changing any value.
 */
function reuseSuccessors(expr, predecessors) {
  if (!expr || typeof expr !== 'object') return expr;
  switch (expr.k) {
    case 'binary': {
      const { left, right } = expr;
      if (expr.op === 'add' && expr.type.kind === 'nat' && left.k === 'var' && predecessors.has(left.name)
        && right.k === 'lit' && right.value === '1') {
        return { k: 'var', name: predecessors.get(left.name), type: expr.type, span: expr.span };
      }
      return { ...expr, left: reuseSuccessors(left, predecessors), right: reuseSuccessors(right, predecessors) };
    }
    case 'let': {
      const inner = without(predecessors, [expr.name]);
      return { ...expr, value: reuseSuccessors(expr.value, predecessors), body: reuseSuccessors(expr.body, inner) };
    }
    case 'match':
      return {
        ...expr,
        scrutinee: reuseSuccessors(expr.scrutinee, predecessors),
        cases: expr.cases.map((kase) => {
          const bound = kase.pattern.k === 'natSucc' ? [kase.pattern.name] : (kase.pattern.binds ?? []).filter(Boolean);
          const inner = without(predecessors, bound);
          if (kase.pattern.k === 'natSucc' && expr.scrutinee.k === 'var' && !bound.includes(expr.scrutinee.name)) {
            inner.set(kase.pattern.name, expr.scrutinee.name);
          }
          return { ...kase, body: reuseSuccessors(kase.body, inner) };
        }),
      };
    default: {
      const copy = { ...expr };
      for (const [key, value] of Object.entries(expr)) {
        if (key === 'type' || key === 'domain' || key === 'from' || key === 'to') continue;
        if (Array.isArray(value)) copy[key] = value.map((item) => reuseSuccessors(item, predecessors));
        else if (value && typeof value === 'object' && value.k) copy[key] = reuseSuccessors(value, predecessors);
      }
      return copy;
    }
  }
}

function without(map, names) {
  const copy = new Map(map);
  for (const [key, value] of map) if (names.includes(key) || names.includes(value)) copy.delete(key);
  return copy;
}

function arithmeticSemantics(language, op, type, node) {
  const division = op === 'div' || op === 'rem';
  if (type.kind === 'fixed') {
    // Rust integer arithmetic with overflow checks: division truncates and panics on zero.
    return division ? { semantics: 'checked', rounding: 'trunc', byZero: 'abort' } : { semantics: 'checked' };
  }
  if (op === 'sub') return { semantics: type.kind === 'nat' ? 'truncated' : 'exact' };
  if (!division) return { semantics: 'exact' };
  if (language === 'JavaScript') return { semantics: 'exact', rounding: 'trunc', byZero: 'abort' };
  // Lean and Rocq division is total: x / 0 = 0 and x % 0 = x.
  if (type.kind === 'nat') return { semantics: 'exact', rounding: 'trunc', byZero: 'total' };
  if (language === 'Lean') return { semantics: 'exact', rounding: node.rounding ?? 'euclid', byZero: 'total' };
  return { semantics: 'exact', rounding: node.rounding ?? 'floor', byZero: 'total' };
}

function unify(left, right, language, span) {
  if (sameType(left.type, right.type)) return [left, right];
  if (language === 'JavaScript') {
    // Guarded parameters are naturals, but every BigInt operation is an integer operation.
    return [coerce(left, INT, language, span), coerce(right, INT, language, span)];
  }
  throw typeError(`operand types differ: ${typeKey(left.type)} and ${typeKey(right.type)}`, span);
}

function unifyBranches(then, otherwise, expected, language, span) {
  if (sameType(then.type, otherwise.type)) return [then, otherwise];
  if (then.k === 'abort') return [{ ...then, type: otherwise.type }, otherwise];
  if (otherwise.k === 'abort') return [then, { ...otherwise, type: then.type }];
  if (expected) return [coerce(then, expected, language, span), coerce(otherwise, expected, language, span)];
  return unify(then, otherwise, language, span);
}

/** Inserts the source language's implicit conversions, which only JavaScript has. */
export function coerce(value, type, language, span, flavor = undefined) {
  if (value.type.kind === 'literal') throw typeError('untyped literal', span);
  if (sameType(value.type, type)) return value;
  if (value.k === 'abort') return { ...value, type };
  if (language === 'JavaScript' && value.type.kind === 'nat' && type.kind === 'int') {
    return castTo(value, INT, 'exact', span);
  }
  if (language === 'JavaScript' && value.type.kind === 'int' && type.kind === 'nat') {
    return castTo(value, NAT, flavor ?? 'checked', span);
  }
  throw typeError(`expected ${typeKey(type)} but found ${typeKey(value.type)}`, span);
}

function castTo(arg, to, flavor, span) {
  if (sameType(arg.type, to)) return arg;
  const from = arg.type;
  if (from.kind === 'nat' && to.kind === 'int') return { k: 'cast', arg, from, to, flavor: 'exact', type: to };
  if (from.kind === 'int' && to.kind === 'nat') {
    if (flavor !== 'checked' && flavor !== 'clamp') throw typeError('int to nat conversion needs checked or clamp semantics', span);
    return { k: 'cast', arg, from, to, flavor, type: to };
  }
  if (from.kind === 'fixed' && to.kind === 'fixed') {
    const source = fixedBounds(from);
    const target = fixedBounds(to);
    if (source.min >= target.min && source.max <= target.max) return { k: 'cast', arg, from, to, flavor: 'exact', type: to };
    throw unsupported('narrowing integer cast', `${typeKey(from)} as ${typeKey(to)} wraps in Rust and has no portable encoding yet`, span);
  }
  throw typeError(`cannot convert ${typeKey(from)} to ${typeKey(to)}`, span);
}

function checkExhaustive(type, patterns, items, span) {
  if (patterns.some((pattern) => pattern.k === 'wild' || pattern.k === 'bind')) return;
  if (type.kind === 'data') {
    const ctors = items.get(type.name).ctors.map((ctor) => ctor.name);
    const covered = new Set(patterns.map((pattern) => pattern.ctor));
    const missing = ctors.filter((ctor) => !covered.has(ctor));
    if (missing.length) throw typeError(`match on ${type.name} misses ${missing.join(', ')}`, span);
    return;
  }
  if (isNatural(type)) {
    const kinds = new Set(patterns.map((pattern) => pattern.k));
    if (kinds.has('natZero') && kinds.has('natSucc')) return;
  }
  throw typeError(`non-exhaustive match on ${typeKey(type)}`, span);
}

/**
 * Marks each function recursive or not and, for recursive functions, finds a
 * structurally decreasing parameter. Lean and Rocq targets require one; the
 * other targets accept general recursion.
 */
function analyseRecursion(program) {
  for (const entry of program.declarations.values()) {
    if (entry.k !== 'fn') continue;
    const calls = [];
    collectCalls(entry.body, calls);
    const selfCalls = calls.filter((call) => call.fn === entry.fullName);
    entry.recursive = selfCalls.length > 0;
    entry.decreasing = entry.recursive ? structuralParameter(entry) : null;
  }
  const graph = new Map();
  for (const entry of program.declarations.values()) {
    if (entry.k !== 'fn') continue;
    const calls = [];
    collectCalls(entry.body, calls);
    graph.set(entry.fullName, new Set(calls.map((call) => call.fn).filter((name) => name !== entry.fullName)));
  }
  for (const [name, targets] of graph) {
    for (const target of targets) {
      if (reaches(graph, target, name, new Set())) {
        const entry = program.declarations.get(name);
        entry.mutual = true;
      }
    }
  }
}

function reaches(graph, from, to, seen) {
  if (from === to) return true;
  if (seen.has(from)) return false;
  seen.add(from);
  for (const next of graph.get(from) ?? []) {
    if (reaches(graph, next, to, seen)) return true;
  }
  return false;
}

export function collectCalls(expr, calls) {
  if (!expr || typeof expr !== 'object') return;
  if (expr.k === 'call') calls.push(expr);
  for (const value of Object.values(expr)) {
    if (Array.isArray(value)) value.forEach((item) => collectCalls(item, calls));
    else if (value && typeof value === 'object' && value !== expr.type) collectCalls(value, calls);
  }
}

function structuralParameter(entry) {
  for (let index = 0; index < entry.params.length; index += 1) {
    const param = entry.params[index];
    if (decreasesOn(entry.body, entry.fullName, index, param.name, new Set())) return index;
  }
  return null;
}

/** Every recursive call's argument at `index` must be a strict subterm of `name`. */
function decreasesOn(expr, fn, index, name, subterms) {
  if (!expr || typeof expr !== 'object') return true;
  switch (expr.k) {
    case 'call':
      if (expr.fn === fn) {
        const arg = expr.args[index];
        if (!(arg.k === 'var' && subterms.has(arg.name))) return false;
      }
      return expr.args.every((arg) => decreasesOn(arg, fn, index, name, subterms));
    case 'match': {
      if (!decreasesOn(expr.scrutinee, fn, index, name, subterms)) return false;
      const isParam = expr.scrutinee.k === 'var' && (expr.scrutinee.name === name || subterms.has(expr.scrutinee.name));
      return expr.cases.every((kase) => {
        const inner = new Set(subterms);
        if (isParam) {
          if (kase.pattern.k === 'natSucc') inner.add(kase.pattern.name);
          if (kase.pattern.k === 'ctor') {
            for (const bind of kase.pattern.binds) if (bind) inner.add(bind);
          }
        }
        return decreasesOn(kase.body, fn, index, name, inner);
      });
    }
    case 'let': {
      if (!decreasesOn(expr.value, fn, index, name, subterms)) return false;
      const inner = new Set(subterms);
      inner.delete(expr.name);
      // `let k := subterm` (introduced by pattern compilation) is itself a subterm.
      if (expr.value.k === 'var' && subterms.has(expr.value.name)) inner.add(expr.name);
      return decreasesOn(expr.body, fn, index, name, inner);
    }
    default:
      for (const [key, value] of Object.entries(expr)) {
        if (key === 'type' || key === 'domain' || key === 'from' || key === 'to') continue;
        if (Array.isArray(value)) {
          if (!value.every((item) => decreasesOn(item, fn, index, name, subterms))) return false;
        } else if (value && typeof value === 'object' && !decreasesOn(value, fn, index, name, subterms)) {
          return false;
        }
      }
      return true;
  }
}
