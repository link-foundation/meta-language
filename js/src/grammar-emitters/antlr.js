// Mirrors rust/src/grammar/emit/antlr.rs: the ANTLR v4 grammar emitter. Output
// text, `report.lossy` notes and GrammarEmitError messages are byte-identical
// to the Rust `emit_antlr` for the same grammar.

import { importAntlr } from '../grammar-importers/antlr.js';
import { readLexerDeclaration } from '../grammar-runtime/lexer-election.js';
import { renderLinksExpression } from '../grammar-links.js';
import { acceptGrammarParserTokenNegation, readGrammarLexerCommandEmission, readGrammarLexerCommandPlan, readGrammarLexerEmissionIssue, readGrammarLexerEmissionDisposition } from '../translation/grammar-lexer-rules.js';
import { GrammarEmitError, codePoint, decorateEmitted, emitReport, finishLines, unsupportedError } from './common.js';
import {
  NamePlan,
  Precedence,
  asciiIdentifier,
  caseVariants,
  classItems,
  collectReferences,
  docLines,
  isAsciiAlphabetic,
  isAsciiUpper,
  isControl,
  joinChoice,
  joinSequence,
  lowerRepeat,
  negatedLookaheadSet,
  repeatBounds,
  ruleDoc,
  ruleKind,
  rustDebugCharLiteral,
  rustDebugOption,
  rustDebugString,
  rustTrim,
  wrap,
} from './structural.js';

const FORMAT = 'antlr';
const LABEL = 'ANTLR';

/** Lower-case words ANTLR reserves, which cannot name parser rules or labels. */
const RESERVED_WORDS = [
  'catch', 'channels', 'finally', 'fragment', 'grammar', 'import', 'lexer', 'locals', 'mode',
  'options', 'parser', 'returns', 'throws', 'tokens',
];

/** Token names ANTLR predefines, which cannot name lexer rules. */
const RESERVED_TOKENS = ['EOF'];

/**
 * Emits a combined ANTLR v4 grammar.
 *
 * Normal rules become parser rules (lower-case first letter), token and atomic
 * rules become lexer rules (upper-case first letter) and silent rules become
 * `fragment` lexer rules. Rule names are adjusted deterministically to ANTLR's
 * case conventions and reserved words; every rename and every lowering
 * (ordered choice, lookahead, case-insensitive literals, counted repetition,
 * captures) is recorded in `report.lossy`. Rule documentation is written as
 * comments, and a trailing `-> command` part of a lexer rule's documentation
 * is written back as the lexer command.
 *
 * Throws GrammarEmitError for empty non-negated character classes, descending
 * character ranges, repetitions whose maximum is below their minimum, and
 * counted repetitions that would expand to more than 256 copies.
 */
export function emitAntlr(grammar, options = {}) {
  return decorateEmitted('antlr', writeAntlr(grammar), options.decorators);
}

function writeAntlr(grammar) {
  return new AntlrEmitter(grammar).emit();
}

class AntlrEmitter {
  constructor(grammar) {
    this.grammar = grammar;
    this.rules = [...grammar.rules.values()];
    this.report = emitReport();
    const candidates = this.rules.map((rule) => ruleCandidate(rule.name, isLexerKind(ruleKind(rule))));
    this.names = new NamePlan(grammar, LABEL, candidates, [], referenceCandidate, this.report);
    this.foreign = grammar.sourceFormat !== FORMAT;
    this.rule = '';
    this.parserProduction = false;
    this.lexicalEmission = sourceFreeLexerEmission(grammar, this.rules, this.names);
  }

  emit() {
    const order = this.emissionOrder();
    this.noteStart(order);

    const hasParserRules = this.rules.some((rule) => ruleKind(rule) === 'normal');
    let grammarName = order.length === 0 ? '' : pascalCase(this.names.ruleName(order[0]));
    if (!isAsciiAlphabetic(grammarName[0])) grammarName = 'Grammar';
    const lines = [hasParserRules ? `grammar ${grammarName};` : `lexer grammar ${grammarName};`];
    if (order.length > 0) lines.push('');
    for (const index of order) this.emitRule(index, lines);
    return { source: finishLines(lines), report: this.report };
  }

  /** Orders rules so that `importAntlr` picks the same start rule (the first parser rule). */
  emissionOrder() {
    const { start } = this.grammar;
    const found = start === null || start === undefined
      ? -1
      : this.rules.findIndex((rule) => rule.name === start);
    const startIndex = found < 0 ? null : found;
    let effective;
    if (startIndex !== null && ruleKind(this.rules[startIndex]) === 'normal') {
      effective = startIndex;
    } else {
      const firstParser = this.rules.findIndex((rule) => ruleKind(rule) === 'normal');
      effective = firstParser >= 0 ? firstParser : startIndex;
    }
    const order = effective === null ? [] : [effective];
    this.rules.forEach((_rule, index) => {
      if (index !== effective) order.push(index);
    });
    if (this.lexicalEmission === null) return order;
    let lexical = 0;
    return order.map((index) => isLexerKind(ruleKind(this.rules[index]))
      ? this.lexicalEmission.order[lexical++] : index);
  }

  noteStart(order) {
    const { start } = this.grammar;
    if (start === null || start === undefined || order.length === 0) return;
    const expected = this.names.resolve(start);
    const actual = this.names.ruleName(order[0]);
    if (expected !== actual) {
      this.report.lossy.push(`ANTLR re-imports start rule as ${rustDebugString(actual)} instead of ${rustDebugString(expected)}, because ANTLR starts at the first parser rule`);
    }
  }

  emitRule(index, lines) {
    const rule = this.rules[index];
    const kind = ruleKind(rule);
    const name = this.names.ruleName(index);
    this.rule = name;
    this.parserProduction = kind === 'normal';
    const lexer = isLexerKind(kind);
    this.noteKind(rule, kind, name);

    const sourceDoc = ruleDoc(this.grammar, rule);
    const doc = planDoc(sourceDoc, lexer);
    if (doc.reimported !== sourceDoc) {
      this.report.lossy.push(`ANTLR re-imports the documentation of rule ${rustDebugString(name)} as ${rustDebugOption(doc.reimported)}`);
    }
    lines.push(...doc.comments);

    const lexical = this.lexicalEmission?.rules.get(rule.name);
    const [body] = this.render(lexical?.expression ?? rule.expression);
    let line = kind === 'silent' ? `fragment ${name} :` : `${name} :`;
    if (body !== '') line += ` ${body}`;
    const command = doc.command ?? lexical?.command ?? null;
    if (command !== null && command !== '') line += ` ${command}`;
    line += ' ;';
    lines.push(line);

    if (this.foreign) this.noteRulePlacement(rule, name, lexer);
  }

  noteKind(rule, kind, name) {
    if (kind === 'atomic') {
      this.report.lossy.push(`ANTLR emitted atomic rule ${rustDebugString(rule.name)} as lexer rule ${rustDebugString(name)}, which re-imports as a token rule`);
    } else if (kind === 'silent' && this.foreign) {
      this.report.lossy.push(`ANTLR emitted silent rule ${rustDebugString(rule.name)} as lexer fragment ${rustDebugString(name)}`);
    }
  }

  /**
   * Records rules that real ANTLR would reject: lexer rules referring to parser
   * rules, parser rules referring to fragments, and parser rules using
   * character-level constructs.
   */
  noteRulePlacement(rule, name, lexer) {
    for (const reference of collectReferences(rule.expression)) {
      const target = this.grammar.rule(reference);
      if (target === undefined) continue;
      const emitted = this.names.resolve(reference);
      const targetKind = ruleKind(target);
      if (lexer && targetKind === 'normal') {
        this.report.lossy.push(`ANTLR lexer rule ${rustDebugString(name)} references parser rule ${rustDebugString(emitted)}`);
      } else if (!lexer && targetKind === 'silent') {
        this.report.lossy.push(`ANTLR parser rule ${rustDebugString(name)} references lexer fragment ${rustDebugString(emitted)}`);
      }
    }
    if (!lexer && usesCharacterConstructs(rule.expression)) {
      this.report.lossy.push(`ANTLR parser rule ${rustDebugString(name)} uses character-level constructs that ANTLR matches only in lexer rules`);
    }
  }

  note(message) {
    this.report.lossy.push(`ANTLR ${message} in rule ${rustDebugString(this.rule)}`);
  }

  render(expression) {
    switch (expression.kind) {
      case 'empty':
        return ['', Precedence.SEQUENCE];
      case 'literal':
        if (expression.value === '') this.note('keeps an empty literal, which the ANTLR tool rejects,');
        return [quote(expression.value), Precedence.ATOM];
      case 'literalInsensitive':
        return this.renderInsensitive(expression.value);
      case 'charRange': {
        const { start, end } = expression;
        if (codePoint(start) > codePoint(end)) {
          throw unsupportedError(FORMAT, `descending character range ${rustDebugCharLiteral(start)}..${rustDebugCharLiteral(end)}`);
        }
        return [`${quote(start)}..${quote(end)}`, Precedence.ATOM];
      }
      case 'charClass':
        return this.renderClass(Boolean(expression.negated), classItems(FORMAT, expression));
      case 'any':
        return ['.', Precedence.ATOM];
      case 'ref':
        return [this.names.resolve(expression.name), Precedence.ATOM];
      case 'choice': {
        if (expression.ordered && expression.items.length > 1) {
          this.note('treats ordered choice as unordered choice');
        }
        return joinChoice(expression.items.map((alternative) => this.render(alternative)));
      }
      case 'seq':
        return this.renderSequence(expression.items);
      case 'optional':
        return this.renderPostfix(expression.item, '?');
      case 'repeat0':
        return this.renderPostfix(expression.item, '*');
      case 'repeat1':
        return this.renderPostfix(expression.item, '+');
      case 'repeat': {
        const max = expression.max ?? null;
        const lowered = lowerRepeat(FORMAT, expression.item, expression.min, max);
        this.note(`expanded counted repetition ${repeatBounds(expression.min, max)}`);
        return this.render(lowered);
      }
      case 'and':
        this.note('dropped positive lookahead');
        return ['', Precedence.SEQUENCE];
      case 'not':
        return this.renderNot(expression.item);
      case 'capture':
        return this.renderCapture(expression.label ?? null, expression.item);
      default:
        throw unsupportedError(FORMAT, expression.kind);
    }
  }

  renderSequence(items) {
    const rendered = [];
    let index = 0;
    while (index < items.length) {
      const prefix = items[index];
      if (!this.foreign && this.parserProduction && prefix.kind === 'not' && items[index + 1]?.kind === 'any') {
        const excluded = prefix.item;
        const operands = excluded.kind === 'choice' ? excluded.items : [excluded];
        const selected = acceptGrammarParserTokenNegation(true, operands.map(({ kind }) => kind),
          operands.map((item) => item.kind === 'ref' ? this.names.resolve(item.name)
            : item.value ?? (item.kind === 'not' ? item.item?.kind : '') ?? ''));
        if (selected) {
          rendered.push(this.renderNot(excluded));
          index += 2;
          continue;
        }
      }
      const set = this.foreign ? negatedLookaheadSet(FORMAT, items, index) : null;
      if (set !== null) {
        this.note('lowered negative lookahead followed by any character to a complemented set');
        rendered.push(renderSet(set[0], set[1]));
        index += 2;
        continue;
      }
      rendered.push(this.render(items[index]));
      index += 1;
    }
    return joinSequence(rendered);
  }

  renderPostfix(inner, operator) {
    const operand = this.render(inner);
    return [`${wrap(operand, Precedence.PREFIX)}${operator}`, Precedence.POSTFIX];
  }

  renderNot(inner) {
    if (!this.foreign && this.parserProduction && inner.kind === 'any') return ['EOF', Precedence.ATOM];
    if (inner.kind === 'charClass' && !inner.negated) {
      this.note('emitted negative lookahead over a character set as a complemented set');
    } else if (this.foreign) {
      this.note('emitted negative lookahead as the `~` complement, which consumes one symbol');
    }
    const operand = this.render(inner);
    return [`~${wrap(operand, Precedence.ATOM)}`, Precedence.PREFIX];
  }

  renderCapture(label, inner) {
    if (label === null) {
      this.note('dropped anonymous capture');
      return this.render(inner);
    }
    if (label === 'non_greedy' && ['optional', 'repeat0', 'repeat1'].includes(inner.kind)) {
      const [text] = this.render(inner);
      return [`${text}?`, Precedence.POSTFIX];
    }
    const emitted = labelCandidate(label);
    if (emitted !== label) {
      this.note(`renamed capture label ${rustDebugString(label)} to ${rustDebugString(emitted)}`);
    }
    if (label === 'regex' && this.grammar.sourceFormat === 'lark' && inner.kind === 'literal') {
      this.note(`emitted regex /${inner.value}/ as a literal labelled \`regex\`; regex semantics are not translated`);
    }
    const operand = this.render(inner);
    return [`${emitted}=${wrap(operand, Precedence.POSTFIX)}`, Precedence.LABELED];
  }

  renderInsensitive(value) {
    if (value === '') return this.render({ kind: 'literal', value: '' });
    const items = [];
    let literal = '';
    for (const character of value) {
      const variants = caseVariants(character);
      if (variants !== null) {
        if (literal !== '') {
          items.push({ kind: 'literal', value: literal });
          literal = '';
        }
        items.push({
          kind: 'charClass',
          negated: false,
          items: variants.map((variant) => ({ kind: 'char', value: variant })),
        });
      } else {
        literal += character;
      }
    }
    if (literal !== '') items.push({ kind: 'literal', value: literal });
    this.note(`expanded case-insensitive literal ${rustDebugString(value)} to per-letter character sets`);
    return this.render(items.length === 1 ? items[0] : { kind: 'seq', items });
  }

  renderClass(negated, items) {
    if (items.length === 0) {
      if (negated) {
        this.note('emitted an empty negated character class as `.`');
        return ['.', Precedence.ATOM];
      }
      throw unsupportedError(FORMAT, 'empty character class');
    }
    const descending = items.find((item) => item.kind === 'range' &&
      codePoint(item.start) > codePoint(item.end));
    if (descending !== undefined) {
      throw unsupportedError(FORMAT, `descending character class range ${rustDebugCharLiteral(descending.start)}-${rustDebugCharLiteral(descending.end)}`);
    }
    return renderSet(negated, items);
  }
}

/**
 * Recover executable commands only from a complete, unambiguous source-local
 * default lexer. Unchanged doc-backed rules keep their historical path. The
 * canonical public bodies can include applyRetypes aliases; when recovery is
 * needed, the ordered local declarations are the actual lexer definitions.
 */
function sourceFreeLexerEmission(grammar, rules, names) {
  if (grammar.sourceFormat !== FORMAT) return null;
  const lexers = grammar.declarations.lexers ?? [];
  const refuse = (reason) => { throw unsupportedError(FORMAT, `source-free lexical commands: ${reason}`); };
  const commandOf = (entry) => {
    const result = readGrammarLexerCommandEmission(entry.name, entry.type, entry.channel, entry.disposition,
      (entry.action ?? []).map(({ operation }) => operation), (entry.action ?? []).map(({ mode }) => mode ?? ''));
    if (result.length !== 1) refuse(`cannot replay the declaration of ${rustDebugString(entry.name)}`);
    return result[0];
  };
  let needsCommandRecovery = false;
  let needsSourceBody = false;
  for (const lexer of lexers) {
    if (lexer.expression?.kind !== 'seq') refuse('a lexer needs an ordered declaration list');
    for (const entry of lexer.expression.items) {
      if (entry.kind !== 'lexicalRule' && entry.kind !== 'lexicalFragment') continue;
      const rule = grammar.rule(entry.name);
      if (rule !== undefined && !entry.item) refuse('invalid lexical declaration descriptors');
      if (rule !== undefined && renderLinksExpression(rule.expression) !== renderLinksExpression(entry.item)) needsSourceBody = true;
      if (entry.kind !== 'lexicalRule') continue;
      if (rule !== undefined && planDoc(ruleDoc(grammar, rule), true).command !== null) continue;
      if (commandOf(entry) !== '') needsCommandRecovery = true;
    }
  }
  if (!needsCommandRecovery && !needsSourceBody) return null;
  let plans;
  try {
    plans = lexers.map((declaration) => readLexerDeclaration(declaration, refuse));
  } catch (error) {
    // Only descriptor validation is caught. Unsupported raw-doc grammars keep
    // their previous explicitly lossy path; missing commands never do.
    if (readGrammarLexerEmissionDisposition(needsCommandRecovery, 'invalid lexical declaration descriptors') === 0) return null;
    if (error instanceof GrammarEmitError) throw error;
    refuse('invalid lexical declaration descriptors');
  }
  const entries = lexers.flatMap((declaration) => declaration.expression.items);
  const indices = new Map(rules.map((rule, index) => [rule.name, index]));
  const locals = entries.filter((entry) => entry.kind === 'lexicalRule' || entry.kind === 'lexicalFragment');
  const issue = readGrammarLexerEmissionIssue(lexers.length, plans.reduce((sum, plan) => sum + plan.obligations.length, 0),
    plans.flatMap((plan) => [...plan.modes]), plans.flatMap((plan) => [...plan.channels]), plans.reduce((sum, plan) => sum + plan.types.size, 0),
    entries.map(({ kind }) => kind), entries.map(({ name }) => name ?? ''), entries.map(({ mode }) => mode ?? ''),
    entries.map(({ item }) => item?.kind === 'ref' ? item.name : ''),
    entries.map(({ name }) => indices.has(name) ? ruleKind(rules[indices.get(name)]) : ''),
    entries.map(({ name }) => indices.has(name) ? names.ruleName(indices.get(name)) : ''),
    rules.map(({ name }) => name), rules.map(ruleKind), locals.flatMap(({ item }) => collectReferences(item)));
  const disposition = readGrammarLexerEmissionDisposition(needsCommandRecovery, issue);
  if (disposition === 0) return null;
  if (disposition === 2) refuse(issue);
  const selected = new Map();
  const order = [];
  for (const entry of locals) {
    const index = indices.get(entry.name);
    const command = entry.kind === 'lexicalRule' ? commandOf(entry) : '';
    const original = planDoc(ruleDoc(grammar, rules[index]), true).command;
    if (original !== null && JSON.stringify(readGrammarLexerCommandPlan(original, entry.name))
      !== JSON.stringify(readGrammarLexerCommandPlan(command, entry.name))) refuse('rule documentation disagrees with its actual lexical declaration');
    selected.set(entry.name, { expression: entry.item, command });
    order.push(index);
  }
  return { rules: selected, order };
}

function isLexerKind(kind) {
  return kind !== 'normal';
}

/** Returns ANTLR's conventional spelling of a rule name. */
function ruleCandidate(name, lexer) {
  let output = asciiIdentifier(name);
  const first = output[0];
  if (isAsciiAlphabetic(first)) {
    output = (lexer ? first.toUpperCase() : first.toLowerCase()) + output.slice(1);
  } else {
    output = (lexer ? 'T' : 'r') + output;
  }
  if ((lexer ? RESERVED_TOKENS : RESERVED_WORDS).includes(output)) output += '_';
  return output;
}

/** Returns a valid spelling for a reference without a rule, keeping its case. */
function referenceCandidate(name) {
  const lexer = isAsciiUpper(name[0]);
  let output = asciiIdentifier(name);
  if (!isAsciiAlphabetic(output[0])) output = (lexer ? 'T' : 'r') + output;
  if (RESERVED_WORDS.includes(output)) output += '_';
  return output;
}

function labelCandidate(label) {
  let output = asciiIdentifier(label);
  if (!(isAsciiAlphabetic(output[0]) || output[0] === '_')) output = `l${output}`;
  if (RESERVED_WORDS.includes(output)) output += '_';
  return output;
}

function pascalCase(name) {
  return name.split('_')
    .filter((part) => part !== '')
    .map((part) => {
      const first = part[0];
      return (isAsciiAlphabetic(first) ? first.toUpperCase() : first) + part.slice(1);
    })
    .join('');
}

function usesCharacterConstructs(expression) {
  switch (expression.kind) {
    case 'literalInsensitive': case 'charRange': case 'charClass': case 'any':
      return true;
    case 'choice': case 'seq':
      return expression.items.some(usesCharacterConstructs);
    case 'optional': case 'repeat0': case 'repeat1': case 'and': case 'not':
    case 'repeat': case 'capture':
      return usesCharacterConstructs(expression.item);
    default:
      return false;
  }
}

/** Quotes an ANTLR string literal. */
function quote(value) {
  let output = "'";
  for (const character of value) {
    switch (character) {
      case "'": output += "\\'"; break;
      case '\\': output += '\\\\'; break;
      case '\n': output += '\\n'; break;
      case '\r': output += '\\r'; break;
      case '\t': output += '\\t'; break;
      case '\u0008': output += '\\b'; break;
      case '\u000c': output += '\\f'; break;
      default:
        output += isControl(character)
          ? `\\u${codePoint(character).toString(16).toUpperCase().padStart(4, '0')}`
          : character;
    }
  }
  return `${output}'`;
}

/** Renders a lexer set as `[...]`, or `~[...]` when negated. */
function renderSet(negated, items) {
  let body = '';
  for (const item of items) {
    if (item.kind === 'range') {
      body = pushSetChar(body, item.start);
      body += '-';
      body = pushSetChar(body, item.end);
    } else {
      body = pushSetChar(body, item.value);
    }
  }
  return negated ? [`~[${body}]`, Precedence.PREFIX] : [`[${body}]`, Precedence.ATOM];
}

function pushSetChar(body, character) {
  switch (character) {
    case '\\': case ']': case '-': return `${body}\\${character}`;
    case '^': return body === '' ? '\\^' : `${body}^`;
    case '\n': return `${body}\\n`;
    case '\r': return `${body}\\r`;
    case '\t': return `${body}\\t`;
    case '\u0008': return `${body}\\b`;
    case '\u000c': return `${body}\\f`;
    default: return body + character;
  }
}

/**
 * Plans how rule documentation is written so that `importAntlr` reads the same
 * documentation back whenever it consists of comments and a lexer command.
 */
function planDoc(doc, lexer) {
  if (doc === null) return { comments: [], command: null, reimported: null };
  const [text, command] = lexer ? splitCommand(doc) : [doc, null];
  const comments = commentLines(text);
  const parts = comments.map(rustTrim);
  if (command !== null) parts.push(command);
  return { comments, command, reimported: parts.length === 0 ? null : parts.join('; ') };
}

/** Splits a trailing `-> command` part off lexer rule documentation when it re-imports verbatim. */
function splitCommand(doc) {
  let text;
  let command;
  if (doc.startsWith('->')) {
    text = '';
    command = doc;
  } else {
    const position = doc.lastIndexOf('; ->');
    if (position < 0) return [doc, null];
    text = doc.slice(0, position);
    command = doc.slice(position + 2);
  }
  return !/[\n\r]/.test(command) && reimportedCommand(command) === command
    ? [text, command]
    : [doc, null];
}

function reimportedCommand(command) {
  try {
    return importAntlr(`A : 'a' ${command} ;`).rule('A')?.doc ?? null;
  } catch {
    return null;
  }
}

/** Writes documentation as ANTLR comments; existing comments are kept verbatim. */
function commentLines(text) {
  const lines = [];
  for (const part of splitCommentParts(text)) {
    const block = part.startsWith('/*') && part.indexOf('*/', 2) === part.length - 2;
    const line = part.startsWith('//') && !/[\n\r]/.test(part);
    if (block || line) {
      lines.push(part);
    } else {
      for (const docLine of docLines(part)) lines.push(docLine.startsWith('//') ? docLine : `// ${docLine}`);
    }
  }
  return lines;
}

/** Splits documentation at `; ` separators that precede a comment. */
function splitCommentParts(text) {
  const parts = [];
  let rest = text;
  for (;;) {
    const positions = ['; //', '; /*'].map((separator) => rest.indexOf(separator))
      .filter((position) => position >= 0);
    if (positions.length === 0) {
      parts.push(rest);
      break;
    }
    const position = Math.min(...positions);
    parts.push(rest.slice(0, position));
    rest = rest.slice(position + 2);
  }
  return parts.filter((part) => rustTrim(part) !== '');
}
