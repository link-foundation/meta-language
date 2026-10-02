// Faithful lowering of a grammar into a less expressive grammar notation.
// Every construct the target notation cannot write so that its own importer
// reads it back unchanged is moved into a fresh helper rule whose body is an
// encoding in constructs the target does write: an optional item becomes a
// choice with the empty expression, a repetition a recursive helper, a
// character set a choice of its characters, a case-insensitive literal a
// sequence of per-letter character sets, a capture its item. Each rewrite is
// a step of the reconstruction metadata, which names the helper, the rule it
// was lowered from, the construct, whether the encoding is exact (accepts the
// same texts) or approximate (an ordered choice written as an unordered one, a
// lookahead written as the empty expression, an arbitrary character limited
// to printable ASCII), a note and the original expression as native links.
// Rule names, kinds and documentation the target does not keep
// are metadata steps as well. The executable grammar (the emitted text) is
// therefore distinct from the lossless interchange package (executable plus
// metadata), and the check below reconstructs the original from the package
// and reports every feature the package does not carry. It mirrors
// rust/src/grammar/interchange/lowering.rs.
import { Parser } from 'links-notation';
import { carryRuleDocs, Grammar } from './grammar.js';
import { caseVariants, ruleDoc } from './grammar-emitters/structural.js';
import { GrammarImportError } from './grammar-importers.js';
import { grammarEmitter, grammarImporter } from './grammar-interchange.js';
import {
  parseLinksExpression, percentDecodeLinksText, percentEncodeLinksText, renderLinksExpression,
} from './grammar-links.js';
import { acceptsText, canonicalRuleDefinition } from './grammar-round-trip.js';

/** The notations the lowering writes. */
export const GRAMMAR_LOWERING_FORMATS = Object.freeze([
  'abnf', 'antlr', 'bnf', 'ebnf', 'gbnf', 'lark', 'pest', 'tree-sitter-json',
]);

/**
 * The constructs each notation's emitter and importer do not carry unchanged,
 * as measured by experiments/lowering-capability-probe.mjs.
 */
const UNSUPPORTED = {
  abnf: ['negatedClass', 'any', 'orderedChoice', 'and', 'not', 'labeledCapture', 'unlabeledCapture', 'empty'],
  antlr: [
    'parserCharacters', 'literalInsensitive', 'orderedChoice', 'countedRepeat', 'unboundedRepeat', 'and', 'not',
    'unlabeledCapture', 'labelNotIdentifier',
  ],
  bnf: [
    'quotedLiteral', 'literalInsensitive', 'charRange', 'charClass', 'negatedClass', 'any', 'orderedChoice',
    'nestedChoice', 'optional', 'repeat0', 'repeat1', 'countedRepeat', 'unboundedRepeat', 'and', 'not',
    'labeledCapture', 'unlabeledCapture',
  ],
  ebnf: [
    'literalInsensitive', 'charRange', 'charClass', 'negatedClass', 'any', 'orderedChoice', 'countedRepeat',
    'unboundedRepeat', 'and', 'not', 'labeledCapture', 'unlabeledCapture',
  ],
  gbnf: ['literalInsensitive', 'any', 'orderedChoice', 'and', 'not', 'labeledCapture', 'unlabeledCapture', 'empty'],
  lark: [
    'literalInsensitive', 'charRange', 'any', 'orderedChoice', 'unboundedRepeat', 'and', 'not',
    'labeledCapture', 'unlabeledCapture',
  ],
  pest: ['negatedClass', 'unorderedChoice', 'labeledCapture', 'unlabeledCapture', 'empty'],
  'tree-sitter-json': [
    'literalInsensitive', 'any', 'orderedChoice', 'choiceWithEmpty', 'countedRepeat', 'unboundedRepeat', 'and', 'not',
    'unlabeledCapture',
  ],
};

/**
 * The rule kinds each notation writes and reads back unchanged, as measured by
 * experiments/lowering-kind-probe.mjs; other rules are lowered to normal
 * rules with a kind step.
 */
const RULE_KINDS = {
  pest: ['normal', 'atomic', 'silent', 'token'],
  'tree-sitter-json': ['normal', 'token'],
};

/** The constructs ANTLR matches only in lexer rules. */
const CHARACTER_CONSTRUCTS = new Set(['literalInsensitive', 'charRange', 'charClass', 'any']);

/** The characters an approximate encoding of an arbitrary character stands for: printable ASCII. */
const APPROXIMATE_ALPHABET = Object.freeze(Array.from({ length: 0x7f - 0x20 }, (_, offset) => String.fromCodePoint(0x20 + offset)));

/** The most characters a character set is expanded to before it is approximated. */
export const MAX_LOWERED_CHARACTERS = 256;

const MAX_CODE_POINT = 0x10ffff;
const HELPER_PREFIX = 'lowered';
const ENCODINGS = new Set(['exact', 'approximate']);

/** A lowering or reconstruction that cannot be carried out. */
export class GrammarLoweringError extends Error {
  constructor(message) {
    super(message);
    this.name = 'GrammarLoweringError';
  }
}

function checkFormat(format) {
  if (!GRAMMAR_LOWERING_FORMATS.includes(format)) {
    throw new GrammarLoweringError(`the lowering does not support the format ${format}`);
  }
}

/**
 * The constructs of `expression` at `position` (`rule` for a whole rule body,
 * `inner` otherwise), outside a lexer rule unless `lexical`.
 */
function constructsOf(expression, position, lexical) {
  const characters = !lexical && CHARACTER_CONSTRUCTS.has(expression.kind)
    && !(expression.kind === 'literalInsensitive' && expression.value === '') ? ['parserCharacters'] : [];
  return [...characters, ...ownConstructs(expression, position)];
}

function ownConstructs(expression, position) {
  switch (expression.kind) {
    case 'literal': return expression.value.includes('"') && expression.value.includes("'") ? ['quotedLiteral'] : [];
    case 'literalInsensitive': return expression.value === '' ? [] : ['literalInsensitive'];
    case 'charRange': return ['charRange'];
    case 'charClass': return [expression.negated ? 'negatedClass' : 'charClass'];
    case 'any': return ['any'];
    case 'choice': return [
      expression.ordered ? 'orderedChoice' : 'unorderedChoice',
      ...(expression.items.some((item) => item.kind === 'empty') ? ['choiceWithEmpty'] : []),
      ...(position === 'inner' ? ['nestedChoice'] : []),
    ];
    case 'optional': case 'repeat0': case 'repeat1': case 'and': case 'not': case 'empty': return [expression.kind];
    case 'repeat': return [expression.max === null || expression.max === undefined ? 'unboundedRepeat' : 'countedRepeat'];
    case 'capture':
      if (expression.label === null || expression.label === undefined) return ['unlabeledCapture'];
      return /^[A-Za-z_][A-Za-z0-9_]*$/.test(expression.label) ? ['labeledCapture'] : ['labeledCapture', 'labelNotIdentifier'];
    default: return [];
  }
}

const literal = (value) => ({ kind: 'literal', value });
const ref = (name) => ({ kind: 'ref', name });
const unordered = (items) => (items.length === 1 ? items[0] : { kind: 'choice', ordered: false, items });
const seq = (items) => (items.length === 1 ? items[0] : { kind: 'seq', items });
const codePoint = (character) => character.codePointAt(0);
const fromCodePoint = (point) => String.fromCodePoint(point);

/** The expression a notation reads back as matching the empty text. */
function emptyEncoding(format) {
  if (format === 'abnf') return { kind: 'literalInsensitive', value: '' };
  return UNSUPPORTED[format].includes('empty') ? literal('') : { kind: 'empty' };
}

/** The code point ranges of a character set's items. */
function itemRanges(items) {
  return items.map((item) => (item.kind === 'range'
    ? [codePoint(item.start), codePoint(item.end)]
    : [codePoint(item.value), codePoint(item.value)]));
}

/** Sorted, merged, non-overlapping ranges. */
function mergeRanges(ranges) {
  const sorted = [...ranges].sort((left, right) => left[0] - right[0]);
  const merged = [];
  for (const [start, end] of sorted) {
    const last = merged.at(-1);
    if (last && start <= last[1] + 1) last[1] = Math.max(last[1], end);
    else merged.push([start, end]);
  }
  return merged;
}

/** The ranges of every code point outside `ranges`. */
function complementRanges(ranges) {
  const complement = [];
  let next = 0;
  for (const [start, end] of mergeRanges(ranges)) {
    if (start > next) complement.push([next, start - 1]);
    next = end + 1;
  }
  if (next <= MAX_CODE_POINT) complement.push([next, MAX_CODE_POINT]);
  return complement;
}

const rangeSize = (ranges) => ranges.reduce((total, [start, end]) => total + end - start + 1, 0);
const inRanges = (ranges, point) => ranges.some(([start, end]) => point >= start && point <= end);

/**
 * A character set as constructs `format` writes: one character class where
 * the notation has classes and a choice of single characters otherwise. A set too large to spell character
 * by character is limited to the approximate alphabet.
 */
function characterSetEncoding(format, ranges) {
  const unsupported = UNSUPPORTED[format];
  if (!unsupported.includes('charClass')) {
    const items = ranges.map(([start, end]) => (start === end
      ? { kind: 'char', value: fromCodePoint(start) }
      : { kind: 'range', start: fromCodePoint(start), end: fromCodePoint(end) }));
    const single = items.length === 1 && items[0].kind === 'char';
    return { expression: single ? literal(items[0].value) : { kind: 'charClass', negated: false, items }, exact: true };
  }
  const exact = rangeSize(ranges) <= MAX_LOWERED_CHARACTERS;
  const characters = exact
    ? ranges.flatMap(([start, end]) => Array.from({ length: end - start + 1 }, (_, offset) => fromCodePoint(start + offset)))
    : APPROXIMATE_ALPHABET.filter((character) => inRanges(ranges, codePoint(character)));
  if (characters.length === 0) {
    throw new GrammarLoweringError(`a character set without printable ASCII characters cannot be written in ${format}`);
  }
  return { expression: unordered(characters.map(literal)), exact };
}

/** Whether ordered and unordered readings of a choice accept the same texts. */
function choiceOrderIrrelevant(items) {
  if (!items.every((item) => item.kind === 'literal' && item.value !== '')) return false;
  return items.every((item, index) => items.every((other, at) => at === index || !other.value.startsWith(item.value)));
}

/** The helper body encoding `expression`'s `construct` in `format`, and whether it is exact. */
function encode(format, construct, expression, helper) {
  const exact = (body, note) => ({ body, encoding: 'exact', note });
  switch (construct) {
    case 'quotedLiteral': {
      const runs = [];
      for (const character of expression.value) {
        const last = runs.at(-1);
        const candidate = last === undefined ? null : last + character;
        if (candidate !== null && !(candidate.includes('"') && candidate.includes("'"))) runs[runs.length - 1] = candidate;
        else runs.push(character);
      }
      return exact(seq(runs.map(literal)), 'a literal with both quote characters as a sequence of literals');
    }
    case 'literalInsensitive': {
      const items = [];
      for (const character of expression.value) {
        const variants = caseVariants(character);
        if (variants !== null) {
          items.push({ kind: 'charClass', negated: false, items: variants.map((value) => ({ kind: 'char', value })) });
        } else if (items.at(-1)?.kind === 'literal') {
          items[items.length - 1] = literal(items.at(-1).value + character);
        } else {
          items.push(literal(character));
        }
      }
      return exact(seq(items), 'a case-insensitive literal as per-letter character sets');
    }
    case 'charRange': case 'charClass': {
      const ranges = mergeRanges(construct === 'charRange'
        ? [[codePoint(expression.start), codePoint(expression.end)]]
        : itemRanges(expression.items));
      const { expression: body, exact: isExact } = characterSetEncoding(format, ranges);
      return isExact
        ? exact(body, 'a character set as the characters or ranges it holds')
        : { body, encoding: 'approximate', note: 'a character set limited to printable ASCII' };
    }
    case 'negatedClass': {
      if (format === 'pest') {
        return exact(
          seq([{ kind: 'not', item: { kind: 'charClass', negated: false, items: expression.items } }, { kind: 'any' }]),
          'a negated character class as a negative lookahead and any character',
        );
      }
      const complement = complementRanges(itemRanges(expression.items));
      const { expression: body, exact: isExact } = characterSetEncoding(format, complement);
      return isExact
        ? exact(body, 'a negated character class as the ranges of its complement')
        : { body, encoding: 'approximate', note: 'a negated character class limited to printable ASCII' };
    }
    case 'any': {
      const { expression: body, exact: isExact } = characterSetEncoding(format, [[0, MAX_CODE_POINT]]);
      return isExact
        ? exact(body, 'any character as the range of every code point')
        : { body, encoding: 'approximate', note: 'any character limited to printable ASCII' };
    }
    case 'orderedChoice': case 'unorderedChoice': {
      const body = { kind: 'choice', ordered: construct === 'unorderedChoice', items: expression.items };
      return choiceOrderIrrelevant(expression.items)
        ? exact(body, 'a choice of literals none of which starts another, whose order is irrelevant')
        : { body, encoding: 'approximate', note: `an ${construct === 'orderedChoice' ? 'ordered' : 'unordered'} choice written as an ${construct === 'orderedChoice' ? 'unordered' : 'ordered'} one` };
    }
    case 'nestedChoice': return exact(expression, 'a nested choice as a rule of its own');
    case 'choiceWithEmpty': {
      const items = expression.items.filter((item) => item.kind !== 'empty');
      return exact(
        { kind: 'optional', item: items.length === 1 ? items[0] : { ...expression, items } },
        'a choice with the empty expression as an optional choice',
      );
    }
    case 'parserCharacters': return exact(expression, 'a character-level construct as a lexer rule of its own');
    case 'optional': return exact(unordered([expression.item, { kind: 'empty' }]), 'an optional item as a choice with the empty expression');
    case 'repeat0':
      return exact(unordered([seq([expression.item, ref(helper)]), { kind: 'empty' }]), 'a repetition as a right-recursive rule');
    case 'repeat1':
      return exact(unordered([seq([expression.item, ref(helper)]), expression.item]), 'a repetition as a right-recursive rule');
    case 'countedRepeat': case 'unboundedRepeat': {
      const copies = Array.from({ length: expression.min }, () => expression.item);
      let rest = null;
      if (expression.max === null || expression.max === undefined) rest = { kind: 'repeat0', item: expression.item };
      else {
        for (let count = expression.max - expression.min; count > 0; count -= 1) {
          rest = { kind: 'optional', item: rest === null ? expression.item : seq([expression.item, rest]) };
        }
      }
      const items = rest === null ? copies : [...copies, rest];
      return exact(items.length === 0 ? { kind: 'empty' } : seq(items), 'a counted repetition as copies and nested optional items');
    }
    case 'and': case 'not':
      return {
        body: emptyEncoding(format),
        encoding: 'approximate',
        note: `a ${construct === 'and' ? 'positive' : 'negative'} lookahead written as the empty expression`,
      };
    case 'labeledCapture': case 'unlabeledCapture': case 'labelNotIdentifier':
      return exact(expression.item, 'a capture as a rule of its own');
    case 'empty': return exact(emptyEncoding(format), 'the empty expression as the empty literal');
    default: throw new GrammarLoweringError(`no encoding of ${construct}`);
  }
}

/** Rebuilds `expression` with `map` applied to its direct sub-expressions. */
function mapChildren(expression, map) {
  if (Array.isArray(expression.items) && expression.kind !== 'charClass') {
    return { ...expression, items: expression.items.map(map) };
  }
  if (expression.item) return { ...expression, item: map(expression.item) };
  return expression;
}

/**
 * Lowers `grammar` into `format`: the lowered grammar, the executable text the
 * target emitter writes for it, the steps and the metadata links that
 * reconstruct the original from the executable, and whether every encoding is
 * exact. `emitGrammar` and `importGrammar` replace the format's own pair.
 */
export function lowerGrammar(grammar, format, {
  emitGrammar = grammarEmitter(GRAMMAR_LOWERING_FORMATS.includes(format) ? format : 'native'),
  importGrammar = grammarImporter(GRAMMAR_LOWERING_FORMATS.includes(format) ? format : 'native'),
} = {}) {
  checkFormat(format);
  const start = grammar.startRule();
  if (!start) throw new GrammarLoweringError('the grammar has no start rule');
  const unsupported = new Set(UNSUPPORTED[format]);
  const taken = new Set(grammar.ruleNames().map((name) => name.toLowerCase()));
  let counter = 0;
  // A lexer helper gets an upper-case name, which ANTLR and Lark read as a token rule.
  const helperName = (lexical) => {
    let name;
    do {
      counter += 1;
      name = `${HELPER_PREFIX}${counter}`;
    } while (taken.has(name));
    taken.add(name);
    return lexical ? name.toUpperCase() : name;
  };
  const keptKinds = new Set(RULE_KINDS[format] ?? ['normal']);

  const helpers = new Map();
  const memo = new Map();
  const steps = [];
  // `lexical` marks a lexer rule body, where character-level constructs are
  // matched; ANTLR's parser rules get them through a lexer helper.
  const lower = (expression, owner, position, lexical) => {
    const construct = constructsOf(expression, position, lexical).find((name) => unsupported.has(name));
    if (construct === undefined) return mapChildren(expression, (item) => lower(item, owner, 'inner', lexical));
    const helperLexical = lexical || construct === 'parserCharacters';
    const key = `${construct} ${helperLexical} ${renderLinksExpression(expression)}`;
    if (memo.has(key)) return ref(memo.get(key));
    const helper = helperName(helperLexical);
    memo.set(key, helper);
    const { body, encoding, note } = encode(format, construct, expression, helper);
    steps.push({ kind: 'helper', helper, owner, construct, encoding, note, original: expression });
    helpers.set(helper, null);
    helpers.set(helper, { kind: helperLexical ? 'token' : 'normal', expression: lower(body, helper, 'rule', helperLexical) });
    return ref(helper);
  };

  const rules = new Map();
  for (const rule of grammar.rules.values()) {
    const expression = lower(rule.expression, rule.name, 'rule', false);
    // A tree-sitter token cannot refer to other rules, so a token rule that
    // needs a helper becomes a normal rule with a kind step.
    const refersToHelper = format === 'tree-sitter-json'
      && renderLinksExpression(expression) !== renderLinksExpression(rule.expression);
    rules.set(rule.name, { kind: keptKinds.has(rule.kind) && !refersToHelper ? rule.kind : 'normal', expression });
  }
  for (const [name, helper] of helpers) rules.set(name, helper);
  // The lowered grammar keeps the original source format, so the emitter
  // reports every construct it cannot write for a grammar of that format.
  const lowered = carryRuleDocs(new Grammar(start.name, rules, grammar.sourceFormat), grammar);

  // A first emission shows the names the target gives the rules (GBNF starts
  // at root, ANTLR and Lark adjust case), matched by position, which every
  // emitter keeps. The lowered grammar takes those names, so the target
  // writes it without renaming, and the renames become steps.
  const first = importGrammar(emitGrammar(lowered).source).ruleNames();
  const executableNames = new Map(lowered.ruleNames()
    .map((name, index) => [name, first[index]])
    .filter(([name, executableName]) => executableName !== undefined && executableName !== name));
  let executable = lowered;
  if (executableNames.size > 0) {
    const named = (name) => executableNames.get(name) ?? name;
    const renamed = (expression) => (expression.kind === 'ref' ? ref(named(expression.name)) : mapChildren(expression, renamed));
    executable = new Grammar(named(start.name), new Map([...lowered.rules.values()].map((rule) => [
      named(rule.name), { ...rule, expression: renamed(rule.expression) },
    ])), lowered.sourceFormat);
  }
  let emitted = emitGrammar(executable);
  let imported = importGrammar(emitted.source);
  // Documentation the target does not read back verbatim (ANTLR keeps the
  // comment markers, BNF has no comments) leaves the executable and is
  // carried by a doc step instead.
  const unkept = executable.ruleNames().filter((name) => {
    const doc = ruleDoc(executable, executable.rule(name));
    return doc !== null && (imported.rule(name) === undefined || ruleDoc(imported, imported.rule(name)) !== doc);
  });
  if (unkept.length > 0) {
    for (const name of unkept) {
      const { kind, expression } = executable.rule(name);
      executable.rules.set(name, Object.freeze({ name, kind, expression }));
    }
    emitted = emitGrammar(executable);
    imported = importGrammar(emitted.source);
  }
  const renames = [...executableNames].map(([rule, value]) => ({ kind: 'rename', rule, value }));
  for (const rule of grammar.rules.values()) {
    const back = imported.rule(executableNames.get(rule.name) ?? rule.name);
    if (back && back.kind !== rule.kind) steps.push({ kind: 'kind', rule: rule.name, value: rule.kind });
    const doc = ruleDoc(grammar, rule);
    if (back && doc !== null && ruleDoc(imported, back) !== doc) steps.push({ kind: 'doc', rule: rule.name, value: doc });
  }
  steps.unshift(...renames);

  const status = steps.some((step) => step.encoding === 'approximate') ? 'approximate' : 'exact';
  const metadata = renderLoweringMetadata({
    format, status, source: grammar.sourceFormat, start: start.name, order: grammar.ruleNames(), steps,
  });
  return {
    format,
    status,
    grammar: executable,
    executable: emitted.source,
    report: emitted.report,
    imported,
    steps,
    metadata,
  };
}

/** Renders the reconstruction metadata as links, one link per line. */
export function renderLoweringMetadata({ format, status, source, start, order, steps }) {
  const text = percentEncodeLinksText;
  const lines = [
    `(lowering ${format} ${status})`,
    source === null || source === undefined ? '(source)' : `(source ${text(source)})`,
    `(start ${text(start)})`,
    `(rules ${order.map(text).join(' ')})`,
  ];
  for (const step of steps) {
    if (step.kind === 'helper') {
      lines.push(`(helper ${text(step.helper)} ${text(step.owner)} ${step.construct} ${step.encoding} ${text(step.note)} ${renderLinksExpression(step.original)})`);
    } else {
      lines.push(`(${step.kind} ${text(step.rule)} ${text(step.value)})`);
    }
  }
  return lines.map((line) => `${line}\n`).join('');
}

function metadataError(detail) {
  return new GrammarImportError('meta-language', 'parse', `lowering metadata: ${detail}`);
}

/** Reads the metadata written by `renderLoweringMetadata`. */
export function parseLoweringMetadata(source) {
  let statements;
  try {
    statements = new Parser({ comments: false }).parse(source);
  } catch (error) {
    throw metadataError(error.message);
  }
  const head = (statement) => {
    if (statement.values.length === 0 || statement.id !== null) throw metadataError('every metadata entry is a link');
    const [first, ...rest] = statement.values;
    if (first.values.length !== 0 || first.id === null) throw metadataError('a metadata link starts with a word');
    return [first.id, rest];
  };
  const word = (value) => {
    if (value === undefined || value.values.length !== 0 || value.id === null) throw metadataError('expected a word');
    return value.id;
  };
  const links = statements.map(head);
  if (links.length < 4) throw metadataError('the metadata starts with the lowering, source, start and rules links');
  const [[header, headerArgs], [sourceHead, sourceArgs], [startHead, startArgs], [rulesHead, rulesArgs], ...rest] = links;
  if (header !== 'lowering' || headerArgs.length !== 2) throw metadataError('the first link must be (lowering FORMAT STATUS)');
  const format = word(headerArgs[0]);
  checkFormat(format);
  const status = word(headerArgs[1]);
  if (status !== 'exact' && status !== 'approximate') throw metadataError(`unknown status ${status}`);
  if (sourceHead !== 'source' || sourceArgs.length > 1) throw metadataError('the second link must be (source [FORMAT])');
  if (startHead !== 'start' || startArgs.length !== 1) throw metadataError('the third link must be (start NAME)');
  if (rulesHead !== 'rules') throw metadataError('the fourth link must be (rules NAME...)');
  const steps = rest.map(([kind, args]) => {
    if (kind === 'helper' && args.length === 6) {
      const encoding = word(args[3]);
      if (!ENCODINGS.has(encoding)) throw metadataError(`unknown encoding ${encoding}`);
      return {
        kind,
        helper: percentDecodeLinksText(word(args[0])),
        owner: percentDecodeLinksText(word(args[1])),
        construct: word(args[2]),
        encoding,
        note: percentDecodeLinksText(word(args[4])),
        original: parseLinksExpression(args[5]),
      };
    }
    if ((kind === 'rename' || kind === 'kind' || kind === 'doc') && args.length === 2) {
      return { kind, rule: percentDecodeLinksText(word(args[0])), value: percentDecodeLinksText(word(args[1])) };
    }
    throw metadataError(`unexpected link ${kind}`);
  });
  return {
    format,
    status,
    source: sourceArgs.length === 0 ? null : percentDecodeLinksText(word(sourceArgs[0])),
    start: percentDecodeLinksText(word(startArgs[0])),
    order: rulesArgs.map((value) => percentDecodeLinksText(word(value))),
    steps,
  };
}

/**
 * Reconstructs the original grammar from an executable text and its lowering
 * metadata: the executable is imported, every renamed rule gets its original
 * name back, every helper reference is replaced with the original expression
 * the metadata records, the helpers are removed and the rule kinds and
 * documentation the target did not keep are restored.
 */
export function reconstructGrammar(executable, metadataSource, {
  importGrammar = null,
} = {}) {
  const metadata = parseLoweringMetadata(metadataSource);
  const imported = (importGrammar ?? grammarImporter(metadata.format))(executable);
  const stepsOf = (kind) => metadata.steps.filter((step) => step.kind === kind);
  const helpers = new Map(stepsOf('helper').map((step) => [step.helper, step.original]));
  const renames = new Map(stepsOf('rename').map((step) => [step.rule, step.value]));
  const originals = new Map(stepsOf('rename').map((step) => [step.value, step.rule]));
  const restore = (expression) => {
    if (expression.kind === 'ref') {
      const name = originals.get(expression.name) ?? expression.name;
      return helpers.has(name) ? helpers.get(name) : ref(name);
    }
    return mapChildren(expression, restore);
  };
  const executableName = (name) => renames.get(name) ?? name;
  const kinds = new Map(metadata.steps.filter((step) => step.kind === 'kind').map((step) => [step.rule, step.value]));
  const docs = new Map(metadata.steps.filter((step) => step.kind === 'doc').map((step) => [step.rule, step.value]));
  const rules = new Map();
  for (const name of metadata.order) {
    const rule = imported.rule(executableName(name));
    if (!rule) throw new GrammarLoweringError(`the executable grammar has no rule ${executableName(name)}`);
    rules.set(name, { kind: kinds.get(name) ?? rule.kind, expression: restore(rule.expression) });
  }
  const grammar = new Grammar(metadata.start, rules, metadata.source);
  for (const name of metadata.order) {
    const doc = docs.get(name) ?? ruleDoc(imported, imported.rule(executableName(name)));
    if (doc !== null) grammar.rules.set(name, Object.freeze({ ...grammar.rules.get(name), doc }));
  }
  return grammar;
}

/**
 * Lowers `grammar` into `format`, reconstructs it from the package and
 * reports every way the lowering failed: an emission note, an executable
 * that does not read back as the lowered grammar, a feature the
 * reconstruction lost, and for an exact lowering an accept or reject sample
 * on which the executable disagrees. `editMetadata` rewrites the metadata
 * before the reconstruction, for negative controls.
 */
export function checkGrammarLowering(grammar, format, {
  accepts = [],
  rejects = [],
  emitGrammar,
  importGrammar,
  editMetadata = (metadata) => metadata,
} = {}) {
  const lowering = lowerGrammar(grammar, format, { emitGrammar, importGrammar });
  const failures = [];
  for (const note of lowering.report.lossy) failures.push({ kind: 'lossy-emission', detail: note });
  // The executable must read back as the lowered grammar, which already
  // carries the target's rule names: the same rules, start and definitions.
  const expectedNames = lowering.grammar.ruleNames();
  const actualNames = lowering.imported.ruleNames();
  if (expectedNames.join('\n') !== actualNames.join('\n')) {
    failures.push({ kind: 'not-executable', detail: `rules [${expectedNames.join(', ')}] read back as [${actualNames.join(', ')}]` });
  }
  if (lowering.imported.startRule()?.name !== lowering.grammar.startRule().name) {
    failures.push({ kind: 'not-executable', detail: `the executable does not start at rule ${lowering.grammar.startRule().name}` });
  }
  const body = (expression) => canonicalRuleDefinition({ kind: 'normal', expression });
  for (const rule of lowering.grammar.rules.values()) {
    const back = lowering.imported.rule(rule.name);
    if (back && body(back.expression) !== body(rule.expression)) {
      failures.push({ kind: 'not-executable', detail: `rule ${rule.name} reads back with a different definition` });
    }
  }

  let reconstructed = null;
  try {
    reconstructed = reconstructGrammar(lowering.executable, editMetadata(lowering.metadata), { importGrammar });
  } catch (error) {
    failures.push({ kind: 'feature-dropped', detail: error.message });
  }
  if (reconstructed !== null) {
    failures.push(...droppedFeatures(grammar, reconstructed).map((detail) => ({ kind: 'feature-dropped', detail })));
  }
  if (lowering.status === 'exact') {
    for (const text of accepts) {
      if (!acceptsText(lowering.imported, text)) failures.push({ kind: 'sample-rejected', detail: text });
    }
    for (const text of rejects) {
      if (acceptsText(lowering.imported, text)) failures.push({ kind: 'sample-accepted', detail: text });
    }
  }
  return {
    status: failures.length === 0 ? lowering.status : 'broken',
    failures,
    lowering,
    reconstructed,
  };
}

/** The features of `expected` that `actual` does not carry, as readable details. */
export function droppedFeatures(expected, actual) {
  const details = [];
  if (expected.ruleNames().join('\n') !== actual.ruleNames().join('\n')) {
    details.push(`rules [${expected.ruleNames().join(', ')}] became [${actual.ruleNames().join(', ')}]`);
  }
  if ((expected.startRule()?.name ?? null) !== (actual.startRule()?.name ?? null)) details.push('the start rule changed');
  for (const rule of expected.rules.values()) {
    const other = actual.rule(rule.name);
    if (!other) continue;
    if (other.kind !== rule.kind) details.push(`rule ${rule.name} lost its kind ${rule.kind}`);
    if (canonicalRuleDefinition(other) !== canonicalRuleDefinition(rule)) details.push(`rule ${rule.name} changed its definition`);
    if (ruleDoc(actual, other) !== ruleDoc(expected, rule)) details.push(`rule ${rule.name} changed its documentation`);
  }
  return details;
}
