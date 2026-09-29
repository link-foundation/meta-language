// Output as a value, for the targets whose programs are pure. Rust and
// JavaScript print where the source prints; in Lean and Rocq a function that
// prints, directly or through a function it calls, takes the lines printed
// before it and returns them, with the lines it prints added in front, paired
// with its value, and main prints the lines each of its steps adds. The
// rewriting is target-independent: it yields a portable-core program without
// `print` nodes, whose pairs are generated data types.

import { unsupported } from './diagnostics.js';
import { BOOL, OUTPUT, data, typeKey } from './types.js';

const SKIP = new Set(['type', 'domain', 'from', 'to', 'span']);

/** The checked program with its output threaded through, or the program itself when nothing prints below main. */
export function threadOutput(program) {
  const functions = [...program.declarations.values()].filter((entry) => entry.k === 'fn');
  const effectful = new Set();
  const prints = (node) => {
    if (!node || typeof node !== 'object') return false;
    if (Array.isArray(node)) return node.some(prints);
    if (node.k === 'print' || (node.k === 'call' && effectful.has(node.fn))) return true;
    return Object.entries(node).some(([key, value]) => !SKIP.has(key) && prints(value));
  };
  for (let grew = true; grew;) {
    grew = false;
    for (const entry of functions) {
      if (!effectful.has(entry.fullName) && prints(entry.body)) {
        effectful.add(entry.fullName);
        grew = true;
      }
    }
  }
  // A step of main is a `print` effect itself; what threads is a value that prints.
  const operand = (effect) => (effect.k === 'let' ? effect.value : effect.k === 'assert' ? effect.prop : effect.expr);
  if (!effectful.size && !(program.main?.effects ?? []).some((effect) => prints(operand(effect)))) return program;
  return new Threader(program, effectful, prints).program();
}

function variable(name, type) {
  return { k: 'var', name, type };
}

class Threader {
  constructor(source, effectful, prints) {
    this.source = source;
    this.effectful = effectful;
    this.prints = prints;
    this.pairs = new Map();
    this.count = 0;
  }

  program() {
    const declarations = new Map();
    for (const [name, entry] of this.source.declarations) {
      if (entry.k === 'fn' && this.effectful.has(name)) {
        const out = 'ml_out';
        declarations.set(name, {
          ...entry,
          params: [...entry.params, { name: out, type: OUTPUT, guard: null }],
          ret: this.pairType(entry.ret),
          body: this.io(entry.body, variable(out, OUTPUT)),
        });
      } else if (entry.k === 'theorem' && this.prints(entry.prop)) {
        declarations.set(name, { ...entry, prop: this.pureProp(entry.prop) });
      } else declarations.set(name, entry);
    }
    const main = this.source.main && { ...this.source.main, effects: this.source.main.effects.flatMap((effect) => this.effect(effect)) };
    // The pairs come first; a pair of a data type follows it in the emitted order.
    return { ...this.source, declarations: new Map([...[...this.pairs.values()].map((entry) => [entry.fullName, entry]), ...declarations]), main, outputThreaded: true };
  }

  fresh(prefix) {
    this.count += 1;
    return `ml_${prefix}${this.count}`;
  }

  /** The generated data type pairing the lines printed so far with a value of `type`. */
  pair(type) {
    const key = typeKey(type);
    if (!this.pairs.has(key)) {
      const name = `ml_io${this.pairs.size + 1}`;
      this.pairs.set(key, {
        k: 'data',
        name,
        fullName: name,
        modulePath: [],
        ctors: [{ name: `${name}_mk`, fields: [{ name: 'output', type: OUTPUT }, { name: 'value', type }] }],
        generated: true,
        output: true,
      });
    }
    return this.pairs.get(key);
  }

  pairType(type) {
    return data(this.pair(type).fullName);
  }

  make(out, value) {
    const entry = this.pair(value.type);
    return { k: 'ctor', data: entry.fullName, ctor: entry.ctors[0].name, args: [out, value], type: data(entry.fullName) };
  }

  /** `match pair with mk o v => body(o, v)`. */
  open(pair, type, body) {
    const entry = this.pair(type);
    const out = this.fresh('o');
    const value = this.fresh('v');
    const inner = body(variable(out, OUTPUT), variable(value, type));
    return {
      k: 'match',
      scrutinee: pair,
      cases: [{ pattern: { k: 'ctor', data: entry.fullName, ctor: entry.ctors[0].name, binds: [out, value] }, body: inner }],
      type: inner.type,
    };
  }

  /** `e` run after the lines `out`, a variable: its value paired with the lines printed by then. */
  io(e, out) {
    if (!this.prints(e)) return this.make(out, e);
    const type = this.pairType(e.type);
    switch (e.k) {
      case 'print':
        return this.bind(e.text, out, (before, text) => {
          const after = this.fresh('o');
          return {
            k: 'let',
            name: after,
            value: { k: 'outCons', head: text, tail: before, type: OUTPUT },
            body: this.io(e.body, variable(after, OUTPUT)),
            type,
          };
        });
      case 'call':
        return this.bindAll(e.args, out, (after, args) => (this.effectful.has(e.fn)
          ? { ...e, args: [...args, after], type }
          : this.make(after, { ...e, args })));
      case 'if':
        return this.bind(e.cond, out, (after, cond) => ({ ...e, cond, then: this.io(e.then, after), else: this.io(e.else, after), type }));
      case 'let':
        return this.bind(e.value, out, (after, value) => ({ ...e, value, body: this.io(e.body, after), type }));
      case 'match':
        return this.bind(e.scrutinee, out, (after, scrutinee) => ({
          ...e,
          scrutinee,
          cases: e.cases.map((kase) => ({ ...kase, body: this.io(kase.body, after) })),
          type,
        }));
      case 'binary':
        // The right operand of `&&` and `||` runs only when the left one does not decide.
        if (e.op === 'and' || e.op === 'or') {
          const decided = { k: 'lit', type: BOOL, value: e.op === 'or' };
          const test = e.op === 'and'
            ? { k: 'if', cond: e.left, then: e.right, else: decided, type: BOOL }
            : { k: 'if', cond: e.left, then: decided, else: e.right, type: BOOL };
          return this.io(test, out);
        }
        break;
      default:
    }
    // Every other node runs its operands in order, then computes from their values.
    const keys = Object.keys(e).filter((key) => !SKIP.has(key) && (Array.isArray(e[key]) || (e[key] && typeof e[key] === 'object' && e[key].k)));
    const operands = keys.flatMap((key) => (Array.isArray(e[key]) ? e[key] : [e[key]]));
    return this.bindAll(operands, out, (after, values) => {
      const copy = { ...e };
      let at = 0;
      for (const key of keys) {
        if (Array.isArray(e[key])) {
          copy[key] = values.slice(at, at + e[key].length);
          at += e[key].length;
        } else {
          copy[key] = values[at];
          at += 1;
        }
      }
      return this.make(after, copy);
    });
  }

  /** `k` of the lines after `e` and the value of `e`; a value that prints nothing is used as it is. */
  bind(e, out, k) {
    if (!this.prints(e)) return k(out, e);
    return this.open(this.io(e, out), e.type, k);
  }

  bindAll(list, out, k, done = []) {
    if (done.length === list.length) return k(out, done);
    return this.bind(list[done.length], out, (after, value) => this.bindAll(list, after, k, [...done, value]));
  }

  /** The value of `e` alone, for a theorem, which states facts of values. */
  value(e) {
    const pair = this.io(e, { k: 'outNil', type: OUTPUT });
    return this.open(pair, e.type, (_, value) => value);
  }

  pureProp(prop) {
    const map = (node) => (this.prints(node) ? this.value(node) : node);
    switch (prop.p) {
      case 'forall':
        return { ...prop, body: this.pureProp(prop.body) };
      case 'and':
      case 'or':
      case 'implies':
        return { ...prop, left: this.pureProp(prop.left), right: this.pureProp(prop.right) };
      case 'not':
        return { ...prop, arg: this.pureProp(prop.arg) };
      case 'bool':
        return { ...prop, expr: map(prop.expr) };
      default:
        return { ...prop, left: map(prop.left), right: map(prop.right) };
    }
  }

  /**
   * A main step whose value prints: the pair of its value and the lines it
   * prints, a step printing those lines, and the step itself on the value.
   */
  effect(effect) {
    if (effect.k === 'let' && this.prints(effect.value)) {
      const [steps, value] = this.run(effect.value);
      return [...steps, { ...effect, value }];
    }
    if (effect.k === 'print' && this.prints(effect.expr)) {
      const [steps, expr] = this.run(effect.expr);
      return [...steps, { ...effect, expr }];
    }
    if (effect.k === 'assert' && this.prints(effect.prop)) {
      const steps = [];
      const prop = this.hoist(effect.prop, steps, effect.span);
      return [...steps, { ...effect, prop }];
    }
    return [effect];
  }

  run(e) {
    const name = this.fresh('run');
    const pair = variable(name, this.pairType(e.type));
    return [[
      { k: 'let', name, value: this.io(e, { k: 'outNil', type: OUTPUT }), span: e.span },
      { k: 'output', expr: this.open(pair, e.type, (out) => out), span: e.span },
    ], this.open(pair, e.type, (_, value) => value)];
  }

  /** An assertion's operands that print run before it, in order, as steps of main. */
  hoist(prop, steps, where) {
    const lift = (node) => {
      if (!this.prints(node)) return node;
      const [run, value] = this.run(node);
      steps.push(...run);
      return value;
    };
    switch (prop.p) {
      case 'bool':
        return { ...prop, expr: lift(prop.expr) };
      case 'not':
        return { ...prop, arg: this.hoist(prop.arg, steps, where) };
      case 'and':
      case 'or':
      case 'implies':
      case 'forall':
        throw unsupported('output in a compound assertion', 'an operand that prints runs only when the assertion evaluates it; assert on a value computed before', where);
      default:
        return { ...prop, left: lift(prop.left), right: lift(prop.right) };
    }
  }
}
