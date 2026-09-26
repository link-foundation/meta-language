// Lean 4 emitter. Naturals are `Nat`, integers `Int`; machine integers are
// represented by `Nat`/`Int` with explicit range checks that `panic!` where
// Rust would panic. Recursion is structural and annotated as such, so the
// Lean kernel checks termination. Theorems are reconstructed from portable
// proof plans with Lean tactics and re-checked by the Lean kernel.

import { unsupported } from './diagnostics.js';
import { propFunctions } from './proof.js';
import { fixedBounds, typeKey } from './types.js';
import { renameFunction, renameMain, renameTheorem } from './ir.js';
import { EmitState, orderDeclarations } from './emit-common.js';
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
  'Nat', 'Int', 'String', 'Bool', 'Unit', 'IO', 'true', 'false', 'main', 'toString', 'decide', 'panic',
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
};

function ident(name) {
  let result = name.replace(/[^A-Za-z0-9_'À-￿]/gu, '_');
  if (/^[0-9']/u.test(result)) result = `x${result}`;
  if (KEYWORDS.has(result)) result = `«${result}»`;
  return result;
}

export function emitLean(program) {
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
    for (const entry of orderDeclarations(this.program)) {
      moveTo(entry.modulePath);
      if (entry.k === 'data') blocks.push(this.data(entry));
      else if (entry.k === 'fn') blocks.push(this.fn(entry));
      else blocks.push(this.theorem(entry));
    }
    moveTo([]);
    const main = this.program.main ? this.main(this.program.main) : null;
    const text = [
      `-- Translated from ${this.program.sourceLanguage} by meta-language: portable core, Lean target.`,
      '-- Source binders are kept even where unused, and proof hints are shared by every closing tactic.',
      'set_option linter.unusedVariables false',
      'set_option linter.unusedSimpArgs false',
      '',
      ...['fixed', 'fixedNat', 'toNatChecked', 'divide', 'divideNat'].filter((name) => this.helpers.has(name)).flatMap((name) => [HELPERS[name], '']),
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
      case 'bool':
        return 'Bool';
      case 'string':
        return 'String';
      case 'unit':
        return 'Unit';
      case 'data':
        return this.state.ref(type.name);
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
    return [`inductive ${name} where`, ...ctors, '  deriving Repr, DecidableEq, Inhabited'].join('\n');
  }

  fn(entry) {
    const { params, body } = renameFunction(entry, ident, this.state.localReserved());
    const name = this.state.localName(entry.fullName);
    this.state.map(entry, name);
    if (entry.mutual) throw unsupported('mutual recursion', `${entry.fullName} is mutually recursive; the Lean target emits only single recursive definitions`, entry.span);
    const binders = params.map((param) => ` (${param.name} : ${this.type(param.type)})`).join('');
    const text = `def ${name}${binders} : ${this.type(entry.ret)} :=\n  ${this.expr(body, 1)}`;
    if (!entry.recursive) return text;
    if (entry.decreasing === null) {
      this.state.encode('general-recursion', 'recursion without a structurally decreasing argument is a Lean partial def: it runs as the source does but its equations are opaque to proofs');
      return `partial ${text}`;
    }
    return `${text}\ntermination_by structural ${params[entry.decreasing].name}`;
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
    const lines = [`${pad}${plan.k === 'induction' ? 'induction' : 'cases'} ${plan.variable} with`];
    for (const kase of plan.cases) {
      const ctor = plan.type.kind === 'nat' ? kase.ctor : this.state.ctorLocal(plan.type.name, kase.ctor);
      const names = [...kase.fields, ...(plan.k === 'induction' ? kase.ihs : [])];
      lines.push(`${pad}| ${ctor}${names.map((item) => ` ${item}`).join('')} =>`);
      lines.push(this.plan(kase.plan, functions, depth + 2));
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
        `(simp only [${list}]; done)`,
        `(simp only [${list}] at * <;> omega)`,
        `(simp [${list}]; done)`,
        `(simp [${list}] at * <;> omega)`,
        `(simp [${[...rules, ...ring].join(', ')}] at * <;> omega)`,
      ] : ['omega', `(simp [${ring.join(', ')}] at * <;> omega)`]),
    ];
    return `first\n${alternatives.map((alternative) => `${' '.repeat(depth * 2 + 2)}| ${alternative}`).join('\n')}`;
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
        return e.arg.type.kind === 'string' ? this.expr(e.arg, depth) : this.toText(e.arg, depth);
      case 'cast':
        return this.cast(e, depth);
      case 'abort':
        this.state.abortToTotal(e.message);
        return `(panic! ${JSON.stringify(e.message)} : ${this.type(e.type)})`;
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
      case 'bool':
        return String(e.value);
      case 'string':
        return JSON.stringify(String(e.value));
      default:
        throw new Error(`no Lean literal for ${e.type.kind}`);
    }
  }

  toText(arg, depth) {
    if (arg.type.kind === 'data' || arg.type.kind === 'unit') {
      throw unsupported('output of structured values', `a ${arg.type.kind} value has no portable textual form`, arg.span);
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
      else if (effect.k === 'let') lines.push(`  let ${effect.name} := ${this.expr(effect.value, 1)}`);
      else {
        assertion += 1;
        const lets = effects.slice(0, index).filter((item) => item.k === 'let')
          .map((item) => `let ${item.name} := ${this.expr(item.value, 1)}; `).join('');
        const name = `ml_assertion_${assertion}`;
        theorems.push(`theorem ${name} : ${lets}${this.prop(effect.prop)} := by\n  first\n    | rfl\n    | decide`);
        this.state.assertionTheorem(name, effect);
      }
    });
    this.state.encode('program-output', 'main prints the lines the source program prints, in order, with IO.println');
    return [...theorems, `def main : IO Unit := do\n${lines.length ? lines.join('\n') : '  pure ()'}`].join('\n\n');
  }
}

function collectUnfold(plan) {
  if (plan.k === 'close') return plan.hints.unfold;
  return plan.cases.flatMap((kase) => collectUnfold(kase.plan));
}

function pad(depth) {
  return '  '.repeat(depth);
}
