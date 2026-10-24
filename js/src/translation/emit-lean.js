import {UNICODE_CASE_LEAN_PRELUDE} from './unicode-case-formal-runtime.js';
import {requireFormalUnitStrings,prepareFormalUnitStrings,readFormalUnitStringExpression,formalUnitLiteral,formalScalarUnits,formalUnitDisplay,formalUnitLength,formalUnitTruth} from './formal-unit-strings.js';
import {readFormalStringUnitType,renderFormalStringUnitComparison} from './formal-string-unit-rules.js';
import {hasSourceExceptionOutput,hasSourceExceptionPayload,sourceExceptionPatternSuffix} from './source-exception-observation.js';
import {readSourceExceptionObservationStatement} from './source-exception-rules.js';
import {acceptLinkedJavaScriptStringConstruction,renderJavaScriptStringConstruction} from './string-construction-rules.js';
import {UTF16_TEXT_LEAN_PRELUDE} from './utf16-text-formal-runtime.js';
import { renderStringLengthExpression, readStringMapRefusal, renderStringTestExpression, readStringTestHelper, readStringTestSupport } from './frontend-rules.js';
import {readStringOrderRefusal} from './string-order-rules.js';
import { readBigIntegerBitwiseArity, renderBigIntegerBitwiseExpression } from './big-integer-bitwise-rules.js';
import {acceptLinkedJavaScriptNumberConversion,renderJavaScriptNumberScalarConversion} from './string-number-rules.js';
import {STRING_NUMBER_LEAN_PRELUDE} from './string-number-formal-runtime.js';
import {BIG_INTEGER_CONVERSION_LEAN_PRELUDE} from './big-integer-conversion-formal-runtime.js';
import {acceptLinkedJavaScriptBigIntegerResult,readJavaScriptBigIntegerResultOperation,renderJavaScriptBigIntegerResultExpression,renderJavaScriptBigIntegerDivision,renderJavaScriptBigIntegerPower,renderJavaScriptNumberPower,renderJavaScriptBigIntegerComparison} from './big-integer-conversion-rules.js';
import { readRustPrimitiveBitwiseArity } from './rust-primitive-bitwise-rules.js';
import { emitRustPrimitiveBitwise } from './rust-primitive-bitwise.js';
import { acceptLinkedRustCheckedShift } from './rust-shift-rules.js';
import { emitRustCheckedShift } from './rust-shifts.js';
import { renderWrappingFixedIntegerCast, readFixedIntegerCastStatement } from './fixed-integer-cast-rules.js';
import { renderCollectionSizeExpression, renderCollectionReadExpression } from './collection-access-rules.js';
// Lean 4 emitter. Naturals are `Nat`, integers `Int`; machine integers are
// represented by `Nat`/`Int` with explicit range checks that `panic!` where
// Rust would panic. Recursion is structural and annotated as such, so the
// Lean kernel checks termination. Theorems are reconstructed from portable
// proof plans with Lean tactics and re-checked by the Lean kernel. A
// JavaScript Number is a `Float`; the kernel cannot compute with `Float`,
// so an assertion about Numbers is checked when `main` runs, as in the source.

import { typeError, unsupported } from './diagnostics.js';
import { renderCollectionConstruction, renderCollectionConstructionType } from './collection-construction-rules.js';
import { renderOptionalAbsentValue, renderOptionalCoalescing, renderOptionalConsoleText, renderOptionalFlattening, renderOptionalMatch, renderOptionalNumericPresent, renderOptionalPresentValue, renderOptionalPrimitiveTruth, renderOptionalProjection, renderOptionalStrictEquality, renderOptionalTypeQueryPresent, renderOptionalUnwrapping, requireOptionalNumericPredicate } from './optional-value-rules.js';
import { renderCollectionIteration, renderOptionalArrayRead, renderStringPointIteration } from './array-value-rules.js';
import { renderArrayFromMapping } from './array-from-rules.js';
import { readPrimitiveMethodRenderingRefusal } from './primitive-method-rules.js';
import { renderRecordProductField, renderRecordType, renderRecordValue } from './record-emission-rules.js';
import { readUnionTargetRefusal } from './type-syntax-rules.js';
import { renderCallableApplication, renderCallableFunctionReference, renderCallableFunctionType, renderCallableLambda } from './callable-source-rules.js';
import { acceptProgramTupleProductModel } from '../typed-program-link-rules.js';
import { propFunctions } from './proof.js';
import { renameFunction, renameMain, renameTheorem } from './ir.js';
import { threadOutput } from './output.js';
import { EmitState, mutualGroups, numberDependence, opaqueDependence, orderDeclarations, typeContains, unthreaded } from './emit-common.js';
import { LEAN_ROOT_NAMES } from './lean-root-names.js';
import { renderAbsentNumberExpression } from './primitive-value-rules.js';

const KEYWORDS = new Set([
  'MLStringNumber',
  'MLBigIntegerConversion','MLUtf16Text','MLUnicodeCase',
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
  stringIncludes: readStringTestSupport('Lean', 'includes'),
  arrayAt: `/-- The element at an index; a read outside the array, undefined in JavaScript, panics. -/
def ml_array_at {α : Type} [Inhabited α] (values : Array α) (index : Int) : α :=
  if h : 0 ≤ index ∧ index.toNat < values.size then values[index.toNat]'h.2
  else panic! "array index out of range"`,
  arrayAtFloat: `/-- The index a Number names: a non-negative integer, -0 included. -/
def ml_array_at_float {α : Type} [Inhabited α] (values : Array α) (index : Float) : α :=
  if index ≥ 0 && index.floor == index && index < 9007199254740992 then ml_array_at values (Int.ofNat index.toUInt64.toNat)
  else panic! "array index out of range"`,
  math: `/-- Math.trunc: towards zero, a zero result keeping the sign of the argument. -/
def ml_trunc (x : Float) : Float := if x < 0 then x.ceil else x.floor
/-- Math.round: the nearest integer, the one towards +Infinity on a tie, and -0 from -0.5 up to 0. -/
def ml_round (x : Float) : Float :=
  if x.isNaN || x.isInf || x == 0 then x
  else if x > 0 ∧ x < 0.5 then 0
  else if x < 0 ∧ x ≥ -0.5 then -0.0
  else if x - x.floor ≥ 0.5 then x.floor + 1 else x.floor
/-- Math.sign: 1 or -1, and -0, 0 and NaN as they are. -/
def ml_sign (x : Float) : Float := if x > 0 then 1 else if x < 0 then -1 else x
/-- Math.max of two Numbers: NaN when either is, and 0 above -0. -/
def ml_max (a b : Float) : Float :=
  if a > b then a else if b > a then b else if a == b then (if 1 / a < 0 then b else a) else 0.0 / 0.0
/-- Math.min of two Numbers: NaN when either is, and -0 below 0. -/
def ml_min (a b : Float) : Float :=
  if a < b then a else if b < a then b else if a == b then (if 1 / a < 0 then a else b) else 0.0 / 0.0
def ml_is_integer (x : Float) : Bool := x.isFinite && ml_trunc x == x
def ml_is_safe_integer (x : Float) : Bool := ml_is_integer x && decide (x.abs ≤ 9007199254740991)`,
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
  const program = threadOutput(source, 'Lean');
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
    this.target='Lean';
    this.utf16Strings=requireFormalUnitStrings(program);
    this.state = state;
    this.helpers = new Set();
    // Inside \`| p + 1 =>\` of a match on \`x\`, \`x\` is written \`p + 1\`: Lean's
    // structural recursion sees through the pattern but not the variable.
    this.successors = new Map();
    this.usesNumber = numberDependence(program);
    this.usesPartial = opaqueDependence(program);
  }

  file() {
    prepareFormalUnitStrings(this);
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
      ...['jsNumber', 'jsConsole', 'floatRem', 'floatSame', 'math', 'arrayAt', 'arrayAtFloat', 'stringIncludes'].filter((name) => this.helpers.has(name)).flatMap((name) => [HELPERS[name], '']),
      ...(this.helpers.has('stringNumber')?[STRING_NUMBER_LEAN_PRELUDE,'']:[]),
      ...(this.helpers.has('bigIntegerConversion')?[BIG_INTEGER_CONVERSION_LEAN_PRELUDE,'']:[]),
      ...(this.helpers.has('utf16Text')?[UTF16_TEXT_LEAN_PRELUDE,'']:[]),
      ...(this.helpers.has('unicodeCase')?[UNICODE_CASE_LEAN_PRELUDE,'']:[]),
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
    if (type.reference || type.kind === 'cell') throw unsupported('mutable structural value', 'shared reference state needs an explicit target state model');
    switch (type.kind) {
      case 'nat':
        return 'Nat';
      case 'int':
        return 'Int';
      case 'fixed':
        this.state.machineInteger(type, type.signed ? 'Int' : 'Nat');
        return type.signed ? 'Int' : 'Nat';
      case 'float':
        this.state.encode('floats', 'a JavaScript Number is a Lean Float, the same IEEE-754 binary64 with the same arithmetic; % is ml_float_rem, the exact truncated remainder, and ml_js_number prints a value as JavaScript does');
        return 'Float';
      case 'bool':
        return 'Bool';
      case 'string':
        return this.utf16Strings?readFormalStringUnitType('Lean'):'String';
      case 'unit':
        return 'Unit';
      case 'output':
        return this.utf16Strings?'List (Array Nat)': 'List String';
      case 'data':
        return this.state.ref(type.name);
      case 'array':
        this.state.encode('arrays', 'a JavaScript array, which the portable core never mutates, is a Lean Array; a read outside it panics');
        return `(Array ${this.type(type.element)})`;
      case 'option':
        return type.absence === 'both' ? `(Option (Option ${this.type(type.element)}))` : `(Option ${this.type(type.element)})`;
      case 'tuple':
        return type.elements.length ? `(${type.elements.map((element) => this.type(element)).join(' × ')})` : 'Unit';
      case 'record':
        return renderRecordType('Lean', type.fields.map(field => this.type(field.type)));
      case 'function':
        return renderCallableFunctionType('Lean', type.params.map(param => this.type(param)), this.type(type.ret));
      case 'set':
        return renderCollectionConstructionType('Lean', type.kind, this.type(type.element), '');
      case 'map':
        return renderCollectionConstructionType('Lean', type.kind, this.type(type.key), this.type(type.value));
      case 'union':
        throw typeError(readUnionTargetRefusal('Lean'));
      default:
        throw new Error(`no Lean type for ${type.kind}`);
    }
  }

  data(entry) {
    if (typeContains(this.program, { kind: 'data', name: entry.fullName }, 'function')) throw unsupported('callable data field', 'callable fields need their own representation and equality encoding before deriving data instances', entry.span);
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
    if (typeContains(this.program, { kind: 'data', name }, 'float')) return true;
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
    const units=readFormalUnitStringExpression(this,e,depth);
    if(units!==null)return units;
    if (e.k === 'ctor' && e.frozen && this.program.sourceLanguage === 'JavaScript') throw unsupported('explicit data object freeze', 'this target needs a JavaScript property descriptor model to retain explicit constructor freezing', e.span);
    if (e.k === 'tuple' && Object.hasOwn(e, 'productModel')) {
      if (!acceptProgramTupleProductModel(e.productModel, e.frozen)) throw unsupported('immutable tuple product model', 'the tuple model must be immutable value product with explicit frozen evidence', e.span);
      const pending = [e.type];
      while (pending.length) {
        const type = pending.pop();
        if (type.kind !== 'tuple') continue;
        if (type.elements.length < 2) throw unsupported('immutable tuple arity', 'this target needs a nominal product representation to distinguish unit and single-element tuple arities', e.span);
        pending.push(...type.elements);
      }
    }
    if (['cellAllocate', 'cellRead', 'cellWrite', 'referenceAllocate', 'referenceRead', 'referenceWrite'].includes(e.k)) throw unsupported('mutable structural value', 'shared reference state needs an explicit target state model', e.span);
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
      case 'functionRef':
        return renderCallableFunctionReference('Lean', this.state.ref(e.fn));
      case 'apply':
        return renderCallableApplication('Lean', this.expr(e.fn, depth), e.args.map(arg => this.expr(arg, depth)));
      case 'lambda':
        return renderCallableLambda('Lean', e.params.map(param => param.name), e.params.map(param => this.type(param.type)), '', this.expr(e.body, depth), []);
      case 'record':
        return renderRecordValue('Lean', [], e.entries.map(entry => this.expr(entry.value, depth)));
      case 'recordField':
        return renderRecordProductField('Lean', this.expr(e.object, depth), e.index, e.object.type.fields.length);
      case 'tupleIndex': {
        const count = e.object.type.kind === 'record' ? e.object.type.fields.length : e.object.type.elements.length;
        return count === 1 ? this.expr(e.object, depth) : `(${this.expr(e.object, depth)})${'.2'.repeat(e.index)}${e.index < count - 1 ? '.1' : ''}`;
      }
      case 'tuple':
        return e.items.length ? `(${e.items.map((item) => this.expr(item, depth)).join(', ')})` : '()';
      case 'none':
        return renderOptionalAbsentValue('Lean', e.type.absence, e.absence, '');
      case 'some':
        return renderOptionalPresentValue('Lean', e.type.absence, this.expr(e.value, depth));
      case 'nullishNumber': {
        const element = e.arg.type.element;
        const present = renderOptionalNumericPresent('Lean', element.kind, 'arithmetic');
        return this.optionalMatch(e.arg, 'ml_present', present, renderAbsentNumberExpression('Lean', 'arithmetic', 'null'), renderAbsentNumberExpression('Lean', 'arithmetic', 'undefined'), depth);
      }
      case 'optionalIndexNumber':
        return this.optionalMatch(e.arg, 'ml_present', renderOptionalNumericPresent('Lean', e.arg.type.element.kind, 'array-index'), renderAbsentNumberExpression('Lean', 'array-index', 'null'), renderAbsentNumberExpression('Lean', 'array-index', 'undefined'), depth);
      case 'truthy':
        return this.truthy(e.arg, depth);
      case 'strictEqual':
        return this.strictEqual(e, depth);
      case 'optionalText': {
        const element = e.arg.type.element;
        let present = element.kind === 'string' ? 'ml_present' : element.kind === 'unit' ? '"undefined"' : this.toText({ k: 'var', name: 'ml_present', type: element }, depth, e.console);
        present = renderOptionalConsoleText('Lean', present, ['int', 'nat', 'fixed'].includes(element.kind), e.console, false);
        return this.optionalMatch(e.arg, 'ml_present', present, '"null"', '"undefined"', depth);
      }
      case 'optionalTypeOf':
        return this.optionalMatch(e.arg, 'ml_present', renderOptionalTypeQueryPresent('Lean', JSON.stringify(e.present ?? 'undefined')), '"object"', '"undefined"', depth);
      case 'unwrapOption':
        return renderOptionalUnwrapping('Lean', e.arg.type.absence, this.expr(e.arg, depth), '', '');
      case 'optionalPredicate': {
        const present = requireOptionalNumericPredicate(e.arg.type.element.kind) ? this.math({ k: 'math', op: e.op, args: [{ k: 'var', name: 'ml_present', type: { kind: 'float' } }], type: { kind: 'bool' } }, depth) : 'false';
        return this.optionalMatch(e.arg, 'ml_present', present, 'false', 'false', depth);
      }
      case 'flattenOption': {
        return renderOptionalFlattening('Lean', e.value.type.absence, e.value.type.element.absence, e.type.absence, this.expr(e.value, depth));
      }
      case 'coalesce':
        return renderOptionalCoalescing('Lean', e.left.type.absence, this.expr(e.left, depth), '', this.expr(e.right, depth));
      case 'optionalField': {
        const count = e.object.type.element.fields.length;
        const field = count === 1 ? 'ml_record' : `ml_record${'.2'.repeat(e.index)}${e.index < count - 1 ? '.1' : ''}`;
        return renderOptionalProjection('Lean', e.object.type.absence, this.expr(e.object, depth), '', 'ml_record', field, '');
      }
      case 'optionalProject': {
        const length = e.object.type.element.kind === 'string' ? renderStringLengthExpression('Lean', 'ml_value') : '(Float.ofNat ml_value.size)';
        return renderOptionalProjection('Lean', e.object.type.absence, this.expr(e.object, depth), '', 'ml_value', length, '');
      }
      case 'optionalIndex': {
        const index = this.expr(e.index, depth);
        const value = this.expr(e.array, depth);
        return renderOptionalArrayRead('Lean', e.index.type.kind, value, index);
      }
      case 'method':
        return this.method(e, depth);
      case 'stringPoints':
        return renderStringPointIteration('Lean', false, this.expr(e.object, depth));
      case 'set':
      case 'map': {
        const set = e.k === 'set';
        const keyType = set ? e.type.element : e.type.key;
        const values = this.expr(set ? e.items : e.entries, depth);
        return renderCollectionConstruction('Lean', e.k, keyType.kind, values, this.type(e.type), '', '');
      }
      case 'collectionSize':
        return renderCollectionSizeExpression('Lean', this.expr(e.object, depth));
      case 'collectionRead': {
        const keyType = e.object.type.kind === 'map' ? e.object.type.key : e.object.type.element;
        return renderCollectionReadExpression('Lean', e.object.type.kind, e.op, this.expr(e.object, depth), this.expr(e.key, depth), keyType.kind === 'float', '', '', '');
      }
      case 'collectionItems': {
        const map = e.object.type.kind === 'map';
        return renderCollectionIteration('Lean', map, e.op, this.expr(e.object, depth), '', '');
      }
      case 'ctor': {
        const head = this.state.ctorRef(e.data, e.ctor);
        return e.args.length ? `(${head} ${e.args.map((arg) => this.expr(arg, depth)).join(' ')})` : head;
      }
      case 'unary':
        if (e.op === 'not') return `(!${this.expr(e.arg, depth)})`;
        if (e.semantics === 'checked') throw unthreaded('a machine-integer negation');
        return `(-${this.expr(e.arg, depth)})`;
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
      case 'stringMap':
        throw unsupported(`.${e.op}()`, readStringMapRefusal('Lean', e.op), e.span);
      case 'stringTest': {
        const refusal = readStringOrderRefusal('Lean', e.op);
        if (refusal) throw unsupported('string comparison', refusal, e.span);
        const string = this.expr(e.string, depth);
        const search = this.expr(e.search, depth);
        const helper = readStringTestHelper('Lean', e.op);
        if (helper) this.helpers.add(helper);
        return renderStringTestExpression('Lean', e.op, string, search);
      }
      case 'cast':
        return this.cast(e, depth);
      case 'abort':
        throw unthreaded('an abort');
      case 'array':
        return `(#[${e.items.map((item) => this.expr(item, depth)).join(', ')}] : ${this.type(e.type)})`;
      case 'append':
        return `(${this.expr(e.left, depth)} ++ ${this.expr(e.right, depth)})`;
      case 'index': {
        if (!e.provenInBounds) this.state.arrayRead();
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
        if (e.array.type.kind === 'string') return renderStringLengthExpression('Lean', this.expr(e.array, depth));
        return `(${e.type.kind === 'float' ? 'Float.ofNat' : 'Int.ofNat'} ${this.expr(e.array, depth)}.size)`;
      case 'math':
        if (acceptLinkedRustCheckedShift(e.k,e.op)) return emitRustCheckedShift('Lean',e,argument=>this.expr(argument,depth));
        if (readRustPrimitiveBitwiseArity(e.op)) return emitRustPrimitiveBitwise('Lean', e, argument => this.expr(argument, depth));
        if (readBigIntegerBitwiseArity(e.op)) return renderBigIntegerBitwiseExpression('Lean', e.op, e.args.map(argument => this.expr(argument, depth)));
        return this.math(e, depth);
      default:
        throw new Error(`no Lean expression for ${e.k}`);
    }
  }

  /** Math and Number functions: Float's where they agree with JavaScript, the math helpers otherwise. */
  method(e, depth) {
    if (e.object.type.kind !== 'array') throw unsupported(`.${e.op}()`, readPrimitiveMethodRenderingRefusal('Lean', e.object.type.kind, e.op), e.span);
    const object = this.expr(e.object, depth);
    if (e.op === 'join') {
      const text = (name, type) => {
        if (type.kind === 'unit') return '""';
        if (type.kind === 'option') return `(match ${name} with | ${type.absence === 'both' ? 'some (some ml_present)' : 'some ml_present'} => ${text('ml_present', type.element)} | _ => "")`;
        return type.kind === 'string' ? name : this.toText({ k: 'var', name, type }, depth, false);
      };
      return `(String.intercalate ${this.expr(e.args[0], depth)} (${object}.toList.map (fun ml_value => ${text('ml_value', e.object.type.element)})))`;
    }
    if (e.op === 'reduce' && e.args.length === 1) throw unsupported('array reduce without an initial value', 'the empty-array TypeError needs an operational abort encoding in Lean', e.span);
    if (!['map', 'filter', 'find', 'findIndex', 'some', 'every', 'flatMap', 'reduce'].includes(e.op)) throw unsupported(`.${e.op}()`, readPrimitiveMethodRenderingRefusal('Lean', 'array', e.op) || 'this array operation needs an operational encoding in Lean', e.span);
    const callback = e.args[0];
    if (e.op === 'map') return renderArrayFromMapping('Lean', object, this.expr(callback, depth), callback.type.params.length, '');
    const args = e.op === 'reduce' ? ['ml_accumulator', 'ml_pair.1', '(Float.ofNat ml_pair.2)', 'ml_values'] : ['ml_pair.1', '(Float.ofNat ml_pair.2)', 'ml_values'];
    const call = callback.type.params.length ? `(ml_callback ${args.slice(0, callback.type.params.length).join(' ')})` : 'ml_callback';
    const head = `(let ml_values := ${object}; let ml_callback := ${this.expr(callback, depth)}; `;
    const pairs = 'ml_values.toList.zipIdx';
    const lambda = `(fun ml_pair => ${call})`;
    if (e.op === 'flatMap') return `${head} (${pairs}.flatMap (fun ml_pair => ${call}.toList)).toArray)`;
    if (e.op === 'some' || e.op === 'every') return `${head} ${pairs}.${e.op === 'some' ? 'any' : 'all'} ${lambda})`;
    if (e.op === 'filter') return `${head} ((${pairs}.filter ${lambda}).map (fun ml_pair => ml_pair.1)).toArray)`;
    if (e.op === 'find') return `${head} (${pairs}.find? ${lambda}).map (fun ml_pair => ml_pair.1))`;
    if (e.op === 'findIndex') return `${head} ((${pairs}.find? ${lambda}).map (fun ml_pair => Float.ofNat ml_pair.2)).getD (-1 : Float))`;
    return `${head} ${pairs}.foldl (fun ml_accumulator ml_pair => ${call}) ${this.expr(e.args[1], depth)})`;
  }

  optionalMatch(arg, binder, present, nullCase, undefinedCase, depth) {
    return renderOptionalMatch('Lean', arg.type.absence, this.expr(arg, depth), binder, present, nullCase, undefinedCase);
  }

  truthy(arg, depth) {
    if (arg.type.kind === 'option') return this.optionalMatch(arg, 'ml_present', this.truthy({ k: 'var', name: 'ml_present', type: arg.type.element }, depth), 'false', 'false', depth);
    const value = this.expr(arg, depth);
    if(this.utf16Strings&&arg.type.kind==='string')return formalUnitTruth(this,value);
    return renderOptionalPrimitiveTruth('Lean', arg.type.kind, value, '', false, Boolean(arg.type.signed));
  }

  strictEqual(e, depth) {
    const a = e.left.type.kind === 'option' ? e.left.type.element : e.left.type;
    const b = e.right.type.kind === 'option' ? e.right.type.element : e.right.type;
    const compare = JSON.stringify(a) === JSON.stringify(b) ? '(ml_left == ml_right)' : 'false';
    return renderOptionalStrictEquality('Lean', e.left.type.kind, e.left.type.absence ?? '', e.right.type.kind, e.right.type.absence ?? '', compare, this.expr(e.left, depth), this.expr(e.right, depth), e.negate);
  }

  math(e, depth) {
    if(acceptLinkedJavaScriptStringConstruction(e.k,e.op)){this.helpers.add('utf16Text');return renderJavaScriptStringConstruction('Lean',e.op,this.expr(e.args[0],depth));}
    if(acceptLinkedJavaScriptBigIntegerResult(e.k,e.op)){
      this.helpers.add('bigIntegerConversion');
      if(['number-compare','string-compare'].includes(readJavaScriptBigIntegerResultOperation(e.op)))return renderJavaScriptBigIntegerComparison('Lean',readJavaScriptBigIntegerResultOperation(e.op),this.expr(e.args[0],depth),this.expr(e.args[1],depth),false);
          if(readJavaScriptBigIntegerResultOperation(e.op)==='number-power')return renderJavaScriptNumberPower('Lean',this.expr(e.args[0],depth),this.expr(e.args[1],depth));
          if(readJavaScriptBigIntegerResultOperation(e.op)==='power')return renderJavaScriptBigIntegerPower('Lean',this.expr(e.args[0],depth),this.expr(e.args[1],depth));
          if(readJavaScriptBigIntegerResultOperation(e.op)==='division')return renderJavaScriptBigIntegerDivision('Lean',this.expr(e.args[0],depth),this.expr(e.args[1],depth));
          return renderJavaScriptBigIntegerResultExpression('Lean',readJavaScriptBigIntegerResultOperation(e.op),this.expr(e.args[0],depth),false);
    }
    if(acceptLinkedJavaScriptNumberConversion(e.k,e.op)){
      if(['string','int','nat'].includes(e.args[0].type.kind))this.helpers.add('stringNumber');
      const text=renderJavaScriptNumberScalarConversion('Lean',e.op,e.args[0].type.kind,this.expr(e.args[0],depth),false);
      if(!text)throw unsupported('numeric conversion operand','the checked conversion must retain its total value or threaded abort',e.span);
      return text;
    }
    const args = e.args.map((arg) => this.expr(arg, depth));
    const native = { abs: 'Float.abs', floor: 'Float.floor', ceil: 'Float.ceil', sqrt: 'Float.sqrt', isFinite: 'Float.isFinite', isNaN: 'Float.isNaN' }[e.op];
    if (native) return `(${native} ${args[0]})`;
    this.helpers.add('math');
    switch (e.op) {
      case 'max':
      case 'min':
        if (!args.length) return leanFloat(e.op === 'max' ? '-Infinity' : 'Infinity', 'Float');
        return args.reduce((left, right) => `(ml_${e.op} ${left} ${right})`);
      case 'maxOf':
      case 'minOf':
        return `(${args[0]}.foldl ml_${e.op.slice(0, 3)} ${leanFloat(e.op === 'maxOf' ? '-Infinity' : 'Infinity', 'Float')})`;
      default:
        return `(ml_${{ isInteger: 'is_integer', isSafeInteger: 'is_safe_integer' }[e.op] ?? e.op} ${args[0]})`;
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
    if(this.utf16Strings&&arg.type.kind==='string')return this.expr(arg,depth);
    if(this.utf16Strings&&arg.type.kind==='unit')return formalUnitLiteral(this,'undefined');
    if (arg.type.kind === 'data' || arg.type.kind === 'unit') {
      throw unsupported('output of structured values', `a ${arg.type.kind} value has no portable textual form`, arg.span);
    }
    if (arg.type.kind === 'float') {
      this.helpers.add('jsNumber');
      if (console) this.helpers.add('jsConsole');
      const text=`(${console ? 'ml_js_console' : 'ml_js_number'} ${this.expr(arg, depth)})`;
      return this.utf16Strings?formalScalarUnits(this,text):text;
    }
    const text=`(toString ${this.expr(arg, depth)})`;
    return this.utf16Strings?formalScalarUnits(this,text):text;
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
    // A machine-integer operation, one that may abort, reaches here as its abort test and its value computed on the representation (output.js).
    if (e.semantics === 'checked' || e.byZero === 'abort' || e.type.kind === 'fixed') throw unthreaded(`a machine-integer ${e.op}`);
    switch (e.op) {
      case 'add':
        return `(${left} + ${right})`;
      case 'mul':
        return `(${left} * ${right})`;
      case 'sub':
        return `(${left} - ${right})`;
      case 'div':
      case 'rem': {
        const division = e.op === 'div';
        if (e.domain.kind === 'nat') return `(${left} ${division ? '/' : '%'} ${right})`;
        if (e.rounding === 'trunc') return `(Int.${division ? 'tdiv' : 'tmod'} ${left} ${right})`;
        if (e.rounding === 'floor') return `(Int.${division ? 'fdiv' : 'fmod'} ${left} ${right})`;
        return `(${left} ${division ? '/' : '%'} ${right})`;
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
    if (e.flavor === 'wrapping') {
      if (e.from.kind !== 'fixed' || e.to.kind !== 'fixed') throw unsupported('wrapping integer cast', 'requires actual fixed integer source and target types', e.span);
      const code = renderWrappingFixedIntegerCast('Lean', arg, e.to.bits, e.to.signed, e.from.signed);
      if (!code) throw unsupported('wrapping integer cast', 'the integer width needs an exact target encoding', e.span);
      this.state.encode('fixed integer casts', readFixedIntegerCastStatement());
      return code;
    }
    const naturalFrom = e.from.kind === 'nat' || (e.from.kind === 'fixed' && !e.from.signed);
    const naturalTo = e.to.kind === 'nat' || (e.to.kind === 'fixed' && !e.to.signed);
    if (naturalFrom === naturalTo) return arg;
    if (naturalFrom) return `(Int.ofNat ${arg})`;
    if (e.flavor === 'checked') throw unthreaded('a checked conversion to a natural');
    return `(Int.toNat ${arg})`;
  }

  main(main) {
    const { effects } = renameMain(main, ident, this.state.localReserved());
    const lines = [];
    const theorems = [];
    let assertion = 0;
    const sourceExceptions=hasSourceExceptionOutput(this.program);
    effects.forEach((effect, index) => {
      if (effect.k === 'print') lines.push(`  IO.println ${this.utf16Strings?formalUnitDisplay(this,this.expr(effect.expr,1)):this.expr(effect.expr, 1)}`);
      else if (effect.k === 'output') lines.push(`  for ml_line in ${this.expr(effect.expr, 1)}.reverse do IO.println ${this.utf16Strings?formalUnitDisplay(this,'ml_line'):'ml_line'}`);
      else if (effect.k === 'let') lines.push(`  let ${effect.name} := ${this.expr(effect.value, 1)}`);
      else if (effect.k === 'unwrap') {
        // The value of a step, or the source's abort: its message, uncaught, ends main.
        const [made, aborted] = effect.ctors.map((ctor) => this.state.ctorRef(effect.data, ctor));
        const rich=sourceExceptions&&hasSourceExceptionPayload(this.program.declarations.get(effect.data));
        const message=this.utf16Strings?formalUnitDisplay(this,'ml_m'):'ml_m';
        const failure=rich?'throw (ml_class, ml_m, ml_present)':`throw (IO.userError ${message})`;
        lines.push(`  let ${effect.name} ← (match ${this.expr(effect.pair, 1)} with\n    | ${made} _ ml_v => pure ml_v\n    | ${aborted} _ ml_m${rich?' ml_class ml_present':sourceExceptionPatternSuffix(this.program,effect.data)} => ${failure})`);
      } else if (this.usesNumber(effect.prop) || this.onPartial(effect.prop, effects.slice(0, index))) {
        // The kernel cannot evaluate Float or unfold a partial def, so the assertion runs where the source's does.
        assertion += 1;
        lines.push(`  if !${this.check(effect.prop)} then throw (IO.userError "assertion ${assertion} failed")`);
        this.state.assertionTheorem(`assertion ${assertion}`, effect, { discharge: 'runtime-assertion' });
      } else {
        assertion += 1;
        // The assertion is stated of the values main computes before it; a run that aborts before it never reaches it.
        // Where a run may abort, the statement is a Bool the kernel evaluates: the decided assertion, or true where one aborts.
        const threaded = this.program.abortsThreaded;
        const body = effects.slice(0, index).reduceRight((rest, item) => {
          if (item.k === 'let') return `let ${item.name} := ${this.expr(item.value, 1)}; ${rest}`;
          if (item.k !== 'unwrap') return rest;
          const [made, aborted] = item.ctors.map((ctor) => this.state.ctorRef(item.data, ctor));
          return `(match ${this.expr(item.pair, 1)} with | ${made} _ ${item.name} => ${rest} | ${aborted} _ _${sourceExceptionPatternSuffix(this.program,item.data)} => ${threaded ? 'true' : 'True'})`;
        }, threaded ? `decide (${this.prop(effect.prop)})` : this.prop(effect.prop));
        const statement = threaded ? `(${body}) = true` : body;
        const name = `ml_assertion_${assertion}`;
        theorems.push(`theorem ${name} : ${statement} := by\n  try rfl\n  try decide`);
        this.state.assertionTheorem(name, effect);
      }
    });
    if (main.sequentialAsync) this.state.encode('sequential-async', 'an async function is the function its body computes and await is its call: every call of one is awaited where it is made, so nothing runs concurrently and the output is the same, in the same order');
    this.state.encode('program-output', 'main prints the lines the source program prints, in order, with IO.println');
    if (this.program.outputThreaded) this.state.encode('output-threading', 'a function that prints, directly or through a function it calls, takes the lines printed before it and returns them, with its own in front, paired with its value in a generated ml_io data type; main prints the lines of each step in the order they were printed');
    if (this.program.abortsThreaded) this.state.encode('abort-threading', 'a function that may abort, directly or through a function it calls, returns the source\'s abort message with the lines printed before it in the generated ml_io data type; main prints those lines and throws IO.userError with the message, which lean --run reports as "uncaught exception: <message>" with exit code 1');
    if(sourceExceptions){
      this.state.encode('source-exception-observation',readSourceExceptionObservationStatement());
      const type=this.type({kind:'string'}),message=this.utf16Strings?formalUnitDisplay(this,'ml_error.2.1'):'ml_error.2.1';
      return [...theorems,`def ml_source_exception_observation : ExceptT (${type} × ${type} × Bool) IO Unit := do\n${[...lines,'  pure ()'].join('\n')}`,
        `def main : IO Unit := do\n  let ml_result ← ml_source_exception_observation.run\n  match ml_result with\n  | .ok _ => pure ()\n  | .error ml_error => throw (IO.userError ${message})`].join('\n\n');
    }
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
      const value = effect.k === 'let' ? effect.value : effect.k === 'unwrap' ? effect.pair : null;
      if (value && reads.has(effect.name)) {
        partial = this.usesPartial(value);
        collect(value);
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
