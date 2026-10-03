// Loading a grammar for the native executor: imports are resolved through
// the injected `resolveGrammar` (no file system or network access), macros
// are expanded, parameterized rules are instantiated, every operation is
// checked against the context it runs in, and the terminals are compiled to
// byte matchers. The result is the immutable program executor.js runs.
// docs/grammar/feature-union.md specifies each step.
import { OPERATION_CONTEXTS } from './operations.js';
import { decodeAt, encodeText, foldCase, unicodePropertyMatcher } from './text.js';

/** A grammar that cannot be loaded or a parse that cannot run; `reason` names the step. */
export class GrammarRuntimeError extends Error {
  constructor(message, reason) {
    super(message);
    this.name = 'GrammarRuntimeError';
    this.reason = reason;
  }
}

const INSTANCE_LIMIT = 1000;
const RULE_KINDS = { normal: 'normal', nonterminal: 'normal', token: 'token', terminal: 'token', atomic: 'atomic', silent: 'silent' };

function loadError(reason, message) {
  throw new GrammarRuntimeError(message, reason);
}

// The parts of a Grammar instance or of its normalized document.
function grammarParts(value) {
  let grammar = value;
  if (typeof grammar === 'string') {
    try {
      grammar = JSON.parse(grammar);
    } catch {
      loadError('import', 'a resolved grammar is a Grammar or its serialized document');
    }
  }
  if (!grammar || typeof grammar !== 'object') loadError('import', 'a resolved grammar is a Grammar or its serialized document');
  const rules = grammar.rules instanceof Map ? [...grammar.rules.values()] : grammar.rules;
  if (!Array.isArray(rules)) loadError('import', 'a resolved grammar has no rules');
  return {
    start: grammar.start ?? rules[0]?.name ?? null,
    sourceFormat: grammar.sourceFormat ?? null,
    declarations: grammar.declarations ?? {},
    rules,
  };
}

// Imports: every imported grammar's rules first (recursively, in import
// order), then the local rules, which override imported rules of the same
// name in place; declarations accumulate, and a local macro overrides an
// imported one of the same name.
function resolveImports(value, context, chain) {
  const parts = grammarParts(value);
  const rules = new Map();
  const declarations = { modes: [], extras: [], conflicts: [], macros: new Map(), scanners: [] };
  for (const name of parts.declarations.imports ?? []) {
    if (chain.includes(name)) loadError('import', `import cycle ${[...chain, name].join(' -> ')}`);
    const imported = context.resolve(name, 'import');
    const resolved = resolveImports(imported, context, [...chain, name]);
    for (const [ruleName, rule] of resolved.rules) rules.set(ruleName, rule);
    declarations.modes.push(...resolved.declarations.modes);
    declarations.extras.push(...resolved.declarations.extras);
    declarations.conflicts.push(...resolved.declarations.conflicts);
    for (const [macroName, macro] of resolved.declarations.macros) declarations.macros.set(macroName, macro);
    declarations.scanners.push(...resolved.declarations.scanners);
  }
  for (const rule of parts.rules) rules.set(rule.name, rule);
  const local = parts.declarations;
  declarations.modes.push(...(local.modes ?? []));
  declarations.extras.push(...(local.extras ?? []));
  declarations.conflicts.push(...(local.conflicts ?? []));
  for (const macro of local.macros ?? []) declarations.macros.set(macro.name, macro);
  declarations.scanners.push(...(local.scanners ?? []));
  return { parts, rules, declarations };
}

/** Maps the expressions inside an operation. */
function mapOperation(operation, map) {
  const copy = {};
  for (const [key, value] of Object.entries(operation)) {
    if (key === 'item') copy[key] = map(value);
    else if (Array.isArray(value)) copy[key] = value.map((item) => mapOperation(item, map));
    else if (value && typeof value === 'object') copy[key] = mapOperation(value, map);
    else copy[key] = value;
  }
  return copy;
}

/** Maps the direct sub-expressions of an expression (character and byte class items are not expressions). */
export function mapChildren(expression, map) {
  if (expression.kind === 'charClass' || expression.kind === 'byteClass') return expression;
  const copy = { ...expression };
  if (expression.item) copy.item = map(expression.item);
  if (expression.synchronize) copy.synchronize = map(expression.synchronize);
  if (expression.items) copy.items = expression.items.map(map);
  if (expression.arguments) copy.arguments = expression.arguments.map(map);
  if (expression.condition) copy.condition = mapOperation(expression.condition, map);
  return copy;
}

/** Visits an expression and everything below it, operations included. */
export function visitExpression(expression, visit) {
  visit(expression);
  mapChildren(expression, (child) => {
    visitExpression(child, visit);
    return child;
  });
}

function visitOperations(operations, visit) {
  for (const operation of operations) mapOperation(operation, (expression) => {
    visitExpression(expression, visit);
    return expression;
  });
}

function substitute(expression, bindings) {
  if (expression.kind === 'parameter' && bindings.has(expression.name)) return bindings.get(expression.name);
  return mapChildren(expression, (child) => substitute(child, bindings));
}

// Macros expand inline at load: `expand(NAME, ARG...)` becomes the macro body
// with each `parameter(P)` replaced by its argument.
function expandMacros(expression, macros, active) {
  if (expression.kind === 'expand') {
    const macro = macros.get(expression.name);
    if (!macro) loadError('macro', `undefined macro ${expression.name}`);
    if (active.includes(expression.name)) loadError('macro', `recursive macro ${[...active, expression.name].join(' -> ')}`);
    const parameters = macro.parameters ?? [];
    if (parameters.length !== expression.arguments.length) {
      loadError('macro', `macro ${expression.name} takes ${parameters.length} arguments, not ${expression.arguments.length}`);
    }
    const bindings = new Map(parameters.map((name, index) => [name, expandMacros(expression.arguments[index], macros, active)]));
    return expandMacros(substitute(macro.expression, bindings), macros, [...active, expression.name]);
  }
  return mapChildren(expression, (child) => expandMacros(child, macros, active));
}

/** Loads `grammar` into an executable program; `options.resolveGrammar(name)` resolves imports and embedded languages. */
export function loadProgram(grammar, options = {}) {
  const context = {
    languages: new Map(),
    resolve(name, purpose) {
      const resolved = typeof options.resolveGrammar === 'function' ? options.resolveGrammar(name) : undefined;
      if (resolved === undefined || resolved === null) {
        loadError(purpose, `${purpose === 'import' ? 'imported grammar' : 'embedded language'} ${name} cannot be resolved`);
      }
      return resolved;
    },
  };
  return loadInContext(grammar, context);
}

function loadInContext(grammar, context) {
  const { parts, rules: sourceRules, declarations } = resolveImports(grammar, context, []);
  const macros = declarations.macros;
  const expand = (expression) => expandMacros(expression, macros, []);
  const expandOperations = (operations) => operations.map((operation) => mapOperation(operation, expand));

  const externalTokens = new Map();
  for (const scanner of declarations.scanners) {
    for (const token of scanner.tokens) {
      if (externalTokens.has(token)) loadError('declaration', `external token ${token} is declared twice`);
      if (sourceRules.has(token)) loadError('declaration', `external token ${token} is also a rule`);
      externalTokens.set(token, scanner);
    }
  }
  const modes = new Set(['default', ...declarations.modes]);

  const ruleSources = new Map();
  for (const [name, rule] of sourceRules) {
    const kind = RULE_KINDS[rule.kind ?? 'normal'];
    if (!kind) loadError('declaration', `rule ${name} has an unknown kind ${rule.kind}`);
    ruleSources.set(name, {
      ...rule,
      kind,
      expression: expand(rule.expression),
      action: rule.action ? expandOperations(rule.action) : undefined,
    });
  }

  // Parameterized rules are instantiated per distinct argument list; an
  // instance is a rule named `NAME(ARGUMENT, ...)` whose nodes keep the kind NAME.
  const rules = new Map();
  const pending = [];
  const instantiate = (expression, bindings, owner) => {
    if (expression.kind === 'parameter') {
      if (!bindings?.has(expression.name)) loadError('parameter', `parameter ${expression.name} is not a parameter of ${owner}`);
      return bindings.get(expression.name);
    }
    if (expression.kind === 'ref') {
      const args = expression.arguments ?? [];
      if (externalTokens.has(expression.name)) {
        if (args.length > 0) loadError('parameter', `external token ${expression.name} takes no arguments`);
        return { kind: 'ref', name: expression.name };
      }
      const target = ruleSources.get(expression.name);
      if (!target) loadError('reference', `undefined rule ${expression.name} in ${owner}`);
      const arity = target.parameters?.length ?? 0;
      if (args.length !== arity) loadError('parameter', `rule ${expression.name} takes ${arity} arguments, not ${args.length}`);
      if (arity === 0) return { kind: 'ref', name: expression.name };
      const concrete = args.map((argument) => instantiate(argument, bindings, owner));
      const name = `${expression.name}(${concrete.map((argument) => JSON.stringify(argument)).join(', ')})`;
      if (!rules.has(name) && !pending.some((item) => item.name === name)) {
        if (rules.size + pending.length >= INSTANCE_LIMIT + ruleSources.size) {
          loadError('parameter', `parameterized rule ${expression.name} needs more than ${INSTANCE_LIMIT} instances`);
        }
        pending.push({ name, target, bindings: new Map(target.parameters.map((parameter, index) => [parameter, concrete[index]])) });
      }
      return { kind: 'ref', name };
    }
    return mapChildren(expression, (child) => instantiate(child, bindings, owner));
  };
  const instantiateOperations = (operations, owner) => operations.map((operation) => mapOperation(operation, (expression) => instantiate(expression, null, owner)));

  for (const [name, rule] of ruleSources) {
    if (rule.parameters?.length > 0) continue;
    rules.set(name, {
      ...rule,
      nodeKind: name,
      expression: instantiate(rule.expression, null, `rule ${name}`),
      action: rule.action ? instantiateOperations(rule.action, `rule ${name}`) : undefined,
    });
  }
  while (pending.length > 0) {
    const { name, target, bindings } = pending.shift();
    rules.set(name, {
      ...target,
      parameters: undefined,
      nodeKind: target.name,
      expression: instantiate(target.expression, bindings, `rule ${target.name}`),
      action: target.action ? target.action.map((operation) => mapOperation(operation, (expression) => instantiate(expression, bindings, `rule ${target.name}`))) : undefined,
    });
  }

  const start = parts.start;
  if (!ruleSources.has(start)) loadError('reference', `undefined start rule ${start}`);
  if (ruleSources.get(start).parameters?.length > 0) loadError('parameter', `start rule ${start} is parameterized`);

  const extras = declarations.extras.map((extra) => instantiate(expand(extra), null, 'an extra'));
  const scanners = new Map();
  for (const scanner of declarations.scanners) {
    const compiled = { ...scanner, operations: instantiateOperations(expandOperations(scanner.operations), `scanner ${scanner.name}`) };
    for (const token of scanner.tokens) scanners.set(token, compiled);
  }

  const conflicts = new Set();
  for (const group of declarations.conflicts) {
    for (const name of group) {
      if (!ruleSources.has(name)) loadError('declaration', `conflict names undefined rule ${name}`);
      conflicts.add(name);
    }
  }

  const matching = parts.declarations.matching ?? (parts.sourceFormat === 'peg' ? 'peg' : 'generalized');
  const program = {
    matching,
    start,
    rules,
    externalTokens,
    scanners,
    conflicts,
    modes,
    trivia: [],
    matchers: new WeakMap(),
    language: (name) => context.languages.get(name),
  };

  // Checks and compiles every expression and operation.
  const embedded = new Set();
  const checkMode = (mode, owner) => {
    if (!modes.has(mode)) loadError('declaration', `undeclared mode ${mode} in ${owner}`);
  };
  const checkExpression = (expression, owner) => visitExpression(expression, (item) => {
    compileMatcher(item, program.matchers);
    if (item.kind === 'embed') embedded.add(item.language);
    if (item.kind === 'expand' || item.kind === 'parameter') loadError('macro', `${item.kind} ${item.name} survives loading in ${owner}`);
    if (item.kind === 'predicate') checkOperations([item.condition], 'predicate', owner);
    if (item.kind === 'recover' && item.synchronize === undefined) loadError('declaration', `recover in ${owner} has no synchronization`);
  });
  const checkOperations = (operations, operationContext, owner, tokens = null) => {
    const allowed = OPERATION_CONTEXTS[operationContext];
    const walk = (operation) => {
      if (!allowed.has(operation.operation)) loadError('operation', `${operation.operation} cannot run in a ${operationContext} (${owner})`);
      if (operation.operation === 'pushMode' || operation.operation === 'setMode') checkMode(operation.mode, owner);
      if ((operation.operation === 'emit' || operation.operation === 'valid') && tokens && !tokens.includes(operation.token)) {
        loadError('operation', `${operation.operation} names ${operation.token}, which ${owner} does not produce`);
      }
      for (const [key, value] of Object.entries(operation)) {
        if (key === 'item') checkExpression(value, owner);
        else if (Array.isArray(value)) value.forEach(walk);
        else if (value && typeof value === 'object') walk(value);
      }
    };
    operations.forEach(walk);
  };

  // Trivia: the `extra` expressions first, in declaration order, then every
  // rule on a channel other than `default`, each limited to its modes.
  for (const extra of extras) {
    checkExpression(extra, 'an extra');
    program.trivia.push({ expression: extra, kind: extra.kind === 'ref' ? extra.name : null, modes: null });
  }
  let index = 0;
  for (const [name, rule] of rules) {
    rule.index = index;
    index += 1;
    const owner = `rule ${rule.nodeKind}`;
    checkExpression(rule.expression, owner);
    if (rule.action) {
      if (rule.kind === 'silent' && rule.action.some(buildsNode)) loadError('operation', `silent ${owner} has no node for its action`);
      checkOperations(rule.action, 'action', owner);
    }
    for (const mode of rule.modes ?? []) checkMode(mode, owner);
    rule.lexicalPriority = rule.expression.kind === 'lexicalPrecedence' ? rule.expression.level : 0;
    if (rule.channel !== undefined && rule.channel !== null && rule.channel !== 'default') {
      program.trivia.push({ expression: { kind: 'ref', name }, kind: rule.nodeKind, modes: rule.modes ?? null });
    }
  }
  for (const scanner of new Set(scanners.values())) {
    checkOperations(scanner.operations, 'scanner', `scanner ${scanner.name}`, scanner.tokens);
  }
  if (matching === 'longest') program.tokenRanks = tokenRanks(rules);

  for (const language of embedded) {
    if (context.languages.has(language)) continue;
    context.languages.set(language, null);
    context.languages.set(language, loadInContext(context.resolve(language, 'embed'), context));
  }
  return program;
}

// The rank of each token of a `(matching longest)` grammar, by which two
// tokens over the same text conflict as a lexer orders them: the higher
// lexical precedence, then (at one length) a literal over a pattern, an
// immediate token by one more, then the earlier token. Tokens are numbered in
// rule order as they first appear: a token rule at its own definition, a
// literal, an inline token and an alias of either where they are used. The
// ranks are keyed by leaf kind (`kinds`) and, for a literal leaf, which has
// none, by text (`literals`).
function tokenRanks(rules) {
  const kinds = new Map();
  const literals = new Map();
  const aliased = [];
  let order = 0;
  const assign = (map, key, rank) => {
    if (map.has(key)) return;
    map.set(key, { ...rank, order });
    order += 1;
  };
  // A literal closed by lookaheads, such as a keyword, ranks as the literal.
  const closed = (expression) => {
    if (expression.kind !== 'seq') return expression;
    const items = expression.items.filter((item) => item.kind !== 'not' && item.kind !== 'and');
    return items.length === 1 ? closed(items[0]) : expression;
  };
  const rankOf = (expression, priority = 0) => {
    expression = closed(expression);
    if (expression.kind === 'lexicalPrecedence') return rankOf(expression.item, expression.level);
    if (expression.kind === 'immediateToken' || expression.kind === 'token') {
      const inner = rankOf(expression.item, priority);
      return { priority: inner.priority, specificity: inner.specificity + (expression.kind === 'immediateToken' ? 1 : 0) };
    }
    return { priority, specificity: expression.kind === 'literal' ? 2 : 0 };
  };
  const lexical = (expression) => ['literal', 'token', 'immediateToken'].includes(expression.kind)
    || (expression.kind === 'lexicalPrecedence' && lexical(expression.item));
  const bare = (expression) => {
    const inner = closed(expression);
    if (inner.kind === 'literal') return inner;
    return ['lexicalPrecedence', 'token', 'immediateToken'].includes(inner.kind) ? bare(inner.item) : null;
  };
  const walk = (expression) => {
    if (expression.kind === 'alias' && expression.item.kind === 'ref') {
      aliased.push(expression);
    } else if (expression.kind === 'alias' && lexical(expression.item)) {
      assign(kinds, expression.name, rankOf(expression.item));
    } else if (lexical(expression)) {
      const literal = bare(expression);
      if (literal) assign(literals, literal.value, rankOf(expression));
    } else {
      // A predicate condition consumes no text, so its tokens are not ranked.
      mapChildren(expression.condition ? { ...expression, condition: null } : expression, (child) => {
        walk(child);
        return child;
      });
    }
  };
  for (const rule of rules.values()) {
    if (rule.kind === 'token') assign(kinds, rule.nodeKind, rankOf(rule.expression));
    else walk(rule.expression);
  }
  for (const alias of aliased) {
    const rule = rules.get(alias.item.name);
    if (rule?.kind === 'token' && !kinds.has(alias.name)) kinds.set(alias.name, kinds.get(rule.nodeKind));
  }
  return { kinds, literals };
}

function buildsNode(operation) {
  if (operation.operation === 'setAttribute' || operation.operation === 'buildNode') return true;
  return [operation.consequent, operation.alternative, operation.body].some((block) => block?.some(buildsNode));
}

// Terminal matchers: `(bytes, position, end) => end of the match or -1`.
function compileMatcher(expression, matchers) {
  if (matchers.has(expression)) return;
  const matcher = terminalMatcher(expression);
  if (matcher) matchers.set(expression, matcher);
}

function codePointMatcher(test) {
  return (bytes, position, end) => {
    if (position >= end) return -1;
    const { codePoint, length } = decodeAt(bytes, position, end);
    return codePoint >= 0 && test(codePoint) ? position + length : -1;
  };
}

function classItemTest(item) {
  switch (item.kind) {
    case 'char': return (codePoint) => codePoint === item.value.codePointAt(0);
    case 'range': {
      const low = item.start.codePointAt(0);
      const high = item.end.codePointAt(0);
      return (codePoint) => codePoint >= low && codePoint <= high;
    }
    case 'category': case 'script': {
      const pattern = unicodePropertyMatcher(item.kind, item.value);
      if (!pattern) loadError('pattern', `unknown Unicode ${item.kind} ${item.value}`);
      return (codePoint) => pattern.test(String.fromCodePoint(codePoint));
    }
    default: return loadError('pattern', `unknown character class item ${item.kind}`);
  }
}

function terminalMatcher(expression) {
  switch (expression.kind) {
    case 'literal': {
      const literal = encodeText(expression.value);
      return (bytes, position, end) => {
        if (position + literal.length > end) return -1;
        for (let index = 0; index < literal.length; index += 1) {
          if (bytes[position + index] !== literal[index]) return -1;
        }
        return position + literal.length;
      };
    }
    case 'literalInsensitive': {
      const expected = foldCase(expression.value);
      const count = [...expression.value].length;
      return (bytes, position, end) => {
        let cursor = position;
        let text = '';
        for (let index = 0; index < count; index += 1) {
          if (cursor >= end) return -1;
          const { codePoint, length } = decodeAt(bytes, cursor, end);
          if (codePoint < 0) return -1;
          text += String.fromCodePoint(codePoint);
          cursor += length;
        }
        return foldCase(text) === expected ? cursor : -1;
      };
    }
    case 'charRange': {
      const low = expression.start.codePointAt(0);
      const high = expression.end.codePointAt(0);
      return codePointMatcher((codePoint) => codePoint >= low && codePoint <= high);
    }
    case 'charClass': {
      if (typeof expression.value === 'string') {
        let pattern;
        try {
          pattern = new RegExp(`^[${expression.negated ? '^' : ''}${expression.value}]$`, 'u');
        } catch {
          loadError('pattern', `invalid character class [${expression.value}]`);
        }
        return codePointMatcher((codePoint) => pattern.test(String.fromCodePoint(codePoint)));
      }
      const tests = expression.items.map(classItemTest);
      const negated = expression.negated === true;
      return codePointMatcher((codePoint) => tests.some((test) => test(codePoint)) !== negated);
    }
    case 'byteClass': {
      const negated = expression.negated === true;
      const contains = (byte) => expression.items.some((item) => (item.kind === 'byteRange'
        ? byte >= item.start && byte <= item.end
        : byte === item.value));
      return (bytes, position, end) => (position < end && contains(bytes[position]) !== negated ? position + 1 : -1);
    }
    case 'any':
      return (bytes, position, end) => (position < end ? position + decodeAt(bytes, position, end).length : -1);
    case 'regex': {
      let pattern;
      for (const flags of ['uy', 'y']) {
        try {
          pattern = new RegExp(expression.value, flags);
          break;
        } catch {
          pattern = null;
        }
      }
      if (!pattern) loadError('pattern', `invalid regular expression ${expression.value}`);
      return { regex: pattern };
    }
    default: return null;
  }
}
