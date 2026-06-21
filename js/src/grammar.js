export class Grammar {
  constructor(start, rules) {
    this.start = start;
    this.rules = rules;
  }
}

export class GrammarBuilder {
  constructor(start) {
    this.start = start;
    this.rules = new Map();
  }

  terminal(name, expression) {
    this.rules.set(name, { kind: 'terminal', expression });
    return this;
  }

  nonterminal(name, expression) {
    this.rules.set(name, { kind: 'nonterminal', expression });
    return this;
  }

  build() {
    return new Grammar(this.start, new Map(this.rules));
  }

  static literal(value) {
    return { kind: 'literal', value };
  }

  static ref(name) {
    return { kind: 'ref', name };
  }

  static seq(...items) {
    return { kind: 'seq', items };
  }

  static choice(...items) {
    return { kind: 'choice', items };
  }

  static repeat0(item) {
    return { kind: 'repeat0', item };
  }

  static repeat1(item) {
    return { kind: 'repeat1', item };
  }

  static optional(item) {
    return { kind: 'optional', item };
  }

  static charRange(start, end) {
    return { kind: 'charRange', start, end };
  }

  static charClass(value) {
    return { kind: 'charClass', value };
  }

  static any() {
    return { kind: 'any' };
  }
}

export const ExprBuilder = GrammarBuilder;

export function emitPeggy(grammar) {
  const lines = [`start = ${grammar.start}`, ''];
  for (const [name, rule] of grammar.rules) {
    lines.push(`${name} = ${emitExpression(rule.expression)}`);
  }
  return `${lines.join('\n')}\n`;
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

function emitExpression(expression) {
  switch (expression.kind) {
    case 'literal':
      return JSON.stringify(expression.value);
    case 'ref':
      return expression.name;
    case 'seq':
      return expression.items.map((item) => parenthesize(item, 'seq')).join(' ');
    case 'choice':
      return expression.items.map((item) => parenthesize(item, 'choice')).join(' / ');
    case 'repeat0':
      return `${parenthesize(expression.item, 'suffix')}*`;
    case 'repeat1':
      return `${parenthesize(expression.item, 'suffix')}+`;
    case 'optional':
      return `${parenthesize(expression.item, 'suffix')}?`;
    case 'charRange':
      return `[${escapeClassChar(expression.start)}-${escapeClassChar(expression.end)}]`;
    case 'charClass':
      return `[${expression.value}]`;
    case 'any':
      return '.';
    default:
      throw new Error(`unsupported grammar expression kind: ${expression.kind}`);
  }
}

function parenthesize(expression, context) {
  const rendered = emitExpression(expression);
  if (
    context === 'suffix' &&
    ['literal', 'ref', 'charRange', 'charClass', 'any'].includes(expression.kind)
  ) {
    return rendered;
  }
  if (context === 'seq' && !['choice'].includes(expression.kind)) {
    return rendered;
  }
  if (context === 'choice' && !['choice'].includes(expression.kind)) {
    return rendered;
  }
  return `(${rendered})`;
}

function escapeClassChar(value) {
  return String(value).replace(/\\/g, '\\\\').replace(/]/g, '\\]').replace(/-/g, '\\-');
}
