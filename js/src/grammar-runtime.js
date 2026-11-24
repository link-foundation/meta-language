// The public face of the native grammar executor: compile a grammar once,
// then parse sources into lossless concrete syntax trees. Loading
// (grammar-runtime/load.js) resolves imports, macros and parameterized rules
// and checks every scanner, action and predicate; the executor
// (grammar-runtime/executor.js) runs it within explicit resource limits; the
// tree module (grammar-runtime/syntax-tree.js) copies and renders the result.
// docs/grammar/feature-union.md is the specification the Rust port follows.
import { Executor, Expectations, KeywordLexing, MemoryBudgetReached, memoryBudget, NestingTooDeep, StepLimitReached, stepBudget } from './grammar-runtime/executor.js';
import { GrammarRuntimeError, loadProgram } from './grammar-runtime/load.js';
import { decorateGrammar, decorateSyntaxTree } from './grammar-decorators.js';
import { DecoratorError, decoratorSet } from './decorators.js';
import { collectAmbiguities, firstRecovery, publicTree, renderSyntaxTree } from './grammar-runtime/syntax-tree.js';
import { inputBytes, lineAndColumn, utf16View } from './grammar-runtime/text.js';
import { qualifyGrammarRoot } from './grammar-runtime/grammar-selection.js';
import { withGrammarSelectionSources } from './grammar-runtime/selection-source-mapping.js';
import { readGrammarCompositionPublicationIssue } from './translation/grammar-rule-composition-rules.js';
import { sourceTextPredicateHost } from './query.js';
import { insideGrammarSearchWord, skipGrammarSearchStart } from './grammar-search-policy.js';
import { textLocationCharacterClass } from './text-location-character-classes.js';

export { GrammarRuntimeError, renderSyntaxTree };
export { checkGrammarSelectionMappedTree } from './grammar-runtime/mapped-tree-validation.js';

/** A source the grammar rejects; `rejection` holds the reason and position. */
export class GrammarParseError extends Error {
  constructor(rejection) {
    super(describeRejection(rejection));
    this.name = 'GrammarParseError';
    this.rejection = rejection;
    this.reason = rejection.reason;
    this.offset = rejection.offset ?? null;
    this.line = rejection.line ?? null;
    this.column = rejection.column ?? null;
    this.expected = rejection.expected ?? [];
  }
}

function describeRejection(rejection) {
  const where = rejection.line ? ` at line ${rejection.line} column ${rejection.column}` : '';
  switch (rejection.reason) {
    case 'syntax': return `syntax error${where}: expected ${rejection.expected.join(', ') || 'nothing more'}`;
    case 'nestingDepth': return `the input nests deeper than ${rejection.limit} rules`;
    case 'stepLimit': return `the parse needed more than ${rejection.limit} steps`;
    case 'memoryBudget': return `the parse needed more than ${rejection.limit} memo cells`;
    case 'recovered': return `the input needed error recovery${where}`;
    case 'ambiguity': return `the input is ambiguous${where}`;
    default: return `the input is rejected (${rejection.reason})`;
  }
}

function positioned(reason, bytes, offset, extra = {}) {
  return { reason, offset, ...lineAndColumn(bytes, offset), ...extra };
}

/**
 * Parses the whole input with a fresh executor that `prepare` sets up. While
 * a scanner's `expected` was answered before the parse made the request it
 * asks about, the parse runs again with the requests so far (see
 * `Expectations` in grammar-runtime/executor.js). Each run has its own step
 * budget; all runs share the parse's `memory` budget.
 */
function runParse(program, bytes, startRule, options, maxDepth, expectations, memory, prepare, search) {
  for (;;) {
    const budget = stepBudget(options, bytes.length, memory);
    if (search) budget.steps = search.steps;
    const executor = new Executor(program, bytes, search?.begin ?? 0, bytes.length, options, budget, maxDepth);
    executor.expectations = expectations.restart();
    prepare(executor);
    const outcome = executor.run(startRule, search);
    if (search) search.steps = budget.steps;
    if (!expectations.stale) return { ...outcome, executor };
  }
}

/**
 * Automatic error recovery after a failed parse: each round reparses with
 * one more repair point, the farthest offset where an element failed without
 * a repair, until a parse completes. After `maxRepairs` rounds (default 32),
 * or when no new point appears, the last round's partial tree stands, the
 * rest of the input an ERROR leaf. Each round has its own step budget; a
 * round that runs out of steps ends the rounds as well.
 */
function repairParse(program, bytes, startRule, options, maxDepth, failed, keywords, expectations, memory) {
  const points = new Set();
  const maxRepairs = options.maxRepairs ?? 32;
  let outcome = failed;
  while (points.size < maxRepairs) {
    const point = outcome.elementFarthest >= 0 ? outcome.elementFarthest : outcome.farthest;
    if (points.has(point)) break;
    points.add(point);
    try {
      outcome = runParse(program, bytes, startRule, options, maxDepth, expectations, memory, (executor) => {
        executor.repairPoints = points;
        executor.keywords = keywords;
      });
    } catch (error) {
      if (!(error instanceof StepLimitReached)) throw error;
      break;
    }
    if (outcome.ok) return outcome;
  }
  return { ok: true, root: outcome.partial, executor: outcome.executor };
}

/**
 * Parses `source` (a string or UTF-8 bytes) with a loaded program. Returns
 * `{ ok, tree, ambiguities, rejection }`: `ok` is true only without a
 * rejection; the tree is kept whenever one was built.
 */
function parseProgram(program, source, options, search) {
  const bytes = inputBytes(source);
  const startRule = options.startRule ?? program.start;
  if (!program.rules.has(startRule)) throw new GrammarRuntimeError(`undefined start rule ${startRule}`, 'reference');
  const memory = memoryBudget(options);
  const budget = stepBudget(options, bytes.length, memory);
  const maxDepth = options.maxDepth ?? 1000;
  let outcome;
  // Under `(matching longest)` the input is parsed again while the tree takes
  // a token rule's leaf where a keyword a lexer prefers matched.
  const keywords = program.tokenRanks && program.settling?.tokens !== false ? new KeywordLexing({ ...program.tokenRanks, bytes }, program) : null;
  const expectations = new Expectations();
  try {
    do {
      outcome = runParse(program, bytes, startRule, options, maxDepth, expectations, memory, (executor) => {
        executor.keywords = keywords;
        // With no repair point yet, recovery only notes where elements fail.
        if (options.errorRecovery) executor.repairPoints = new Set();
      }, search);
      if (!outcome.ok && options.errorRecovery) outcome = repairParse(program, bytes, startRule, options, maxDepth, outcome, keywords, expectations, memory);
    } while (keywords && outcome.ok && keywords.conflicts(outcome.root));
    if (outcome.ok && options.grammarName && options.globalConceptSpace) outcome.root = qualifyGrammarRoot(outcome.executor, outcome.root, options.grammarName);
  } catch (error) {
    if (error instanceof StepLimitReached) {
      return withGrammarSelectionSources({ ok: false, tree: null, ambiguities: [], rejection: { reason: 'stepLimit', limit: budget.limit } }, memory);
    }
    if (error instanceof MemoryBudgetReached) {
      return withGrammarSelectionSources({ ok: false, tree: null, ambiguities: [], rejection: { reason: 'memoryBudget', limit: memory.limit } }, memory);
    }
    if (error instanceof NestingTooDeep || (error instanceof RangeError && /call stack/u.test(error.message))) {
      const tree = publicTree({ type: 'error', start: 0, end: bytes.length, reason: 'nestingDepth' }, bytes);
      return withGrammarSelectionSources({ ok: false, tree, ambiguities: [], rejection: { reason: 'nestingDepth', limit: maxDepth } }, memory);
    }
    throw error;
  }
  if (!outcome.ok) {
    if (search) return { ok: false, rejection: { reason: 'syntax' } };
    const diagnostics = budget.memory.selectionDiagnostics;
    if (diagnostics?.length) return withGrammarSelectionSources({ ok: false, tree: null, ambiguities: [], rejection: positioned(diagnostics[0].reason, bytes, diagnostics[0].start, { diagnostics }) }, memory);
    const mappingDiagnostics = memory.selectionMappingFailures;
    if (mappingDiagnostics?.length) {
      const primary = mappingDiagnostics.reduce((left, right) => right.start >= left.start ? right : left);
      const reason = primary.reason === 'mappedSyntax' ? 'syntax' : primary.reason;
      return withGrammarSelectionSources({ ok: false, tree: null, ambiguities: [], rejection: positioned(reason, bytes,
        primary.start, { expected: outcome.expected, diagnostics: mappingDiagnostics,
          offsetSemantics: 'covering-source-spelling' }) }, memory);
    }
    return withGrammarSelectionSources({ ok: false, tree: null, ambiguities: [], rejection: positioned('syntax', bytes, outcome.farthest, { expected: outcome.expected }) }, memory);
  }
  // A materialized grammar, resolved child or per-parse override can reach
  // mapped execution without an authoring policy in this decorator set.
  // Tree changes have no proof for the retained physical source witnesses.
  const decorators = decoratorSet(options.decorators);
  const publicationIssue = readGrammarCompositionPublicationIssue(Boolean(memory.selectionSources?.records.length),
    decorators.has('executor') || decorators.has('recovery'));
  if (publicationIssue) throw new DecoratorError(publicationIssue);
  const matchEnd = search ? outcome.root.end : undefined;
  const tree = decorateSyntaxTree(publicTree(outcome.root, bytes), decorators);
  const ambiguities = collectAmbiguities(outcome.root, program);
  let rejection = null;
  const diagnostics = memory.selectionDiagnostics;
  const recovery = firstRecovery(tree);
  if (diagnostics?.length) rejection = positioned(diagnostics[0].reason, bytes, diagnostics[0].start, { diagnostics });
  else if (recovery && options.recovery !== 'accept') rejection = positioned('recovered', bytes, recovery.start,
    recovery.source === undefined ? {} : { diagnostics: [recovery], offsetSemantics: 'covering-source-spelling' });
  else if (ambiguities.length > 0 && options.ambiguity === 'reject') rejection = positioned('ambiguity', bytes, ambiguities[0].start);
  return withGrammarSelectionSources({ ok: rejection === null, tree, ambiguities, rejection, ...(search ? { matchEnd } : {}) }, memory);
}

function wordCharacters(source) {
  const words = new Map();
  let offset = 0;
  let previous = false;
  for (const character of source) {
    const characterClass = textLocationCharacterClass(character);
    const current = characterClass === 'word' || characterClass === 'connector';
    words.set(offset, insideGrammarSearchWord(previous, current));
    offset += character.length;
    previous = current;
  }
  words.set(offset, false);
  return words;
}

function searchCaptures(tree, source, byteToUnit, captureNames) {
  const captures = Object.create(null);
  for (const name of captureNames) captures[name] = [];
  const pending = [tree];
  while (pending.length) {
    const node = pending.pop();
    if ((node.type === 'node' || node.type === 'token') && Object.hasOwn(captures, node.kind)) {
      const start = byteToUnit.get(node.start);
      const end = byteToUnit.get(node.end);
      if (start !== undefined && end !== undefined) captures[node.kind].push({ start, end, text: source.slice(start, end) });
    }
    const children = node.children ?? node.sourceChildren ?? (node.root ? [node.root] : []);
    for (let index = children.length - 1; index >= 0; index -= 1) pending.push(children[index]);
  }
  return captures;
}

function findGrammarMatches(program, source, options, {
  rule = options.startRule ?? program.start, overlapping = false, wordBoundaries = false,
  matchPolicy = 'leftmost-longest', captures = [],
} = {}) {
  if (typeof source !== 'string') throw new TypeError('grammar search reads Unicode text');
  if (!program.rules.has(rule)) throw new GrammarRuntimeError(`undefined start rule ${rule}`, 'reference');
  if (matchPolicy !== 'leftmost-longest' && matchPolicy !== 'peg-first') throw new TypeError('unknown grammar search match policy');
  if (!Array.isArray(captures) || captures.some(name => typeof name !== 'string')) throw new TypeError('grammar search capture names must be strings');
  const bytes = inputBytes(source);
  const { byteToUnit } = utf16View(bytes, 0, bytes.length);
  const words = wordBoundaries ? wordCharacters(source) : null;
  const acceptsBoundary = (position) => byteToUnit.has(position) && !words?.get(byteToUnit.get(position));
  const search = { begin: 0, acceptEnd: acceptsBoundary, matchPolicy, steps: 0 };
  const parseOptions = { ...options, startRule: rule, errorRecovery: false };
  const matches = [];
  let next = 0;
  for (const [begin, start] of byteToUnit) {
    if (begin === bytes.length || skipGrammarSearchStart(begin < next, overlapping) || !acceptsBoundary(begin)) continue;
    search.begin = begin;
    const outcome = parseProgram(program, bytes, parseOptions, search);
    if (!outcome.ok) {
      if (['stepLimit', 'memoryBudget', 'nestingDepth'].includes(outcome.rejection.reason)) {
        throw new GrammarParseError(outcome.rejection);
      }
      continue;
    }
    const { tree, sourcePlanes, matchEnd } = outcome;
    const end = byteToUnit.get(matchEnd);
    matches.push({ start, end, text: source.slice(start, end), tree,
      captures: searchCaptures(tree, source, byteToUnit, captures), ...(sourcePlanes ? { sourcePlanes } : {}) });
    next = matchEnd;
  }
  return matches;
}

/**
 * Compiles `grammar` (a Grammar or its normalized document) for the native
 * executor. `options.resolveGrammar(name)` returns the grammar an import or
 * an embedded language names; `maxDepth`, `stepLimit`, `memoLimit` and
 * `memoryLimit` (the memo cells a parse keeps) bound a parse;
 * `ambiguity: 'reject'` turns a reported ambiguity into a rejection and
 * `recovery: 'accept'` accepts a tree with ERROR or MISSING nodes;
 * `errorRecovery: true` repairs a failed parse into such a tree (at most
 * `maxRepairs` repair points) instead of rejecting it without one.
 * Each parse may override the options and choose a `startRule`.
 * Linked `any-grammar` and `all-grammar` expressions retain candidate CSTs.
 * Supplying both `grammarName` and an existing `globalConceptSpace` also
 * stores the root invocation's qualified provenance in an embed boundary;
 * other parses keep their existing root shape. The explicit json-string
 * mapping retains the original syntactic region and decodes only Unicode
 * scalar JSON strings; its child offsets belong to a separate source plane.
 * `options.decorators` (a DecoratorSet) decorates the grammar's rules at the
 * `grammar-rule` level before it is loaded and every tree at the `executor`
 * and `recovery` levels (see `decorateSyntaxTree`).
 */
export function createGrammarParser(grammar, options = {}) {
  const decorated = grammar?.rules instanceof Map ? decorateGrammar(grammar, options.decorators) : grammar;
  const program = loadProgram(decorated, options);
  const parseTree = (source, parseOptions = {}) => parseProgram(program, source, { ...options, ...parseOptions });
  return {
    parseTree,
    findAll(searchSource, searchOptions) {
      return findGrammarMatches(program, searchSource, options, searchOptions);
    },
    searchPredicate(searchOptions) {
      const textPredicate = sourceTextPredicateHost((network, id) => network.capturedText(id));
      return (predicate, captures, network) => {
        if (predicate.name !== 'grammar-match?') return textPredicate(predicate, captures, network);
        const [argument] = predicate.arguments;
        if (predicate.arguments.length !== 1 || argument?.capture === undefined) return false;
        const id = captures.first(argument.capture);
        if (id === undefined) return false;
        const text = network.capturedText(id);
        try {
          return findGrammarMatches(program, text, options, searchOptions).length > 0;
        } catch (error) {
          if (error instanceof GrammarParseError || error instanceof GrammarRuntimeError) return false;
          throw error;
        }
      };
    },
    parse(source, parseOptions = {}) {
      const outcome = parseTree(source, parseOptions);
      if (!outcome.ok) throw new GrammarParseError(outcome.rejection);
      if (outcome.sourcePlanes) throw new GrammarRuntimeError('a mapped grammar result needs parseTree to retain its sourcePlanes', 'selection');
      return outcome.tree;
    },
  };
}
