import {
  charClassItems,
  codePoint,
  debugString,
  decorateEmitted,
  emitReport,
  finishLines,
  hex4,
  orderedRules,
  pegChoiceAlternatives,
  PEST_RULE_TEMPLATE,
  renderRuleLine,
  unsupportedError,
} from './common.js';
import { pestBuiltInSource } from '../grammar-importers/pest.js';
import { parseGrammarLinks } from '../grammar-links.js';

const FORMAT = 'peg';
const CHOICE = 0;
const SEQUENCE = 1;
const PREFIX = 2;
const POSTFIX = 3;
const ATOM = 4;
const MODIFIERS = { normal: '', atomic: '@', silent: '_', token: '$' };

/**
 * Emits pest PEG text: ordered choice uses `|`, sequences use `~`, predicates
 * use `&`/`!`, and token rules use pest's compound-atomic `$` modifier.
 */
export function emitPest(grammar, options = {}) {
  return decorateEmitted('pest', writePest(grammar), options.decorators);
}

function writePest(grammar) {
  const report = emitReport();
  const rules = orderedRules(grammar);
  const explicitTrivia = grammar.sourceFormat === 'peg' && rules.some(rule => containsExplicitTrivia(rule.expression));
  const names = new Map();
  if (explicitTrivia) {
    for (const name of ['WHITESPACE', 'COMMENT']) {
      if (!grammar.rules.has(name)) continue;
      let alias = `_pest_${name}`;
      while (grammar.rules.has(alias)) alias += '_';
      names.set(name, alias);
    }
  }
  const context = { report, names, explicitTrivia };
  const lines = [];
  for (const rule of rules) {
    if (grammar.sourceFormat === 'peg' && rule.name !== grammar.start && rule.kind === 'silent' &&
        /^[A-Z][A-Z_0-9]+$/u.test(rule.name) && matchesBuiltIn(rule)) continue;
    if (containsUnorderedChoice(rule.expression)) {
      lines.push('// NOTE: unordered choice in source is emitted as ordered pest choice.');
    }
    lines.push(renderRuleLine(
      PEST_RULE_TEMPLATE,
      names.get(rule.name) ?? rule.name,
      emitExpression(rule.expression, CHOICE, context),
      MODIFIERS[rule.kind] ?? '',
    ));
  }
  return { source: finishLines(lines), report };
}

function matchesBuiltIn(rule) {
  const source = pestBuiltInSource(rule.name);
  if (!source) return false;
  const definition = parseGrammarLinks(`(grammar (start ${rule.name}))\n${source}`).rule(rule.name);
  return JSON.stringify(definition) === JSON.stringify(rule);
}

function containsExplicitTrivia(expression) {
  if (expression.kind === 'token') {
    const repeat = expression.item.kind === 'seq' ? expression.item.items[0] : expression.item;
    const item = repeat.kind === 'repeat0' && repeat.item;
    const reference = item && (item.kind === 'seq' ? item.items[0] : item);
    if (reference?.kind === 'ref' && ['WHITESPACE', 'COMMENT'].includes(reference.name)) return true;
  }
  return expression.items?.some(containsExplicitTrivia) || Boolean(expression.item && containsExplicitTrivia(expression.item));
}

function emitExpression(expression, parent, context) {
  const { report, names, explicitTrivia } = context;
  let text;
  let precedence = ATOM;
  switch (expression.kind) {
    case 'empty': text = quoteTerminal(''); break;
    case 'literal': text = quoteTerminal(expression.value); break;
    case 'literalInsensitive': text = `^${quoteTerminal(expression.value)}`; break;
    case 'charRange': text = emitCharRange(expression.start, expression.end); break;
    case 'charClass': text = emitCharClass(expression); break;
    case 'any': text = 'ANY'; break;
    case 'ref': text = names.get(expression.name) ?? expression.name; break;
    case 'choice':
      if (expression.items.length === 0) throw unsupportedError(FORMAT, 'empty Choice');
      if (!expression.ordered) report.lossy.push('PEG treats unordered choice as ordered choice');
      text = pegChoiceAlternatives(expression.ordered, expression.items)
        .map((item) => emitExpression(item, CHOICE, context))
        .join(' | ');
      precedence = CHOICE;
      break;
    case 'seq': {
      const items = expression.items
        .filter((item) => item.kind !== 'empty')
        .map((item) => emitExpression(item, SEQUENCE, context))
        .filter((item) => item !== '');
      text = items.length === 0 ? quoteTerminal('') : items.join(' ~ ');
      precedence = SEQUENCE;
      break;
    }
    case 'optional': text = `${emitExpression(expression.item, POSTFIX, context)}?`; precedence = POSTFIX; break;
    case 'repeat0': text = `${emitExpression(expression.item, POSTFIX, context)}*`; precedence = POSTFIX; break;
    case 'repeat1': text = `${emitExpression(expression.item, POSTFIX, context)}+`; precedence = POSTFIX; break;
    case 'repeat': text = emitRepeat(expression, context); precedence = POSTFIX; break;
    case 'and': text = `&${emitExpression(expression.item, PREFIX, context)}`; precedence = PREFIX; break;
    case 'not': text = `!${emitExpression(expression.item, PREFIX, context)}`; precedence = PREFIX; break;
    case 'token':
      if (!explicitTrivia) throw unsupportedError(FORMAT, expression.kind);
      return emitExpression(expression.item, parent, context);
    case 'capture':
      if (expression.label !== null && expression.label !== undefined) {
        report.lossy.push(`PEG dropped capture label ${debugString(expression.label)}`);
      }
      text = `(${emitExpression(expression.item, CHOICE, context)})`;
      break;
    default: throw unsupportedError(FORMAT, expression.kind);
  }
  return precedence < parent ? `(${text})` : text;
}

function emitRepeat({ item, min, max }, context) {
  if (max !== null && max < min) {
    throw unsupportedError(FORMAT, `Repeat with min ${min} greater than max Some(${max})`);
  }
  const inner = emitExpression(item, POSTFIX, context);
  if (max === null) return `${inner}{${min},}`;
  return min === max ? `${inner}{${min}}` : `${inner}{${min},${max}}`;
}

function emitCharClass(expression) {
  const items = charClassItems(FORMAT, expression);
  if (items.length === 0) throw unsupportedError(FORMAT, 'empty CharClass');
  const inner = items.map((item) => item.kind === 'range'
    ? emitCharRange(item.start, item.end)
    : quoteTerminal(item.value)).join(' | ');
  return expression.negated ? `(!(${inner}) ~ ANY)` : `(${inner})`;
}

function emitCharRange(start, end) {
  const first = codePoint(start);
  const last = codePoint(end);
  if (first > last) {
    throw unsupportedError(
      FORMAT,
      `CharRange has descending bounds U+${hex4(first)}..=U+${hex4(last)}`,
    );
  }
  return `${quoteCharacter(start)}..${quoteCharacter(end)}`;
}

function containsUnorderedChoice(expression) {
  if (expression.kind === 'choice' && !expression.ordered) return true;
  if (expression.items) return expression.items.some(containsUnorderedChoice);
  if (expression.item) return containsUnorderedChoice(expression.item);
  return false;
}

function quoteTerminal(value) {
  return `"${[...value].map((character) => escapeCharacter(character, '"')).join('')}"`;
}

function quoteCharacter(value) {
  return `'${escapeCharacter(String(value), "'")}'`;
}

function escapeCharacter(character, quote) {
  if (character === quote) return `\\${quote}`;
  switch (character) {
    case '\\': return '\\\\';
    case '\n': return '\\n';
    case '\r': return '\\r';
    case '\t': return '\\t';
    default:
      return /\p{Cc}/u.test(character)
        ? `\\u{${codePoint(character).toString(16).toUpperCase()}}`
        : character;
  }
}
