// Rocq emitter. Naturals become binary `N` and integers `Z`, so programs
// run at their source sizes (unary `nat` cannot hold 20!). Structural
// recursion over data is a `Fixpoint`; recursion that decreases a natural is
// a `Function` with the measure `N.to_nat`, whose obligations `lia`
// discharges and whose equation lemma the reconstructed proofs rewrite with.

import { unsupported } from './diagnostics.js';
import { propFunctions } from './proof.js';
import { renameFunction, renameMain, renameTheorem } from './ir.js';
import { EmitState, orderDeclarations } from './emit-common.js';

const KEYWORDS = new Set([
  'as', 'at', 'cofix', 'else', 'end', 'exists', 'exists2', 'fix', 'for', 'forall', 'fun', 'if', 'IF', 'in', 'let',
  'match', 'mod', 'Prop', 'return', 'Set', 'SProp', 'then', 'Type', 'using', 'where', 'with', 'struct', 'measure',
  'wf', 'Definition', 'Fixpoint', 'Function', 'Theorem', 'Lemma', 'Proof', 'Qed', 'Defined', 'Module', 'End',
  'Inductive', 'Import', 'Require', 'From', 'Open', 'Scope', 'Eval', 'Compute', 'Check', 'Print',
  // Standard-library names the emitted code uses unqualified.
  'N', 'Z', 'String', 'EmptyString', 'negb', 'andb', 'orb', 'true', 'false', 'bool', 'string', 'list', 'nil', 'cons',
  'unit', 'tt', 'nat', 'O', 'S', 'Bool', 'Ascii', 'main', 'lia', 'nia',
  // Imported constructors: a pattern variable with one of these names would match the constructor instead.
  'left', 'right', 'inleft', 'inright', 'Some', 'None', 'pair', 'inl', 'inr', 'exist', 'existT', 'I', 'conj', 'or_introl',
  'or_intror', 'ex_intro', 'eq_refl', 'Eq', 'Lt', 'Gt', 'CompEq', 'CompLt', 'CompGt', 'xI', 'xO', 'xH', 'N0', 'Npos', 'Z0',
  'Zpos', 'Zneg', 'ReflectT', 'ReflectF', 'identity_refl',
]);

export const ROCQ_PRELUDE = [
  'From Stdlib Require Import NArith ZArith Lia Recdef String List Ascii.',
];

const HELPERS = {
  digits: `Fixpoint ml_digits (fuel : nat) (n : N) (acc : string) : string :=
  match fuel with
  | O => acc
  | S rest =>
      let acc' := String (ascii_of_N (48 + N.modulo n 10)) acc in
      if N.ltb n 10 then acc' else ml_digits rest (N.div n 10) acc'
  end.
Definition ml_N_to_string (n : N) : string := ml_digits (S (N.size_nat n)) n EmptyString.`,
  zToString: `Definition ml_Z_to_string (z : Z) : string :=
  if Z.ltb z 0 then String.append "-" (ml_N_to_string (Z.to_N (Z.opp z))) else ml_N_to_string (Z.to_N z).`,
  boolToString: `Definition ml_bool_to_string (b : bool) : string := if b then "true" else "false".`,
  euclid: `(* Euclidean division, the rounding of Lean's Int / and %: the remainder is never negative. *)
Definition ml_Z_ediv (a b : Z) : Z := Z.mul (Z.sgn b) (Z.div a (Z.abs b)).
Definition ml_Z_emod (a b : Z) : Z := Z.modulo a (Z.abs b).`,
  tactics: `Ltac ml_N_norm := repeat first
  [ rewrite N.pred_succ
  | rewrite (proj2 (N.eqb_neq (N.succ _) 0) (N.neq_succ_0 _))
  | rewrite N.eqb_refl ].
Ltac ml_obligation := intros; repeat match goal with H : N.eqb _ _ = false |- _ => apply N.eqb_neq in H end; lia.
Ltac ml_close := first [reflexivity | lia | nia | congruence].`,
  // A closed assertion computes to connectives over literal (in)equalities:
  // prove the goal, or refute a hypothesis, one connective at a time.
  decide: `Ltac ml_prove := first
  [ reflexivity | discriminate | exact I | lia
  | split; ml_prove
  | left; ml_prove
  | right; ml_prove
  | let H := fresh "H" in intro H; first [ml_prove | ml_refute H] ]
with ml_refute H := first
  [ discriminate H | exact H | lia
  | let A := fresh "H" in let B := fresh "H" in destruct H as [A B]; first [ml_refute A | ml_refute B]
  | let A := fresh "H" in let B := fresh "H" in destruct H as [A | B]; [ml_refute A | ml_refute B]
  | apply H; ml_prove ].
Ltac ml_decide := vm_compute; ml_prove.`,
};

function ident(name) {
  let result = name.replace(/[^A-Za-z0-9_']/gu, '_');
  if (/^[0-9']/u.test(result)) result = `x${result}`;
  if (KEYWORDS.has(result)) result = `${result}_`;
  return result;
}

export function emitRocq(program) {
  const state = new EmitState(program, 'Rocq', ident, KEYWORDS, {
    ctorStyle: 'module',
    // Rocq derives these from inductives and Functions in the same namespace.
    generated: (name) => ['_ind', '_rect', '_rec', '_sind', '_equation', '_tcc', '_terminate', '_F', '_graph', '_complete', '_correct', '_ind_', '_rect_']
      .map((suffix) => `${name}${suffix}`).concat([`R_${name}`]),
  });
  const emitter = new RocqEmitter(program, state);
  return emitter.file();
}

class RocqEmitter {
  constructor(program, state) {
    this.program = program;
    this.state = state;
    this.helpers = new Set(['tactics']);
    this.natFunctions = new Map();
    this.current = null;
  }

  file() {
    const blocks = [];
    const order = orderDeclarations(this.program, { requireContiguousModules: true });
    let openModules = [];
    const moveTo = (path) => {
      let common = 0;
      while (common < openModules.length && common < path.length && openModules[common] === path[common]) common += 1;
      while (openModules.length > common) blocks.push(`End ${this.state.moduleName(openModules.slice(0, openModules.length))}.`) && openModules.pop();
      while (openModules.length < path.length) {
        openModules.push(path[openModules.length]);
        blocks.push(`Module ${this.state.moduleName(openModules)}.`);
      }
    };
    for (const entry of order) {
      moveTo(entry.modulePath);
      if (entry.k === 'data') blocks.push(this.data(entry));
      else if (entry.k === 'fn') blocks.push(this.fn(entry));
      else if (entry.k === 'theorem') blocks.push(this.theorem(entry));
    }
    moveTo([]);
    const mainText = this.program.main ? this.main(this.program.main) : null;
    const helperText = ['digits', 'zToString', 'boolToString', 'euclid', 'tactics', 'decide']
      .filter((name) => this.helpers.has(name) || (name === 'digits' && this.helpers.has('zToString')))
      .map((name) => HELPERS[name]);
    const text = [
      `(* Translated from ${this.program.sourceLanguage} by meta-language: portable core, Rocq target. *)`,
      ...ROCQ_PRELUDE,
      '',
      ...helperText.flatMap((helper) => [helper, '']),
      ...blocks.flatMap((block) => [block, '']),
      ...(mainText ? [mainText, '', 'Eval vm_compute in main.', ''] : []),
    ].join('\n');
    return {
      language: 'Rocq',
      text,
      mappings: this.state.mappings,
      assumptions: this.state.assumptionList(),
      encodings: this.state.encodingList(),
      theorems: this.state.theorems,
      entry: mainText ? 'main' : null,
    };
  }

  type(type) {
    switch (type.kind) {
      case 'nat':
        return 'N';
      case 'int':
        return 'Z';
      case 'fixed':
        this.state.fixedToUnbounded(type);
        return type.signed ? 'Z' : 'N';
      case 'bool':
        return 'bool';
      case 'string':
        return 'string';
      case 'unit':
        return 'unit';
      case 'data':
        return this.state.ref(type.name);
      default:
        throw new Error(`no Rocq type for ${type.kind}`);
    }
  }

  data(entry) {
    const name = this.state.localName(entry.fullName);
    const ctors = entry.ctors.map((ctor) => {
      const fields = ctor.fields.map((field) => `${this.type(field.type)} -> `).join('');
      return `| ${this.state.ctorLocal(entry.fullName, ctor.name)} : ${fields}${name}`;
    });
    this.state.map(entry, name);
    return [`Inductive ${name} : Type :=`, ...ctors].join('\n') + '.';
  }

  fn(entry) {
    const { params, body } = renameFunction(entry, ident, this.state.localReserved());
    const name = this.state.localName(entry.fullName);
    this.state.map(entry, name);
    this.current = { entry, name };
    const binders = params.map((param) => ` (${param.name} : ${this.type(param.type)})`).join('');
    const result = this.type(entry.ret);
    const text = this.expr(body);
    this.current = null;
    if (entry.mutual) throw unsupported('mutual recursion', `${entry.fullName} is mutually recursive; the Rocq target emits only single recursive definitions`, entry.span);
    if (!entry.recursive) return `Definition ${name}${binders} : ${result} :=\n  ${text}.`;
    if (entry.decreasing === null) {
      throw unsupported('general recursion', `${entry.fullName} is not structurally recursive and Rocq requires a termination argument`, entry.span);
    }
    const decreasing = params[entry.decreasing];
    if (decreasing.type.kind === 'data') {
      return `Fixpoint ${name}${binders} {struct ${decreasing.name}} : ${result} :=\n  ${text}.`;
    }
    this.natFunctions.set(entry.fullName, { index: entry.decreasing, arity: params.length });
    this.state.encode('nat-recursion', 'recursion that decreases a natural is a Rocq Function with measure N.to_nat; lia discharges the decrease obligations');
    return `Function ${name}${binders} {measure N.to_nat ${decreasing.name}} : ${result} :=\n  ${text}.\nProof. all: ml_obligation. Defined.`;
  }

  theorem(entry) {
    const { binders, prop, plan } = renameTheorem(entry, ident, this.state.localReserved());
    const name = this.state.localName(entry.fullName);
    this.state.map(entry, name);
    const statement = `Theorem ${name}${binders.map((binder) => ` (${binder.name} : ${this.type(binder.type)})`).join('')} : ${this.prop(prop)}.`;
    const functions = [...new Set([...propFunctions(prop), ...collectHintFunctions(plan)])];
    const script = this.plan(plan, functions, binders.length === 0, 1);
    this.state.theorem(entry, name, { closedGoal: binders.length === 0 });
    return `${statement}\nProof.\n${script}\nQed.`;
  }

  plan(plan, functions, closed, depth) {
    const indent = '  '.repeat(depth);
    if (plan.k === 'close') return `${indent}${this.closer(plan.hints, functions, closed)}.`;
    const kase = plan.cases;
    let pattern;
    if (plan.type.kind === 'nat') {
      const succ = kase.find((item) => item.ctor === 'succ');
      const ih = plan.k === 'induction' ? succ.ihs[0] : '_';
      pattern = `induction ${plan.variable} as [|${succ.fields[0]} ${ih}] using N.peano_ind.`;
    } else {
      const groups = kase.map((item) => {
        const names = [];
        item.fields.forEach((field, index) => {
          names.push(field);
          if (plan.k === 'induction' && item.recursive[index]) names.push(item.ihs[item.recursive.slice(0, index).filter(Boolean).length]);
        });
        return names.join(' ');
      });
      pattern = `${plan.k === 'induction' ? 'induction' : 'destruct'} ${plan.variable} as [${groups.join(' | ')}].`;
    }
    const lines = [`${indent}${pattern}`];
    for (const item of kase) {
      lines.push(`${indent}{`);
      lines.push(this.plan(item.plan, functions, false, depth + 1));
      lines.push(`${indent}}`);
    }
    return lines.join('\n');
  }

  closer(hints, functions, closed) {
    const steps = [];
    const unfoldAll = [...new Set([...functions, ...hints.unfold])];
    const plain = unfoldAll.filter((name) => !this.natFunctions.has(name)).map((name) => this.state.ref(name));
    if (plain.length) steps.push(`cbn [${plain.join(' ')}]`);
    for (const name of unfoldAll.filter((fn) => this.natFunctions.has(fn))) {
      const { index, arity } = this.natFunctions.get(name);
      const ref = this.state.ref(name);
      const args = (value) => Array.from({ length: arity }, (_, position) => (position === index ? value : `?a${position}`));
      const uses = (value) => Array.from({ length: arity }, (_, position) => (position === index ? value : `a${position}`));
      steps.push(`repeat match goal with |- context [${ref} ${args('(N.succ ?k)').join(' ')}] => rewrite (${ref}_equation ${uses('(N.succ k)').join(' ')}) end`);
      steps.push(`repeat match goal with |- context [${ref} ${args('0%N').join(' ')}] => rewrite (${ref}_equation ${uses('0%N').join(' ')}) end`);
    }
    steps.push('ml_N_norm', 'cbv beta iota zeta');
    const alternatives = [];
    if (closed) alternatives.push('(vm_compute; reflexivity)');
    alternatives.push('ml_close');
    const rewrites = [...hints.hyps, ...hints.lemmas.map((lemma) => this.state.ref(lemma))];
    if (rewrites.length) {
      alternatives.push(`(rewrite ${rewrites.map((rule) => `?${rule}`).join(', ')}; ml_close)`);
      alternatives.push(`(rewrite <- ${rewrites.map((rule) => `?${rule}`).join(', ')}; ml_close)`);
    }
    return `${steps.join('; ')}; first [${alternatives.join(' | ')}]`;
  }

  prop(prop) {
    switch (prop.p) {
      case 'forall':
        return `(forall ${prop.binders.map((binder) => `(${binder.name} : ${this.type(binder.type)})`).join(' ')}, ${this.prop(prop.body)})`;
      case 'and':
        return `(${this.prop(prop.left)} /\\ ${this.prop(prop.right)})`;
      case 'or':
        return `(${this.prop(prop.left)} \\/ ${this.prop(prop.right)})`;
      case 'implies':
        return `(${this.prop(prop.left)} -> ${this.prop(prop.right)})`;
      case 'not':
        return `(~ ${this.prop(prop.arg)})`;
      case 'bool':
        return `(${this.expr(prop.expr)} = true)`;
      case 'eq':
        return `(${this.expr(prop.left)} = ${this.expr(prop.right)})`;
      case 'ne':
        return `(${this.expr(prop.left)} <> ${this.expr(prop.right)})`;
      default: {
        const module = this.numericModule(prop.domain);
        const [left, right] = [this.expr(prop.left), this.expr(prop.right)];
        const relation = { lt: 'lt', le: 'le', gt: 'lt', ge: 'le' }[prop.p];
        return prop.p === 'gt' || prop.p === 'ge'
          ? `(${module}.${relation} ${right} ${left})`
          : `(${module}.${relation} ${left} ${right})`;
      }
    }
  }

  numericModule(type) {
    if (type.kind === 'nat') return 'N';
    if (type.kind === 'int') return 'Z';
    if (type.kind === 'fixed') {
      this.state.fixedToUnbounded(type);
      return type.signed ? 'Z' : 'N';
    }
    throw new Error(`not numeric: ${type.kind}`);
  }

  expr(e) {
    switch (e.k) {
      case 'lit':
        return this.literal(e);
      case 'unit':
        return 'tt';
      case 'var':
        return e.name;
      case 'call': {
        const head = this.current && e.fn === this.current.entry.fullName && this.current.entry.recursive
          ? this.current.name
          : this.state.ref(e.fn);
        return e.args.length ? `(${head} ${e.args.map((arg) => this.expr(arg)).join(' ')})` : head;
      }
      case 'ctor': {
        const head = this.state.ctorRef(e.data, e.ctor);
        return e.args.length ? `(${head} ${e.args.map((arg) => this.expr(arg)).join(' ')})` : head;
      }
      case 'unary':
        if (e.op === 'not') return `(negb ${this.expr(e.arg)})`;
        if (e.semantics === 'checked') this.state.checkedToTotal('negation');
        return `(Z.opp ${this.expr(e.arg)})`;
      case 'binary':
        return this.binary(e);
      case 'if':
        return `(if ${this.expr(e.cond)} then ${this.expr(e.then)} else ${this.expr(e.else)})`;
      case 'let':
        return `(let ${e.name} := ${this.expr(e.value)} in ${this.expr(e.body)})`;
      case 'match':
        return this.match(e);
      case 'toString':
        return this.toText(e.arg);
      case 'cast':
        return this.cast(e);
      case 'abort':
        this.state.abortToTotal(e.message);
        return `(* unreachable under the non-aborting assumption: ${e.message.replace(/\*\)/gu, '* )')} *) ${this.inhabitant(e.type)}`;
      default:
        throw new Error(`no Rocq expression for ${e.k}`);
    }
  }

  literal(e) {
    switch (e.type.kind) {
      case 'nat':
        return `${e.value}%N`;
      case 'int':
        return e.value.startsWith('-') ? `(${e.value})%Z` : `${e.value}%Z`;
      case 'fixed':
        this.state.fixedToUnbounded(e.type);
        if (!e.type.signed) return `${e.value}%N`;
        return e.value.startsWith('-') ? `(${e.value})%Z` : `${e.value}%Z`;
      case 'bool':
        return String(e.value);
      case 'string':
        return `"${String(e.value).replace(/"/gu, '""')}"%string`;
      default:
        throw new Error(`no Rocq literal for ${e.type.kind}`);
    }
  }

  inhabitant(type) {
    switch (type.kind) {
      case 'nat':
        return '0%N';
      case 'int':
        return '0%Z';
      case 'fixed':
        return type.signed ? '0%Z' : '0%N';
      case 'bool':
        return 'false';
      case 'string':
        return 'EmptyString';
      case 'unit':
        return 'tt';
      case 'data': {
        const entry = this.program.declarations.get(type.name);
        const ctor = entry.ctors.find((candidate) => candidate.fields.every((field) => field.type.kind !== 'data'))
          ?? entry.ctors[0];
        const args = ctor.fields.map((field) => this.inhabitant(field.type));
        const head = this.state.ctorRef(type.name, ctor.name);
        return args.length ? `(${head} ${args.join(' ')})` : head;
      }
      default:
        throw new Error(`no Rocq inhabitant for ${type.kind}`);
    }
  }

  binary(e) {
    const left = this.expr(e.left);
    const right = this.expr(e.right);
    switch (e.op) {
      case 'and':
        return `(andb ${left} ${right})`;
      case 'or':
        return `(orb ${left} ${right})`;
      case 'concat':
        return `(String.append ${left} ${right})`;
      case 'eq':
      case 'ne':
      case 'lt':
      case 'le':
      case 'gt':
      case 'ge':
        return this.comparison(e.op, e.domain, left, right);
      default:
        return this.arithmetic(e, left, right);
    }
  }

  comparison(op, domain, left, right) {
    let module;
    if (domain.kind === 'bool') module = 'Bool';
    else if (domain.kind === 'string') module = 'String';
    else module = this.numericModule(domain);
    const test = {
      eq: `(${module}.eqb ${left} ${right})`,
      ne: `(negb (${module}.eqb ${left} ${right}))`,
      lt: `(${module}.ltb ${left} ${right})`,
      le: `(${module}.leb ${left} ${right})`,
      gt: `(${module}.ltb ${right} ${left})`,
      ge: `(${module}.leb ${right} ${left})`,
    }[op];
    if (!test) throw new Error(`no Rocq comparison ${op}`);
    return test;
  }

  arithmetic(e, left, right) {
    const module = this.numericModule(e.domain);
    if (e.semantics === 'checked') this.state.checkedToTotal(e.op);
    if (e.byZero === 'abort') this.state.abortToTotal('division by zero');
    switch (e.op) {
      case 'add':
        return `(${module}.add ${left} ${right})`;
      case 'mul':
        return `(${module}.mul ${left} ${right})`;
      case 'sub':
        // N.sub truncates at zero, which is exactly natural subtraction; a
        // checked unsigned subtraction agrees with it on non-aborting runs.
        return `(${module}.sub ${left} ${right})`;
      case 'div':
      case 'rem': {
        const division = e.op === 'div';
        if (module === 'N') return `(N.${division ? 'div' : 'modulo'} ${left} ${right})`;
        if (e.rounding === 'trunc') return `(Z.${division ? 'quot' : 'rem'} ${left} ${right})`;
        if (e.rounding === 'floor') return `(Z.${division ? 'div' : 'modulo'} ${left} ${right})`;
        this.helpers.add('euclid');
        return `(ml_Z_${division ? 'ediv' : 'emod'} ${left} ${right})`;
      }
      default:
        throw new Error(`no Rocq arithmetic ${e.op}`);
    }
  }

  match(e) {
    if (e.scrutinee.k !== 'var') {
      const name = `ml_scrutinee`;
      return `(let ${name} := ${this.expr(e.scrutinee)} in ${this.match({ ...e, scrutinee: { k: 'var', name, type: e.scrutinee.type } })})`;
    }
    const subject = e.scrutinee.name;
    if (e.scrutinee.type.kind !== 'data') {
      // Natural-number matches test for zero; the successor case binds the predecessor.
      const zero = e.cases.find((kase) => kase.pattern.k === 'natZero');
      const succ = e.cases.find((kase) => kase.pattern.k === 'natSucc');
      const fallback = e.cases.find((kase) => kase.pattern.k === 'wild' || kase.pattern.k === 'bind');
      const bindFallback = (kase) => (kase.pattern.k === 'bind' ? `(let ${kase.pattern.name} := ${subject} in ${this.expr(kase.body)})` : this.expr(kase.body));
      const zeroText = zero ? this.expr(zero.body) : bindFallback(fallback);
      const succText = succ
        ? `(let ${succ.pattern.name} := N.pred ${subject} in ${this.expr(succ.body)})`
        : bindFallback(fallback);
      return `(if N.eqb ${subject} 0%N then ${zeroText} else ${succText})`;
    }
    const entry = this.program.declarations.get(e.scrutinee.type.name);
    const arms = [];
    for (const ctor of entry.ctors) {
      const kase = e.cases.find((item) => item.pattern.k === 'ctor' && item.pattern.ctor === ctor.name)
        ?? e.cases.find((item) => item.pattern.k === 'wild' || item.pattern.k === 'bind');
      const binds = kase.pattern.k === 'ctor' ? kase.pattern.binds.map((bind) => bind ?? '_') : ctor.fields.map(() => '_');
      let body = this.expr(kase.body);
      if (kase.pattern.k === 'bind') body = `(let ${kase.pattern.name} := ${subject} in ${body})`;
      arms.push(`| ${[this.state.ctorRef(entry.fullName, ctor.name), ...binds].join(' ')} => ${body}`);
    }
    return `(match ${subject} with ${arms.join(' ')} end)`;
  }

  toText(arg) {
    const text = this.expr(arg);
    switch (arg.type.kind) {
      case 'string':
        return text;
      case 'bool':
        this.helpers.add('boolToString');
        return `(ml_bool_to_string ${text})`;
      case 'nat':
        this.helpers.add('digits');
        return `(ml_N_to_string ${text})`;
      case 'int':
        this.helpers.add('zToString');
        return `(ml_Z_to_string ${text})`;
      case 'fixed':
        this.state.fixedToUnbounded(arg.type);
        this.helpers.add(arg.type.signed ? 'zToString' : 'digits');
        return arg.type.signed ? `(ml_Z_to_string ${text})` : `(ml_N_to_string ${text})`;
      default:
        throw unsupported('output of structured values', `a ${arg.type.kind} value has no portable textual form`, arg.span);
    }
  }

  cast(e) {
    const arg = this.expr(e.arg);
    const from = e.from.kind === 'fixed' ? (e.from.signed ? 'Z' : 'N') : (e.from.kind === 'nat' ? 'N' : 'Z');
    const to = e.to.kind === 'fixed' ? (e.to.signed ? 'Z' : 'N') : (e.to.kind === 'nat' ? 'N' : 'Z');
    if (e.flavor === 'checked') this.state.checkedToTotal('conversion to a natural');
    if (from === to) return arg;
    if (from === 'N') return `(Z.of_N ${arg})`;
    return `(Z.to_N ${arg})`;
  }

  main(main) {
    const { effects } = renameMain(main, ident, this.state.localReserved());
    let assertion = 0;
    const theorems = [];
    const build = (index) => {
      if (index >= effects.length) return 'nil';
      const effect = effects[index];
      if (effect.k === 'print') return `${this.expr(effect.expr)} ::\n  ${build(index + 1)}`;
      if (effect.k === 'let') return `let ${effect.name} := ${this.expr(effect.value)} in\n  ${build(index + 1)}`;
      assertion += 1;
      const lets = effects.slice(0, index).filter((item) => item.k === 'let')
        .map((item) => `let ${item.name} := ${this.expr(item.value)} in `).join('');
      const name = `ml_assertion_${assertion}`;
      theorems.push(`Theorem ${name} : ${lets}${this.prop(effect.prop)}.\nProof. ml_decide. Qed.`);
      this.helpers.add('decide');
      this.state.assertionTheorem(name, effect);
      return build(index + 1);
    };
    const body = build(0);
    this.state.encode('program-output', 'main is the list of lines the source program prints, in order; evaluating it with vm_compute runs the program');
    return [...theorems, `Definition main : list string :=\n  ${body}.`].join('\n\n');
  }
}

function collectHintFunctions(plan) {
  if (plan.k === 'close') return plan.hints.unfold;
  return plan.cases.flatMap((kase) => collectHintFunctions(kase.plan));
}

