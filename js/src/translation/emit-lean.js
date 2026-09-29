// Lean 4 emitter. Naturals are `Nat`, integers `Int`; machine integers are
// represented by `Nat`/`Int` with explicit range checks that `panic!` where
// Rust would panic. Recursion is structural and annotated as such, so the
// Lean kernel checks termination. Theorems are reconstructed from portable
// proof plans with Lean tactics and re-checked by the Lean kernel. A
// JavaScript Number is a `Float`; the kernel cannot compute with `Float`,
// so an assertion about Numbers is checked when `main` runs, as in the source.

import { unsupported } from './diagnostics.js';
import { propFunctions } from './proof.js';
import { fixedBounds, typeKey } from './types.js';
import { renameFunction, renameMain, renameTheorem } from './ir.js';
import { threadOutput } from './output.js';
import { EmitState, mutualGroups, numberDependence, opaqueDependence, orderDeclarations } from './emit-common.js';
import { LEAN_ROOT_NAMES } from './lean-root-names.js';

const KEYWORDS = new Set([
  'abbrev', 'at', 'attribute', 'axiom', 'by', 'calc', 'class', 'def', 'deriving', 'do', 'else', 'end', 'example',
  'export', 'extends', 'for', 'from', 'fun', 'have', 'if', 'import', 'in', 'inductive', 'instance', 'let', 'local',
  'match', 'mut', 'mutual', 'namespace', 'noncomputable', 'open', 'partial', 'private', 'protected', 'return',
  'section', 'set_option', 'show', 'structure', 'suffices', 'then', 'theorem', 'lemma', 'unsafe', 'universe', 'variable',
  'where', 'with', 'termination_by', 'decreasing_by', 'macro', 'syntax', 'notation', 'infix', 'infixl', 'infixr',
  'prefix', 'postfix', 'macro_rules', 'elab', 'opaque', 'omit', 'include', 'nomatch', 'nofun', 'try', 'catch',
  'finally', 'unless', 'break', 'continue', 'fun', 'Type', 'Prop', 'Sort', 'this', 'at', 'using', 'obtain',
  // Names the emitted code relies on, and names Lean derives inside a type's namespace.
  'Nat', 'Int', 'String', 'Bool', 'Unit', 'IO', 'true', 'false', 'main', 'toString', 'decide', 'panic', 'Float',
  'rec', 'recOn', 'casesOn', 'noConfusion', 'noConfusionType', 'below', 'brecOn', 'binductionOn', 'ibelow',
  'ctorIdx', 'toCtorIdx', 'sizeOf_spec', 'injEq', 'inj', 'induct', 'eq_def', 'mk',
]);

const HELPERS = {
  fixed: `/-- Machine-integer results: out of range is where Rust panics. -/
def ml_fixed (value lo hi : Int) (what : String) : Int :=
  if value < lo || value > hi then panic! s!"{what} overflowed" else value`,
  fixedNat: `def ml_fixed_nat (value : Int) (hi : Nat) (what : String) : Nat :=
  if value < 0 || value > Int.ofNat hi then panic! s!"{what} overflowed" else value.toNat`,
  toNatChecked: `def ml_to_nat_checked (value : Int) : Nat :=
  if value < 0 then panic! s!"{value} is not a natural number" else value.toNat`,
  divide: `/-- Division that aborts on a zero divisor, as JavaScript and Rust do. -/
def ml_nonzero (divisor : Int) : Int :=
  if divisor == 0 then panic! "division by zero" else divisor`,
  divideNat: `def ml_nonzero_nat (divisor : Nat) : Nat :=
  if divisor == 0 then panic! "division by zero" else divisor`,
  jsNumber: `/-- ECMAScript Number::toString: the shortest decimal that reads back as the
value, the nearer one and then the even one on a tie, laid out as JavaScript
prints it; exact Nat arithmetic on the bits. -/
def ml_js_layout (digits : List Char) (n : Int) : String :=
  let k : Int := digits.length
  let text := String.ofList
  let zeros (count : Int) := List.replicate count.toNat '0'
  if k ≤ n ∧ n ≤ 21 then text (digits ++ zeros (n - k))
  else if 0 < n ∧ n ≤ 21 then text (digits.take n.toNat ++ '.' :: digits.drop n.toNat)
  else if -6 < n ∧ n ≤ 0 then text ('0' :: '.' :: zeros (-n) ++ digits)
  else
    let e := n - 1
    let exp := (if e < 0 then "e-" else "e+") ++ toString e.natAbs
    if k == 1 then text digits ++ exp
    else text (digits.take 1 ++ '.' :: digits.drop 1) ++ exp

/-- \`c · 10^q\` compared with \`x · s / d\`. -/
def ml_js_cmp (c : Nat) (q : Int) (x s d : Nat) : Ordering :=
  if 0 ≤ q then compare (c * 10 ^ q.toNat * d) (x * s)
  else compare (c * d) (x * s * 10 ^ (-q).toNat)

def ml_js_shortest (m : Nat) (e : Int) (lowerCloser : Bool) : String :=
  -- The value is 4m and the rounding interval [lo, hi], in units of 2^(e-2) = s / d.
  let s := 2 ^ (e - 2).toNat
  let d := 2 ^ (2 - e).toNat
  let v := 4 * m
  let lo := if lowerCloser then v - 1 else v - 2
  let hi := v + 2
  let inclusive := m % 2 == 0
  let above (c : Nat) (q : Int) : Bool :=
    match ml_js_cmp c q lo s d with
    | .gt => true | .eq => inclusive | .lt => false
  let below (c : Nat) (q : Int) : Bool :=
    match ml_js_cmp c q hi s d with
    | .lt => true | .eq => inclusive | .gt => false
  -- floor (log10 value) from the bit length, then corrected exactly
  let bits : Int := (Nat.log2 m : Int) + 1 + e
  let guess : Int := Int.fdiv ((bits - 1) * 78913) 262144
  let e10 :=
    if ml_js_cmp 1 (guess + 1) v s d != .gt then guess + 1
    else if ml_js_cmp 1 guess v s d == .gt then guess - 1
    else guess
  let rec search (fuel : Nat) (p : Nat) : String :=
    match fuel with
    | 0 => ""
    | fuel + 1 =>
      let q : Int := e10 - p + 1
      let c :=
        if 0 ≤ q then v * s / (d * 10 ^ q.toNat)
        else v * s * 10 ^ (-q).toNat / d
      let lowOk := above c q ∧ below c q
      let highOk := above (c + 1) q ∧ below (c + 1) q
      if lowOk ∨ highOk then
        let pick :=
          if lowOk ∧ highOk then
            match ml_js_cmp (2 * c + 1) q (2 * v) s d with
            | .gt => c | .lt => c + 1 | .eq => if c % 2 == 0 then c else c + 1
          else if lowOk then c else c + 1
        let text := (toString pick).toList
        let n : Int := q + text.length
        ml_js_layout (text.reverse.dropWhile (· == '0')).reverse n
      else search fuel (p + 1)
  search 17 1

def ml_js_number (x : Float) : String :=
  let bits : Nat := x.toBits.toNat
  let negative := bits ≥ 2 ^ 63
  let exponent : Nat := (bits / 2 ^ 52) % 2048
  let fraction : Nat := bits % 2 ^ 52
  if exponent == 2047 then
    if fraction != 0 then "NaN" else if negative then "-Infinity" else "Infinity"
  else if exponent == 0 ∧ fraction == 0 then "0"
  else
    let body :=
      if exponent == 0 then ml_js_shortest fraction (-1074) false
      else ml_js_shortest (fraction + 2 ^ 52) ((exponent : Int) - 1075) (fraction == 0 ∧ exponent > 1)
    if negative then "-" ++ body else body`,
  jsConsole: `/-- What console.log prints: -0 as "-0", where String(-0) is "0". -/
def ml_js_console (x : Float) : String :=
  if x.toBits == 0x8000000000000000 then "-0" else ml_js_number x`,
  floatRem: `/-- ECMAScript \`%\` on binary64 (C fmod): the exact remainder, truncated, with
the sign of the dividend; computed on the bits, so it is exact. -/
def ml_float_rem (x y : Float) : Float :=
  let decompose (f : Float) : Nat × Int :=
    let b : Nat := f.toBits.toNat
    let exponent : Nat := (b / 2 ^ 52) % 2048
    let fraction : Nat := b % 2 ^ 52
    if exponent == 0 then (fraction, -1074) else (fraction + 2 ^ 52, (exponent : Int) - 1075)
  if x.isNaN || y.isNaN || x.isInf || y == 0 then 0 / 0
  else if y.isInf || x == 0 then x
  else
    let (mx, ex) := decompose x
    let (my, ey) := decompose y
    let e := min ex ey
    let r := (mx * 2 ^ (ex - e).toNat) % (my * 2 ^ (ey - e).toNat)
    let magnitude := Float.scaleB (Float.ofNat r) e
    if x.toBits.toNat ≥ 2 ^ 63 then -magnitude else magnitude`,
  arrayAt: `/-- The element at an index; a read outside the array, undefined in JavaScript, panics. -/
def ml_array_at {α : Type} [Inhabited α] (values : Array α) (index : Int) : α :=
  if h : 0 ≤ index ∧ index.toNat < values.size then values[index.toNat]'h.2
  else panic! "array index out of range"`,
  arrayAtFloat: `/-- The index a Number names: a non-negative integer, -0 included. -/
def ml_array_at_float {α : Type} [Inhabited α] (values : Array α) (index : Float) : α :=
  if index ≥ 0 && index.floor == index && index < 9007199254740992 then ml_array_at values (Int.ofNat index.toUInt64.toNat)
  else panic! "array index out of range"`,
  floatSame: `/-- SameValue, as Object.is and assert.strictEqual compare: NaN equals NaN,
and 0 differs from -0. -/
def ml_float_same (a b : Float) : Bool :=
  if a.isNaN then b.isNaN else a.toBits == b.toBits`,
};

function ident(name) {
  let result = name.replace(/[^A-Za-z0-9_'À-￿]/gu, '_');
  if (/^[0-9']/u.test(result)) result = `x${result}`;
  if (KEYWORDS.has(result)) result = `«${result}»`;
  return result;
}

export function emitLean(source) {
  const program = threadOutput(source);
  const state = new EmitState(program, 'Lean', ident, KEYWORDS, {
    ctorStyle: 'data',
    generated: (name) => [`${name}.eq_1`],
    rootReserved: LEAN_ROOT_NAMES,
  });
  return new LeanEmitter(program, state).file();
}

class LeanEmitter {
  constructor(program, state) {
    this.program = program;
    this.state = state;
    this.helpers = new Set();
    // Inside \`| p + 1 =>\` of a match on \`x\`, \`x\` is written \`p + 1\`: Lean's
    // structural recursion sees through the pattern but not the variable.
    this.successors = new Map();
    this.usesNumber = numberDependence(program);
    this.usesPartial = opaqueDependence(program);
  }

  file() {
    const blocks = [];
    let open = [];
    const moveTo = (path) => {
      let common = 0;
      while (common < open.length && common < path.length && open[common] === path[common]) common += 1;
      while (open.length > common) {
        blocks.push(`end ${this.state.moduleName(open)}`);
        open = open.slice(0, -1);
      }
      while (open.length < path.length) {
        open = [...open, path[open.length]];
        blocks.push(`namespace ${this.state.moduleName(open)}`);
      }
    };
    const groups = mutualGroups(this.program);
    for (const entry of orderDeclarations(this.program)) {
      moveTo(entry.modulePath);
      const group = groups.get(entry.fullName);
      if (group) {
        if (group[0] === entry) blocks.push(this.mutual(group));
      } else if (entry.k === 'data') blocks.push(this.data(entry));
      else if (entry.k === 'fn') blocks.push(this.fn(entry));
      else blocks.push(this.theorem(entry));
    }
    moveTo([]);
    const main = this.program.main ? this.main(this.program.main) : null;
    const text = [
      `-- Translated from ${this.program.sourceLanguage} by meta-language: portable core, Lean target.`,
      '-- Source binders are kept even where unused or named like a constructor, and proof hints are shared by every closing tactic.',
      'set_option linter.unusedVariables false',
      'set_option linter.unusedSimpArgs false',
      'set_option linter.constructorNameAsVariable false',
      '',
      ...['fixed', 'fixedNat', 'toNatChecked', 'divide', 'divideNat', 'jsNumber', 'jsConsole', 'floatRem', 'floatSame', 'arrayAt', 'arrayAtFloat'].filter((name) => this.helpers.has(name)).flatMap((name) => [HELPERS[name], '']),
      ...blocks.flatMap((block) => [block, '']),
      ...(main ? [main, ''] : []),
    ].join('\n');
    return {
      language: 'Lean',
      text,
      mappings: this.state.mappings,
      assumptions: this.state.assumptionList(),
      encodings: this.state.encodingList(),
      theorems: this.state.theorems,
      entry: main ? 'main' : null,
    };
  }

  type(type) {
    switch (type.kind) {
      case 'nat':
        return 'Nat';
      case 'int':
        return 'Int';
      case 'fixed':
        this.state.encode(`machine-integer:${typeKey(type)}`, `${typeKey(type)} values are ${type.signed ? 'Int' : 'Nat'} values; every operation checks the ${typeKey(type)} range and panics outside it, where Rust panics`);
        return type.signed ? 'Int' : 'Nat';
      case 'float':
        this.state.encode('floats', 'a JavaScript Number is a Lean Float, the same IEEE-754 binary64 with the same arithmetic; % is ml_float_rem, the exact truncated remainder, and ml_js_number prints a value as JavaScript does');
        return 'Float';
      case 'bool':
        return 'Bool';
      case 'string':
        return 'String';
      case 'unit':
        return 'Unit';
      case 'output':
        return 'List String';
      case 'data':
        return this.state.ref(type.name);
      case 'array':
        this.state.encode('arrays', 'a JavaScript array, which the portable core never mutates, is a Lean Array; a read outside it panics');
        return `(Array ${this.type(type.element)})`;
      default:
        throw new Error(`no Lean type for ${type.kind}`);
    }
  }

  data(entry) {
    const name = this.state.localName(entry.fullName);
    this.state.map(entry, name);
    const ctors = entry.ctors.map((ctor) => {
      const fields = ctor.fields.map((field, index) => ` (${ident(field.name ?? `field${index}`)} : ${this.type(field.type)})`).join('');
      return `  | ${this.state.ctorLocal(entry.fullName, ctor.name)}${fields} : ${name}`;
    });
    // A pair of printed lines and a value is only built and taken apart; its value may be a Float, which has no DecidableEq.
    // Lean derives no DecidableEq for a Float, nor through an Array of data; the portable core never compares such values.
    const deriving = entry.output || this.undecidable(entry.fullName) ? (entry.output ? 'Inhabited' : 'Repr, Inhabited') : 'Repr, DecidableEq, Inhabited';
    return [`inductive ${name} where`, ...ctors, `  deriving ${deriving}`].join('\n');
  }

  undecidable(name, seen = new Set()) {
    if (seen.has(name)) return false;
    seen.add(name);
    const bad = (type, nested) => type.kind === 'float' || (type.kind === 'array' && bad(type.element, true))
      || (type.kind === 'data' && (nested || this.undecidable(type.name, seen)));
    return this.program.declarations.get(name).ctors.some((ctor) => ctor.fields.some((field) => bad(field.type, false)));
  }

  definition(entry) {
    const { params, body } = renameFunction(entry, ident, this.state.localReserved());
    const name = this.state.localName(entry.fullName);
    this.state.map(entry, name);
    const binders = params.map((param) => ` (${param.name} : ${this.type(param.type)})`).join('');
    return `def ${name}${binders} : ${this.type(entry.ret)} :=\n  ${this.expr(body, 1)}`;
  }

  fn(entry) {
    const text = this.definition(entry);
    if (!entry.recursive) return text;
    if (entry.decreasing === null) {
      this.state.encode('general-recursion', 'recursion without a structurally decreasing argument is a Lean partial def: it runs as the source does but its equations are opaque to proofs');
      return `partial ${text}`;
    }
    // Lean tries structural recursion on each argument before well-founded
    // recursion, so the structurally decreasing argument needs no annotation.
    return text;
  }

  /** Mutually recursive functions are one `mutual` block of partial defs. */
  mutual(group) {
    this.state.encode('mutual-recursion', 'mutually recursive functions are a Lean mutual block of partial defs: they run as the source does but their equations are opaque to proofs');
    return ['mutual', ...group.map((entry) => `partial ${this.definition(entry)}`), 'end'].join('\n\n');
  }

  theorem(entry) {
    const { binders, prop, plan } = renameTheorem(entry, ident, this.state.localReserved());
    const name = this.state.localName(entry.fullName);
    this.state.map(entry, name);
    const functions = [...new Set([...propFunctions(prop), ...collectUnfold(plan)])];
    const statement = `theorem ${name}${binders.map((binder) => ` (${binder.name} : ${this.type(binder.type)})`).join('')} : ${this.prop(prop)} := by`;
    this.state.theorem(entry, name, { closedGoal: binders.length === 0 });
    return `${statement}\n${this.plan(plan, functions, 1)}`;
  }

  plan(plan, functions, depth) {
    const pad = '  '.repeat(depth);
    if (plan.k === 'close') return `${pad}${this.closer(plan.hints, functions, depth)}`;
    // `case tag names =>` names each constructor's fields, then its induction hypotheses.
    const lines = [`${pad}${plan.k === 'induction' ? 'induction' : 'cases'} ${plan.variable}`];
    for (const kase of plan.cases) {
      const ctor = plan.type.kind === 'nat' ? kase.ctor : this.state.ctorLocal(plan.type.name, kase.ctor);
      const names = [...kase.fields, ...(plan.k === 'induction' ? kase.ihs : [])];
      lines.push(`${pad}case ${ctor}${names.map((item) => ` ${item}`).join('')} =>`);
      lines.push(this.plan(kase.plan, functions, depth + 1));
    }
    return lines.join('\n');
  }

  closer(hints, functions, depth) {
    const rules = [
      ...[...new Set([...functions, ...hints.unfold])].map((fn) => this.state.ref(fn)),
      ...hints.lemmas.map((lemma) => this.state.ref(lemma)),
      ...hints.hyps,
    ];
    const ring = ['Nat.mul_add', 'Nat.add_mul', 'Int.mul_add', 'Int.add_mul'];
    const list = rules.join(', ');
    const alternatives = [
      'rfl',
      'decide',
      ...(rules.length ? [
        `simp only [${list}] <;> done`,
        `simp_all only [${list}] <;> omega`,
        `simp [${list}] <;> done`,
        `simp_all [${list}] <;> omega`,
        `simp_all [${[...rules, ...ring].join(', ')}] <;> omega`,
      ] : ['omega', `simp_all [${ring.join(', ')}] <;> omega`]),
    ];
    // Each `try` closes the goal or leaves it untouched, and once one closes it
    // the rest find no goal, so the lines act as ordered alternatives.
    return alternatives.map((alternative) => `try ${alternative}`).join(`\n${'  '.repeat(depth)}`);
  }

  prop(prop) {
    switch (prop.p) {
      case 'forall':
        return `(∀ ${prop.binders.map((binder) => `(${binder.name} : ${this.type(binder.type)})`).join(' ')}, ${this.prop(prop.body)})`;
      case 'and':
        return `(${this.prop(prop.left)} ∧ ${this.prop(prop.right)})`;
      case 'or':
        return `(${this.prop(prop.left)} ∨ ${this.prop(prop.right)})`;
      case 'implies':
        return `(${this.prop(prop.left)} → ${this.prop(prop.right)})`;
      case 'not':
        return `(¬ ${this.prop(prop.arg)})`;
      case 'bool':
        return `(${this.expr(prop.expr, 0)} = true)`;
      default: {
        const operator = { eq: '=', ne: '≠', lt: '<', le: '≤', gt: '>', ge: '≥' }[prop.p];
        return `(${this.expr(prop.left, 0)} ${operator} ${this.expr(prop.right, 0)})`;
      }
    }
  }

  expr(e, depth) {
    switch (e.k) {
      case 'lit':
        return this.literal(e);
      case 'unit':
        return '()';
      case 'outNil':
        return '([] : List String)';
      case 'outCons':
        return `(${this.expr(e.head, depth)} :: ${this.expr(e.tail, depth)})`;
      case 'var':
        return this.successors.get(e.name) ?? e.name;
      case 'call': {
        const head = this.state.ref(e.fn);
        return e.args.length ? `(${head} ${e.args.map((arg) => this.expr(arg, depth)).join(' ')})` : head;
      }
      case 'ctor': {
        const head = this.state.ctorRef(e.data, e.ctor);
        return e.args.length ? `(${head} ${e.args.map((arg) => this.expr(arg, depth)).join(' ')})` : head;
      }
      case 'unary':
        if (e.op === 'not') return `(!${this.expr(e.arg, depth)})`;
        return this.checked(`(-${this.expr(e.arg, depth)})`, e.type, 'negation');
      case 'binary':
        return this.binary(e, depth);
      case 'if':
        return `(if ${this.expr(e.cond, depth)} then ${this.expr(e.then, depth)} else ${this.expr(e.else, depth)})`;
      case 'let':
        return `(let ${e.name} := ${this.expr(e.value, depth)};\n${pad(depth + 1)}${this.expr(e.body, depth + 1)})`;
      case 'match':
        return this.match(e, depth);
      case 'toString':
        return e.arg.type.kind === 'string' ? this.expr(e.arg, depth) : this.toText(e.arg, depth, e.console);
      case 'cast':
        return this.cast(e, depth);
      case 'abort':
        this.state.abortToTotal(e.message);
        return `(panic! ${JSON.stringify(e.message)} : ${this.type(e.type)})`;
      case 'array':
        return `(#[${e.items.map((item) => this.expr(item, depth)).join(', ')}] : ${this.type(e.type)})`;
      case 'append':
        return `(${this.expr(e.left, depth)} ++ ${this.expr(e.right, depth)})`;
      case 'index': {
        this.state.arrayRead();
        this.state.abortToTotal('array index out of range');
        this.helpers.add('arrayAt');
        const values = this.expr(e.array, depth);
        const index = this.expr(e.index, depth);
        if (e.index.type.kind === 'float') {
          this.helpers.add('arrayAtFloat');
          return `(ml_array_at_float ${values} ${index})`;
        }
        return `(ml_array_at ${values} ${e.index.type.kind === 'nat' ? `(Int.ofNat ${index})` : index})`;
      }
      case 'length':
        return `(${e.type.kind === 'float' ? 'Float.ofNat' : 'Int.ofNat'} ${this.expr(e.array, depth)}.size)`;
      default:
        throw new Error(`no Lean expression for ${e.k}`);
    }
  }

  literal(e) {
    switch (e.type.kind) {
      case 'nat':
        return `(${e.value} : Nat)`;
      case 'int':
        return `(${e.value} : Int)`;
      case 'fixed':
        return `(${e.value} : ${e.type.signed ? 'Int' : 'Nat'})`;
      case 'float':
        return leanFloat(e.value, this.type(e.type));
      case 'bool':
        return String(e.value);
      case 'string':
        return JSON.stringify(String(e.value));
      default:
        throw new Error(`no Lean literal for ${e.type.kind}`);
    }
  }

  toText(arg, depth, console = false) {
    if (arg.type.kind === 'data' || arg.type.kind === 'unit') {
      throw unsupported('output of structured values', `a ${arg.type.kind} value has no portable textual form`, arg.span);
    }
    if (arg.type.kind === 'float') {
      this.helpers.add('jsNumber');
      if (console) this.helpers.add('jsConsole');
      return `(${console ? 'ml_js_console' : 'ml_js_number'} ${this.expr(arg, depth)})`;
    }
    return `(toString ${this.expr(arg, depth)})`;
  }

  /** Range-checks a machine-integer result computed in Int. */
  checked(text, type, what) {
    if (type.kind !== 'fixed') return text;
    this.state.abortToTotal(`${typeKey(type)} ${what} overflow`);
    const { min, max } = fixedBounds(type);
    if (type.signed) {
      this.helpers.add('fixed');
      return `(ml_fixed ${text} (${min}) ${max} "${typeKey(type)} ${what}")`;
    }
    this.helpers.add('fixedNat');
    return `(ml_fixed_nat ${text} ${max} "${typeKey(type)} ${what}")`;
  }

  binary(e, depth) {
    const left = this.expr(e.left, depth);
    const right = this.expr(e.right, depth);
    switch (e.op) {
      case 'and':
        return `(${left} && ${right})`;
      case 'or':
        return `(${left} || ${right})`;
      case 'concat':
        return `(${left} ++ ${right})`;
      case 'eq':
        return `(${left} == ${right})`;
      case 'ne':
        return `(${left} != ${right})`;
      case 'lt':
      case 'le':
      case 'gt':
      case 'ge':
        return `(decide (${left} ${{ lt: '<', le: '≤', gt: '>', ge: '≥' }[e.op]} ${right}))`;
      default:
        return this.arithmetic(e, left, right);
    }
  }

  arithmetic(e, left, right) {
    if (e.semantics === 'ieee') {
      if (e.op !== 'rem') return `(${left} ${{ add: '+', sub: '-', mul: '*', div: '/' }[e.op]} ${right})`;
      this.helpers.add('floatRem');
      return `(ml_float_rem ${left} ${right})`;
    }
    const fixed = e.type.kind === 'fixed';
    // Machine-integer operations run in Int and are range-checked afterwards.
    const wide = (text, operand) => (operand.type.kind === 'fixed' && !operand.type.signed ? `(Int.ofNat ${text})` : text);
    const [a, b] = fixed ? [wide(left, e.left), wide(right, e.right)] : [left, right];
    switch (e.op) {
      case 'add':
        return this.checked(`(${a} + ${b})`, e.type, 'addition');
      case 'mul':
        return this.checked(`(${a} * ${b})`, e.type, 'multiplication');
      case 'sub':
        return this.checked(`(${a} - ${b})`, e.type, 'subtraction');
      case 'div':
      case 'rem': {
        const division = e.op === 'div';
        let divisor = b;
        if (e.byZero === 'abort') {
          this.state.abortToTotal('division by zero');
          const natural = !fixed && e.domain.kind === 'nat';
          this.helpers.add(natural ? 'divideNat' : 'divide');
          divisor = `(${natural ? 'ml_nonzero_nat' : 'ml_nonzero'} ${b})`;
        }
        let text;
        if (!fixed && e.domain.kind === 'nat') text = `(${a} ${division ? '/' : '%'} ${divisor})`;
        else if (e.rounding === 'trunc') text = `(Int.${division ? 'tdiv' : 'tmod'} ${a} ${divisor})`;
        else if (e.rounding === 'floor') text = `(Int.${division ? 'fdiv' : 'fmod'} ${a} ${divisor})`;
        else text = `(${a} ${division ? '/' : '%'} ${divisor})`;
        return this.checked(text, e.type, division ? 'division' : 'remainder');
      }
      default:
        throw new Error(`no Lean operator ${e.op}`);
    }
  }

  match(e, depth) {
    const subject = this.expr(e.scrutinee, depth);
    const arms = e.cases.map((kase) => {
      const { pattern } = kase;
      let text;
      if (pattern.k === 'natZero') text = '0';
      else if (pattern.k === 'natSucc') text = `${pattern.name} + 1`;
      else if (pattern.k === 'wild') text = '_';
      else if (pattern.k === 'bind') text = pattern.name;
      else {
        const head = this.state.ctorRef(pattern.data, pattern.ctor);
        text = [head, ...pattern.binds.map((bind) => bind ?? '_')].join(' ');
      }
      const saved = this.successors;
      if (pattern.k === 'natSucc' && e.scrutinee.k === 'var') {
        this.successors = new Map(saved).set(e.scrutinee.name, `(${pattern.name} + 1)`);
      }
      const body = this.expr(kase.body, depth + 2);
      this.successors = saved;
      return `${pad(depth + 1)}| ${text} => ${body}`;
    });
    return `(match ${subject} with\n${arms.join('\n')})`;
  }

  cast(e, depth) {
    const arg = this.expr(e.arg, depth);
    const naturalFrom = e.from.kind === 'nat' || (e.from.kind === 'fixed' && !e.from.signed);
    const naturalTo = e.to.kind === 'nat' || (e.to.kind === 'fixed' && !e.to.signed);
    if (naturalFrom === naturalTo) return arg;
    if (naturalFrom) return `(Int.ofNat ${arg})`;
    if (e.flavor === 'clamp') return `(Int.toNat ${arg})`;
    this.state.checkedToTotal('conversion to a natural');
    this.helpers.add('toNatChecked');
    return `(ml_to_nat_checked ${arg})`;
  }

  main(main) {
    const { effects } = renameMain(main, ident, this.state.localReserved());
    const lines = [];
    const theorems = [];
    let assertion = 0;
    effects.forEach((effect, index) => {
      if (effect.k === 'print') lines.push(`  IO.println ${this.expr(effect.expr, 1)}`);
      else if (effect.k === 'output') lines.push(`  for ml_line in ${this.expr(effect.expr, 1)}.reverse do IO.println ml_line`);
      else if (effect.k === 'let') lines.push(`  let ${effect.name} := ${this.expr(effect.value, 1)}`);
      else if (this.usesNumber(effect.prop) || this.onPartial(effect.prop, effects.slice(0, index))) {
        // The kernel cannot evaluate Float or unfold a partial def, so the assertion runs where the source's does.
        assertion += 1;
        lines.push(`  if !${this.check(effect.prop)} then throw (IO.userError "assertion ${assertion} failed")`);
        this.state.assertionTheorem(`assertion ${assertion}`, effect, { discharge: 'runtime-assertion' });
      } else {
        assertion += 1;
        const lets = effects.slice(0, index).filter((item) => item.k === 'let')
          .map((item) => `let ${item.name} := ${this.expr(item.value, 1)}; `).join('');
        const name = `ml_assertion_${assertion}`;
        theorems.push(`theorem ${name} : ${lets}${this.prop(effect.prop)} := by\n  try rfl\n  try decide`);
        this.state.assertionTheorem(name, effect);
      }
    });
    if (main.sequentialAsync) this.state.encode('sequential-async', 'an async function is the function its body computes and await is its call: every call of one is awaited where it is made, so nothing runs concurrently and the output is the same, in the same order');
    this.state.encode('program-output', 'main prints the lines the source program prints, in order, with IO.println');
    if (this.program.outputThreaded) this.state.encode('output-threading', 'a function that prints, directly or through a function it calls, takes the lines printed before it and returns them, with its own in front, paired with its value in a generated ml_io data type; main prints the lines of each step in the order they were printed');
    return [...theorems, `def main : IO Unit := do\n${lines.length ? lines.join('\n') : '  pure ()'}`].join('\n\n');
  }

  /** True when the proposition, or a main binding it reads, calls a partial def. */
  onPartial(prop, before) {
    const reads = new Set();
    const collect = (node) => {
      if (!node || typeof node !== 'object') return;
      if (Array.isArray(node)) node.forEach(collect);
      else {
        if (node.k === 'var') reads.add(node.name);
        Object.entries(node).forEach(([key, value]) => key !== 'type' && collect(value));
      }
    };
    collect(prop);
    let partial = this.usesPartial(prop);
    for (const effect of [...before].reverse()) {
      if (partial) break;
      if (effect.k === 'let' && reads.has(effect.name)) {
        partial = this.usesPartial(effect.value);
        collect(effect.value);
      }
    }
    return partial;
  }

  /** A proposition as a Bool computed at run time. */
  check(prop) {
    switch (prop.p) {
      case 'and':
        return `(${this.check(prop.left)} && ${this.check(prop.right)})`;
      case 'or':
        return `(${this.check(prop.left)} || ${this.check(prop.right)})`;
      case 'implies':
        return `(!${this.check(prop.left)} || ${this.check(prop.right)})`;
      case 'not':
        return `(!${this.check(prop.arg)})`;
      case 'bool':
        return this.expr(prop.expr, 1);
      case 'forall':
        throw new Error('no run-time check for a quantified proposition');
      default: {
        const left = this.expr(prop.left, 1);
        const right = this.expr(prop.right, 1);
        if (prop.sameValue) {
          this.helpers.add('floatSame');
          return `(${prop.p === 'eq' ? '' : '!'}ml_float_same ${left} ${right})`;
        }
        if (prop.p === 'eq' || prop.p === 'ne') return `(${left} ${prop.p === 'eq' ? '==' : '!='} ${right})`;
        return `(decide (${left} ${{ lt: '<', le: '≤', gt: '>', ge: '≥' }[prop.p]} ${right}))`;
      }
    }
  }
}

/** A Number's canonical JavaScript text as a Float term; Lean reads decimal literals correctly rounded. */
function leanFloat(value, type) {
  const special = { NaN: '0.0 / 0.0', Infinity: '1.0 / 0.0', '-Infinity': '-1.0 / 0.0' }[value];
  return `(${special ?? value.replace('e+', 'e')} : ${type})`;
}

function collectUnfold(plan) {
  if (plan.k === 'close') return plan.hints.unfold;
  return plan.cases.flatMap((kase) => collectUnfold(kase.plan));
}

function pad(depth) {
  return '  '.repeat(depth);
}
