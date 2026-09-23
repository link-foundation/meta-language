import peggy from 'peggy';

/** An order-preserving, serializable grammar intermediate representation. */
export class Grammar {
  constructor(start, rules, sourceFormat = null) {
    this.start = start ?? null;
    this.sourceFormat = sourceFormat;
    this.rules = new Map();
    for (const [name, rule] of rules ?? []) {
      this.rules.set(name, Object.freeze({
        name,
        kind: rule.kind ?? 'normal',
        expression: rule.expression,
      }));
    }
  }

  rule(name) {
    return this.rules.get(name);
  }

  ruleNames() {
    return [...this.rules.keys()];
  }

  rule_names() {
    return this.ruleNames();
  }

  startRule() {
    return this.start === null
      ? this.rules.values().next().value
      : this.rule(this.start);
  }

  start_rule() {
    return this.startRule();
  }

  source_format() {
    return this.sourceFormat;
  }

  referencedNonterminals() {
    const names = new Set();
    for (const rule of this.rules.values()) collectReferences(rule.expression, names);
    return [...names].sort();
  }

  referenced_nonterminals() {
    return this.referencedNonterminals();
  }

  undefinedNonterminals(allowed = []) {
    const permitted = new Set([...this.rules.keys(), ...allowed]);
    return this.referencedNonterminals().filter((name) => !permitted.has(name));
  }

  undefined_nonterminals(allowed = []) {
    return this.undefinedNonterminals(allowed);
  }

  normalized() {
    return {
      schemaVersion: 1,
      start: this.start,
      sourceFormat: this.sourceFormat,
      rules: [...this.rules.values()].map(({ name, kind, expression }) => ({
        name,
        kind,
        expression: cloneJson(expression),
      })),
    };
  }
}

export class GrammarBuilder {
  constructor(start = null) {
    this.start = start;
    this.sourceFormat = null;
    this.rules = new Map();
  }

  source(format) {
    this.sourceFormat = format;
    return this;
  }

  rule(name, expression, kind = 'normal') {
    this.rules.set(name, { name, kind, expression });
    return this;
  }

  terminal(name, expression) {
    return this.rule(name, expression, 'terminal');
  }

  nonterminal(name, expression) {
    return this.rule(name, expression, 'nonterminal');
  }

  build() {
    return new Grammar(this.start, this.rules, this.sourceFormat);
  }

  static empty() {
    return { kind: 'empty' };
  }

  static literal(value) {
    return { kind: 'literal', value };
  }

  static literalInsensitive(value) {
    return { kind: 'literalInsensitive', value };
  }

  static ref(name) {
    return { kind: 'ref', name };
  }

  static seq(...items) {
    return sequence(items);
  }

  static choice(...items) {
    return choice(items, false);
  }

  static orderedChoice(...items) {
    return choice(items, true);
  }

  static repeat0(item) {
    return { kind: 'repeat0', item };
  }

  static repeat1(item) {
    return { kind: 'repeat1', item };
  }

  static repeat(item, min, max = null) {
    return canonicalRepeat(item, min, max);
  }

  static optional(item) {
    return { kind: 'optional', item };
  }

  static and(item) {
    return { kind: 'and', item };
  }

  static not(item) {
    return { kind: 'not', item };
  }

  static capture(label, item) {
    return { kind: 'capture', label, item };
  }

  static charRange(start, end) {
    return { kind: 'charRange', start, end };
  }

  static charClass(value, negated = false) {
    if (typeof value === 'string') return { kind: 'charClass', value, negated };
    return { kind: 'charClass', items: value, negated };
  }

  static regex(value) {
    return { kind: 'regex', value };
  }

  static any() {
    return { kind: 'any' };
  }
}

export const ExprBuilder = GrammarBuilder;

export function emitPeggy(grammar) {
  const names = namePlan(grammar);
  const start = grammar.start ?? grammar.startRule()?.name;
  if (!start) throw new Error('cannot emit a grammar without a start rule');
  const lines = [`start = ${names.get(start) ?? sanitizeIdentifier(start)}`, ''];
  for (const [name, rule] of grammar.rules) {
    lines.push(`${names.get(name)} = ${emitExpression(rule.expression, names)}`);
  }
  return `${lines.join('\n')}\n`;
}

export function compileGrammar(grammar, options = {}) {
  return peggy.generate(emitPeggy(grammar), options);
}

export function parseWithGrammar(grammar, source, options = {}) {
  return compileGrammar(grammar).parse(source, options);
}

export function emitJavascriptParser(grammar) {
  const peggyGrammar = JSON.stringify(emitPeggy(grammar));
  return [
    "import peggy from 'peggy';",
    '',
    `const GRAMMAR = ${peggyGrammar};`,
    'export const parser = peggy.generate(GRAMMAR);',
    'export function parse(source, options = {}) {',
    '  return parser.parse(source, options);',
    '}',
    '',
  ].join('\n');
}

export function serializeGrammar(grammar) {
  return `${JSON.stringify(grammar.normalized(), null, 2)}\n`;
}

export function deserializeGrammar(source) {
  let value;
  try {
    value = typeof source === 'string' ? JSON.parse(source) : source;
  } catch (error) {
    throw new TypeError(`invalid serialized grammar JSON: ${error.message}`, { cause: error });
  }
  if (!value || value.schemaVersion !== 1 || !Array.isArray(value.rules)) {
    throw new TypeError('invalid serialized grammar document');
  }
  const rules = new Map();
  for (const rule of value.rules) {
    if (!rule || typeof rule.name !== 'string' || !rule.expression?.kind) {
      throw new TypeError('invalid serialized grammar rule');
    }
    if (rules.has(rule.name)) throw new TypeError(`duplicate serialized grammar rule ${rule.name}`);
    rules.set(rule.name, rule);
  }
  const grammar = new Grammar(value.start, rules, value.sourceFormat ?? null);
  if (!grammar.startRule()) throw new TypeError('serialized grammar has no start rule');
  return grammar;
}

export function sequence(items) {
  const flattened = [];
  for (const item of items) {
    if (item.kind === 'empty') continue;
    if (item.kind === 'seq') flattened.push(...item.items);
    else flattened.push(item);
  }
  if (flattened.length === 0) return GrammarBuilder.empty();
  if (flattened.length === 1) return flattened[0];
  return { kind: 'seq', items: flattened };
}

export function choice(items, ordered = false) {
  const flattened = [];
  for (const item of items) {
    if (item.kind === 'choice' && item.ordered === ordered) flattened.push(...item.items);
    else flattened.push(item);
  }
  if (flattened.length === 0) return GrammarBuilder.empty();
  if (flattened.length === 1) return flattened[0];
  return { kind: 'choice', items: flattened, ordered };
}

export function canonicalRepeat(item, min, max = null) {
  if (!Number.isSafeInteger(min) || min < 0 || (max !== null &&
    (!Number.isSafeInteger(max) || max < min))) {
    throw new RangeError(`invalid repetition bounds ${min}..${max ?? ''}`);
  }
  if (min === 0 && max === null) return GrammarBuilder.repeat0(item);
  if (min === 1 && max === null) return GrammarBuilder.repeat1(item);
  if (min === 0 && max === 1) return GrammarBuilder.optional(item);
  return { kind: 'repeat', item, min, max };
}

function emitExpression(expression, names) {
  switch (expression.kind) {
    case 'empty': return '""';
    case 'literal': return JSON.stringify(expression.value);
    case 'literalInsensitive': return `${JSON.stringify(expression.value)}i`;
    case 'ref': return names.get(expression.name) ?? sanitizeIdentifier(expression.name);
    case 'seq': return expression.items.map((item) => parenthesize(item, 'seq', names)).join(' ');
    case 'choice':
      return expression.items.map((item) => parenthesize(item, 'choice', names)).join(' / ');
    case 'repeat0': return `${parenthesize(expression.item, 'suffix', names)}*`;
    case 'repeat1': return `${parenthesize(expression.item, 'suffix', names)}+`;
    case 'optional': return `${parenthesize(expression.item, 'suffix', names)}?`;
    case 'repeat': return emitRepeat(expression, names);
    case 'and': return `&${parenthesize(expression.item, 'prefix', names)}`;
    case 'not': return `!${parenthesize(expression.item, 'prefix', names)}`;
    case 'capture': {
      const label = sanitizeIdentifier(expression.label ?? 'capture');
      return `${label}:${parenthesize(expression.item, 'prefix', names)}`;
    }
    case 'charRange':
      return `[${escapeClassChar(expression.start)}-${escapeClassChar(expression.end)}]`;
    case 'charClass': return emitCharClass(expression);
    case 'regex': return `(${expression.value})`;
    case 'any': return '.';
    default: throw new Error(`unsupported grammar expression kind: ${expression.kind}`);
  }
}

function emitRepeat(expression, names) {
  const item = parenthesize(expression.item, 'suffix', names);
  if (expression.max === expression.min) return `${item}|${expression.min}|`;
  return `${item}|${expression.min}..${expression.max ?? ''}|`;
}

function parenthesize(expression, context, names) {
  const rendered = emitExpression(expression, names);
  const atoms = ['empty', 'literal', 'literalInsensitive', 'ref', 'charRange', 'charClass', 'any'];
  if ((context === 'suffix' || context === 'prefix') && atoms.includes(expression.kind)) {
    return rendered;
  }
  if (context === 'seq' && expression.kind !== 'choice') return rendered;
  if (context === 'choice' && expression.kind !== 'choice') return rendered;
  return `(${rendered})`;
}

function emitCharClass(expression) {
  const prefix = expression.negated ? '^' : '';
  if (typeof expression.value === 'string') return `[${prefix}${expression.value}]`;
  const content = expression.items.map((item) => item.kind === 'range'
    ? `${escapeClassChar(item.start)}-${escapeClassChar(item.end)}`
    : escapeClassChar(item.value)).join('');
  return `[${prefix}${content}]`;
}

function namePlan(grammar) {
  const plan = new Map();
  const used = new Set(['start']);
  for (const name of [...grammar.ruleNames(), ...grammar.referencedNonterminals()]) {
    if (plan.has(name)) continue;
    const base = sanitizeIdentifier(name);
    let candidate = base;
    for (let suffix = 2; used.has(candidate); suffix += 1) candidate = `${base}_${suffix}`;
    used.add(candidate);
    plan.set(name, candidate);
  }
  return plan;
}

function sanitizeIdentifier(name) {
  const normalized = String(name).replace(/[^A-Za-z0-9_]/g, '_');
  return /^[A-Za-z_]/.test(normalized) ? normalized : `_${normalized}`;
}

function collectReferences(expression, names) {
  if (!expression) return;
  if (expression.kind === 'ref') names.add(expression.name);
  for (const item of expression.items ?? []) collectReferences(item, names);
  if (expression.item) collectReferences(expression.item, names);
}

function escapeClassChar(value) {
  return String(value)
    .replace(/\\/g, '\\\\')
    .replace(/\n/g, '\\n')
    .replace(/\r/g, '\\r')
    .replace(/\t/g, '\\t')
    .replace(/]/g, '\\]')
    .replace(/-/g, '\\-')
    .replace(/\^/g, '\\^');
}

function cloneJson(value) {
  return JSON.parse(JSON.stringify(value));
}
