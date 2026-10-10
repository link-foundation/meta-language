// The native grammar executor: it interprets a loaded program (load.js) over
// a byte string and builds the lossless concrete syntax tree. Generalized
// matching keeps every result an expression can produce, deduplicated by end
// offset and parser state; PEG matching keeps at most one. Rule calls are
// memoized, left recursion grows a seed to a fixpoint, and the nesting depth,
// the step count, the memo size and the memory a parse keeps are bounded, so
// a hostile or a huge input ends in a rejection instead of a stack overflow,
// a runaway parse or an exhausted heap. Automatic
// recovery reruns a failed parse with repair points, where a failing element
// becomes a MISSING leaf or skips to its next match behind an ERROR leaf.
// docs/grammar/feature-union.md#executor specifies every case below.
import {
  evaluateCondition,
  INITIAL_STATE,
  OperationFailed,
  runStatements,
  settleState,
  workingState,
} from './operations.js';
import { columnOf, decodeAt, encodeText, quoteText, textOf, utf16View } from './text.js';
import { readScannerContinuationAction } from '../translation/frontend-rules.js';

/** Thrown when the rule nesting exceeds `maxDepth`; the driver turns it into an `ERROR` root. */
export class NestingTooDeep extends Error {}

/** Thrown when a parse runs out of its step budget. */
export class StepLimitReached extends Error {}

/** Thrown when a parse runs out of its memory budget (see `memoryBudget`). */
export class MemoryBudgetReached extends Error {}

const DEFAULT_MAX_DEPTH = 1000;
const DEFAULT_MEMO_LIMIT = 1_000_000;
const DEFAULT_MEMORY_LIMIT = 2_000_000;
const NO_CHILDREN = Object.freeze([]);
// The text of an extra that is no extra where it starts (see `extraNode`).
const NO_EXTRA = Object.freeze({});
// The verdict of the precedence filter on a result whose right operand is a
// node of one part: valid unless a result ends before the operand (see
// `lonePending`).
const LONE = Symbol('lone operand');

/**
 * What a scanner's `(expected ITEM)` answers: whether the parse requested the
 * item, a literal or a rule, at the scanner's context offset, as a tree-sitter
 * scanner asks whether a token is valid in the parse state. The context of a
 * token is the offset before the trivia its terminal skips, and of an extra
 * the offset its trivia starts at. A request is recorded where the parse
 * makes it, so one the scanner asks about before the parse made it was
 * answered wrong: the parse is then `stale` and runs again, seeded with the
 * requests so far. The requests only grow, so the runs end. One parse, its
 * rounds and its embedded languages share one record; each item has a
 * distinct id per program.
 */
export class Expectations {
  constructor() {
    this.requested = new Map();
    this.consulted = new Map();
    this.stale = false;
  }

  /** Starts a run of the parse, keeping the requests. */
  restart() {
    this.consulted = new Map();
    this.stale = false;
    return this;
  }

  request(position, id) {
    let ids = this.requested.get(position);
    if (!ids) this.requested.set(position, ids = new Set());
    if (ids.has(id)) return;
    ids.add(id);
    if (this.consulted.get(position)?.has(id)) this.stale = true;
  }

  holds(position, id) {
    if (this.requested.get(position)?.has(id)) return true;
    let ids = this.consulted.get(position);
    if (!ids) this.consulted.set(position, ids = new Set());
    ids.add(id);
    return false;
  }
}

/**
 * The step budget of one parse, shared with every embedded-language executor:
 * by default twice as large with error recovery, whose rounds parse each
 * repaired alternative too.
 */
export function stepBudget(options, length, memory = memoryBudget(options)) {
  return { steps: 0, limit: options.stepLimit ?? (100_000 + 1000 * length) * (options.errorRecovery ? 2 : 1), memory };
}

/**
 * The memory budget of one parse, shared by its runs, its recovery rounds and
 * its embedded languages: the memo cells it may make, one per rule call it
 * records and one per result such a call keeps (`memoryLimit`, default
 * 2000000). The cells are what a parse keeps until it ends (the calls under
 * keyword lexing outlive their run), so they are counted, never released,
 * and a parse that needs more is rejected with `memoryBudget` instead of
 * growing until the process runs out of memory.
 */
export function memoryBudget(options) {
  return { cells: 0, limit: options.memoryLimit ?? DEFAULT_MEMORY_LIMIT };
}

// The repair cost of a MISSING leaf; an ERROR leaf costs the bytes it skips.
const MISSING_COST = 2;

function makeResult(end, state, children = NO_CHILDREN, dynamic = 0, cost = 0) {
  return { end, state, children, dynamic, precedence: null, tail: null, ambiguous: false, cost };
}

function resultKey(result) {
  return `${result.end}|${result.state.key}`;
}

// The settling of a program loaded before grammars declared one: every step.
const ALL_SETTLING = Object.freeze({ steps: Object.freeze(['tokens', 'precedence', 'dynamic', 'ambiguity']), tokens: true, precedence: true, dynamic: true, ambiguity: true });

// The order of two results of one text that end alike by the grammar's
// settling steps (see SETTLING_STEPS): 1 when `a` is preferred, -1 when `b`
// is, 0 on a tie. `tokens` holds the token ranks, the input bytes and the
// precedence orders the `tokens` step (the tokens a lexer prefers, as a
// lexer decides them before any parse does) and the `precedence` step (the
// shift or reduction an LR parser keeps by precedence) read; `dynamic`
// prefers the higher dynamic precedence.
function settledOrder(a, b, settling, tokens) {
  for (const step of settling.steps) {
    let order = 0;
    if (step === 'tokens') order = tokens ? preferredTokens(a, b, tokens) : 0;
    else if (step === 'precedence') order = tokens ? shiftOrder(a, b, tokens.orders, tokens.grammar, tokens.bytes, tokens.owner) : 0;
    else if (step === 'dynamic') order = Math.sign(a.dynamic - b.dynamic);
    else break;
    if (order !== 0) return order;
  }
  return 0;
}

// Adds a result to a deduplicating map: of two results with the same end
// and state, the lower repair cost wins, then the one the grammar's settling
// steps prefer (see settledOrder); on a tie the first stays and, when the
// settling ends in `ambiguity`, without repairs and unless both build the
// same trees, is marked ambiguous (as a copy, since results are shared
// through the memo).
function addResult(results, result, tokens = null, settling = ALL_SETTLING) {
  const key = resultKey(result);
  const existing = results.get(key);
  if (!existing || result.cost < existing.cost) {
    results.set(key, result);
    return;
  }
  if (result.cost > existing.cost) return;
  const order = settledOrder(result, existing, settling, tokens);
  // A trace, off unless a probe sets the array (see
  // experiments/native-order-trace.mjs): every decided pair and its order.
  globalThis.__orderTrace?.push([result, existing, order]);
  if (order > 0) results.set(key, result);
  else if (order === 0 && settling.ambiguity && existing.cost === 0 && !existing.ambiguous && !sameOutput(result.children, existing.children)) {
    // A trace, off unless a probe sets the array (see
    // experiments/native-rust-ambiguity-pair.mjs): the two results it ties.
    globalThis.__ambiguityPairs?.push([result, existing]);
    results.set(key, copyResult(existing, { ambiguous: true }));
  }
}

// Whether two child lists build the same trees, trivia, fields and
// attributes included: two results that reach them in different ways (Rust's
// `+.` in a token tree, one run of `(precedence 0 right (repeat1 ...))` or two,
// both flattened by the silent rule) are one parse, not an ambiguity.
function sameOutput(a, b) {
  if (a === b) return true;
  if (a.length !== b.length) return false;
  return a.every((first, index) => {
    const second = b[index];
    if (first === second) return true;
    if (first.type !== second.type || first.kind !== second.kind || first.start !== second.start || first.end !== second.end) return false;
    if (Boolean(first.trivia) !== Boolean(second.trivia) || (first.field ?? null) !== (second.field ?? null) || (first.language ?? null) !== (second.language ?? null)) return false;
    if (!sameValue(first.attributes ?? null, second.attributes ?? null)) return false;
    return first.type !== 'node' || sameOutput(first.children, second.children);
  });
}

// Whether two attribute values are equal, objects whatever their key order.
function sameValue(a, b) {
  if (a === b) return true;
  if (typeof a !== 'object' || typeof b !== 'object' || a === null || b === null || Array.isArray(a) !== Array.isArray(b)) return false;
  const keys = Object.keys(a);
  return keys.length === Object.keys(b).length && keys.every((key) => Object.hasOwn(b, key) && sameValue(a[key], b[key]));
}

// Which of two results over the same text has the tokens a lexer prefers:
// their leaves are walked in order, skipping trivia and the subtrees both
// share, and the first leaf pair that differs decides, as a tree-sitter lexer
// decides a conflict between two tokens at one offset: the higher lexical
// precedence, then the longer token, then the more specific (a literal over a
// pattern) and the earlier one (see `tokenRanks` in load.js). A token where
// the other result skipped a separator it covers (whitespace trivia) wins
// too, as the lexer takes a valid token over a separator. 1 when `result`
// has the preferred token, -1 when `existing` has, 0 when neither (a pair
// without ranks that ends alike but differs in kind, or every leaf alike).
function preferredTokens(result, existing, tokens) {
  const left = [[result.children, 0]];
  const right = [[existing.children, 0]];
  // The trivia each side skipped since the last leaf both share.
  let skipped = [[], []];
  // Where one parse has a leaf alone and the other a node that begins with
  // it, one of them reduced that leaf (to a silent rule, or to the node) where
  // the other shifted on: an LR parser decides between them on the token
  // after the leaf, the lookahead, and a leaf conflict after the lookahead is
  // no lexer's, as the two then lex in different parse states (Rust's
  // `$(...);*`, whose `;` is a separator only after `$(` was shifted): a
  // silent rule reduced the leaf alone in one of them, or the node, or one
  // along its leftmost chain, ends with it. Where neither reduced it (the `{`
  // of a JavaScript block and of an object), the two shift it alike. Two
  // leaves alike part at their end too where only one was reduced alone.
  // `pending` is the end of that leaf, `decided` the lookahead's start.
  let pending = Infinity;
  let decided = Infinity;
  const lookahead = (tree) => {
    if (decided === Infinity && !isTrivia(tree) && firstLeafStart(tree) >= pending) decided = firstLeafStart(tree);
  };
  // An extra token of another kind is covered where the leaf wins the
  // lexical conflict with it (Make's `raw_line` `#comment\n`, longer than the
  // comment of a lower precedence).
  const covers = (leaf, trivia) => trivia.some((item) => item.start === leaf.start
    && ((item.kind === null || item.kind === leaf.kind) ? leaf.end >= item.end : (tokens !== null && item.type === 'token' && tokenConflict(leaf, item, tokens) > 0)));
  const peek = (stack) => {
    while (stack.length > 0) {
      const top = stack[stack.length - 1];
      if (top[1] < top[0].length) return top[0][top[1]];
      stack.pop();
    }
    return null;
  };
  const skip = (stack) => { stack[stack.length - 1][1] += 1; };
  const enter = (stack, node) => {
    skip(stack);
    stack.push([node.children, 0]);
  };
  for (;;) {
    const a = peek(left);
    const b = peek(right);
    if (a === null || b === null) {
      // One parse ends where the other goes on with a token of the external
      // scanner, which then scans it, of no width, in the state both share
      // (TypeScript's automatic semicolon after `namespace A {}` before a
      // line break, which the expression statement takes and the
      // declaration cannot): the lexer took it before any parse decided, if
      // the two did not part before it (`declare module "m" {}` before a line
      // break, whose `{` one parse shifts as the module's body where the
      // other reduced the module, decides first), and if the node the ended
      // parse closes with is one the other reduced too (where `{}` after a
      // line break is a statement block in one and an object in the other,
      // the two reduced apart, and the block could take the token itself).
      // Where the ended parse built nothing (an optional layout semicolon of
      // Lean, taken or not), the two part at the token, which the scanner
      // scans first.
      const rest = a === null ? right : left;
      if (a === null && b === null) return 0;
      // The next token of the other parse covers a separator the ended one
      // skipped (Go's `\n` that ends the last line, where `\s` is trivia):
      // the lexer takes that valid token over the separator.
      const first = peek(rest);
      if (first.type === 'token' && !isTrivia(first) && covers(first, skipped[a === null ? 0 : 1])) return a === null ? -1 : 1;
      const ended = (a === null ? result : existing).children.filter((child) => !isTrivia(child)).at(-1);
      if (ended !== undefined && (ended.type !== 'node' || !hasNode((a === null ? existing : result).children, ended))) return 0;
      for (let next = peek(rest); next !== null; next = peek(rest)) {
        if (isTrivia(next)) skip(rest);
        else if (next.type === 'node') enter(rest, next);
        else if (next.start !== next.end || !scannedToken(next) || next.start > decided) return 0;
        else return next.start <= partingEnd(result.children, existing.children) ? (a === null ? -1 : 1) : 0;
      }
      return 0;
    }
    if (a === b) {
      lookahead(a);
      skip(left);
      skip(right);
      skipped = [[], []];
    } else if (a.type === 'node' && b.type === 'node' && (nestsFirst(a, b) || nestsFirst(b, a))) {
      // A node one parse wraps deeper (Rust's `.. ..` as the left operand of
      // `..=` or of an assignment) is the first part of the other parse's
      // node: that one is entered alone until the two line up, so the node
      // meets its match, not a leaf of it.
      if (nestsFirst(a, b)) enter(left, a);
      else enter(right, b);
    } else if (a.type === 'node' || b.type === 'node') {
      const [node, lone] = a.type === 'node' ? [a, b] : [b, a];
      if (lone.type !== 'node' && !isTrivia(lone) && reducedFirst(node, lone)) pending = Math.min(pending, lone.end);
      if (a.type === 'node') enter(left, a);
      if (b.type === 'node') enter(right, b);
    } else if (a.trivia || b.trivia) {
      if (a.trivia) skipped[0].push(a);
      if (b.trivia) skipped[1].push(b);
      if (a.trivia) skip(left);
      if (b.trivia) skip(right);
    } else if (a.start < b.start && covers(a, skipped[1])) {
      return startsAtSeparator(b, a, tokens) ? -1 : 1;
    } else if (b.start < a.start && covers(b, skipped[0])) {
      return startsAtSeparator(a, b, tokens) ? 1 : -1;
    } else if (a.end !== b.end || (a.kind !== b.kind && !(a.start === b.start && a.lexed !== undefined && a.lexed === b.lexed) && !sameLiteral(a, b, tokens)) || otherRanks(a, b)) {
      // Two leaves of one span the same token rule built (JavaScript's
      // `identifier` and its alias `shorthand_property_identifier`) are one
      // token to the lexer, as are two of one literal (Make's `$$`, an
      // `escape` and the start of a variable reference). Two leaves of one
      // kind may be two tokens (see `otherRanks`).
      lookahead(a);
      lookahead(b);
      if (Math.min(a.start, b.start) > decided) return 0;
      // A scanner token against a token the lexer would lex there: tree-sitter
      // runs the external scanner before its lexer wherever one of its tokens
      // is valid, and takes the token it scans (JavaScript's automatic
      // semicolon after `return` before a line break, not the next line's
      // expression).
      const scanned = scannedToken(a) - scannedToken(b);
      return scanned !== 0 ? scanned : tokenConflict(a, b, tokens);
    } else {
      lookahead(a);
      // One leaf a silent rule reduced alone, the other not: the two part
      // at its end (Rust's `$` of a token tree pattern, a lone token, and
      // of `$(...);*`).
      if (Boolean(a.alone) !== Boolean(b.alone)) pending = Math.min(pending, a.end);
      skip(left);
      skip(right);
      skipped = [[], []];
    }
  }
}

// Whether `children`, or a node below them, is a node of the kind and span of
// `node`.
function hasNode(children, node) {
  return children.some((child) => child.type === 'node' && ((child.kind === node.kind && child.start === node.start && child.end === node.end) || hasNode(child.children, node)));
}

// The first end where two parses part, a node of one kind from one offset
// that one of them ends there and the other goes on past (the `module` of
// `declare module "m" {}`, which ends before the body in one), or Infinity.
function partingEnd(first, second) {
  const spans = (children, found = new Map()) => {
    for (const child of children) {
      if (child.type !== 'node') continue;
      const key = `${child.kind}@${child.start}`;
      if (!found.has(key)) found.set(key, new Set());
      found.get(key).add(child.end);
      spans(child.children, found);
    }
    return found;
  };
  const [mine, theirs] = [spans(first), spans(second)];
  let end = Infinity;
  for (const [key, ends] of mine) {
    const other = theirs.get(key);
    if (!other) continue;
    for (const at of ends) if (!other.has(at)) end = Math.min(end, at, ...other);
  }
  return end;
}

// Whether a parse that has `leaf` alone and one that has `node`, which begins
// with it, part at its end: one of them reduced the leaf, a silent rule alone
// or as the end of `node` or of a node along its leftmost chain.
function reducedFirst(node, leaf) {
  const chain = leftmostChain(node);
  const first = chain.at(-1)?.children.find((child) => !isTrivia(child));
  return Boolean(leaf.alone || (first?.start === leaf.start && first.alone) || chain.some((inner) => inner.end === leaf.end));
}

// 1 when a leaf is a token of an external scanner, 0 otherwise.
function scannedToken(leaf) {
  return leaf.scanned === true ? 1 : 0;
}

// Whether a node of the kind and span of `inner` is on the leftmost chain of
// the longer node `outer`.
function nestsFirst(outer, inner) {
  return outer.end > inner.end && leftmostChain(outer).some((node) =>
    node.kind === inner.kind && node.start === inner.start && node.end === inner.end);
}

// The rank of a token leaf, or null for another leaf or an unranked token.
// A leaf matched under a lexical precedence ranks at that level, as the
// token defined there (Rust's `//!` marker `!` outranks the comment text).
function tokenRank(leaf, tokens) {
  if (leaf.type !== 'token') return null;
  const rank = leaf.rank ?? (leaf.kind !== null ? tokens.kinds.get(leaf.kind) : tokens.literals.get(textOf(tokens.bytes, leaf.start, leaf.end))) ?? null;
  if (leaf.priority === undefined) return rank;
  return { specificity: 0, order: Infinity, ...rank, priority: leaf.priority };
}

// Two leaves that differ in end or kind: 1 when a lexer prefers `a`, -1 when
// it prefers `b`, 0 when it cannot tell. The ranks decide only between two
// tokens at one offset; otherwise the longer leaf wins, but for a token of
// separator text alone: a tree-sitter lexer that has lexed it, its separators
// still going on, takes no transition of another token of no higher
// precedence (`prefer_transition`), so Make's immediate blank after `=` is
// a token of its own before the value's text, which could take it.
function tokenConflict(a, b, tokens) {
  const first = a.start === b.start ? tokenRank(a, tokens) : null;
  const second = a.start === b.start ? tokenRank(b, tokens) : null;
  if (first && second && first.priority !== second.priority) return first.priority > second.priority ? 1 : -1;
  // Two ends of one token at one offset are two tokens (Make's blank and
  // text are both `unnamed_token`), as a token lexes its longest match; the
  // longer, of separators too, may be the same token gone on.
  if (a.end !== b.end && a.start === b.start && !(a.lexed !== undefined && a.lexed === b.lexed)) {
    const [shorter, longer, sign] = a.end < b.end ? [a, b, 1] : [b, a, -1];
    if (tokens.separatorText?.(shorter.start, shorter.end) && !tokens.separatorText(longer.start, longer.end)) return sign;
  }
  if (a.end !== b.end) return a.end > b.end ? 1 : -1;
  if (!first || !second || sameRank(first, second)) return 0;
  if (first.specificity !== second.specificity) return first.specificity > second.specificity ? 1 : -1;
  return first.order < second.order ? 1 : -1;
}

// Whether `leaf`, after separators `cover` takes, wins over it: a lexer
// that goes on with a separator as the first of a valid token skips it no
// more, so `leaf` starts there too (see `widenTokens`), and wins at a higher
// precedence (Make's `@` of a recipe line after `; `, over its shell text).
// A cover of separator text alone the lexer completes first, and takes no
// separator transition after it (see `tokenConflict`): Make's line breaks
// after `;` end the recipe's first line, before a `\t@` that could go on.
function startsAtSeparator(leaf, cover, tokens) {
  if (tokens === null || leaf.type !== 'token' || cover.type !== 'token') return false;
  if (tokens.separatorText?.(cover.start, cover.end)) return false;
  const [mine, theirs] = [tokenRank(leaf, tokens), tokenRank(cover, tokens)];
  return Boolean(mine && theirs) && mine.priority > theirs.priority;
}

// Whether two leaves of one span are one literal of one rank, whatever kind
// each is aliased to.
function sameLiteral(a, b, tokens) {
  if (a.start !== b.start || a.end !== b.end || a.type !== 'token' || b.type !== 'token') return false;
  const [first, second] = [tokenRank(a, tokens), tokenRank(b, tokens)];
  return Boolean(first && second) && first.specificity >= 2 && first.priority === second.priority && first.specificity === second.specificity;
}

// Whether two leaves of one alias name are tokens of other precedences or
// specificities (Make's immediate blank after `=` and the text there, both
// `unnamed_token`): a lexer tells them apart. Tokens that differ only in
// order may be one token the grammar repeats.
function otherRanks(a, b) {
  return a.rank !== undefined && b.rank !== undefined && (a.rank.priority !== b.rank.priority || a.rank.specificity !== b.rank.specificity);
}

// Whether two token ranks are the same, as one token's.
function sameRank(a, b) {
  return a.priority === b.priority && a.specificity === b.specificity && a.order === b.order;
}

// The node a node only wraps, with the precedence it comes from, under
// every such wrapper (Solidity's visible `expression` around a
// `binary_expression`): a precedence conflict is between the parts of the
// node wrapped, as the parser reduces its wrappers after the conflict.
function wrapped(node) {
  let current = node;
  while (current.type === 'node' && current.precedence) {
    const meaningful = current.children.filter((child) => !isTrivia(child));
    if (meaningful.length !== 1 || meaningful[0].type !== 'node' || !samePrecedence(meaningful[0].precedence, current.precedence)) break;
    current = meaningful[0];
  }
  return current;
}

// Whether two precedences are the same, or both none.
function samePrecedence(a, b) {
  if (!a || !b) return !a && !b;
  return a.level === b.level && a.name === b.name && a.associativity === b.associativity;
}

// The precedence of a production reduced or shifted under none: level 0,
// ranked only by its rule's entries in the precedence orders.
function unranked(rule = null) {
  return { level: 0, name: null, associativity: 'none', rule };
}

// The rules `expression` takes directly at its edge facing the operator on
// `side` (its first part for the right operand, as `edgeRules`), without
// entering them, each with the precedence of `owner`'s production that
// takes it there (or null under none).
function directEdge(expression, side, owner, found = [], tag = null) {
  switch (expression.kind) {
    case 'ref': found.push([expression.name, tag]); break;
    case 'seq': {
      const items = side === 'left' ? [...expression.items].reverse() : expression.items;
      for (const item of items) {
        directEdge(item, side, owner, found, tag);
        if (!(item.kind === 'optional' || item.kind === 'repeat0' || (item.kind === 'repeat' && item.min === 0))) break;
      }
      break;
    }
    case 'choice': for (const item of expression.items) directEdge(item, side, owner, found, tag); break;
    case 'precedence': case 'namedPrecedence':
      directEdge(expression.item, side, owner, found, { level: expression.level ?? 0, name: expression.name ?? null, associativity: expression.associativity, rule: owner });
      break;
    case 'capture': case 'dynamicPrecedence': case 'alias':
    case 'optional': case 'repeat0': case 'repeat1': case 'repeat': directEdge(expression.item, side, owner, found, tag);
  }
  return found;
}

// What the conflicts of two results ask of `program`'s rules, computed once
// per program: the rules each rule takes directly as its first part, by rule
// name (see `shiftReduction`), the order the rules are defined in, and the
// groups of rules whose conflicts the grammar declares (see `forkedOrder`).
// The alternatives of an unordered choice that are an immediate token of no
// literal (`patterns`, each to its token's expression) and the immediate
// literals the other alternatives begin with (`literals`), or null where
// there are not both: see `literalOutranks`.
const IMMEDIATE_LITERALS = new WeakMap();
function immediateLiterals(choice) {
  if (IMMEDIATE_LITERALS.has(choice)) return IMMEDIATE_LITERALS.get(choice);
  const patternOf = (expression) => {
    if (expression.kind === 'alias' || expression.kind === 'capture') return patternOf(expression.item);
    return expression.kind === 'immediateToken' && expression.item.kind !== 'literal' && expression.item.kind !== 'lexicalPrecedence' ? expression.item : null;
  };
  const leading = (expression) => {
    switch (expression.kind) {
      case 'immediateToken': return expression.item.kind === 'literal' ? [expression.item.value] : [];
      case 'seq': return expression.items.length > 0 ? leading(expression.items[0]) : [];
      case 'choice': return expression.items.flatMap(leading);
      case 'alias': case 'capture': case 'precedence': case 'namedPrecedence': case 'dynamicPrecedence': return leading(expression.item);
      default: return [];
    }
  };
  const patterns = new Map();
  const literals = new Set();
  for (const item of choice.items) {
    const pattern = patternOf(item);
    if (pattern) patterns.set(item, pattern);
    else for (const value of leading(item)) literals.add(value);
  }
  const found = patterns.size > 0 && literals.size > 0 ? { patterns, literals } : null;
  IMMEDIATE_LITERALS.set(choice, found);
  return found;
}

const GRAMMAR_FACTS = new WeakMap();

function grammarFacts(program) {
  let facts = GRAMMAR_FACTS.get(program);
  if (!facts) {
    const heads = new Map();
    const ranks = new Map();
    for (const [name, rule] of program.rules) {
      heads.set(name, new Set(directEdge(rule.expression, 'right', name).map(([ref]) => ref)));
      ranks.set(name, rule.index);
    }
    facts = { heads, ranks, conflicts: (program.conflictGroups ?? []).map((group) => new Set(group)), rules: program.rules, first: null, forks: new Map(), items: new Map(), reductions: null };
    GRAMMAR_FACTS.set(program, facts);
  }
  return facts;
}

// The tokens each of the `rules` can begin with, as tree-sitter's FIRST
// sets: a literal by its text (`literal <`), a token rule, an external token
// or a rule a lexer matches whole by its name (`ref identifier`).
function firstSets(rules) {
  const sets = new Map([...rules.keys()].map((name) => [name, new Set()]));
  for (let changed = true; changed;) {
    changed = false;
    for (const [name, rule] of rules) {
      if (rule.kind === 'token' || rule.kind === 'atomic') continue;
      const set = sets.get(name);
      const size = set.size;
      for (const key of firstOf(rule.expression, rules, sets)) set.add(key);
      if (set.size !== size) changed = true;
    }
  }
  return sets;
}

// The tokens `expression` can begin with, by the FIRST sets of the rules
// known so far (see `firstSets`).
function firstOf(expression, rules, sets, found = new Set()) {
  switch (expression.kind) {
    case 'literal': found.add(`literal ${expression.value}`); break;
    case 'ref': {
      const rule = rules.get(expression.name);
      if (!rule || rule.kind === 'token' || rule.kind === 'atomic') found.add(`ref ${expression.name}`);
      else for (const key of sets.get(expression.name)) found.add(key);
      break;
    }
    case 'seq':
      for (const item of expression.items) {
        firstOf(item, rules, sets, found);
        if (!nullable(item)) break;
      }
      break;
    case 'choice': for (const item of expression.items) firstOf(item, rules, sets, found); break;
    case 'alias': found.add(`ref ${expression.name}`); firstOf(expression.item, rules, sets, found); break;
    case 'token': case 'immediateToken': case 'lexicalPrecedence': case 'capture': case 'precedence': case 'namedPrecedence':
    case 'dynamicPrecedence': case 'optional': case 'repeat0': case 'repeat1': case 'repeat':
      firstOf(expression.item, rules, sets, found);
  }
  return found;
}

// The tokens that may follow each of the `rules`, as the FOLLOW sets of an
// LR parser's lookaheads, by the FIRST sets `first` (see `firstSets`).
function followSets(rules, first) {
  const sets = new Map([...rules.keys()].map((name) => [name, new Set()]));
  const walk = (expression, after) => {
    switch (expression.kind) {
      case 'ref': {
        const set = sets.get(expression.name);
        const rule = rules.get(expression.name);
        if (!set || rule.kind === 'token' || rule.kind === 'atomic') break;
        for (const key of after) if (!set.has(key)) { set.add(key); walk.changed = true; }
        break;
      }
      case 'seq': {
        let rest = after;
        for (const item of [...expression.items].reverse()) {
          walk(item, rest);
          const own = firstOf(item, rules, first);
          rest = nullable(item) ? new Set([...own, ...rest]) : own;
        }
        break;
      }
      case 'repeat0': case 'repeat1': case 'repeat':
        walk(expression.item, new Set([...firstOf(expression.item, rules, first), ...after]));
        break;
      case 'choice': for (const item of expression.items) walk(item, after); break;
      case 'alias': case 'capture': case 'precedence': case 'namedPrecedence': case 'dynamicPrecedence': case 'optional':
        walk(expression.item, after);
    }
  };
  do {
    walk.changed = false;
    for (const [name, rule] of rules) {
      if (rule.kind !== 'token' && rule.kind !== 'atomic') walk(rule.expression, sets.get(name));
    }
  } while (walk.changed);
  return sets;
}

// The tokens that may follow each literal the `rules` lex in syntactic
// context, by its text, wherever it occurs: the lookaheads of the literal's
// token in every LR state, by the FIRST sets `first`.
function literalFollowSets(rules, first) {
  const follow = followSets(rules, first);
  const sets = new Map();
  const walk = (expression, after) => {
    switch (expression.kind) {
      case 'literal': {
        let set = sets.get(expression.value);
        if (!set) sets.set(expression.value, (set = new Set()));
        for (const key of after) set.add(key);
        break;
      }
      case 'seq': {
        let rest = after;
        for (const item of [...expression.items].reverse()) {
          walk(item, rest);
          const own = firstOf(item, rules, first);
          rest = nullable(item) ? new Set([...own, ...rest]) : own;
        }
        break;
      }
      case 'repeat0': case 'repeat1': case 'repeat':
        walk(expression.item, new Set([...firstOf(expression.item, rules, first), ...after]));
        break;
      case 'choice': for (const item of expression.items) walk(item, after); break;
      case 'alias': case 'capture': case 'precedence': case 'namedPrecedence': case 'dynamicPrecedence': case 'optional':
        walk(expression.item, after);
    }
  };
  for (const [name, rule] of rules) if (rule.kind !== 'token' && rule.kind !== 'atomic') walk(rule.expression, follow.get(name));
  return sets;
}

// What a lexer of merged lex states asks of `program`, computed once per
// program (see `mergedLonger`): the token rules that are no extra, the
// tokens that may follow each literal, and the tokens an alias of no rule
// names, by that name (Make's `unnamed_token`, the alias of its inline
// tokens of text and blanks), any of which follows where the name does.
function mergedLexing(program) {
  const grammar = grammarFacts(program);
  if (!grammar.merged) {
    grammar.first ??= firstSets(grammar.rules);
    const extras = new Set(program.trivia.map(({ kind }) => kind).filter((kind) => kind !== null));
    const tokens = [...program.rules].filter(([name, rule]) => rule.kind === 'token' && !extras.has(name)).map(([name, rule]) => ({ name, rule }));
    const aliased = new Map();
    const walk = (expression) => {
      if (expression.kind === 'alias' && !grammar.rules.has(expression.name) && (expression.item.kind === 'token' || expression.item.kind === 'immediateToken')) {
        if (!aliased.has(expression.name)) aliased.set(expression.name, []);
        aliased.get(expression.name).push(expression.item.item);
      }
      for (const item of expression.items ?? (expression.item ? [expression.item] : [])) walk(item);
    };
    for (const rule of grammar.rules.values()) if (rule.kind !== 'token' && rule.kind !== 'atomic') walk(rule.expression);
    grammar.merged = { tokens, follow: literalFollowSets(grammar.rules, grammar.first), aliased };
  }
  return grammar.merged;
}

// The tokens that follow each of the `rules` where it is the first part of a
// production, through its unit chains (a choice, a precedence, a field or an
// alias over it): the lookaheads its reduction has in every LR state it
// begins in, whatever the context (Rust's `-`, `(` or `[` after
// `break_expression`, the left operand of a binary, call or index
// expression), by the FIRST sets `first`.
function leftCornerFollow(rules, first) {
  const units = (expression) => {
    switch (expression.kind) {
      case 'ref': return [expression.name];
      case 'choice': return expression.items.flatMap(units);
      case 'capture': case 'precedence': case 'namedPrecedence': case 'dynamicPrecedence': case 'alias': return units(expression.item);
      default: return [];
    }
  };
  const chains = new Map();
  const chain = (name) => {
    let found = chains.get(name);
    if (found) return found;
    found = new Set([name]);
    const pending = [name];
    while (pending.length > 0) {
      const rule = rules.get(pending.pop());
      if (!rule || rule.kind === 'token' || rule.kind === 'atomic') continue;
      for (const unit of units(rule.expression)) if (!found.has(unit)) { found.add(unit); pending.push(unit); }
    }
    chains.set(name, found);
    return found;
  };
  const sets = new Map([...rules.keys()].map((name) => [name, new Set()]));
  const walk = (expression) => {
    switch (expression.kind) {
      case 'seq':
        for (const [index, item] of expression.items.entries()) {
          const rest = firstOf({ kind: 'seq', items: expression.items.slice(index + 1) }, rules, first);
          for (const unit of units(item)) for (const name of chain(unit)) for (const key of rest) sets.get(name)?.add(key);
          if (!nullable(item)) break;
        }
        expression.items.forEach(walk);
        break;
      case 'choice': expression.items.forEach(walk); break;
      case 'repeat0': case 'repeat1': case 'repeat': case 'capture': case 'precedence': case 'namedPrecedence': case 'dynamicPrecedence': case 'alias': case 'optional':
        walk(expression.item);
    }
  };
  for (const rule of rules.values()) if (rule.kind !== 'token' && rule.kind !== 'atomic') walk(rule.expression);
  return sets;
}

// The reductions an LR parser makes before a token its rule could go on
// with, as tree-sitter settles a shift-reduce conflict by precedence: a rule
// of a left-associative precedence whose sequence ends in optional parts
// (Lean's `hash_command`, `#check` and any expressions, of level 0 left;
// Rust's `break_expression`) is complete before them, and a token they may
// begin with that may also follow the rule is shifted by the rule's own
// item of that precedence and reduced by its completed one, of the same
// precedence: left associativity reduces. `splits` holds, by the items of
// each such sequence, the index its optional parts begin at, the tokens that
// follow the rule in every context (`always`, reduced before wherever the
// parts begin with them) and the tokens that follow it in some (`marked`,
// reduced before where the enclosing parts may go on with them, see
// `reducedEarly`); `keys` all the marked tokens.
function reductionFacts(grammar) {
  if (grammar.reductions) return grammar.reductions;
  grammar.first ??= firstSets(grammar.rules);
  const { rules, first } = grammar;
  let follow = null;
  let corner = null;
  let shared = null;
  const splits = new Map();
  // A conflict the grammar declares of the rule and a rule that begins with
  // the token keeps both parses (Lean's `hash_command` and `explicit`).
  const forked = (name, key) => grammar.conflicts.some((group) => group.has(name)
    && [...group].some((other) => other !== name && firstOf({ kind: 'ref', name: other }, rules, first).has(key)));
  const keys = new Set();
  const visit = (name, expression) => {
    switch (expression.kind) {
      case 'choice': expression.items.forEach((item) => visit(name, item)); return;
      case 'capture': case 'alias': visit(name, expression.item); return;
      case 'precedence': case 'namedPrecedence': break;
      default: return;
    }
    if (expression.associativity !== 'left' || expression.item.kind !== 'seq') return;
    const { items } = expression.item;
    let split = items.length;
    while (split > 0 && nullable(items[split - 1])) split -= 1;
    if (split === 0 || split === items.length) return;
    follow ??= followSets(rules, first);
    corner ??= leftCornerFollow(rules, first);
    shared ??= sharedAliases(rules);
    const parts = [...firstOf({ kind: 'seq', items: items.slice(split) }, rules, first)]
      .filter((key) => follow.get(name).has(key) && !shared.has(key) && !forked(name, key));
    if (parts.length === 0) return;
    const always = new Set(parts.filter((key) => corner.get(name).has(key)));
    const marked = new Set(parts.filter((key) => !always.has(key)));
    for (const key of marked) keys.add(key);
    splits.set(items, { split, always, marked });
  };
  for (const [name, rule] of rules) if (rule.kind === 'normal') visit(name, rule.expression);
  grammar.reductions = { splits, keys, rests: new Map() };
  return grammar.reductions;
}

// The keys of the aliases that name tokens of different content (Lean's
// `unnamed_token`, a number and the `#` of a command alike): the token an
// LR parser sees is the aliased one, so such a key names no one lookahead.
function sharedAliases(rules) {
  const contents = new Map();
  const walk = (expression) => {
    if (!expression || typeof expression !== 'object') return;
    if (expression.kind === 'alias') {
      const content = JSON.stringify(expression.item);
      const seen = contents.get(expression.name) ?? new Set();
      contents.set(expression.name, seen.add(content));
    }
    if (expression.item) walk(expression.item);
    if (expression.items) expression.items.forEach(walk);
  };
  for (const rule of rules.values()) walk(rule.expression);
  return new Set([...contents].filter(([, seen]) => seen.size > 1).map(([name]) => `ref ${name}`));
}

// Whether a rule on the right edge of `result` was reduced before its
// optional parts in the parse that goes on with one of the tokens `keys`
// (Lean's `#check` before `@`, which may begin the next command's
// attributes): an LR parser decides by that lookahead alone, so the rule
// never took the parts that begin with it (see `reductionFacts`).
function reducedEarly(result, keys) {
  let children = result.children;
  for (;;) {
    let last = null;
    for (let index = children.length - 1; index >= 0; index -= 1) {
      if (!isTrivia(children[index])) { last = children[index]; break; }
    }
    if (last?.type !== 'node') return false;
    if (last.before !== undefined && keys.has(last.before)) return true;
    children = last.children;
  }
}

// The token a parser sees first among `children` at or after `offset`, as
// `lookaheadOf` names it, or null.
function lookaheadAfter(children, offset, bytes) {
  for (const child of children) {
    const found = tokenAt(child, offset);
    if (found) return lookaheadOf(found, bytes);
  }
  return null;
}

// The token a parser sees first in `tree`, as its FIRST sets name it (see
// `firstSets`), or null when the tree holds none.
function lookaheadOf(tree, bytes) {
  let token = tree;
  if (!token) return null;
  while (token?.type === 'node') token = token.children.find((child) => !isTrivia(child));
  if (!token) return null;
  return token.kind ? `ref ${token.kind}` : `literal ${textOf(bytes, token.start, token.end) ?? ''}`;
}

// The first meaningful leaf of `tree` that begins at or after `offset`, or
// null.
function tokenAt(tree, offset) {
  if (tree.type !== 'node') return !isTrivia(tree) && tree.start >= offset ? tree : null;
  for (const child of tree.children) {
    if (child.end <= offset && child.end > child.start) continue;
    const found = tokenAt(child, offset);
    if (found) return found;
  }
  return null;
}

// Whether a generated parser forks where the left operand `child` ends
// before the token `lookahead`, as tree-sitter's handle_conflict leaves a
// shift-reduce conflict to the grammar's declared conflicts: the items that
// shift that token after the part the child's rule ends with (TypeScript's
// `!g` before `<`: a call, an instantiation and a binary expression after
// the `expression` `g`) rank some above the child's reduction and some below
// it, and a declared conflict names the child's rule with all of theirs.
// Both parses then go on, and the one that reduced the child is kept (see
// `shiftOrder`).
function declaredFork(grammar, child, lookahead, orders) {
  const rule = grammar?.rules.get(child.rule);
  if (!rule || grammar.conflicts.length === 0 || !child.precedence || lookahead === null) return false;
  const key = `${child.rule}|${lookahead}`;
  let forks = grammar.forks.get(key);
  if (forks !== undefined) return forks;
  grammar.first ??= firstSets(grammar.rules);
  const rules = new Set([child.rule]);
  let [more, less] = [false, false];
  for (const [slot] of directEdge(rule.expression, 'left', rule.name)) {
    for (const item of shiftItems(grammar, slot)) {
      if (!firstOf({ kind: 'seq', items: item.rest }, grammar.rules, grammar.first).has(lookahead)) continue;
      rules.add(item.rule);
      const order = comparePrecedence(item.tag, child.precedence, orders);
      if (order > 0) more = true;
      if (order < 0) less = true;
    }
  }
  forks = more && less && grammar.conflicts.some((group) => [...rules].every((name) => group.has(name)));
  grammar.forks.set(key, forks);
  return forks;
}

// How the items that shift `lookahead` after the part a node of `short`'s
// rightmost chain reduced with `reduced` ends with rank against that
// reduction, as tree-sitter's handle_conflict ranks a shift-reduce conflict:
// 1 when some rank above it and none below, -1 when some rank below it and
// none above, else 0. A precedence of none and a named one are incomparable,
// so such an item ranks neither way. A node that ends with a token (Rust's
// range `a + b..`) has no part an item goes on after.
function itemsOrder(grammar, short, reduced, lookahead, orders) {
  const rule = grammar?.rules.get(reduced.rule);
  if (!rule || lookahead === null) return 0;
  let node = short;
  while (node?.type === 'node' && node.rule !== reduced.rule) node = node.children.findLast((child) => !isTrivia(child));
  const last = node?.type === 'node' ? node.children.findLast((child) => !isTrivia(child)) : null;
  if (last?.type !== 'node') return 0;
  const slots = directEdge(rule.expression, 'left', rule.name).map(([slot]) => slot)
    .filter((slot) => last.reducedTo?.includes(slot) || unitClosure(grammar, slot).has(last.rule));
  const key = `${reduced.rule}|${reduced.level}|${reduced.name}|${lookahead}|${slots.join(' ')}`;
  let order = grammar.items.get(key);
  if (order !== undefined) return order;
  grammar.first ??= firstSets(grammar.rules);
  let [more, less] = [false, false];
  for (const slot of new Set(slots)) {
    for (const item of shiftItems(grammar, slot)) {
      if (!firstOf({ kind: 'seq', items: item.rest }, grammar.rules, grammar.first).has(lookahead)) continue;
      const rank = comparePrecedence(item.tag, reduced, orders);
      if (rank > 0) more = true;
      if (rank < 0) less = true;
    }
  }
  order = more === less ? 0 : more ? 1 : -1;
  grammar.items.set(key, order);
  return order;
}

// The rules `slot` may be as a whole, itself included: a rule an alternative
// of it is alone, through such rules (Rocq's `ltac_expression`, a
// `tactic_invocation` among others).
function unitClosure(grammar, slot) {
  grammar.units ??= new Map();
  let units = grammar.units.get(slot);
  if (units) return units;
  units = new Set();
  const pending = [slot];
  const walk = (expression) => {
    switch (expression.kind) {
      case 'ref': pending.push(expression.name); break;
      case 'choice': for (const item of expression.items) walk(item); break;
      case 'seq': {
        const solid = expression.items.filter((item) => !nullable(item));
        if (solid.length === 1 && expression.items.length === 1) walk(solid[0]);
        break;
      }
      case 'precedence': case 'namedPrecedence': case 'capture': case 'dynamicPrecedence': walk(expression.item);
    }
  };
  while (pending.length > 0) {
    const name = pending.pop();
    if (units.has(name)) continue;
    units.add(name);
    const rule = grammar.rules.get(name);
    if (rule) walk(rule.expression);
  }
  grammar.units.set(slot, units);
  return units;
}

// The items of an LR parser that go on after the part `slot`: each
// production of a rule that may begin where `slot` begins and whose own
// first part is `slot`, with its rule, its precedence and the parts after
// that first one.
function shiftItems(grammar, slot) {
  const items = [];
  const seen = new Set();
  const pending = [slot];
  while (pending.length > 0) {
    const name = pending.pop();
    const rule = grammar.rules.get(name);
    if (seen.has(name) || !rule) continue;
    seen.add(name);
    pending.push(...(grammar.heads.get(name) ?? []));
    const walk = (expression, tag, rest) => {
      switch (expression.kind) {
        case 'ref':
          if (expression.name === slot) items.push({ rule: name, tag: tag ?? unranked(name), rest });
          break;
        case 'seq':
          for (const [index, item] of expression.items.entries()) {
            walk(item, tag, [...expression.items.slice(index + 1), ...rest]);
            if (!nullable(item)) break;
          }
          break;
        case 'choice': for (const item of expression.items) walk(item, tag, rest); break;
        case 'precedence': case 'namedPrecedence':
          walk(expression.item, { level: expression.level ?? 0, name: expression.name ?? null, associativity: expression.associativity, rule: name }, rest);
          break;
        case 'repeat0': case 'repeat1': case 'repeat': walk(expression.item, tag, [expression, ...rest]); break;
        case 'capture': case 'dynamicPrecedence': case 'alias': case 'optional': walk(expression.item, tag, rest);
      }
    };
    walk(rule.expression, null, []);
  }
  return items;
}

// How precedence `a` compares with `b` (1 higher, -1 lower, 0 neither), as
// tree-sitter's compare_precedence: two levels compare when either is
// nonzero; otherwise the first of the `orders` with an entry for each
// decides, the earlier entry higher. A `name` entry stands for a named
// precedence, a `rule` entry for any precedence of the rule's productions.
function comparePrecedence(a, b, orders) {
  if (!a.name && !b.name && (a.level !== 0 || b.level !== 0)) return Math.sign(a.level - b.level);
  const matches = (entry, tag) => (entry.kind === 'name' ? entry.value === tag.name : entry.value === tag.rule);
  for (const order of orders ?? []) {
    let [left, right] = [-1, -1];
    for (const [position, entry] of order.entries()) {
      if (matches(entry, a)) left = position;
      if (matches(entry, b)) right = position;
      if (left >= 0 && right >= 0) return Math.sign(right - left);
    }
  }
  return 0;
}

// Which of two results over the same text an LR parser keeps when it decides
// a shift-reduce conflict by precedence, as tree-sitter does when the grammar
// is generated: they are walked in order, skipping the subtrees both share,
// to the first node that the two build from one offset but end apart. The
// shorter one was reduced where the longer one shifted on, and as the item in
// progress is the node's own rule, its precedence in the two decides: the
// higher level wins and, on equal levels, the associativity of the reduced
// node, right to shift and left to reduce. `owner` is the precedence the two
// results are parts of, when a precedence expression holds them (see
// `extraReduction`). 1 when `result` is kept, -1 when `existing` is, 0 when
// neither.
function shiftOrder(result, existing, orders, grammar, bytes, owner = null) {
  const left = [[result.children, 0, null]];
  const right = [[existing.children, 0, null]];
  const peek = (stack) => {
    while (stack.length > 0) {
      const top = stack[stack.length - 1];
      if (top[1] < top[0].length) {
        const child = top[0][top[1]];
        if (!isTrivia(child)) return child;
        top[1] += 1;
      } else {
        stack.pop();
      }
    }
    return null;
  };
  const skip = (stack) => { stack[stack.length - 1][1] += 1; };
  // Whether the leaves next in the two are children of one kind of node at
  // one offset, so that no reduction above them parts the two first.
  const sameParent = (one, other) => {
    const [x, y] = [one.at(-1)[2], other.at(-1)[2]];
    return x === y || (x !== null && y !== null && x.kind === y.kind && x.start === y.start);
  };
  const enter = (stack, node) => {
    skip(stack);
    stack.push([node.children, 0, node]);
  };
  for (;;) {
    const a = peek(left);
    const b = peek(right);
    if (a === null || b === null) return 0;
    if (a === b || (a.type !== 'node' && b.type !== 'node' && a.start === b.start && a.end === b.end)) {
      const parted = a === b ? 0 : silentParting(a, b, right) || -silentParting(b, a, left);
      if (parted !== 0) return parted;
      // Or where one token rule lexed the leaf both reduced alone, each to a
      // rule a declared conflict names, as children of one node (Java's `b`
      // of `a = b::m;`, a `type_identifier` of an `unannotated_type` and an
      // `identifier` of a `primary_expression`): the higher dynamic
      // precedence wins, and then the rule defined first, as tree-sitter
      // keeps where the forks merge. Under nodes apart, the reductions
      // above part the two first (C's `aff;`, an `expression_statement`
      // against a silent `empty_declaration`).
      const forked = a === b || !sameParent(left, right) ? 0 : forkedLeaves(a, b, grammar);
      if (forked !== 0) return Math.sign(result.dynamic - existing.dynamic) || forked;
      skip(left);
      skip(right);
      continue;
    }
    if (a.type === 'node' && b.type === 'node' && a.start === b.start) {
      if (a.end === b.end) {
        const reduced = firstReduction(a, b, orders);
        if (reduced !== 0) return reduced;
      }
      const pair = chainPair(a, b);
      const split = chainPair(a, b, true);
      // A node the other builds whole on its leftmost chain (Lean's
      // `foo 2` in `#check foo 2 3`, an application both parses reduce, one
      // to an argument of the command, the other to the function of the
      // application `foo 2 3`) is reduced alike in both: they part above
      // it, where one reduces on what the other shifts (see
      // `extraReduction`).
      const whole = !split && (leftmostChain(b).some((node) => sameTree(node, a)) || leftmostChain(a).some((node) => sameTree(node, b)));
      const parting = split && split[0].end !== split[1].end ? split : whole ? null : pair;
      if (parting && parting[0].end !== parting[1].end) {
        const [first, second] = partedPair(parting[0], parting[1]);
        const inner = childParting(first, second, orders);
        if (inner !== 0) return inner;
        const [long, short, sign] = first.end > second.end ? [first, second, 1] : [second, first, -1];
        const lookahead = lookaheadOf(tokenAt(long, short.end), bytes);
        if (declaredFork(grammar, short, lookahead, orders)) return -sign;
        return sign * shiftPreferred(long, short, orders, grammar, lookahead);
      }
      // The same node reduced on in two ways (Rust's `m!(x);` in a block, a
      // macro invocation that `_expression_except_range` reduces under
      // `(precedence 1 none (ref macro_invocation))` and
      // `_declaration_statement` of level 0) conflicts at its end: the higher
      // level it was reduced with wins. A node reduced under none is ranked
      // by the first rule only it was reduced to (TypeScript's
      // `namespace N {}`, an `internal_module` that `declaration` reduces
      // under the name `declaration` and `expression` under none, which the
      // order ranks below).
      if (pair) {
        const [x, y] = pair;
        const own = (node, other) => node.reduced ?? unranked(node.reducedTo?.find((name) => !other.reducedTo?.includes(name)) ?? null);
        const order = comparePrecedence(own(x, y), own(y, x), orders);
        if (order !== 0) return order;
      }
      // Two nodes of different kinds over the same tokens (JavaScript's
      // `{}`, a `statement_block` and an `object`), neither of which holds
      // the other, conflict where both are reduced: the higher precedence
      // they are reduced with wins.
      if (a.end === b.end && a.kind !== b.kind && !holdsFirst(a, b) && !holdsFirst(b, a) && sameTokens(a, b)) {
        // Unless one of them reduced their first token alone where the other
        // shifted it (Lean's `f a.b`, a projection of the application `f a`
        // and a `tactic_apply` of `f` to `a.b`): they part there, on that
        // token, before either is reduced.
        const [x, y] = [leftmostChain(a).at(-1).children.find((child) => !isTrivia(child)), leftmostChain(b).at(-1).children.find((child) => !isTrivia(child))];
        const lone = x?.type === 'token' && y?.type === 'token' && x.alone !== y.alone ? (x.alone ? -loneReduction(b, x, orders) : loneReduction(a, y, orders)) : 0;
        if (lone !== 0) return lone;
        // When their parses forked at a declared conflict, tree-sitter
        // keeps the higher dynamic precedence where they merge, before the
        // lower symbol (Java's `A<B> c;`, a `generic_type` of dynamic
        // precedence 10 against the `binary_expression` `A < B`).
        const forked = forkedOrder(a, b, grammar, bytes);
        if (forked !== null) return Math.sign(result.dynamic - existing.dynamic) || leadingDynamic(a, b) || forked;
        // Where the reductions that close the two differ, those decide
        // the reduce/reduce conflict on the last token (Java's `v = 1` of
        // `@A(v = 1)`, an `element_value_pair` that `_element_value` of
        // precedence 2 closes, against an `assignment_expression` of 1).
        // Unless the two part inside first, one reducing where the other
        // shifts (Solidity's `revert(x);`, whose `x` a `call_argument` of
        // `revert_arguments` reduces on `)` where a `parenthesized_expression`
        // of level 2 shifts it).
        const inside = shiftOrder({ children: a.children, dynamic: result.dynamic }, { children: b.children, dynamic: existing.dynamic }, orders, grammar, bytes, owner);
        if (inside !== 0) return inside;
        const [closeA, closeB] = [closingReduction(a), closingReduction(b)];
        const apart = !samePrecedence(closeA, closeB);
        const order = comparePrecedence((apart && closeA) || reduction(a), (apart && closeB) || reduction(b), orders);
        if (order !== 0) return order;
      }
    }
    const ending = endingReduction(a, b, left, orders) || -endingReduction(b, a, right, orders);
    if (ending !== 0) return ending;
    const lone = loneReduction(a, b, orders) || -loneReduction(b, a, orders);
    if (lone !== 0) return lone;
    const reduced = extraReduction(a, b, right, orders, owner, grammar) || -extraReduction(b, a, left, orders, owner, grammar);
    if (reduced !== 0) return reduced;
    if (a.type === 'node' && b.type === 'node' && a.start === b.start) {
      const parted = chainConflict(a, b, orders, grammar, bytes);
      if (parted !== 0) return parted;
    }
    if (a.type !== 'node' && b.type !== 'node') return 0;
    if (a.type === 'node') enter(left, a);
    if (b.type === 'node') enter(right, b);
  }
}

// The precedence of the reduction that closes `node` on its last token, or
// null: the precedence the token was reduced with alone, or the one it was
// lexed under, or the one its last child node closes with (Java's
// `element_value_pair` `v = 1`, whose `1` the silent `_element_value` of
// precedence 2 reduced, while the pair itself is reduced with none).
function closingReduction(node) {
  const last = node.children.findLast((child) => !isTrivia(child));
  if (last?.type === 'token') return last.reduced ?? last.precedence ?? null;
  return last?.type === 'node' ? last.closes ?? null : null;
}

// Which of two leaves over one span a generalized LR parser keeps when one
// token rule lexed both and each was reduced alone to rules the other was
// not, and a declared conflict names one rule of each (Java's `b` of
// `a = b::m;`, reduced to an `unannotated_type` in one parse and to a
// `primary_expression` in the other): tree-sitter forks on the
// reduce/reduce conflict and, where the forks merge, keeps the tree of the
// lower symbol, the rule defined first. 1 when `a` is kept, -1 when `b` is,
// and 0 when no declared conflict parts them.
function forkedLeaves(a, b, grammar) {
  if (!grammar || grammar.conflicts.length === 0 || !a.alone || !b.alone || a.lexed === undefined || a.lexed !== b.lexed) return 0;
  const [mine, theirs] = [[a, b], [b, a]].map(([own, other]) => (own.forkedTo ?? []).filter((name) => !other.forkedTo?.includes(name)));
  if (!grammar.conflicts.some((group) => mine.some((name) => group.has(name)) && theirs.some((name) => group.has(name)))) return 0;
  const rank = (names) => Math.min(...names.map((name) => grammar.ranks.get(name) ?? Infinity));
  return Math.sign(rank(theirs) - rank(mine));
}

// Which of two results an LR parser keeps when one of them reduced a token
// alone, to a silent rule of level 0 or under a precedence (Rust's
// `(precedence -1 none (literal $))`), where the other shifted on in a node
// `a` that begins with the token (Rust's `$x:expr` binding of level 1): the
// innermost such node is the item in progress, and its precedence against the
// token's decides, as in `shiftPreferred`. A token that ends a silent rule
// reduced under a precedence (Rust's `_let_chain`, `let ... && c` of level 3
// left before the `&&` of `c && d`) is reduced with that rule's precedence,
// and so is a node that ends one (`closes`, the `!c` of `let ... && !c`).
// 1 when `a`'s result is kept, -1 when `b`'s is, 0 when neither.
function loneReduction(a, b, orders) {
  if (a.type !== 'node' || (b.type !== 'token' && !(b.type === 'node' && b.closes))) return 0;
  let progress = a;
  for (;;) {
    const first = progress.children.find((child) => !isTrivia(child));
    if (!first) return 0;
    if (sameTree(first, b)) break;
    if (first.type !== 'node') return 0;
    progress = first;
  }
  const shifted = progress.precedence ?? unranked(progress.rule);
  const reduced = b.type === 'node' ? b.closes : b.reduced ?? b.precedence ?? unranked();
  const order = comparePrecedence(shifted, reduced, orders);
  if (order !== 0) return order;
  if (reduced.associativity === 'right') return 1;
  if (reduced.associativity === 'left') return -1;
  return 0;
}

// Which of two results an LR parser keeps when one of them ended a silent
// rule under a precedence with the token `a` (Lean's `pp` in `set_option
// pp.all true`, a `name` of level 0 right before the projection `.all`) where
// the other shifted on over the same token `b` in the same silent rule, which
// a later item of the other result (`stack` holds its place) ends with that
// precedence (the `all` of the name `pp.all`): the two are that rule reduced
// from one offset and ending apart, and on its equal levels the
// associativity of the rule decides, `right` to shift and `left` to reduce.
// 1 when `a`'s result is kept, -1 when `b`'s is, 0 when neither.
function silentParting(a, b, stack) {
  const reduced = a.reduced ?? a.precedence;
  if (!reduced?.rule || b.precedence || b.reduced) return 0;
  const [siblings, index] = stack[stack.length - 1];
  for (const child of siblings.slice(index + 1)) {
    if (isTrivia(child)) continue;
    const closes = child.type === 'node' ? child.closes : child.reduced ?? child.precedence;
    if (!closes) continue;
    if (closes.rule !== reduced.rule || !samePrecedence(closes, reduced)) return 0;
    if (reduced.associativity === 'right') return -1;
    if (reduced.associativity === 'left') return 1;
    return 0;
  }
  return 0;
}

// Which of two results an LR parser keeps when one of them reduces a node the
// other does not build: `a` is that node when `b`, at the same offset in the
// other result, is a subtree on its leftmost chain and the children of the
// innermost such node are, subtree for subtree, the next children of the
// other result's node in progress (`stack` holds its place). The two
// reductions of the same text then conflict at the end of that node, which
// one result reduces where the other reduces or shifts in its own node: the
// higher precedence level wins and, on equal levels where the other node
// goes on, the associativity of the reduced node. The results themselves
// are in progress under `owner`, the precedence a precedence expression
// holds them with (TypeScript's `extends A<X>`, whose
// `_extends_clause_single` takes `A` and `<X>` under the name `extends` where
// an `instantiation_expression` reduces them under `instantiation`, which the
// order ranks below), else under none. 1 when `a`'s result is kept, -1 when
// the other is, 0 when neither.
function extraReduction(a, b, stack, orders, owner = null, grammar = null) {
  if (a.type !== 'node') return 0;
  let parent = a;
  const chain = [a];
  for (;;) {
    const first = parent.children.find((child) => !isTrivia(child));
    if (!first) return 0;
    if (sameTree(first, b)) {
      chain.push(first);
      break;
    }
    if (first.type !== 'node') return 0;
    parent = first;
    chain.push(first);
  }
  const [siblings, index, container] = stack[stack.length - 1];
  const next = siblings.slice(index).filter((child) => !isTrivia(child));
  const own = parent.children.filter((child) => !isTrivia(child));
  const other = container ? container.precedence ?? unranked(container.rule) : owner ?? unranked();
  if (own.length > next.length || own.some((child, at) => !sameTree(child, next[at]))) {
    // Where the other result goes on past `b` in a node of its own (Lean's
    // `(f x).y` in a command, the projection `.y` with no term after the
    // command's `(f x)`, where the projection of level 90 takes `(f x)` as
    // its term), it reduced `b` where `a`'s result shifted on in `parent`:
    // the shift's precedence against the reduction's decides, as in
    // `shiftPreferred`.
    const [mine, theirs] = [own[1], next[1]];
    if (!mine || !theirs || theirs.type !== 'node' || sameTree(mine, theirs) || firstLeafStart(mine) !== firstLeafStart(theirs) || theirs.end !== parent.end) return 0;
    // Unless the other builds the rest of `parent` as one node, the same
    // tokens by another name (Solidity's `revert Error();`, whose `()` the
    // silent `call_arguments` of a call takes where the other parse aliases
    // it to `revert_arguments`): both shift its
    // tokens, and the conflict is at its end, where `a`'s result reduces
    // `parent` and the other shifts on: the reduced node's associativity
    // decides on equal levels.
    if (next.length === 2 && sameOutput(own.slice(1), theirs.children.filter((child) => !isTrivia(child)))) {
      const reduced = reduction(parent);
      const order = comparePrecedence(reduced, other, orders);
      if (order !== 0) return order;
      if (reduced.associativity === 'right') return -1;
      if (reduced.associativity === 'left') return 1;
      return 0;
    }
    const order = comparePrecedence(parent.precedence ?? unranked(parent.rule), other, orders);
    if (order !== 0) return order;
    if (other.associativity === 'right') return 1;
    if (other.associativity === 'left') return -1;
    return 0;
  }
  const mine = reduction(parent);
  const order = comparePrecedence(mine, other, orders);
  if (order !== 0) return order;
  // With nothing after either, where a silent rule a declared conflict names
  // reduced an item of the chain alone (Solidity's `a;`, an `identifier` that
  // `_identifier_path` reduces in a `user_defined_type` where the other
  // parse reduces it to `_primary_expression`), the two reduce apart on one
  // lookahead: tree-sitter goes on with both and keeps the tree of the lower
  // symbol where they merge, a token before any rule and then the rule
  // defined first.
  const forked = chain.flatMap((item) => item.forkedTo ?? []).filter((name) => !b.forkedTo?.includes(name));
  if (!container && own.length === next.length && grammar?.conflicts.some((group) => forked.some((name) => group.has(name)))) {
    if (b.type !== 'node') return -1;
    return Math.sign((grammar.ranks.get(b.rule) ?? 0) - (grammar.ranks.get(a.rule) ?? 0));
  }
  if (!container || container.end > parent.end) {
    if (mine.associativity === 'left') return 1;
    if (mine.associativity === 'right') return -1;
  }
  return 0;
}

// Which of two results an LR parser keeps when both shift the same tokens and
// part on a reduce/reduce conflict at the last of them: `a` is a token whose
// parse goes on, past tokens alone, with a sibling node that ends where the
// node `b` of the other parse, which begins with that token, ends (Go's
// `chan<- chan int`, a `chan <-` channel type of `chan int` against a `chan`
// channel type of `<- chan int`). Both reduce the nodes along the rightmost chains of that
// sibling and of `b` on that token, the shared ones alike; the first pair
// that differs conflicts, and the higher precedence it closes with wins
// (`<- chan T`'s 6 over the 0 of `chan T`). `stack` holds `a`'s siblings.
// 1 when `a`'s result is kept, -1 when `b`'s is, 0 when neither.
function endingReduction(a, b, stack, orders) {
  if (a.type !== 'token' || b.type !== 'node' || !sameTree(leftmostChain(b).at(-1).children.find((child) => !isTrivia(child)) ?? b, a)) return 0;
  const [siblings, index] = stack[stack.length - 1];
  const sibling = siblings.slice(index + 1).find((child) => child.type === 'node' && !isTrivia(child));
  if (sibling?.type !== 'node' || sibling.end !== b.end) return 0;
  const rightmost = (node) => {
    const chain = [];
    for (let current = node; current?.type === 'node'; current = current.children.findLast((child) => !isTrivia(child))) chain.push(current);
    return chain;
  };
  const [mine, theirs] = [rightmost(sibling), rightmost(b)];
  let [i, j] = [mine.length - 1, theirs.length - 1];
  while (i >= 0 && j >= 0 && sameTree(mine[i], theirs[j])) [i, j] = [i - 1, j - 1];
  if (i < 0 || j < 0) return 0;
  return comparePrecedence(reduction(mine[i]), reduction(theirs[j]), orders);
}

// A result whose one meaningful item records that the silent rule `name`
// reduced it alone: a token as `alone`, and any item, when the precedence
// orders name the rule (`ranked`), the rule after the inner ones it was
// reduced to (`reducedTo`), and when a declared conflict names it
// (`forked`), the rule as one it was reduced to there (`forkedTo`, see
// `forkedOrder`); any other result as it is.
function reducedAlone(result, name, ranked, forked = false) {
  const meaningful = result.children.filter((child) => !isTrivia(child));
  if (meaningful.length !== 1) return result;
  const only = meaningful[0];
  const changes = {};
  if (only.type === 'token' && !only.alone) changes.alone = true;
  if (ranked && !only.reducedTo?.includes(name)) changes.reducedTo = [...(only.reducedTo ?? []), name];
  if (forked && !only.forkedTo?.includes(name)) changes.forkedTo = [...(only.forkedTo ?? []), name];
  if (Object.keys(changes).length === 0) return result;
  const tagged = only.type === 'node' ? copyNode(only, changes) : { ...only, ...changes };
  return copyResult(result, { children: result.children.map((child) => (child === only ? tagged : child)) });
}

// Which of two results an LR parser keeps when two nodes of one kind from one
// offset, which end apart, part before either ends, where their children
// first differ by end: the parse whose child ends first shifts on in its node,
// where the other reduced that child alone to the silent rule the longer
// child's node in progress takes it as (JavaScript's `new f()` before a
// template, whose `new_expression` shifts `(` as its arguments under `new`,
// where the call `f()` reduced `f` to an `expression`, which the order
// ranks below `new`). The precedence of the node that shifts against that
// reduction's decides, as in `shiftPreferred`. 1 when `a`'s result is kept,
// -1 when `b`'s is, 0 when neither.
function childParting(a, b, orders) {
  const first = a.children.filter((child) => !isTrivia(child));
  const second = b.children.filter((child) => !isTrivia(child));
  for (let index = 0; index < Math.min(first.length, second.length); index += 1) {
    const [x, y] = [first[index], second[index]];
    if (sameTree(x, y)) continue;
    if (firstLeafStart(x) !== firstLeafStart(y) || x.end === y.end) return 0;
    const [short, long, node, own, sign] = x.end < y.end ? [x, y, a, first, 1] : [y, x, b, second, -1];
    if (index === own.length - 1) return 0;
    let progress = long;
    let head = null;
    for (;;) {
      if (progress.type !== 'node') return 0;
      head = progress.children.find((child) => !isTrivia(child));
      if (!head) return 0;
      if (sameTree(head, short)) break;
      progress = head;
    }
    // The reduction in conflict is the first of the head's the short child
    // was not reduced to as well.
    const rule = head.reducedTo?.find((name) => !short.reducedTo?.includes(name));
    if (progress.end === short.end || !rule) return 0;
    const shifted = node.precedence ?? unranked(node.rule);
    // A node's own precedence is its rule's, not the one it was reduced with.
    const reduced = head.reduced ?? (head.type === 'token' ? head.precedence : null) ?? unranked(rule);
    const order = comparePrecedence(shifted, reduced, orders);
    if (order !== 0) return sign * order;
    if (reduced.associativity === 'right') return sign;
    if (reduced.associativity === 'left') return -sign;
    return 0;
  }
  return 0;
}

// Which of two nodes of different kinds over the same tokens a generalized
// LR parser keeps when their parses forked at a conflict the grammar
// declares, or null when they did not: the two reduce alike up to the first
// reduction they make apart (TypeScript's `<A>(a): T => a`, the `A` that the
// arrow function's type parameters reduce to a `type_parameter` and the
// type assertion's type arguments to a `primary_type`), and when a declared
// conflict names a rule each reduced to there, both parses go on and
// tree-sitter keeps the tree of the lower symbol where they merge, as its
// `ts_subtree_compare` does: here, the rule defined first, unless the two
// reduce one handle. 1 when `a` is kept, -1 when `b` is.
function forkedOrder(a, b, grammar, bytes = null) {
  if (!grammar || grammar.conflicts.length === 0) return null;
  const steps = (tree, out) => {
    if (isTrivia(tree)) return out;
    if (tree.type === 'node') for (const child of tree.children) steps(child, out);
    out.push(tree);
    return out;
  };
  const [first, second] = [steps(a, []), steps(b, [])];
  // Two leaves one token rule lexed are one step, whatever they are named
  // (Java's `A`, a `type_identifier` and an `identifier`).
  const same = (x, y) => x.type === y.type && x.start === y.start && x.end === y.end
    && (x.kind === y.kind || (x.type === 'token' && x.lexed !== undefined && x.lexed === y.lexed));
  let at = 0;
  while (at < first.length && at < second.length && same(first[at], second[at])) at += 1;
  if (at === 0 || at === first.length || at === second.length) return null;
  // An item one parse reduces to a silent rule a conflict declares and the
  // other shifts on in its node (Lean's `let x := 2`, whose `x` a `do_let`
  // reduces to a `pattern` and a `let` takes as its name): the parses fork
  // there, between that rule and the node's.
  const enclosing = (list, index) => list.slice(index + 1).find((step) => step.type === 'node' && step.start <= list[index].start && step.end >= list[index].end);
  for (let index = 0; index < at; index += 1) {
    const [mine, theirs] = [[first, second], [second, first]].map(([own, other]) => (own[index].forkedTo ?? []).filter((name) => !other[index].forkedTo?.includes(name)));
    if (mine.length === theirs.length) continue;
    if (mine.length === 0) mine.push(enclosing(first, index)?.rule);
    else theirs.push(enclosing(second, index)?.rule);
    if (grammar.conflicts.some((group) => mine.some((name) => group.has(name)) && theirs.some((name) => group.has(name)))) {
      return Math.sign((grammar.ranks.get(b.rule) ?? 0) - (grammar.ranks.get(a.rule) ?? 0));
    }
  }
  const reductions = (before, other, next) => {
    const names = [...(before.reducedTo ?? []), ...(before.forkedTo ?? [])].filter((name) => !other.reducedTo?.includes(name) && !other.forkedTo?.includes(name));
    if (next.type === 'node' && next.end === before.end) names.push(next.rule);
    return names;
  };
  const mine = reductions(first[at - 1], second[at - 1], first[at]);
  const theirs = reductions(second[at - 1], first[at - 1], second[at]);
  const declared = grammar.conflicts.some((group) => mine.some((name) => group.has(name)) && theirs.some((name) => group.has(name)));
  if (!declared) return null;
  const order = Math.sign((grammar.ranks.get(b.rule) ?? 0) - (grammar.ranks.get(a.rule) ?? 0));
  // Two reductions of one handle (Lean's `do return x`, a `return` and a
  // `do_return` of the same children) are two reduce actions of one table
  // entry: the version of the last, of the rule defined later, is the one
  // the parser goes on with, and the other merges into it where both shift
  // the next token. With none left but zero-width ones (the `return x` that
  // ends a file's last `do` block), both are accepted and the tree of the
  // lower symbols is kept.
  const handle = at === first.length - 1 && at === second.length - 1;
  const follows = bytes?.subarray(a.end).some((byte) => !ASCII_WHITE_SPACE.has(byte));
  return handle && follows ? -order : order;
}

// Which of two nodes of one span an LR parser builds when the first action
// their parses differ in reduces in both, on the same token: a reduce/reduce
// conflict, which tree-sitter settles for the higher precedence the two
// reductions close with (Go's `<-chan int(c)`, whose `int` reduces a
// `<- chan int` channel type of 6, not a `chan int` one of 0 that a unary
// `<-` would take). The actions are the leaves the two shift and the nodes
// they reduce, in post-order; leaves reduced apart by silent rules part the
// parses before, where this does not decide, and where the two differ only in
// themselves, the reductions that close them do (see `closingReduction`). 1
// when `a` is built, -1 when `b` is, 0 when neither.
function firstReduction(a, b, orders) {
  const steps = (tree, out) => {
    if (isTrivia(tree)) return out;
    if (tree.type === 'node') for (const child of tree.children) steps(child, out);
    out.push(tree);
    return out;
  };
  const [first, second] = [steps(a, []), steps(b, [])];
  const silent = (leaf) => [leaf.reducedTo ?? [], leaf.forkedTo ?? [], leaf.alone ?? false].flat().join(' ');
  let at = 0;
  for (; at < first.length && at < second.length; at += 1) {
    const [x, y] = [first[at], second[at]];
    if (x.type !== y.type || x.kind !== y.kind || x.start !== y.start || x.end !== y.end) break;
    if (x.type !== 'node' && silent(x) !== silent(y)) break;
  }
  const [x, y] = [first[at], second[at]];
  if (x?.type !== 'node' || y?.type !== 'node' || x.end !== y.end || at === 0 || (x === a && y === b)) return 0;
  return comparePrecedence(reduction(x), reduction(y), orders);
}

// Which of two parses that forked at a declared conflict and end with the same
// dynamic precedence tree-sitter keeps where they merge: the one whose stack
// held the higher dynamic precedence at the last token where the two
// differed. After each token it shifts, tree-sitter puts the version of the
// higher stack sum first, the sum of the nodes reduced by then, and where two
// versions merge with links of equal dynamic precedence it keeps the first
// one's (Go's `a[b](c)`: the `generic_type` `a[b]` of 2 is ahead of the
// `index_expression` of 1 at `(`, and its `type_conversion_expression` of 1
// then ties with the `call_expression` of 1). A node's own share, the levels
// in it outside its child nodes, counts from its end. 1 when `a` is kept, -1
// when `b` is, 0 when the sums never differ.
function leadingDynamic(a, b) {
  const shares = (tree) => {
    const out = [];
    const walk = (node) => {
      let inner = 0;
      for (const child of node.children) if (child.type === 'node') inner += walk(child);
      const total = node.dynamic ?? 0;
      if (total !== inner) out.push([node.end, total - inner]);
      return total;
    };
    walk(tree);
    return out;
  };
  const [first, second] = [shares(a), shares(b)];
  if (first.length === 0 && second.length === 0) return 0;
  const starts = [];
  const leaves = (node) => {
    for (const child of node.children) {
      if (isTrivia(child)) continue;
      if (child.type === 'node') leaves(child);
      else starts.push(child.start);
    }
  };
  leaves(a);
  const sum = (list, at) => list.reduce((total, [end, share]) => (end <= at ? total + share : total), 0);
  let order = 0;
  for (const at of starts) {
    const difference = Math.sign(sum(first, at) - sum(second, at));
    if (difference !== 0) order = difference;
  }
  return order;
}

// Whether a node of the kind and span of `inner` is on the leftmost chain of
// `outer`, which ends with it.
function holdsFirst(outer, inner) {
  return leftmostChain(outer).some((node) => node.kind === inner.kind && node.start === inner.start && node.end === inner.end);
}

// Whether a leaf is a token the external scanner scanned of no width.
function widthless(leaf) {
  return leaf.type !== 'node' && leaf.start === leaf.end && scannedToken(leaf) === 1;
}

// Whether two leaves are one token: of one span, and of one kind or built by
// one token rule.
function oneLeaf(leaf, other) {
  return leaf.type === other.type && leaf.start === other.start && leaf.end === other.end
    && (leaf.kind === other.kind || (leaf.lexed !== undefined && leaf.lexed === other.lexed));
}

// The leaves under `children` that are not white space, last first.
function* leavesBackward(children) {
  for (let index = children.length - 1; index >= 0; index -= 1) {
    const child = children[index];
    if (isTrivia(child)) continue;
    if (child.type === 'node') yield* leavesBackward(child.children);
    else yield child;
  }
}

// The last leaf under `children` a repair may have inserted: the last child
// that is not a separator, unless tokens the external scanner scanned of no
// width end `children` (a layout token opening an indented block): then the
// last leaf before them, however deep in a node.
function lastRepaired(children) {
  let skipped = false;
  for (const leaf of leavesBackward(children)) {
    if (!widthless(leaf)) return skipped ? leaf : children.findLast((child) => !isSeparator(child));
    skipped = true;
  }
  return skipped ? undefined : children.findLast((child) => !isSeparator(child));
}

// Whether the first leaf of `result` that is not white space is a token the
// external scanner scanned of no width.
function startsWidthless(result) {
  let first = result.children.find((child) => !isTrivia(child));
  while (first?.type === 'node') first = first.children.find((child) => !isTrivia(child));
  return first !== undefined && widthless(first);
}

// Whether `result` holds the token `last` and goes on past it with a token
// its lexer lexed, not one the external scanner scanned of no width.
function lexedPast(result, last) {
  let after = null;
  for (const leaf of leavesBackward(result.children)) {
    const action = readScannerContinuationAction(widthless(leaf), leaf.start >= last.end, leaf.end > last.end);
    if (action === 1) after = leaf;
    else if (action === -1) return after !== null && oneLeaf(leaf, last);
  }
  return false;
}

// The results among `continued`, each paired with the results of the next
// item after it, that a token the external scanner scans of no width
// preempts: where the next item after a result begins with such a token
// (Lean's layout end after `12` in `def foo := 12` before a line break), an
// LR parser in the state of that result runs the scanner there before its
// lexer, and the token it scans is its lookahead, so another result in the
// same state that holds the same last token and goes on past it with a token
// the lexer lexed (`12 partial`, an application across the line break) is no
// parse. A result in another state has another scanner state, which may scan
// nothing there (a layout end one parse has queued and the other has not).
// Nor does a repaired result preempt one of a lower repair cost, another
// version of the parse to tree-sitter's recovery, which keeps the cheaper
// one (TypeScript's `{ c : 0 @9 , e }`, whose object goes on past `e` where a
// statement block, repaired at more cost, scans an automatic semicolon).
// Null when none is; `globalThis.__preemptTrace`, when set, is called with
// each pair.
function preempted(continued) {
  let pruned = null;
  for (const [left, rights] of continued) {
    if (!rights.some(startsWidthless)) continue;
    let last;
    for (const leaf of leavesBackward(left.children)) {
      if (!widthless(leaf)) {
        last = leaf;
        break;
      }
    }
    if (last === undefined) continue;
    for (const [other] of continued) {
      if (other.end > left.end && other.state.key === left.state.key && other.cost >= left.cost && !pruned?.has(other) && lexedPast(other, last)) {
        (pruned ??= new Set()).add(other);
        globalThis.__preemptTrace?.(left, other);
      }
    }
  }
  return pruned;
}

// Whether two subtrees hold the same tokens: leaves of one span each, of one
// kind or built by one token rule. A token the external scanner scanned of
// no width does not count: where one subtree holds it (Lean's layout
// semicolon in `let y := Foo` before a line break, which a `let` takes before
// its body), the parser of the other scanned it there too, before its lexer.
function sameTokens(a, b) {
  const leaves = (tree, out) => {
    if (isTrivia(tree)) return out;
    if (tree.type === 'node') for (const child of tree.children) leaves(child, out);
    else if (tree.start !== tree.end || scannedToken(tree) === 0) out.push(tree);
    return out;
  };
  const [first, second] = [leaves(a, []), leaves(b, [])];
  return first.length === second.length && first.every((leaf, index) => {
    const other = second[index];
    return leaf.type === other.type && leaf.start === other.start && leaf.end === other.end
      && (leaf.kind === other.kind || (leaf.lexed !== undefined && leaf.lexed === other.lexed));
  });
}

// Whether two subtrees are the same tree over the same text, whichever of
// them holds the white space around it. `memo`, a map of maps the caller
// keeps while no tree changes, holds the pairs of nodes already compared:
// two leftmost chains compared node by node share their subtrees, which
// would otherwise be compared again for every pair above them.
function sameTree(a, b, memo) {
  if (a === b) return true;
  if (a.type !== b.type || a.kind !== b.kind) return false;
  if (a.type !== 'node') return a.start === b.start && a.end === b.end;
  const known = memo?.get(a)?.get(b);
  if (known !== undefined) return known;
  const first = a.children.filter((child) => !isTrivia(child));
  const second = b.children.filter((child) => !isTrivia(child));
  const same = first.length === second.length && first.every((child, index) => sameTree(child, second[index], memo));
  if (memo) {
    if (!memo.has(a)) memo.set(a, new Map());
    memo.get(a).set(b, same);
  }
  return same;
}

// Whether two child lists are the same trees over the same text.
function sameChildren(a, b) {
  if (a === b) return true;
  const first = a.filter((child) => !isTrivia(child));
  const second = b.filter((child) => !isTrivia(child));
  return first.length === second.length && first.every((child, index) => sameTree(child, second[index]));
}

// The nodes along the leftmost chain of a node: itself, then its first
// meaningful child while that is a node.
function leftmostChain(node) {
  const chain = [];
  for (let current = node; current?.type === 'node'; current = current.children.find((child) => !isTrivia(child))) chain.push(current);
  return chain;
}

// The outermost node kind on the leftmost chains of two nodes that start at
// one offset, as the pair of its nodes, or null. When `distinct`, a node the
// other chain holds too, the same tree over the same text, is not paired: the
// two parses built it alike (Rust's `a + b` in `a + b..*c`, the left operand
// of both the range `a + b..` and the binary expression `a + b..*c`), so they
// part above it.
function chainPair(a, b, distinct = false) {
  const mine = leftmostChain(a);
  const other = leftmostChain(b);
  const memo = new Map();
  const shared = (node, chain) => distinct && chain.some((peer) => peer.end === node.end && sameTree(peer, node, memo));
  for (const node of mine) {
    if (shared(node, other)) continue;
    const match = other.find((candidate) => candidate.kind === node.kind && candidate.start === node.start && !shared(candidate, mine));
    if (match) return [node, match];
  }
  return null;
}

// The innermost pair of nodes of one kind from one offset that end apart,
// below two such nodes `a` and `b` along their leftmost chains: the two
// parses part where the first of them ends, so the decision is that pair's
// (Rust's `g(|| a, |p| p)`, where the closures `||` and `|| a, |p|` part at
// the parameters `||`, the or-pattern `| a` of level -2 going on past them).
function partedPair(a, b) {
  for (let pair = [a, b]; ;) {
    const [left, right] = pair.map((node) => node.children.find((child) => !isTrivia(child)));
    const below = left?.type === 'node' && right?.type === 'node' ? chainPair(left, right) : null;
    if (!below || below[0].end === below[1].end) return pair;
    pair = below;
  }
}

// Which of two results an LR parser keeps when two nodes from one offset end
// their leftmost chains apart and nothing else decides them (Rust's closure
// `|a| b` and or-pattern `|a|b` in a tuple pattern): the two parses part at
// the first end only one chain has, where one reduced the innermost node
// ending there (the or-pattern `|a` of level -2) and the other shifted on in
// the innermost node going past it (the closure parameters `|a|`), as in
// `shiftPreferred`, when the reduced node's children begin the other node's,
// so the two parses agree up to that end. 1 when `a`'s result is kept, -1 when
// `b`'s is, 0 when neither.
function chainConflict(a, b, orders, grammar, bytes = null) {
  const first = leftmostChain(a);
  const second = leftmostChain(b);
  const ends = (chain) => new Set(chain.map((node) => node.end));
  const [mine, theirs] = [ends(first), ends(second)];
  const parted = [...mine].filter((end) => !theirs.has(end)).concat([...theirs].filter((end) => !mine.has(end)));
  if (parted.length === 0) return 0;
  const end = Math.min(...parted);
  const [reducing, shifting, sign] = mine.has(end) ? [first, second, -1] : [second, first, 1];
  let short = reducing.findLast((node) => node.end === end);
  let long = shifting.findLast((node) => node.end > end);
  if (!long) return 0;
  for (;;) {
    const own = short.children.filter((child) => !isTrivia(child));
    const next = long.children.filter((child) => !isTrivia(child));
    if (own.length > next.length || own.length === 0) return 0;
    const at = own.length - 1;
    if (own.slice(0, at).some((child, index) => !sameTree(child, next[index]))) return 0;
    const [last, other] = [own[at], next[at]];
    if (own.length < next.length && sameTree(last, other)) break;
    if (last.type !== 'node' || other.type !== 'node' || last.end !== short.end || other.end <= last.end) {
      // The long node's part at the short one's last may go on past it, as
      // a node that begins with it (Lean's `-x ^ 3 * 7`, where the power of
      // level 60 right takes `3 * 7` as its right operand and the other
      // parse ends it with `3`).
      if (other?.type === 'node' && other.end > last.end && beginsWith(other, last)) break;
      return 0;
    }
    // Two last parts of one kind from one offset part where the short one
    // ends, inside it (Lean's `g do return x`, where one parse ends the
    // `do`, and the `return` in it, before `x`): the conflict is theirs. So
    // do two of rules a conflict declares, reductions of one handle (Lean's
    // `do_return` and `return`).
    const forked = last.kind !== other.kind && grammar?.conflicts.some((group) => group.has(last.rule) && group.has(other.rule));
    if ((last.kind !== other.kind && !forked) || last.start !== other.start) {
      if (beginsWith(other, last)) break;
      return 0;
    }
    [short, long] = [last, other];
  }
  return sign * shiftPreferred(long, short, orders, grammar, bytes && lookaheadOf(tokenAt(long, short.end), bytes));
}

// Whether `child` is a subtree along the leftmost chain of `node`.
function beginsWith(node, child) {
  for (let current = node; current?.type === 'node';) {
    current = current.children.find((item) => !isTrivia(item));
    if (current && sameTree(current, child)) return true;
  }
  return false;
}

// 1 when the shift that built `long` is preferred to the reduction that
// ended `short` (the same node kind from the same offset), -1 when the
// reduction is, 0 when the precedences cannot tell. The shift is in the
// innermost node of `long` that goes on past the end of `short`. When that
// node began with `short`, as a binary expression whose left operand is the
// expression a statement is of, the long result reduced that operand to a
// silent rule where the short one reduced its node: the two reductions
// conflict instead, and a silent rule's reduction is of level 0. Where the
// shift's own precedence cannot tell, every item that shifts `lookahead`
// after the reduced part may, as tree-sitter's handle_conflict compares the
// reduction with each (Rocq's `try x; [a | b]`, where `tactic_branch` of none
// cannot rank the `tactical` of `tactic_application` but `tactic_sequence`,
// ranked below it, shifts `;` too: the tactical is reduced).
function shiftPreferred(long, short, orders, grammar, lookahead = null) {
  const begin = firstLeafStart(short);
  let progress = long;
  for (;;) {
    const inner = progress.children.find((child) => child.type === 'node' && child.start < short.end && child.end > short.end);
    if (!inner) break;
    if (firstLeafStart(inner) === begin) {
      return -comparePrecedence(reduction(short), unranked(), orders);
    }
    progress = inner;
  }
  const shifted = progress.precedence ?? unranked(progress.rule);
  const reduced = reducedBefore(short, progress);
  const before = shiftReduction(short, progress, grammar);
  if (before !== null) {
    const order = comparePrecedence(unranked(before), reduced, orders);
    if (order !== 0) return order;
  }
  const order = comparePrecedence(shifted, reduced, orders);
  if (order !== 0) return order;
  const items = itemsOrder(grammar, short, reduced, lookahead, orders);
  if (items !== 0) return items;
  if (reduced.associativity === 'right') return 1;
  if (reduced.associativity === 'left') return -1;
  return 0;
}

// The silent rule the shift in `progress` takes its first part as where
// `short` ends with that part reduced to no such rule (TypeScript's
// `keyof U & V`, whose intersection takes `U` as a `type` where the type
// query `keyof U` takes it as a `primary_type`), or null: an LR parser
// reduces the part to that rule, before it can shift, where it reduces
// `short` instead, and the two reductions conflict (the order ranks
// `index_type_query` above `type`, so `keyof U` is the left operand). Where
// the rule of `progress` takes the part as it ends `short`, by its kind or a
// rule it was reduced to (JavaScript's `new module.Klass()`, whose member
// expression takes `module` as a `primary_expression`), the shift needs no
// reduction and none conflicts.
function shiftReduction(short, progress, grammar) {
  const head = progress.children.find((child) => !isTrivia(child));
  if (!head || head.end !== short.end) return null;
  for (let node = short; node.type === 'node';) {
    const meaningful = node.children.filter((child) => !isTrivia(child));
    const last = meaningful[meaningful.length - 1];
    if (!last) return null;
    if (sameTree(last, head)) {
      const direct = grammar?.heads.get(progress.rule);
      if (direct?.has(last.kind) || last.reducedTo?.some((name) => direct?.has(name))) return null;
      return head.reducedTo?.find((name) => !last.reducedTo?.includes(name)) ?? null;
    }
    node = last;
  }
  return null;
}

// The precedence a node is reduced with: the innermost one over its last
// part, as a generated parser takes the precedence of a production's last
// step (Rust's `let` condition, whose value is `(precedence 3 left (ref
// expression))`, reduces before the `&&` of a binary expression of level 3),
// else none of level 0.
function reduction(node) {
  return node.tail ?? node.precedence ?? unranked(node.rule);
}

// The precedence `short` was reduced with where the shift in `progress` went
// on instead: that of the innermost node along its rightmost chain whose
// last part is the first part of `progress`, as the production a generated
// parser completes at the conflict (Rust's `let bar = || baz && quux`, where
// the closure of level -1 ends with `baz`, not the `let` condition), or the
// precedence a token ending a silent rule keeps (see `loneReduction`); else
// the precedence of the innermost silent rule on the chain that the chain's
// token or node ends (TypeScript's `namespace N {}` and `module "m" {}`,
// whose `module_name_and_body` of level 0 right ends with the name where the
// body is left out, so the body shifts); else the precedence `short` reduces
// with.
function reducedBefore(short, progress) {
  const first = progress.children.find((child) => !isTrivia(child));
  let closing = null;
  for (let node = short; first && node.type === 'node';) {
    const meaningful = node.children.filter((child) => !isTrivia(child));
    const last = meaningful[meaningful.length - 1];
    if (!last) break;
    if (sameTree(last, first)) {
      if (last.type === 'token' && last.reduced) return last.reduced;
      // A token a silent rule reduced alone where the shift takes it as the
      // first part of its node (Lean's `c` in `fun x c s`, a `_pattern` of
      // level 0 in one parse and the constructor of `c s`, of level 80, in
      // the other) is that rule's reduction, not the node's it ends.
      if (last.type === 'token' && last.alone && !first.alone) return last.precedence ?? unranked();
      return reduction(node);
    }
    node = last;
    closing = (node.type === 'token' ? node.reduced ?? node.precedence : node.closes) ?? closing;
  }
  return closing ?? reduction(short);
}

// The offset of the first leaf under a node that is not white space.
function firstLeafStart(node) {
  let current = node;
  while (current.type === 'node') {
    const first = current.children.find((child) => !isTrivia(child));
    if (!first) return current.start;
    current = first;
  }
  return current.start;
}


// The children of a joined result are the children of its left part followed
// by those of its right part. Copying them on every join made a repetition of
// n items cost O(n²) time and memory, as each iteration (and every prefix the
// generalized repetition keeps) copied all the children before it. A join
// instead links its two parts with their child count, and the children are
// flattened, once, when they are first read; the links are then dropped. A
// node built from a result shares its chain, so the nodes of the prefixes of
// a repetition are not flattened either.
const CHAIN = Symbol('children chain');

function partOf(result) {
  return result[CHAIN] ?? result.children;
}

function childCount(result) {
  return result[CHAIN]?.length ?? result.children.length;
}

function flattenChain(chain) {
  if (chain.flat) return chain.flat;
  const flat = [];
  const pending = [chain];
  while (pending.length > 0) {
    const part = pending.pop();
    if (Array.isArray(part)) for (const child of part) flat.push(child);
    else if (part.flat) for (const child of part.flat) flat.push(child);
    else pending.push(part.right, part.left);
  }
  chain.flat = flat;
  chain.left = null;
  chain.right = null;
  return flat;
}

// Gives `target` the children of `source`, shared and still unflattened.
function shareChildren(target, source) {
  const chain = source[CHAIN];
  if (!chain) {
    target.children = source.children;
    return target;
  }
  target[CHAIN] = chain;
  Object.defineProperty(target, 'children', {
    configurable: true,
    enumerable: true,
    get() { return flattenChain(chain); },
  });
  return target;
}

// A copy of a node with `changes`, its children still shared and lazy.
function copyNode(node, changes) {
  const copy = {};
  for (const key of Object.keys(node)) if (key !== 'children') copy[key] = node[key];
  return Object.assign(shareChildren(copy, node), changes);
}

// A copy of a result with `changes`, its children still shared and lazy (a
// spread would flatten them).
function copyResult(result, changes) {
  const copy = {
    end: result.end, state: result.state, dynamic: result.dynamic,
    precedence: result.precedence, tail: result.tail, ambiguous: result.ambiguous, cost: result.cost,
  };
  if (result.before !== undefined) copy.before = result.before;
  if (result.open) copy.open = true;
  if (!('children' in changes)) shareChildren(copy, result);
  return Object.assign(copy, changes);
}

function joinResults(left, right, inToken) {
  const joined = {
    end: right.end,
    state: right.state,
    dynamic: left.dynamic + right.dynamic,
    precedence: null,
    tail: null,
    ambiguous: left.ambiguous || right.ambiguous,
    cost: left.cost + right.cost,
  };
  if (right.open || (left.open && right.end === left.end)) joined.open = true;
  if (inToken) {
    joined.children = NO_CHILDREN;
    return joined;
  }
  const leftCount = childCount(left);
  const rightCount = childCount(right);
  joined.tail = rightCount === 0 ? left.tail : right.tail;
  if (rightCount === 0) return shareChildren(joined, left);
  if (leftCount === 0) return shareChildren(joined, right);
  return shareChildren(joined, { [CHAIN]: { left: partOf(left), right: partOf(right), length: leftCount + rightCount, flat: null } });
}

// A result whose MISSING leaf the rule it ends has reduced (see `sequence`).
function closed(result) {
  return result.open ? copyResult(result, { open: false }) : result;
}

function longestResult(results) {
  let best = null;
  for (const result of results) if (!best || result.end > best.end) best = result;
  return best;
}

// Whether an expression may match nothing, as far as its shape tells.
function nullable(expression) {
  switch (expression.kind) {
    case 'empty': case 'optional': case 'repeat0': case 'and': case 'not': return true;
    case 'repeat': return expression.min === 0 || nullable(expression.item);
    case 'seq': return expression.items.every(nullable);
    case 'choice': return expression.items.some(nullable);
    case 'capture': case 'precedence': case 'namedPrecedence': case 'dynamicPrecedence': case 'alias': case 'repeat1': return nullable(expression.item);
    default: return false;
  }
}

// The union of kind sets, or null when any is unknown.
function union(sets) {
  if (sets.some((set) => set === null)) return null;
  return new Set(sets.flatMap((set) => [...set]));
}

// The bytes of ASCII white space: tab, line feed, vertical tab, form feed,
// carriage return and space.
const ASCII_WHITE_SPACE = new Set([9, 10, 11, 12, 13, 32]);

// A trivia leaf of no kind: white space, which a lexer skips as padding.
function isSeparator(child) {
  return child.type === 'token' && child.trivia === true && child.kind === null;
}

function isTrivia(child) {
  return child.trivia === true;
}

function contentStart(children, fallback) {
  for (const child of children) if (!isTrivia(child)) return child.start;
  return fallback;
}

// An aliased leaf; a MISSING literal named by an alias is no longer a literal.
function renamed(child, kind) {
  if (child.type !== 'missing' || !child.literal) return { ...child, kind };
  const { literal, ...rest } = child;
  return { ...rest, kind };
}

// The kind of the MISSING leaf of a failed terminal or token: the literal
// text for a literal or a keyword (a literal and the lookaheads after it, as
// tree-sitter names its keyword token), also under a lexical precedence (the
// closing `/` of a JavaScript regular expression), else none.
// A literal that took the separators after it (see `beforeSeparator`) is
// still that literal, as a tree-sitter lexer names it: its leaf is an
// anonymous alias of the literal (`'\n` over `\n\n`). Any other terminal
// leaf is named by its text.
function separatorRunKind(expression, start, end) {
  return expression.kind === 'literal' && end - start > encodeText(expression.value).length ? `'${expression.value}` : null;
}

function missingOf(expression) {
  if (isKeyword(expression)) return missingOf(expression.items[0]);
  if (expression.kind === 'lexicalPrecedence') return missingOf(expression.item);
  return expression.kind === 'literal' ? { kind: expression.value, literal: true } : { kind: null };
}

/** Interprets `program` over `bytes[begin, end)`. */
export class Executor {
  constructor(program, bytes, begin, end, options, budget, maxDepth = options.maxDepth ?? DEFAULT_MAX_DEPTH) {
    this.program = program;
    this.bytes = bytes;
    this.begin = begin;
    this.end = end;
    this.options = options;
    this.budget = budget;
    this.maxDepth = maxDepth;
    this.peg = program.matching === 'peg';
    // `(matching longest)`: the token ranks and the input, by which addResult
    // orders two parses that differ in their tokens.
    this.longestTokens = program.tokenRanks ? { ...program.tokenRanks, bytes, orders: program.precedenceOrders ?? [], grammar: grammarFacts(program), separatorText: (start, end) => this.separatorText(start, end) } : null;
    // The grammar's settling steps (see SETTLING_STEPS); the lexing of a
    // tree-sitter lexer follows its `tokens` step, the LR reductions its
    // `precedence` step.
    this.settling = program.settling ?? ALL_SETTLING;
    this.lexing = this.settling.tokens ? this.longestTokens : null;
    this.reducing = this.settling.precedence ? this.longestTokens : null;
    this.depth = 0;
    this.memo = new Map();
    this.memoLimit = options.memoLimit ?? DEFAULT_MEMO_LIMIT;
    this.callStack = [];
    this.triviaMemo = new Map();
    // The starts of separators a token lexed at them took, going on past
    // the trivia, each with the node calls the token was lexed in, their
    // kinds and offsets (see `tokenBeforeExtra` and `widenTokens`).
    this.separatorTaken = new Map();
    this.operandMemo = new Map();
    this.shiftMemo = new Map();
    this.owners = null;
    this.edgeMemo = new Map();
    this.belowMemo = new Map();
    this.inExtra = false;
    this.extraStart = -1;
    // The repair points where a continuation after a MISSING leaf is open.
    this.chained = new Set();
    this.scannerMemo = new Map();
    // The requests a scanner's `expected` asks about, and the offset a
    // scanner run in token context answers for (see `Expectations`).
    this.expectations = new Expectations();
    this.scanContext = null;
    this.embedMemo = new Map();
    this.farthest = begin;
    this.expected = new Set();
    this.suppressed = 0;
    this.view = null;
    // Automatic recovery: the offsets where a failing element is repaired,
    // and the farthest offset where an element failed without a repair.
    this.repairPoints = null;
    this.elementFarthest = -1;
    // The scan past each failing element at a repair point (see elementFailed).
    this.repairMemo = new Map();
    // Under `(matching longest)`, the keyword lexing of the parse (see
    // `KeywordLexing`), or none.
    this.keywords = null;
  }

  // Whether the results of a part are to be pruned by the tokens the external
  // scanner scans of no width (see `preempted`): outside a token, in a
  // grammar with an external scanner.
  scansWidthless(inToken) {
    return !inToken && this.program.externalTokens.size > 0;
  }

  step() {
    this.budget.steps += 1;
    if (this.budget.steps > this.budget.limit) throw new StepLimitReached();
  }

  // Takes `count` memo cells of the parse's memory budget.
  retain(count) {
    const { memory } = this.budget;
    memory.cells += count;
    if (memory.cells > memory.limit) throw new MemoryBudgetReached();
  }

  // Records an expectation at `position`; only the farthest position is kept.
  fail(position, expectation) {
    if (this.suppressed > 0) return;
    if (position > this.farthest) {
      this.farthest = position;
      this.expected = new Set([expectation]);
    } else if (position === this.farthest) {
      this.expected.add(expectation);
    }
  }

  quietly(run) {
    this.suppressed += 1;
    try {
      return run();
    } finally {
      this.suppressed -= 1;
    }
  }

  text(start, end) {
    return textOf(this.bytes, start, end) ?? '';
  }

  // Automatic recovery. An element (a terminal, a token, a token or atomic
  // rule, a scanner token) that fails in syntactic context at `start`, its
  // offset after trivia, is noted; at a repair point it yields instead a
  // zero-width MISSING leaf and, when `retry` matches the element at a later
  // code point boundary, a result that skips the bytes up to the first such
  // offset as an ERROR leaf. The scan for that offset depends only on the
  // element, `start` and the state, so it is made once per `element` (the
  // expression, rule or scanner token that failed) there. A token of an
  // external scanner has no `retry`: as in tree-sitter, whose recovery lexes
  // the skipped input in its error state, where a scanner refuses to run
  // (tree-sitter-rust's error sentinel), no skip ends at such a token, and a
  // scanner that reads to the end of the input before it fails would make
  // the scan quadratic.
  elementFailed(start, leaves, state, missing, element, retry) {
    if (this.suppressed > 0) return [];
    if (!this.repairPoints?.has(start)) {
      if (start > this.elementFarthest) this.elementFarthest = start;
      return [];
    }
    // As in tree-sitter, whose missing leaf has no padding, the MISSING leaf
    // comes before the separators (white space) that end the trivia before
    // it, after any other extra.
    let kept = leaves.length;
    while (kept > 0 && isSeparator(leaves[kept - 1])) kept -= 1;
    const at = kept > 0 ? leaves[kept - 1].end : leaves[0]?.start ?? start;
    const placed = [...leaves.slice(0, kept), { type: 'missing', start: at, end: at, ...missing }, ...leaves.slice(kept)];
    // The leaf is open (see `sequence`) until a rule it ends reduces.
    const results = [Object.assign(makeResult(start, state, placed, 0, MISSING_COST), { open: true })];
    let scans = this.repairMemo.get(element);
    if (!scans) this.repairMemo.set(element, scans = new Map());
    const key = `${start}|${state.key}|${this.inExtra}`;
    let scan = scans.get(key);
    if (!scan) {
      scan = { end: -1, found: [] };
      for (let cursor = start; retry && cursor < this.end;) {
        cursor += decodeAt(this.bytes, cursor, this.end).length;
        const found = this.quietly(() => retry(cursor));
        if (found.length === 0) continue;
        scan = { end: cursor, found };
        break;
      }
      scans.set(key, scan);
    }
    const error = { type: 'error', start, end: scan.end };
    for (const result of scan.found) {
      results.push(copyResult(result, { children: [...leaves, error, ...result.children], cost: result.cost + scan.end - start }));
    }
    return results;
  }

  // Skips trivia: repeatedly the longest match of any trivia expression
  // allowed in the current mode. Returns the new offset and the trivia leaves.
  // Inside an extra that builds a node, only the extras that are no rule
  // (white space) are trivia, unless the extra's rule reaches no scanner
  // token: then every extra nests in it, as a tree-sitter lexer lexes extras
  // there (see `nestingExtras` in load.js), except at its own start, where
  // the extra is its own rule's first token.
  skipTrivia(position, state) {
    const { trivia } = this.program;
    if (trivia.length === 0) return { end: position, leaves: NO_CHILDREN };
    const outrank = this.lexFrame(position)?.outranks?.get(position) ?? 0;
    // A separator an immediate token takes there is lexed, not skipped (see
    // `KeywordLexing`).
    const lexed = this.keywords?.separators.has(position) ?? false;
    const atExtra = this.inExtra === 'nesting' && position === this.extraStart;
    const key = `${position}|${state.key}|${this.inExtra}|${atExtra}|${outrank}`;
    const cached = this.triviaMemo.get(key);
    if (cached) return cached;
    const mode = state.modes[state.modes.length - 1];
    const leaves = [];
    let cursor = position;
    // A tree-sitter lexer skips a byte order mark at the start of the input.
    if (position === 0 && this.longestTokens && this.bytes[0] === 0xef && this.bytes[1] === 0xbb && this.bytes[2] === 0xbf) {
      leaves.push({ type: 'token', kind: null, start: 0, end: 3, trivia: true });
      cursor = 3;
    }
    const context = this.scanContext;
    this.scanContext = position;
    try {
    for (;;) {
      let best = cursor;
      let bestKind = null;
      for (const item of trivia) {
        if (item.modes && !item.modes.includes(mode)) continue;
        if (item.kind !== null && (this.inExtra === true || (atExtra && cursor === position))) continue;
        if (cursor === position && outrank > 0 && this.priorityOf(item.expression) < outrank) continue;
        if (cursor === position && lexed && item.kind === null) continue;
        const end = this.quietly(() => longestResult(this.evaluate(item.expression, cursor, state, true))?.end ?? -1);
        if (end > best) {
          best = end;
          bestKind = item.kind;
        }
      }
      if (best === cursor) break;
      const extra = this.extraNode(bestKind, cursor, best, state);
      if (extra === NO_EXTRA) break;
      leaves.push(extra?.node ?? { type: 'token', kind: bestKind, start: cursor, end: best, trivia: true });
      cursor = extra?.end ?? best;
    }
    } finally {
      this.scanContext = context;
    }
    const skipped = { end: cursor, leaves };
    this.triviaMemo.set(key, skipped);
    return skipped;
  }

  // The node an extra of a rule that builds one makes of its text, parsed as
  // syntax, as a tree-sitter extra of a rule that is no token is a node with
  // its children (Rust's doc comments); null for any other extra, and
  // NO_EXTRA where the text is no extra.
  // Under `(matching longest)` the parse is the one with the tokens a lexer
  // prefers, which may end before the longest (Rust's `////` is a comment
  // without a doc marker); otherwise the one that ends at `end`. Gives the
  // node and its end.
  extraNode(kind, start, end, state) {
    if (kind === null || this.program.rules.get(kind)?.kind !== 'normal') return null;
    const [outer, outerStart] = [this.inExtra, this.extraStart];
    // `nesting` inside an extra another extra may nest in, `true` inside any
    // other.
    this.inExtra = this.program.nestingExtras.has(kind) ? 'nesting' : true;
    this.extraStart = start;
    try {
      const parsed = this.quietly(() => this.evaluate({ kind: 'ref', name: kind }, start, state, false)).filter((result) => result.cost === 0);
      // An extra other extras nest in is syntax to a tree-sitter lexer, which
      // lexes the nested extras in it: where it has no parse as syntax, it is
      // no extra, though its text matches as one token (Rocq's
      // `(* a (* b *)`, whose inner comment closes and leaves the outer open).
      if (parsed.length === 0 && this.program.nestingExtras.has(kind)) return NO_EXTRA;
      const results = parsed.filter((result) => result.children.some((child) => child.type === 'node'));
      let best = null;
      for (const result of results) {
        if (!this.lexing) {
          if (result.end === end) best ??= result;
          continue;
        }
        const order = best === null ? 1 : preferredTokens(result, best, this.lexing);
        if (order > 0 || (order === 0 && result.end > best.end)) best = result;
      }
      const node = best?.children.find((child) => child.type === 'node');
      return node ? { node: { ...node, trivia: true }, end: best.end } : null;
    } finally {
      this.inExtra = outer;
      this.extraStart = outerStart;
    }
  }

  // A lexer takes a valid token of a raised lexical precedence over an extra
  // of a lower one that starts where it does, however longer the extra is: in
  // JavaScript's `"//"` the string fragment `//` after the quote is no
  // comment running to the end of the line. So where an immediate token of a
  // raised level matches, in syntactic context, no trivia of a lower level is
  // lexed at its offset (see `skipTrivia`).
  // The level holds in the rule call the token is lexed for (see `lexFrame`),
  // as a lexer lexes it in one parse state: after the opening quote of a
  // string, not after a string the parse tried to open at its closing quote.
  outrankTrivia(item, position, state) {
    const level = this.priorityOf(item);
    if (level <= 0) return;
    const frame = this.lexFrame(position);
    if (!frame || (frame.outranks?.get(position) ?? 0) >= level) return;
    const end = this.quietly(() => longestResult(this.evaluate(item, position, state, true))?.end ?? -1);
    if (end > position) (frame.outranks ??= new Map()).set(position, level);
  }

  // The innermost rule call that began before `position`: the one a token
  // at `position` is lexed for, the calls that begin there being part of
  // the same parse state.
  lexFrame(position) {
    for (let index = this.callStack.length - 1; index >= 0; index -= 1) {
      if (this.callStack[index].position < position) return this.callStack[index];
    }
    return null;
  }

  // The start of a terminal: after trivia in syntactic context, at once in token context.
  terminalStart(position, state, inToken) {
    return inToken ? { end: position, leaves: NO_CHILDREN } : this.skipTrivia(position, state);
  }

  // The starts of an immediate token: at once, or, in syntactic context under
  // `(matching longest)`, after the trivia up to an extra that is no separator
  // (such as a comment): a lexer skips no separator before an immediate token,
  // but lexes an extra token before it as before any other.
  immediateStarts(position, state, inToken) {
    const starts = [{ end: position, leaves: NO_CHILDREN }];
    if (inToken || !this.lexing) return starts;
    const { leaves } = this.skipTrivia(position, state);
    leaves.forEach((leaf, index) => {
      if (leaf.kind !== null) starts.push({ end: leaf.end, leaves: leaves.slice(0, index + 1) });
    });
    return starts;
  }

  // Under `(matching longest)` a lexer takes a valid token over a separator
  // (an anonymous trivia leaf, such as whitespace) it covers: a terminal that
  // matches at the start of such a leaf, at least as far, is matched there,
  // before it and the trivia after it, so `\n` ends a line where whitespace
  // is trivia. As in a tree-sitter lexer, whose separators loop back to the
  // start of every token, the token then also takes each next separator it
  // matches the same way (`\n\n` is one `\n` token). The end is -1 when no
  // such separator precedes `start`.
  // A token of an extra of a silent rule that builds no node (INI's newline
  // `_blank`) is taken over the same way, but only once: tree-sitter shifts
  // a valid token before it reduces the same token to an extra.
  beforeSeparator(expression, start, leaves) {
    const covers = (leaf, from) => leaf.kind === null && leaf.start === from && this.matchTerminal(expression, from) >= leaf.end;
    const at = leaves.findIndex((leaf) => (covers(leaf, leaf.start) || (this.silentExtra(leaf) && this.matchTerminal(expression, leaf.start) >= leaf.end)));
    if (at < 0) return { start, end: -1, leaves };
    let end = this.matchTerminal(expression, leaves[at].start);
    for (let index = at + 1; index < leaves.length && covers(leaves[index], end); index += 1) end = this.matchTerminal(expression, end);
    return { start: leaves[at].start, end, leaves: leaves.slice(0, at) };
  }

  // Whether the text of `[start, end)` is separators alone, a run of the
  // grammar's extras that are no rule (see `isSeparator`).
  separatorText(start, end) {
    if (start >= end) return false;
    const separators = this.program.trivia.filter(({ kind }) => kind === null);
    for (let at = start; at < end;) {
      let next = at;
      for (const { expression } of separators) {
        const reach = this.quietly(() => longestResult(this.evaluate(expression, at, INITIAL_STATE, true))?.end ?? -1);
        if (reach > next && reach <= end) next = reach;
      }
      if (next === at) return false;
      at = next;
    }
    return true;
  }

  // Whether `leaf` is the token of an extra of a silent rule.
  silentExtra(leaf) {
    return leaf.type === 'token' && leaf.trivia === true && leaf.kind !== null && this.program.rules.get(leaf.kind)?.kind === 'silent';
  }

  // The start of a token under `(matching longest)` after `leaves`, the
  // trivia before `start`: at the first separator or extra of a silent rule
  // (see `beforeSeparator`) whose text the token's item also matches, at
  // least as far, and without the trivia from it on (CSV's row ends with a
  // `\n` token where `\s` is trivia); otherwise `start` after them.
  // An extra that is a token rule is taken over too where the token wins the
  // lexical conflict with it, lexing longer at no lower precedence or matching a
  // nonempty token at a higher one: Make's `raw_line` of a define directive, `#comment\n`,
  // is no comment of a lower precedence.
  tokenBeforeExtra(item, start, leaves, state) {
    const level = this.priorityOf(item);
    const at = leaves.findIndex((leaf) => {
      const plain = isSeparator(leaf) || this.silentExtra(leaf);
      const rule = !plain && leaf.type === 'token' && leaf.trivia === true && leaf.kind !== null ? this.program.rules.get(leaf.kind) : null;
      if (!plain && rule?.kind !== 'token') return false;
      const reach = this.quietly(() => longestResult(this.evaluate(item, leaf.start, state, true))?.end ?? -1);
      if (plain) return reach >= leaf.end;
      const other = rule.lexicalPriority ?? 0;
      return (reach > leaf.end && level >= other) || (reach > leaf.start && level > other);
    });
    if (at >= 0 && isSeparator(leaves[at]) && this.quietly(() => longestResult(this.evaluate(item, leaves[at].start, state, true))?.end ?? -1) > start) {
      const chain = this.callStack.filter((entry) => entry.rule.kind === 'normal').map((entry) => [entry.rule.nodeKind, entry.position]);
      const chains = this.separatorTaken.get(leaves[at].start) ?? new Map();
      this.separatorTaken.set(leaves[at].start, chains.set(chain.join('>'), chain));
    }
    if (this.keywords !== null) {
      // A separator the token begins with, though it does not match on to
      // the token's text, is read on with (see `joinedStart`), unless the
      // token stops inside it: a lexer then resets the token's start where
      // only the separator goes on (Make's `\\` before a newline); one alive
      // at the separator's end reads on from the lexer's start state.
      const call = this.callStack[this.callStack.length - 1] ?? null;
      for (const leaf of leaves.slice(0, at < 0 ? leaves.length : at)) {
        if (!isSeparator(leaf)) continue;
        const reach = this.prefixReach(item, leaf.start, state);
        if (reach >= leaf.end) this.keywords.matchJoin(leaf.start, reach, call);
      }
    }
    return at < 0 ? { end: start, leaves } : { end: leaves[at].start, leaves: leaves.slice(0, at) };
  }

  // The farthest offset to which the text from `position` begins a match of
  // the lexical `expression`, as far as a lexer reads on with it.
  prefixReach(expression, position, state) {
    this.prefixReaches ??= new Map();
    let reaches = this.prefixReaches.get(expression);
    if (!reaches) this.prefixReaches.set(expression, reaches = new Map());
    if (!reaches.has(position)) reaches.set(position, this.prefixReachOf(expression, position, state));
    return reaches.get(position);
  }

  prefixReachOf(expression, position, state) {
    const ends = (item, at) => this.quietly(() => this.evaluate(item, at, state, true)).map((result) => result.end);
    switch (expression.kind) {
      case 'literal': case 'literalInsensitive': {
        const text = Buffer.from(expression.value, 'utf8');
        const fold = (byte) => (expression.kind === 'literalInsensitive' && byte >= 65 && byte <= 90 ? byte + 32 : byte);
        let at = 0;
        while (at < text.length && position + at < this.end && fold(this.bytes[position + at]) === fold(text[at])) at += 1;
        return position + at;
      }
      case 'charRange': case 'charClass': case 'byteClass': case 'any': case 'regex':
        return Math.max(position, this.matchTerminal(expression, position));
      case 'seq': {
        let [best, starts] = [position, [position]];
        for (const item of expression.items) {
          const next = new Set();
          for (const start of starts) {
            best = Math.max(best, this.prefixReach(item, start, state));
            for (const end of ends(item, start)) next.add(end);
          }
          starts = [...next];
          if (starts.length === 0) return best;
        }
        return Math.max(best, ...starts);
      }
      case 'choice': case 'longest':
        return Math.max(position, ...expression.items.map((item) => this.prefixReach(item, position, state)));
      case 'optional': case 'repeat0': case 'repeat1': case 'repeat': {
        const max = expression.kind === 'optional' ? 1 : expression.kind === 'repeat' ? (expression.max ?? Infinity) : Infinity;
        let [best, frontier, seen] = [position, [position], new Set([position])];
        for (let count = 0; count < max && frontier.length > 0; count += 1) {
          const next = [];
          for (const start of frontier) {
            best = Math.max(best, this.prefixReach(expression.item, start, state));
            for (const end of ends(expression.item, start)) if (!seen.has(end)) next.push(seen.add(end) && end);
          }
          frontier = next;
        }
        return best;
      }
      case 'ref': {
        const rule = this.program.rules.get(expression.name);
        return rule && !this.program.externalTokens.has(expression.name) ? this.prefixReach(rule.expression, position, state) : position;
      }
      case 'token': case 'immediateToken': case 'alias': case 'capture': case 'precedence': case 'namedPrecedence': case 'dynamicPrecedence': case 'lexicalPrecedence':
        return this.prefixReach(expression.item, position, state);
      default: return position;
    }
  }

  // The start of the leaf of a token matched at `start` after the trivia
  // `leaves`: a lexer resets the start of a token only on a separator no
  // token valid there reads on with, so a separator some valid token begins
  // with is the token's own (Solidity's `^` of `pragma solidity ^0.8.0;`
  // takes the space before it, as a version may begin with one). The
  // separators so read on with in the tree's parse state are `joined`.
  joinedStart(start, leaves) {
    const joined = this.keywords?.joined;
    if (!joined || joined.size === 0 || leaves.length === 0) return { start, leaves };
    let [reset, carry] = [leaves.length, -1];
    for (let index = 0; index < leaves.length; index += 1) {
      const leaf = leaves[index];
      const reach = isSeparator(leaf) ? Math.max(carry, joined.get(leaf.start) ?? -1) : -1;
      if (reach > leaf.start) {
        if (reset === leaves.length) reset = index;
        carry = reach;
        continue;
      }
      [reset, carry] = [leaves.length, -1];
    }
    return reset === leaves.length ? { start, leaves } : { start: leaves[reset].start, leaves: leaves.slice(0, reset) };
  }

  // The kind of a leaf from `from` of a token matched over `[start, end)`:
  // an anonymous token that took a separator before it is still named by
  // its own text (see `joinedStart`).
  joinedKind(kind, from, start, end) {
    if (kind !== null || from === start) return kind;
    const text = textOf(this.bytes, start, end);
    return text === null ? null : `'${text}`;
  }

  matcher(expression) {
    return this.program.matchers.get(expression);
  }

  matchTerminal(expression, start) {
    const matcher = this.matcher(expression);
    if (typeof matcher === 'function') return matcher(this.bytes, start, this.end);
    // A regular expression runs on the UTF-16 view of the input.
    this.view ??= utf16View(this.bytes, this.begin, this.end);
    const unit = this.view.byteToUnit.get(start);
    if (unit === undefined) return -1;
    matcher.regex.lastIndex = unit;
    const match = matcher.regex.exec(this.view.text);
    if (!match) return -1;
    return this.view.unitToByte.get(unit + match[0].length) ?? -1;
  }

  // Under `(matching longest)`, whether a lexer lexes another token than the
  // literal `text` over `[start, end)`: tree-sitter merges the lex states of
  // parse states whose tokens do not conflict, so a state's lexer also lexes
  // tokens no item of the state takes, and the longest token wins. A token
  // rule merges with a literal when it never matches the literal's text and
  // no token that may follow the literal anywhere in the grammar, nor a
  // separator, begins with the input it matches past the literal. So in
  // TypeScript's `0 .9` the number `.9` is lexed after `0`, where only a
  // member access `.` is valid, as no property name begins with `9`; Lean's
  // projection `.1` keeps its `.`, a number following it.
  mergedLonger(text, start, end, state) {
    const { tokens, follow, aliased } = mergedLexing(this.program);
    const longer = tokens.some(({ rule }) => {
      const ends = this.quietly(() => this.evaluate(rule.expression, start, state, true)).map((result) => result.end);
      return ends.some((reach) => reach > end) && !ends.includes(end);
    });
    if (!longer) return false;
    if (this.skipTrivia(end, state).end > end) return false;
    for (const key of follow.get(text) ?? []) {
      const [kind, name] = [key.slice(0, key.indexOf(' ')), key.slice(key.indexOf(' ') + 1)];
      if (kind === 'literal') {
        if (this.bytes[end] === Buffer.from(name, 'utf8')[0]) return false;
        continue;
      }
      if (this.program.externalTokens.has(name)) return false;
      if ((aliased.get(name) ?? []).some((item) => (this.quietly(() => longestResult(this.evaluate(item, end, state, true))?.end ?? -1)) > end)) return false;
      const rule = this.program.rules.get(name);
      if (rule && (rule.kind === 'token' || rule.kind === 'atomic')
        && (this.quietly(() => longestResult(this.evaluate({ kind: 'ref', name }, end, state, true))?.end ?? -1)) > end) return false;
    }
    return true;
  }

  terminal(expression, position, state, inToken) {
    if (!inToken) this.requestItem(expression, position);
    let { end: start, leaves } = this.terminalStart(position, state, inToken);
    let end = -1;
    if (this.lexing && leaves.length > 0) ({ start, end, leaves } = this.beforeSeparator(expression, start, leaves));
    if (end < 0) end = this.matchTerminal(expression, start);
    // A literal the grammar also takes as an immediate token (see
    // `KeywordLexing`) is not lexed plainly where the immediate one outranks it.
    const plain = !inToken && expression.kind === 'literal' && this.keywords?.tokens.immediate.has(expression.value);
    if (plain && end >= 0 && this.keywords.immediateOnly(start, end)) end = -1;
    if (end >= 0 && !inToken && this.lexing && expression.kind === 'literal' && this.mergedLonger(expression.value, start, end, state)) end = -1;
    if (end < 0) {
      this.fail(start, expectationOf(expression));
      if (inToken) return [];
      return this.elementFailed(start, leaves, state, missingOf(expression), expression, (cursor) => this.terminal(expression, cursor, state, false));
    }
    const joined = this.joinedStart(start, leaves);
    const leaf = { type: 'token', kind: this.joinedKind(separatorRunKind(expression, start, end), joined.start, start, end), start: joined.start, end };
    if (plain) leaf.plain = true;
    const children = inToken ? NO_CHILDREN : [...joined.leaves, leaf];
    return [makeResult(end, state, children)];
  }

  // A leaf over the longest match of `item` in token context: token(),
  // immediateToken() and longest() alternatives build on it.
  // A token under a lexical precedence keeps its level on the leaf, as the
  // token's rank where it is matched (see `tokenRank`).
  tokenLeaf(item, start, leaves, state, inToken, kind) {
    // A scanner owns its token start after skip(). Wrapping that token must
    // retain the skipped trivia, rather than turn it into token content.
    if (!inToken && item.kind === 'ref' && this.program.externalTokens.has(item.name)) {
      return this.scannerToken(item.name, start, state, false, true).map((result) => copyResult(result, {
        children: [...leaves, ...result.children.map((child) => (child.scanned ? { ...child, kind } : child))],
      }));
    }
    const best = longestResult(this.evaluate(item, start, state, true));
    if (!best) return [];
    const joined = inToken ? { start, leaves } : this.joinedStart(start, leaves);
    const leaf = item.kind === 'lexicalPrecedence'
      ? { type: 'token', kind: this.joinedKind(kind, joined.start, start, best.end), start: joined.start, end: best.end, priority: item.level }
      : { type: 'token', kind: this.joinedKind(kind, joined.start, start, best.end), start: joined.start, end: best.end };
    // A token of an external scanner is marked, whatever an alias names it
    // (see `preferredTokens`).
    if (item.kind === 'ref' && this.program.externalTokens.has(item.name)) leaf.scanned = true;
    const children = inToken ? NO_CHILDREN : [...joined.leaves, leaf];
    return [makeResult(best.end, best.state, children, best.dynamic)];
  }

  /** Every result of `expression` at `position` in `state`. */
  evaluate(expression, position, state, inToken) {
    this.step();
    switch (expression.kind) {
      case 'empty': return [makeResult(position, state)];
      case 'literal': case 'literalInsensitive': case 'charRange': case 'charClass': case 'byteClass': case 'any': case 'regex':
        return this.terminal(expression, position, state, inToken);
      case 'ref': return this.reference(expression.name, position, state, inToken);
      case 'seq': return this.sequence(expression.items, position, state, inToken);
      case 'choice': return this.choice(expression, position, state, inToken);
      case 'optional': return this.repetition(expression.item, 0, 1, position, state, inToken);
      case 'repeat0': return this.repetition(expression.item, 0, null, position, state, inToken);
      case 'repeat1': return this.repetition(expression.item, 1, null, position, state, inToken);
      case 'repeat': return this.repetition(expression.item, expression.min, expression.max ?? null, position, state, inToken);
      case 'and': {
        const matched = this.quietly(() => this.evaluate(expression.item, position, state, inToken).length > 0);
        return matched ? [makeResult(position, state)] : [];
      }
      case 'not': {
        const matched = this.quietly(() => this.evaluate(expression.item, position, state, inToken).length > 0);
        return matched ? [] : [makeResult(position, state)];
      }
      case 'capture':
        // A child a silent rule captured keeps its field: tree-sitter names a
        // child by the innermost field over it (Lean's `type` of `Nat` in the
        // `binders` of `∀ x : Nat, p`).
        return this.evaluate(expression.item, position, state, inToken).map((result) => (inToken ? result : copyResult(result, {
          children: result.children.map((child) => (isTrivia(child) || child.field != null ? child : { ...child, field: expression.label })),
        })));
      case 'alias': return this.alias(expression, position, state, inToken);
      case 'precedence': case 'namedPrecedence': return this.precedence(expression, position, state, inToken);
      case 'dynamicPrecedence':
        return this.evaluate(expression.item, position, state, inToken)
          .map((result) => copyResult(result, { dynamic: result.dynamic + expression.level }));
      case 'lexicalPrecedence': return this.evaluate(expression.item, position, state, inToken);
      case 'longest': return this.longest(expression, position, state, inToken);
      case 'token': case 'immediateToken': {
        if (expression.kind === 'immediateToken' && !inToken) this.outrankTrivia(expression.item, position, state);
        let starts = expression.kind === 'token'
          ? [this.terminalStart(position, state, inToken)]
          : this.immediateStarts(position, state, inToken);
        if (expression.kind === 'token' && this.lexing && starts[0].leaves.length > 0) {
          starts = [this.tokenBeforeExtra(expression.item, starts[0].end, starts[0].leaves, state)];
        }
        const found = new Map();
        const keyword = this.keywords !== null && !inToken && isKeyword(expression.item);
        const immediate = this.keywords !== null && !inToken && expression.kind === 'immediateToken' && expression.item.kind === 'literal';
        // A scanner token is lexed at the first start where its scanner
        // succeeds, as a lexer runs the external scanner before it lexes an
        // extra: after a comment only where it fails before it.
        const scanned = expression.item.kind === 'ref' && this.program.externalTokens.has(expression.item.name);
        // The external scanner skips separators itself, so only a token of
        // the lexer takes one in (Lean's layout tokens do not).
        const separating = this.keywords !== null && this.lexing && !inToken && expression.kind === 'immediateToken' && !scanned;
        // The token is requested where the parse asks for it, though its
        // item is evaluated in token context (TypeScript's function
        // signature ends with `(immediateToken (ref
        // function_signature_automatic_semicolon))`, which the automatic
        // semicolon scanner asks about).
        if (!inToken) this.requestItem(expression.item, position);
        const context = this.scanContext;
        if (!inToken) this.scanContext = position;
        try {
          for (const { end, leaves } of starts) {
            for (const result of this.tokenLeaf(expression.item, end, leaves, state, inToken, null)) {
              if (keyword || immediate) this.keywords.match(`${end}|${result.end}`, this.callStack[this.callStack.length - 1] ?? null, immediate);
              if (separating && end === position && this.separatorText(end, result.end)) this.keywords.matchSeparator(end, this.callStack[this.callStack.length - 1] ?? null);
              addResult(found, result, this.longestTokens, this.settling);
            }
            if (scanned && found.size > 0) break;
          }
        } finally {
          this.scanContext = context;
        }
        const results = [...found.values()];
        const { end: start, leaves } = starts[0];
        if (results.length > 0 || inToken) return results;
        return this.elementFailed(start, leaves, state, missingOf(expression.item), expression, scanned ? null : (cursor) => this.evaluate(expression, cursor, state, false));
      }
      case 'predicate': return this.predicate(expression, position, state, inToken);
      case 'recover': return this.recover(expression, position, state, inToken);
      case 'missing': return this.missing(expression, position, state, inToken);
      case 'embed': return this.embed(expression, position, state, inToken);
      default: throw new TypeError(`the grammar executor has no case for expression kind ${expression.kind}`);
    }
  }

  // The results of `item` after `left`. While repairing, a sequence may
  // continue after a MISSING leaf at a repair point, repairing what follows,
  // but what follows may not do so again at that offset: a second
  // continuation there is matched quietly. Chains of zero-width MISSING
  // leaves, which would make every rule left-recursive at that offset, are so
  // never built. The MISSING leaf may end a node `left` holds when tokens the
  // external scanner scanned of no width follow it, as a layout token opening
  // an indented block does: each such repaired block would otherwise open
  // another at the offset, up to the scanner's deepest indentation.
  continuation(item, left, inToken) {
    if (!this.repairPoints?.has(left.end)) return this.evaluate(item, left.end, left.state, inToken);
    const last = lastRepaired(left.children);
    if (last?.type !== 'missing') return this.evaluate(item, left.end, left.state, inToken);
    if (this.chained.has(left.end)) return this.quietly(() => this.evaluate(item, left.end, left.state, inToken));
    this.chained.add(left.end);
    try {
      return this.evaluate(item, left.end, left.state, inToken);
    } finally {
      this.chained.delete(left.end);
    }
  }

  // `keep`, when given, filters the complete sequences before they are
  // deduplicated, so a precedence filter never loses a valid parse to an
  // invalid one that reached the same end first; `owner` is that filter's
  // precedence, the one the sequence's parts are in progress under.
  // A MISSING leaf is open until a rule it ends reduces (a rule other than a
  // token, or an iteration): as tree-sitter inserts a missing token only where
  // a rule reduces before the lookahead, no part of the same rule after it
  // takes input (`[ 0 .92 ]` misses no `,` before `.92`).
  sequence(items, position, state, inToken, keep = null, owner = null) {
    const tokens = owner && this.longestTokens ? { ...this.longestTokens, owner } : this.longestTokens;
    const reductions = this.reducing && !inToken ? reductionFacts(this.reducing.grammar) : null;
    const split = reductions?.splits.get(items) ?? null;
    // The offset each result's optional parts begin at, where its rule
    // could have been reduced (see `reductionFacts`).
    const boundaries = split ? new Map() : null;
    let current = [makeResult(position, state)];
    for (const [index, item] of items.entries()) {
      const next = new Map();
      const last = index === items.length - 1;
      const rest = reductions && !last ? this.restKeys(reductions, items, index) : null;
      const continued = current.map((left) => [left, this.continuation(item, left, inToken)]);
      const pruned = this.scansWidthless(inToken) ? preempted(continued) : null;
      for (const [left, rights] of continued) {
        if (pruned?.has(left)) continue;
        for (const right of rights) {
          if (rest && reducedEarly(right, rest)) continue;
          if (left.open && right.end > left.end) continue;
          let joined = joinResults(left, right, inToken);
          if (split && index >= split.split) {
            const boundary = index === split.split ? left.end : boundaries.get(left);
            if (!last) boundaries.set(joined, boundary);
            else {
              const lookahead = lookaheadAfter(joined.children, boundary, this.bytes);
              if (split.always.has(lookahead)) continue;
              if (split.marked.has(lookahead)) joined = Object.assign(joined, { before: lookahead });
            }
          }
          if (last && keep && !keep(joined)) continue;
          addResult(next, joined, tokens, this.settling);
        }
      }
      current = [...next.values()];
      if (this.peg) current = current.slice(0, 1);
      if (current.length === 0) return current;
    }
    return current;
  }

  // The marked tokens (see `reductionFacts`) an iteration of `item` may begin
  // with, or null when it begins with none.
  iterationKeys(reductions, item) {
    if (reductions.keys.size === 0) return null;
    if (!reductions.rests.has(item)) {
      const keys = firstOf(item, this.program.rules, this.longestTokens.grammar.first);
      const marked = new Set([...keys].filter((key) => reductions.keys.has(key)));
      reductions.rests.set(item, marked.size > 0 ? marked : null);
    }
    return reductions.rests.get(item);
  }

  // The marked tokens (see `reductionFacts`) the parts of `items` after the
  // one at `index` may begin with, or null when they begin with none.
  restKeys(reductions, items, index) {
    if (reductions.keys.size === 0) return null;
    let rests = reductions.rests.get(items);
    if (!rests) reductions.rests.set(items, rests = []);
    if (rests[index] === undefined) {
      const keys = firstOf({ kind: 'seq', items: items.slice(index + 1) }, this.program.rules, this.longestTokens.grammar.first);
      const marked = new Set([...keys].filter((key) => reductions.keys.has(key)));
      rests[index] = marked.size > 0 ? marked : null;
    }
    return rests[index];
  }

  choice(expression, position, state, inToken) {
    if (expression.ordered) {
      for (const item of expression.items) {
        const results = this.evaluate(item, position, state, inToken);
        if (results.length > 0) return results;
      }
      return [];
    }
    if (this.peg) {
      // PEG unordered choice: the longest alternative, the first on a tie.
      let best = null;
      for (const item of expression.items) {
        const [result] = this.evaluate(item, position, state, inToken);
        if (result && (!best || result.end > best.end)) best = result;
      }
      return best ? [best] : [];
    }
    const results = new Map();
    const outranked = this.lexing && !inToken ? immediateLiterals(expression) : null;
    for (const item of expression.items) {
      if (outranked?.patterns.has(item) && this.literalOutranks(outranked.patterns.get(item), outranked.literals, position, state)) continue;
      for (const result of this.evaluate(item, position, state, inToken)) addResult(results, result, this.longestTokens, this.settling);
    }
    return [...results.values()];
  }

  // Whether an immediate literal another alternative begins with matches
  // just what the immediate token `pattern` matches at `position`: a lexer
  // that lexes both takes the string over the pattern of one length, so the
  // pattern's alternative is not taken (Make's `$(` without its `)`, whose
  // `(` is no one-character variable name).
  literalOutranks(pattern, literals, position, state) {
    const end = this.quietly(() => longestResult(this.evaluate(pattern, position, state, true))?.end ?? -1);
    if (end <= position) return false;
    const text = textOf(this.bytes, position, end);
    return literals.has(text);
  }

  repetition(item, min, max, position, state, inToken) {
    // Each iteration reduces (see `sequence`).
    const join = (left, right) => closed(joinResults(left, right, inToken));
    const zeroWidth = (left, right) => right.end === left.end && right.state.key === left.state.key;
    if (this.peg) {
      // Greedy and possessive: as many iterations as match, never fewer.
      let current = makeResult(position, state);
      let count = 0;
      while (max === null || count < max) {
        const [next] = this.evaluate(item, current.end, current.state, inToken);
        if (!next) break;
        if (zeroWidth(current, next)) {
          if (count < min) count = min;
          current = join(current, next);
          break;
        }
        current = join(current, next);
        count += 1;
      }
      return count >= min ? [current] : [];
    }
    // Generalized: a breadth-first frontier by iteration count. Once the
    // minimum is met, a result whose end and state were already reached is
    // not extended again (it is the same continuation) but marks ambiguity,
    // unless it replaces the result reached before: then the continuations
    // of the replaced one are replaced too, by extending it.
    const results = new Map();
    // A rule reduced before a token the next iteration may begin with ends
    // no iteration (see `reducedEarly`).
    const reductions = this.reducing && !inToken ? reductionFacts(this.reducing.grammar) : null;
    const rest = reductions ? this.iterationKeys(reductions, item) : null;
    let frontier = [makeResult(position, state)];
    for (let count = 0; frontier.length > 0; count += 1) {
      if (count >= min) {
        const fresh = [];
        for (const result of frontier) {
          addResult(results, result, this.longestTokens, this.settling);
          if (results.get(resultKey(result)) === result) fresh.push(result);
        }
        frontier = fresh;
      }
      if (max !== null && count >= max) break;
      const next = new Map();
      const continued = frontier.map((left) => [left, this.continuation(item, left, inToken)]);
      const scans = this.scansWidthless(inToken);
      const pruned = scans ? preempted(continued) : null;
      for (const [left, rights] of continued) {
        // A result an iteration goes on from with a token the external
        // scanner scanned of no width is no parse itself (the optional layout
        // end after `def foo := 12` is taken where the scanner scans it, as
        // the token is the lookahead), and neither is one that token
        // preempts (see `preempted`).
        if (scans && (pruned?.has(left) || rights.some(startsWidthless)) && results.get(resultKey(left)) === left) results.delete(resultKey(left));
        if (pruned?.has(left)) continue;
        for (const right of rights) {
          if (rest && reducedEarly(right, rest)) continue;
          if (zeroWidth(left, right)) {
            // Zero-width iterations can pad up to the minimum once; one that
            // takes a token the external scanner scanned of no width (Lean's
            // layout semicolon between two structure fields) is a result too,
            // as the parser shifts that token, though it is not extended again.
            if (count < min || right.children.some((child) => child.scanned === true)) addResult(results, join(left, right), this.longestTokens, this.settling);
            continue;
          }
          addResult(next, join(left, right), this.longestTokens, this.settling);
        }
      }
      frontier = [...next.values()];
    }
    return [...results.values()];
  }

  // An alias of a silent rule names the node the rule does not build, as a
  // tree-sitter alias of a hidden rule does, even around a single child.
  alias(expression, position, state, inToken) {
    const { item } = expression;
    const wraps = item.kind === 'ref' && this.program.rules.get(item.name)?.kind === 'silent';
    return this.evaluate(item, position, state, inToken).map((result) => {
      if (inToken) return result;
      const meaningful = result.children.filter((child) => !isTrivia(child));
      if (meaningful.length === 1 && !wraps) {
        // A token leaf keeps the rank of the token it names (see
        // `tokenRanks` in load.js).
        const rank = meaningful[0].type === 'token' ? this.program.tokenRanks?.expressions?.get(expression) : undefined;
        const rename = (child) => (rank ? { ...renamed(child, expression.name), rank } : renamed(child, expression.name));
        return copyResult(result, { children: result.children.map((child) => (child === meaningful[0] ? rename(child) : child)) });
      }
      const node = shareChildren({
        type: 'node', kind: expression.name, rule: expression.name, start: position, end: result.end,
        ambiguous: result.ambiguous,
      }, result);
      return copyResult(result, { children: [node], ambiguous: false });
    });
  }

  // Precedence and associativity filter the binary-shaped results: a
  // leftmost or rightmost child node of lower precedence, or of equal
  // precedence on the side the associativity forbids, invalidates a result
  // when it conflicts, that is when its own child facing the operator could
  // have been the operand instead (`-a->t` but not `f(a)->t`).
  precedence(expression, position, state, inToken) {
    const { associativity } = expression;
    // The rule a precedence ranks is the one whose body holds it.
    const tag = { level: expression.level ?? 0, name: expression.name ?? null, associativity, rule: this.ownerOf(expression) };
    const orders = this.program.precedenceOrders;
    // A result whose right operand is of one part waits for the others: it
    // stands only when no result of the expression ends before that operand.
    const pending = [];
    const valid = (result) => {
      const verdict = allowed(result);
      if (verdict === LONE) pending.push(result);
      if (verdict === true) return true;
      if (verdict === false) this.fail(result.end, 'precedence');
      return false;
    };
    const conflicts = (wrapper, side, next = null) => {
      const child = wrapped(wrapper);
      if (child.type !== 'node' || !child.precedence) return false;
      const order = comparePrecedence(child.precedence, tag, orders);
      if (order > 0 || (order === 0 && associativity === side)) return false;
      if (side === 'right' && this.shiftsBelow(expression, child, orders)) return false;
      if (side === 'right' && this.lexedShift(child.rule)) return false;
      if (!this.reachesOwner(expression, wrapper.rule, side)) return false;
      if (side === 'left' && next && declaredFork(grammarFacts(this.program), child, lookaheadOf(next, this.bytes), orders)) return false;
      // A child of one part (Rust's bare range `..` in `a ..= ..`) has no
      // operand of its own the operator could have taken instead; on the
      // right, the parse before it could still have been reduced first.
      const facing = child.children.filter((grandchild) => !isTrivia(grandchild));
      if (facing.length < 2) return side === 'right' ? LONE : false;
      const kinds = this.operandKinds(expression)[side];
      if (kinds === null) return true;
      const edge = side === 'left' ? facing[facing.length - 1] : facing[0];
      return edge !== undefined && kinds.has(edge.kind);
    };
    const allowed = (result) => {
      const meaningful = result.children.filter((child) => !isTrivia(child));
      if (meaningful.length < 2 || conflicts(meaningful[0], 'left', meaningful[1])) return meaningful.length < 2;
      const right = conflicts(meaningful[meaningful.length - 1], 'right');
      return right === LONE ? LONE : !right;
    };
    let results = inToken
      ? this.evaluate(expression.item, position, state, inToken)
      : this.filtered(expression.item, position, state, valid, tag);
    if (pending.length > 0) results = this.lonePending(results, pending);
    return results.map((result) => {
      if (inToken) return copyResult(result, { precedence: tag });
      // The innermost precedence over the last part of a result is the one
      // its rule reduces with (see `reduction`).
      const tail = result.tail ?? tag;
      // A token reduced alone keeps the precedence on its leaf, for the
      // conflict with a shift after it (see `loneReduction`), and a node
      // reduced alone keeps it as the precedence it was reduced with, for the
      // conflict with another reduction of it (see `shiftOrder`).
      const meaningful = result.children.filter((child) => !isTrivia(child));
      if (meaningful.length !== 1) return copyResult(result, { precedence: tag, tail });
      const only = meaningful[0];
      const tagged = only.type === 'token' ? { ...only, precedence: tag } : only.type === 'node' ? copyNode(only, { reduced: tag }) : null;
      if (tagged === null) return copyResult(result, { precedence: tag, tail });
      const children = result.children.map((child) => (child === only ? tagged : child));
      return copyResult(result, { precedence: tag, tail, children });
    });
  }

  // The `results` of a precedence expression with the `pending` ones whose
  // right operand is a node of one part (Rust's bare range `..`) that an LR
  // parser shifts: where a result of the expression ends before that operand
  // (`..` in `.. ..`, `a..` in `a .. ..`), the parser reduces it there first,
  // as the operand's level is not above the operator's (`a ..= ..` stands).
  lonePending(results, pending) {
    const ends = new Set(results.map((result) => result.end));
    const found = new Map(results.map((result) => [resultKey(result), result]));
    for (const result of pending) {
      const meaningful = result.children.filter((child) => !isTrivia(child));
      if (ends.has(meaningful[meaningful.length - 2].end)) {
        this.fail(result.end, 'precedence');
        continue;
      }
      addResult(found, result, this.longestTokens, this.settling);
    }
    return [...found.values()];
  }

  // Whether a node of `kind` goes on after its first child only with tokens
  // of raised lexical precedence (Rust's `B<C>`, whose `<` is
  // `(token (lexicalPrecedence 1 (literal <)))`): a lexer takes such a token
  // wherever the parse admits it, so the parse shifts it instead of reducing
  // the operator before the node, and the lower precedence of the node as a
  // right operand is no conflict.
  lexedShift(kind) {
    let shifts = this.shiftMemo.get(kind);
    if (shifts !== undefined) return shifts;
    shifts = false;
    const rule = this.program.rules.get(kind);
    let body = rule?.nodeKind === kind ? rule.expression : null;
    while (body && ['precedence', 'namedPrecedence', 'dynamicPrecedence', 'capture'].includes(body.kind)) body = body.item;
    if (body?.kind === 'seq' && body.items.length > 1 && !nullable(body.items[0])) {
      shifts = (this.leadPriority({ kind: 'seq', items: body.items.slice(1) }, new Set()) ?? 0) > 0;
    }
    this.shiftMemo.set(kind, shifts);
    return shifts;
  }

  // Whether a generated parser shifts into the right operand `child`
  // before it could reduce the operand's first part up to the operator's
  // operand, as tree-sitter's handle_conflict decides: where the child's
  // rule takes a part at its edge directly (`primary_expression` of
  // TypeScript's member expression), the shift into the child conflicts
  // with the reduction of that part into the rule above it on the way to
  // the operand (`expression`), not with the operator's own reduction, and
  // the shift wins when the child's precedence is higher than that
  // reduction's (`member` ranks above the rule `expression`), so that
  // `<C>e.f` asserts the type of `e.f`. The deepest such reduction is the
  // first the parser meets.
  shiftsBelow(expression, child, orders) {
    const first = child.children.find((grandchild) => !isTrivia(grandchild));
    const rule = this.program.rules.get(child.rule);
    if (!first?.kind || !rule) return false;
    let memo = this.belowMemo.get(expression);
    if (!memo) this.belowMemo.set(expression, (memo = new Map()));
    const key = `${child.rule}|${first.kind}`;
    let reduction = memo.get(key);
    if (reduction === undefined) {
      reduction = this.reductionBelow(expression, rule, first.kind);
      memo.set(key, reduction);
    }
    return reduction !== null && comparePrecedence(child.precedence, reduction, orders) > 0;
  }

  // The precedence of the deepest reduction of a part that `rule` takes
  // directly at its start, made on the way from the right operand of the
  // precedence `expression` down to a node of `kind`, or null when none is.
  reductionBelow(expression, rule, kind) {
    const direct = new Set(directEdge(rule.expression, 'right', rule.name).map(([name]) => name));
    const derives = (name) => {
      const names = new Set();
      this.edgeRules({ kind: 'ref', name }, 'right', names);
      return [...names].some((each) => each === kind || this.program.rules.get(each)?.nodeKind === kind);
    };
    let found = null;
    const seen = new Set();
    let level = directEdge(expression.item, 'left', null).map(([name]) => name);
    for (let depth = 0; level.length > 0; depth += 1) {
      const deeper = [];
      for (const name of level) {
        const above = this.program.rules.get(name);
        if (seen.has(name) || !above) continue;
        seen.add(name);
        for (const [part, tag] of directEdge(above.expression, 'right', name)) {
          if (direct.has(part) && derives(part)) found = tag ?? unranked(name);
          if (this.program.rules.get(part)?.kind === 'silent') deeper.push(part);
        }
      }
      level = deeper;
    }
    return found;
  }

  // The lowest lexical precedence of the tokens `expression` can begin with,
  // or null when it may match nothing first.
  leadPriority(expression, visiting) {
    switch (expression.kind) {
      case 'token': case 'immediateToken':
        return expression.item.kind === 'lexicalPrecedence' ? expression.item.level : 0;
      case 'lexicalPrecedence': return expression.level;
      case 'ref': {
        const rule = this.program.rules.get(expression.name);
        if (!rule || rule.kind === 'token' || rule.kind === 'atomic') return rule?.lexicalPriority ?? 0;
        if (visiting.has(expression.name)) return null;
        visiting.add(expression.name);
        return this.leadPriority(rule.expression, visiting);
      }
      case 'seq': {
        let lowest = null;
        for (const item of expression.items) {
          const lead = this.leadPriority(item, visiting);
          if (lead !== null) lowest = lowest === null ? lead : Math.min(lowest, lead);
          if (!nullable(item)) return lowest;
        }
        return lowest;
      }
      case 'choice': {
        const leads = expression.items.map((item) => this.leadPriority(item, visiting)).filter((lead) => lead !== null);
        return leads.length > 0 ? Math.min(...leads) : null;
      }
      case 'capture': case 'precedence': case 'namedPrecedence': case 'dynamicPrecedence': case 'alias':
      case 'optional': case 'repeat0': case 'repeat1': case 'repeat': return this.leadPriority(expression.item, visiting);
      case 'empty': case 'and': case 'not': return null;
      default: return 0;
    }
  }

  // Whether the rule whose body holds the precedence `expression` may stand
  // at the edge of a `kind` node that faces the operator on `side` (its last
  // part for the left operand): only then could a generated parser build the
  // operator's node inside the operand, so that the two conflict. A field of
  // Rust's `a.0.1` is never a field expression, so `a.0` is no operand of
  // lower precedence there. True when either rule is unknown.
  reachesOwner(expression, kind, side) {
    const owner = this.ownerOf(expression);
    const rule = this.program.rules.get(kind);
    if (owner === null || rule?.nodeKind !== kind) return true;
    const key = `${kind}|${side}`;
    let names = this.edgeMemo.get(key);
    if (!names) {
      names = new Set();
      this.edgeRules(rule.expression, side, names);
      this.edgeMemo.set(key, names);
    }
    return names.has(owner);
  }

  // The rule whose body holds the precedence `expression`, or null.
  ownerOf(expression) {
    if (!this.owners) {
      this.owners = new Map();
      const walk = (item, name) => {
        if (!item || typeof item !== 'object') return;
        if ((item.kind === 'precedence' || item.kind === 'namedPrecedence') && !this.owners.has(item)) this.owners.set(item, name);
        walk(item.item, name);
        if (Array.isArray(item.items)) for (const child of item.items) walk(child, name);
      };
      for (const [name, rule] of this.program.rules) walk(rule.expression, name);
    }
    return this.owners.get(expression) ?? null;
  }

  // Adds to `names` the rules `expression` may match at its edge facing the
  // operator on `side`, through the silent rules there, as `unitKinds` walks.
  edgeRules(expression, side, names) {
    switch (expression.kind) {
      case 'ref': {
        if (names.has(expression.name)) return;
        names.add(expression.name);
        const rule = this.program.rules.get(expression.name);
        if (rule?.kind === 'silent') this.edgeRules(rule.expression, side, names);
        return;
      }
      case 'seq': {
        const items = side === 'left' ? [...expression.items].reverse() : expression.items;
        for (const item of items) {
          this.edgeRules(item, side, names);
          if (!(item.kind === 'optional' || item.kind === 'repeat0' || (item.kind === 'repeat' && item.min === 0))) return;
        }
        return;
      }
      case 'choice': for (const item of expression.items) this.edgeRules(item, side, names); return;
      case 'capture': case 'precedence': case 'namedPrecedence': case 'dynamicPrecedence': case 'alias':
      case 'optional': case 'repeat0': case 'repeat1': case 'repeat': this.edgeRules(expression.item, side, names);
    }
  }

  // The node kinds the leftmost and rightmost operand of a precedence
  // expression can match as one child, or null when unknown.
  operandKinds(expression) {
    let kinds = this.operandMemo.get(expression);
    if (kinds) return kinds;
    const edge = (item, side) => {
      if (item.kind === 'capture') return edge(item.item, side);
      if (item.kind === 'seq' && item.items.length > 0) return this.unitKinds(item.items[side === 'left' ? 0 : item.items.length - 1], new Set(), side);
      if (item.kind === 'choice') return union(item.items.map((choice) => edge(choice, side)));
      return null;
    };
    kinds = { left: edge(expression.item, 'left'), right: edge(expression.item, 'right') };
    this.operandMemo.set(expression, kinds);
    return kinds;
  }

  // The node kinds `expression` can match as one child, or null when unknown;
  // the child at the edge of a repetition (or an optional) is one of its item.
  // The operand on the `side` of the operator faces it with the opposite end
  // of a sequence a silent rule inlines: its first item for the right
  // operand, its last for the left one, and the items after it while those
  // may match nothing.
  unitKinds(expression, visiting, side) {
    switch (expression.kind) {
      case 'ref': {
        const rule = this.program.rules.get(expression.name);
        // A scanner token is a leaf of its own kind.
        if (!rule) return this.program.scanners.has(expression.name) ? new Set([expression.name]) : null;
        if (rule.kind !== 'silent') return new Set([rule.nodeKind]);
        if (visiting.has(expression.name)) return new Set();
        visiting.add(expression.name);
        return this.unitKinds(rule.expression, visiting, side);
      }
      case 'seq': {
        const items = side === 'left' ? [...expression.items].reverse() : expression.items;
        const kinds = [];
        for (const item of items) {
          kinds.push(this.unitKinds(item, visiting, side));
          if (!(item.kind === 'optional' || item.kind === 'repeat0' || (item.kind === 'repeat' && item.min === 0))) break;
        }
        return kinds.length > 0 ? union(kinds) : null;
      }
      case 'choice': return union(expression.items.map((item) => this.unitKinds(item, visiting, side)));
      case 'capture': case 'precedence': case 'namedPrecedence': case 'dynamicPrecedence':
      case 'optional': case 'repeat0': case 'repeat1': case 'repeat': return this.unitKinds(expression.item, visiting, side);
      case 'alias': return new Set([expression.name]);
      default: return null;
    }
  }

  // The results of `expression` that `keep` accepts, filtered before a
  // sequence or an unordered choice merges results of the same end and state
  // under `owner`, the precedence of the filter.
  filtered(expression, position, state, keep, owner = null) {
    if (expression.kind === 'seq' && expression.items.length > 0) return this.sequence(expression.items, position, state, false, keep, owner);
    if (expression.kind === 'choice' && !expression.ordered && !this.peg) {
      this.step();
      const results = new Map();
      const tokens = owner && this.longestTokens ? { ...this.longestTokens, owner } : this.longestTokens;
      for (const item of expression.items) {
        for (const result of this.filtered(item, position, state, keep, owner)) addResult(results, result, tokens, this.settling);
      }
      return [...results.values()];
    }
    return this.evaluate(expression, position, state, false).filter(keep);
  }

  priorityOf(item) {
    if (item.kind === 'lexicalPrecedence') return item.level;
    if (item.kind === 'ref') return this.program.rules.get(item.name)?.lexicalPriority ?? 0;
    return 0;
  }

  // Lexical longest match: the alternative with the longest match wins, a
  // tie goes to the higher lexical priority and then to the first.
  longest(expression, position, state, inToken) {
    const { end: start, leaves } = this.terminalStart(position, state, inToken);
    let best = null;
    for (const item of expression.items) {
      const result = longestResult(this.evaluate(item, start, state, true));
      if (!result) continue;
      const priority = this.priorityOf(item);
      if (!best || result.end > best.result.end || (result.end === best.result.end && priority > best.priority)) {
        best = { result, priority, item };
      }
    }
    if (!best) {
      if (inToken) return [];
      return this.elementFailed(start, leaves, state, { kind: null }, expression, (cursor) => this.longest(expression, cursor, state, false));
    }
    const kind = best.item.kind === 'ref' ? (this.program.rules.get(best.item.name)?.nodeKind ?? best.item.name) : null;
    const joined = inToken ? null : this.joinedStart(start, leaves);
    const children = inToken ? NO_CHILDREN : [...joined.leaves, { type: 'token', kind: this.joinedKind(kind, joined.start, start, best.result.end), start: joined.start, end: best.result.end }];
    return [makeResult(best.result.end, best.result.state, children, best.result.dynamic)];
  }

  // The read-only machine conditions and values see over one result.
  valueMachine(result, from, working) {
    const start = contentStart(result.children, from);
    return {
      state: working,
      step: () => this.step(),
      column: () => columnOf(this.bytes, start, this.begin),
      matched: () => this.text(start, result.end),
      atEnd: () => result.end === this.end,
      lookahead: () => false,
      requested: null,
    };
  }

  predicate(expression, position, state, inToken) {
    const kept = this.evaluate(expression.item, position, state, inToken).filter((result) => {
      try {
        return evaluateCondition(expression.condition, this.valueMachine(result, position, workingState(result.state)));
      } catch (error) {
        if (error instanceof OperationFailed) return false;
        throw error;
      }
    });
    if (kept.length === 0) this.fail(this.terminalStart(position, state, inToken).end, 'predicate');
    return kept;
  }

  // Error recovery: when the item fails, skip at least one unit up to the
  // first offset where the synchronization expression matches and record
  // the skipped bytes as an ERROR node.
  recover(expression, position, state, inToken) {
    const results = this.evaluate(expression.item, position, state, inToken);
    if (results.length > 0) return results;
    const { end: start, leaves } = this.terminalStart(position, state, inToken);
    for (let cursor = start; cursor < this.end;) {
      cursor += decodeAt(this.bytes, cursor, this.end).length;
      const synchronized = this.quietly(() => this.evaluate(expression.synchronize, cursor, state, inToken).length > 0);
      if (synchronized) {
        const children = inToken ? NO_CHILDREN : [...leaves, { type: 'error', start, end: cursor }];
        return [makeResult(cursor, state, children)];
      }
    }
    return [];
  }

  missing(expression, position, state, inToken) {
    const results = this.evaluate(expression.item, position, state, inToken);
    if (results.length > 0) return results;
    const { item } = expression;
    const node = { type: 'missing', kind: null, start: position, end: position };
    if (item.kind === 'ref') node.kind = this.program.rules.get(item.name)?.nodeKind ?? item.name;
    else if (item.kind === 'literal') Object.assign(node, { kind: item.value, literal: true });
    return [makeResult(position, state, inToken ? NO_CHILDREN : [node])];
  }

  // An embedded language parses the region the item matches, on the same
  // bytes, so its offsets stay absolute.
  embed(expression, position, state, inToken) {
    const language = this.program.language(expression.language);
    const { end: start, leaves } = this.terminalStart(position, state, inToken);
    const regions = this.evaluate(expression.item, start, state, true);
    const results = [];
    for (const region of regions) {
      const key = `${expression.language}|${start}|${region.end}`;
      let outcome = this.embedMemo.get(key);
      if (!outcome) {
        const nested = new Executor(language, this.bytes, start, region.end, this.options, this.budget, this.maxDepth - this.depth);
        nested.expectations = this.expectations;
        outcome = nested.run(language.start);
        this.embedMemo.set(key, outcome);
      }
      if (!outcome.ok) {
        for (const expectation of outcome.expected) this.fail(outcome.farthest, expectation);
        continue;
      }
      const child = { type: 'embed', language: expression.language, start, end: region.end, root: outcome.root, program: language };
      results.push(makeResult(region.end, region.state, inToken ? NO_CHILDREN : [...leaves, child], region.dynamic));
    }
    return results;
  }

  // Records that the parse requests `item`, a literal or a rule a scanner's
  // `expected` may ask about, at `position` (see `Expectations`).
  requestItem(item, position) {
    const id = item.kind === 'ref' ? this.program.expectedReferences.get(item.name) : this.program.expectedTerminals.get(item);
    if (id !== undefined) this.expectations.request(position, id);
  }

  /** A rule call or an external token, memoized with left-recursion growth. */
  reference(name, position, state, inToken) {
    if (!inToken && this.program.expectedReferences.size > 0) {
      const id = this.program.expectedReferences.get(name);
      if (id !== undefined) this.expectations.request(position, id);
    }
    if (this.program.externalTokens.has(name)) return this.scannerToken(name, position, state, inToken);
    const rule = this.program.rules.get(name);
    const mode = state.modes[state.modes.length - 1];
    if (rule.modes && !rule.modes.includes(mode)) {
      this.fail(position, rule.nodeKind);
      return [];
    }
    // While repairing, a call made quietly (where nothing is repaired) or
    // after a MISSING leaf at its offset is memoized apart from the same call
    // made in the open.
    const quiet = !this.repairPoints ? '' : this.suppressed > 0 ? '|quiet' : this.chained.has(position) ? '|chained' : '';
    const key = `${rule.index}|${position}|${state.key}|${inToken ? 1 : 0}${quiet}`;
    const known = this.memo.get(key);
    if (known) {
      known.parents?.add(this.callStack[this.callStack.length - 1] ?? null);
      if (!known.evaluating) return known.results;
      // Left recursion: answer with the current seed and mark every call
      // between the two as depending on it, so none of them is memoized.
      known.leftRecursive = true;
      for (let index = this.callStack.length - 1; index >= 0 && this.callStack[index] !== known; index -= 1) {
        this.callStack[index].involved = true;
      }
      return known.seed;
    }
    // Under keyword lexing, the calls it was made from (`null` for none),
    // where it began and whether it builds a node: the parse states of a
    // keyword matched in it (see `KeywordLexing`).
    const entry = { evaluating: true, leftRecursive: false, involved: false, seed: [], results: null, parents: null, position, rule };
    if (this.keywords) Object.assign(entry, { parents: new Set([this.callStack[this.callStack.length - 1] ?? null]), builds: rule.kind === 'normal', nodeKind: rule.nodeKind, name });
    this.retain(1);
    this.memo.set(key, entry);
    this.callStack.push(entry);
    this.depth += 1;
    try {
      if (this.depth > this.maxDepth) throw new NestingTooDeep();
      let results = this.ruleBody(rule, position, state, inToken);
      if (entry.leftRecursive) results = this.grow(entry, rule, position, state, inToken, results);
      this.retain(results.length);
      entry.results = results;
    } finally {
      this.depth -= 1;
      this.callStack.pop();
      entry.evaluating = false;
    }
    if (entry.involved || this.memo.size > this.memoLimit) this.memo.delete(key);
    return entry.results;
  }

  grow(entry, rule, position, state, inToken, first) {
    if (this.peg) {
      let best = first[0];
      while (best) {
        entry.seed = [best];
        const [next] = this.ruleBody(rule, position, state, inToken);
        if (!next || next.end <= best.end) break;
        best = next;
      }
      return best ? [best] : [];
    }
    // The seed grows while a pass reaches a new end or settles an end on
    // another tree: a left operand ranked anew (Rust's `impl A + B + C`,
    // where `bounded_type` over `impl A + B` outranks `impl` over `A + B`)
    // changes the trees grown from it, so the next pass grows them again; at
    // most one pass per end changes a tree, for an order that is not
    // transitive. A pass grows only the results the pass before added or
    // changed: the trees grown from the others are already merged, so a
    // chain of n operators takes n passes of one seed each, not n of n.
    // A first pass without results was made with the empty seed a pass
    // would grow from: there is nothing to grow. A result grown by no width,
    // whose left operand already ends where it ends, is kept but not grown
    // again: the zero-width operands a repair inserts would otherwise grow
    // it once per pass while each changes the scanner state (Lean's
    // `elab "a" : term => (`, an application of a repaired `do` block that
    // pushes one more layout indent with every argument).
    if (first.length === 0) return first;
    // The left operand is the first part of the node grown, under the nodes
    // that only wrap it (Solidity's visible `expression` around a
    // `binary_expression`).
    const grownByNoWidth = (result) => {
      let node = result.children.find((child) => !isTrivia(child));
      for (let parts; node?.type === 'node' && (parts = node.children.filter((child) => !isTrivia(child))).length === 1 && parts[0].type === 'node';) [node] = parts;
      const left = node?.type === 'node' ? node.children.find((child) => !isTrivia(child)) : null;
      return left !== null && left !== undefined && left.end === result.end && node.end === result.end;
    };
    let current = new Map(first.map((result) => [resultKey(result), result]));
    let seed = [...current.values()];
    for (let settled = 0; ;) {
      entry.seed = seed;
      const merged = new Map(current);
      let changed = false;
      const renewed = new Set();
      for (const result of this.ruleBody(rule, position, state, inToken)) {
        const key = resultKey(result);
        const existing = merged.get(key);
        if (existing && sameChildren(existing.children, result.children)) {
          merged.set(key, result);
          continue;
        }
        addResult(merged, result, this.longestTokens, this.settling);
        const kept = merged.get(key);
        if (kept === existing) continue;
        // A tie with a tree of an earlier pass is the ambiguity the rule body
        // marks when both trees meet in one pass.
        if (kept !== result) merged.set(key, this.ambiguousResult(rule, existing, inToken));
        if (!grownByNoWidth(merged.get(key))) renewed.add(key);
        if (existing && merged.get(key) === result) changed = true;
      }
      const grew = merged.size > current.size;
      current = merged;
      if (!grew && (!changed || ++settled > current.size)) break;
      seed = [...renewed].map((key) => current.get(key));
    }
    return [...current.values()];
  }

  // `result` of `rule` marked ambiguous where ruleBody marks it: on the node
  // the rule builds, else on the result, unless a conflict declares the
  // silent rule's ambiguity.
  ambiguousResult(rule, result, inToken) {
    if (rule.kind === 'silent' || inToken) {
      if (rule.kind === 'silent' && this.program.conflicts.has(rule.nodeKind)) return result;
      return copyResult(result, { ambiguous: true });
    }
    return copyResult(result, {
      children: result.children.map((child) => (child.type === 'node' ? copyNode(child, { ambiguous: true }) : child)),
    });
  }

  ruleBody(rule, position, state, inToken) {
    if (rule.kind === 'token' || rule.kind === 'atomic') {
      let { end: start, leaves } = this.terminalStart(position, state, inToken);
      // An extra token is the token of a rule that names it where the rule
      // asks for it: a tree-sitter parser takes a token as an extra only in
      // a parse state with no action on it (GraphQL's `comma`, an extra that
      // ends a `variable_definition` and an `object_field`).
      if (this.lexing && !inToken && leaves.length > 0) {
        const at = leaves.findIndex((leaf) => leaf.type === 'token' && leaf.kind !== null && this.program.rules.get(leaf.kind) === rule);
        if (at >= 0) [start, leaves] = [leaves[at].start, leaves.slice(0, at)];
        // A token rule that matches a separator before it takes it, as a
        // token does (see `tokenBeforeExtra`): Make's `raw_line` of a define
        // directive keeps its indentation.
        else if (rule.kind === 'token') ({ end: start, leaves } = this.tokenBeforeExtra(rule.expression, start, leaves, state));
      }
      let results = this.quietly(() => this.evaluate(rule.expression, start, state, true));
      if (rule.kind === 'token' || this.peg) results = results.length > 0 ? [longestResult(results)] : [];
      const built = [];
      for (const result of results) {
        const leaf = { type: 'token', kind: rule.nodeKind, start, end: result.end };
        if (this.keywords && !inToken) {
          if (this.keywords.outranks(leaf)) continue;
          leaf.lexed = rule.nodeKind;
        }
        const acted = this.runAction(rule, result, leaf, start);
        if (!acted) continue;
        const joined = inToken ? null : this.joinedStart(start, leaves);
        const children = inToken ? NO_CHILDREN : [...joined.leaves, joined.start === start ? leaf : { ...leaf, start: joined.start }];
        built.push(copyResult(acted, { children, precedence: null, ambiguous: false }));
      }
      if (built.length > 0) return built;
      this.fail(start, rule.nodeKind);
      if (inToken) return built;
      return this.elementFailed(start, leaves, state, { kind: rule.nodeKind }, rule, (cursor) => this.ruleBody(rule, cursor, state, false));
    }
    const results = this.evaluate(rule.expression, position, state, inToken);
    const built = [];
    // A silent rule builds no node to carry an ambiguity, so one inside a
    // silent rule a conflict declares is expected there, as on a node.
    const expected = rule.kind === 'silent' && this.program.conflicts.has(rule.nodeKind);
    for (const result of results) {
      if (rule.kind === 'silent' || inToken) {
        const scratch = { type: 'node', kind: rule.nodeKind, start: position, end: result.end, children: result.children };
        let acted = this.runAction(rule, result, scratch, position);
        if (!acted) continue;
        // An item a silent rule reduces alone records it, for the conflict
        // with a shift where the item is not reduced (see `preferredTokens`
        // and `childParting`).
        if (!inToken) acted = reducedAlone(acted, rule.nodeKind, this.program.rankedSilent?.has(rule.nodeKind), this.program.conflicts.has(rule.nodeKind));
        if (!(expected && acted.ambiguous) && !acted.tail) {
          built.push(acted);
          continue;
        }
        // The rule is one part of the rule that refers to it, which reduces
        // with the precedence around that part, not inside it; a token or a
        // node the rule ends with keeps the precedence the rule reduces with,
        // for the conflict with a shift after it (see `loneReduction`).
        const changes = { ambiguous: expected ? false : acted.ambiguous, tail: null };
        const meaningful = acted.tail ? acted.children.filter((child) => !isTrivia(child)) : [];
        const last = meaningful[meaningful.length - 1];
        if (last?.type === 'token' && !samePrecedence(last.reduced ?? last.precedence, acted.tail)) {
          changes.children = acted.children.map((child) => (child === last ? { ...child, reduced: acted.tail } : child));
        } else if (last?.type === 'node' && !samePrecedence(last.closes ?? null, acted.tail)) {
          changes.children = acted.children.map((child) => (child === last ? copyNode(child, { closes: acted.tail }) : child));
        }
        built.push(copyResult(acted, changes));
        continue;
      }
      const node = shareChildren({
        type: 'node', kind: rule.nodeKind, rule: rule.nodeKind, start: position, end: result.end,
        precedence: result.precedence, tail: result.tail, ambiguous: result.ambiguous,
      }, result);
      if (result.before !== undefined) node.before = result.before;
      // The dynamic precedence of the node's subtree, for the stack sums
      // `leadingDynamic` reads.
      if (result.dynamic !== 0) node.dynamic = result.dynamic;
      const acted = this.runAction(rule, result, node, position);
      if (acted) built.push(copyResult(acted, { children: [node], tail: null, ambiguous: false }));
    }
    return built.map(closed);
  }

  // Runs the rule's action over one result: the node is fresh, so attributes
  // and a new kind are set in place; `fail` or a failed operation drops the result.
  runAction(rule, result, node, from) {
    if (!rule.action) return result;
    const working = workingState(result.state);
    const start = node.type === 'token' ? node.start : contentStart(node.children, from);
    const children = node.children ?? NO_CHILDREN;
    // A field name selects the direct children captured under it, or else
    // the direct children of that kind.
    const named = (field) => {
      const meaningful = children.filter((child) => !isTrivia(child));
      const captured = meaningful.filter((child) => child.field === field);
      return captured.length > 0 ? captured : meaningful.filter((child) => child.kind === field);
    };
    const attributeOf = (child, name) => {
      const value = child.attributes?.[name];
      if (value === undefined) throw new OperationFailed();
      return value;
    };
    const machine = {
      ...this.valueMachine(copyResult(result, { children }), start, working),
      matched: () => this.text(start, result.end),
      column: () => columnOf(this.bytes, start, this.begin),
      attribute: (field, name) => {
        const [child] = named(field);
        if (!child) throw new OperationFailed();
        return attributeOf(child, name);
      },
      attributes: (field, name) => named(field).map((child) => attributeOf(child, name)),
      fieldText: (field) => {
        const [child] = named(field);
        if (!child) throw new OperationFailed();
        return this.text(child.type === 'node' ? contentStart(child.children, child.start) : child.start, child.end);
      },
      setAttribute: (name, value) => {
        node.attributes = { ...node.attributes, [name]: value };
      },
      buildNode: (kind) => {
        node.kind = kind;
      },
    };
    try {
      runStatements(rule.action, machine);
    } catch (error) {
      if (error instanceof OperationFailed) return null;
      throw error;
    }
    return copyResult(result, { state: settleState(working) });
  }

  // An external scanner run for one requested token: a cursor over the
  // input, a token start that `skip` moves, and an optional end `mark`.
  scannerToken(name, position, state, inToken, rawStart = false) {
    const { end: start, leaves } = rawStart ? { end: position, leaves: NO_CHILDREN } : this.terminalStart(position, state, inToken);
    const scanner = this.program.scanners.get(name);
    const context = inToken || rawStart ? (this.scanContext ?? start) : position;
    const key = scanner.consults ? `${name}|${start}|${context}|${state.key}` : `${name}|${start}|${state.key}`;
    let scanned = this.scannerMemo.get(key);
    if (scanned === undefined) {
      scanned = this.runScanner(name, start, state, context);
      this.scannerMemo.set(key, scanned);
      // A trace, off unless a probe sets the array: every scanner run, the
      // token it was asked for and what it scanned.
      globalThis.__scanTrace?.push([name, start, context, scanned]);
    }
    if (!scanned) {
      this.fail(start, name);
      if (inToken) return [];
      return this.elementFailed(start, leaves, state, { kind: name }, name, null);
    }
    const children = inToken ? NO_CHILDREN : [
      ...leaves,
      ...scanned.skipped,
      { type: 'token', kind: name, start: scanned.tokenStart, end: scanned.end, scanned: true },
    ];
    return [makeResult(scanned.end, scanned.state, children)];
  }

  runScanner(name, start, state, context) {
    const scanner = this.program.scanners.get(name);
    const working = workingState(state);
    const skipped = [];
    let cursor = start;
    let tokenStart = start;
    let markPosition = null;
    const match = (expression) => this.quietly(() => longestResult(this.evaluate(expression, cursor, state, true))?.end ?? -1);
    const machine = {
      state: working,
      requested: name,
      step: () => this.step(),
      matched: () => this.text(tokenStart, cursor),
      column: () => columnOf(this.bytes, cursor, this.begin),
      atEnd: () => cursor >= this.end,
      expected: (item) => this.expectations.holds(context, this.program.expectedItems.get(item)),
      lookahead: (expression) => match(expression) >= 0,
      advance: () => {
        if (cursor >= this.end) throw new OperationFailed();
        cursor += decodeAt(this.bytes, cursor, this.end).length;
      },
      consume: (expression) => {
        const end = match(expression);
        if (end < 0) throw new OperationFailed();
        cursor = end;
      },
      skip: (expression) => {
        const end = match(expression);
        if (cursor !== tokenStart || end < 0) throw new OperationFailed();
        if (end > cursor) skipped.push({ type: 'token', kind: null, start: cursor, end, trivia: true });
        cursor = end;
        tokenStart = end;
      },
      mark: () => {
        markPosition = cursor;
      },
    };
    let signal;
    try {
      signal = runStatements(scanner.operations, machine);
    } catch (error) {
      if (error instanceof OperationFailed) return null;
      throw error;
    }
    if (!signal || signal.emit !== name) return null;
    const end = markPosition ?? cursor;
    if (end < tokenStart) return null;
    return { tokenStart, end, skipped, state: settleState(working) };
  }

  /**
   * Parses the whole range from `startRule`. Returns `{ ok, root }` or
   * `{ ok: false, farthest, expected, elementFarthest, partial }`; resource
   * limits throw. While repairing, `partial` is the root of the result that
   * reaches farthest, with the rest of the input as an ERROR leaf, or of the
   * cheapest complete result when the round asks for one more repair point.
   */
  run(startRule) {
    const results = this.reference(startRule, this.begin, INITIAL_STATE, false);
    const complete = [];
    let partial = null;
    for (const result of results) {
      const trailing = this.skipTrivia(result.end, result.state);
      if (trailing.end === this.end) {
        complete.push({ result, trailing: trailing.leaves });
        continue;
      }
      this.fail(trailing.end, 'end of input');
      if (!this.repairPoints) continue;
      const rest = [...trailing.leaves, ...this.restLeaves(trailing.end, result.state)];
      const repaired = { result: copyResult(result, { cost: result.cost + this.end - trailing.end }), trailing: rest, rest: trailing.end };
      if (this.repairPoints.has(trailing.end)) complete.push(repaired);
      else {
        if (trailing.end > this.elementFarthest) this.elementFarthest = trailing.end;
        if (!partial || trailing.end > partial.end || (trailing.end === partial.end && repaired.result.cost < partial.repaired.result.cost)) {
          partial = { end: trailing.end, cost: result.cost, repaired };
        }
      }
    }
    if (complete.length === 0) {
      // A round that repairs and completes nothing asks for the offset the
      // result that reached farthest stopped at as its next repair point,
      // before any farther element that failed: past that offset no result
      // of the start rule went on.
      const stopped = partial && this.repairPoints?.size > 0 && !this.repairPoints.has(partial.end);
      const failed = { ok: false, farthest: this.farthest, expected: [...this.expected].sort(), elementFarthest: stopped ? partial.end : this.elementFarthest };
      if (this.repairPoints) failed.partial = partial ? this.root(startRule, partial.repaired, false) : this.errorRoot(startRule);
      return failed;
    }
    // The complete results end apart, before their trailing trivia, so they
    // are ranked here as addResult ranks results with one end: the lower
    // cost, then the grammar's settling steps. A tie is an ambiguity when the
    // settling ends in `ambiguity`; repaired results of equal cost are not
    // ambiguities.
    let chosen = complete[0];
    let tied = false;
    for (const candidate of complete.slice(1)) {
      const order = this.completeOrder(candidate, chosen);
      if (order > 0) {
        chosen = candidate;
        tied = false;
      } else if (order === 0) {
        tied = true;
      }
    }
    if (this.separatorTaken.size > 0) {
      chosen = { ...chosen, result: copyResult(chosen.result, { children: this.widenTokens(chosen.result.children, []) }), trailing: this.widenTokens(chosen.trailing, []) };
    }
    const root = this.root(startRule, chosen, tied && this.settling.ambiguity && chosen.result.cost === 0);
    // When the cheapest complete result takes the rest of the input as ERROR
    // at a repair point, a result that reached past that point without
    // completing costs at least one more repair, at its end. While that could
    // still complete for less, the round asks for its end as the next repair
    // point, so a later error is repaired where it is and not by skipping all
    // the input after an earlier one; the complete result stands when the
    // rounds end.
    if (chosen.rest !== undefined && partial && partial.end > chosen.rest && partial.cost + 1 < chosen.result.cost) {
      return { ok: false, farthest: this.farthest, expected: [...this.expected].sort(), elementFarthest: partial.end, partial: root };
    }
    return { ok: true, root };
  }

  // A tree-sitter lexer skips a separator only where no valid token goes on
  // with it: where one does, the token it lexes starts at the separator,
  // whichever token that is (Make's ` endef` after a `raw_line`, which could
  // take the blank). So a token after separators that a token lexed there
  // took (see `separatorTaken`) starts at the first of them, taking them,
  // and so does each node it begins: where that token was lexed in the nodes
  // the token is in, `ancestors`, or in nodes below them begun at the
  // separator (Make's shell text of a recipe line). A token lexed in other
  // nodes was lexed in another parse state (Make's `text` of a variable
  // assignment after `VPATH =`, Rocq's comment text after `*)`).
  widenTokens(children, ancestors) {
    let out = null;
    children.forEach((child, index) => {
      let next = child;
      if (child.type === 'node') {
        const inner = this.widenTokens(child.children, [...ancestors, child.rule]);
        if (inner !== child.children) {
          const first = inner.find((item) => !isTrivia(item));
          next = { ...child, children: inner, start: first && first.start < child.start ? first.start : child.start };
        }
      } else if (child.type === 'token' && !isTrivia(child)) {
        let from = index;
        while (from > 0 && isSeparator(children[from - 1]) && children[from - 1].end === children[from].start) from -= 1;
        const taken = children.slice(from, index).findIndex((leaf) => [...this.separatorTaken.get(leaf.start)?.values() ?? []].some((chain) =>
          chain.length >= ancestors.length && ancestors.every((name, at) => chain[at][0] === name) && chain.slice(ancestors.length).every(([, position]) => position >= leaf.start)));
        if (taken >= 0) {
          out ??= children.slice(0, index);
          out.splice(out.length - (index - from - taken), index - from - taken);
          // An anonymous token keeps its name, an anonymous alias of its text.
          const kind = child.kind ?? `'${textOf(this.bytes, child.start, child.end)}`;
          out.push({ ...child, kind, start: children[from + taken].start });
          return;
        }
      }
      if (out) out.push(next);
      else if (next !== child) out = [...children.slice(0, index), next];
    });
    return out ?? children;
  }

  // The rest of the input from `start` as an ERROR leaf. As tree-sitter keeps
  // the white space at the end of the input out of an ERROR node, the
  // separators that end the input follow the leaf; the rest costs the same.
  restLeaves(start, state) {
    let at = this.end;
    while (at > start && ASCII_WHITE_SPACE.has(this.bytes[at - 1])) at -= 1;
    if (at > start && at < this.end) {
      const tail = this.skipTrivia(at, state);
      if (tail.end === this.end && tail.leaves.every(isSeparator)) return [{ type: 'error', start, end: at }, ...tail.leaves];
    }
    return [{ type: 'error', start, end: this.end }];
  }

  // 1 when the complete result `a` is preferred to `b`, -1 when `b` is, 0 on a tie.
  completeOrder(a, b) {
    if (a.result.cost !== b.result.cost) return a.result.cost < b.result.cost ? 1 : -1;
    const whole = ({ result, trailing }) => ({ children: [...result.children, ...trailing] });
    return settledOrder({ ...whole(a), dynamic: a.result.dynamic }, { ...whole(b), dynamic: b.result.dynamic }, this.settling, this.longestTokens);
  }

  root(startRule, { result, trailing }, several) {
    const ambiguous = several || result.ambiguous;
    const [only] = result.children;
    if (result.children.length === 1 && only.type === 'node') {
      return { ...only, end: this.end, children: [...only.children, ...trailing], ambiguous: only.ambiguous || ambiguous };
    }
    const kind = this.program.rules.get(startRule).nodeKind;
    return {
      type: 'node', kind, rule: kind, start: this.begin, end: this.end,
      children: [...result.children, ...trailing], ambiguous,
    };
  }

  // The root when the start rule matches nothing even with repairs.
  errorRoot(startRule) {
    const kind = this.program.rules.get(startRule).nodeKind;
    return {
      type: 'node', kind, rule: kind, start: this.begin, end: this.end,
      children: [{ type: 'error', start: this.begin, end: this.end }], ambiguous: false,
    };
  }
}

// Whether a token is a keyword, a literal closed by lookaheads (`(seq
// (literal typedef) (not (ref word_characters)))`).
function isKeyword(expression) {
  if (expression.kind !== 'seq' || expression.items.length < 2 || expression.items[0].kind !== 'literal') return false;
  return expression.items.slice(1).every((item) => item.kind === 'not' || item.kind === 'and');
}

// The texts of the keywords of `rules` (see `isKeyword`), each by its UTF-8
// bytes as one character a byte.
function keywordTexts(rules) {
  const texts = new Map();
  const pending = [...rules.values()].map(({ expression }) => expression);
  while (pending.length > 0) {
    const expression = pending.pop();
    if (isKeyword(expression)) texts.set(String.fromCharCode(...new TextEncoder().encode(expression.items[0].value)), expression.items[0].value);
    if (expression.item) pending.push(expression.item);
    if (expression.items) pending.push(...expression.items);
  }
  return texts;
}

/**
 * Keyword lexing under `(matching longest)`. A tree-sitter lexer lexes a
 * keyword wherever the parse state admits it, before any parse goes on, so a
 * token rule's leaf over the same text (an identifier `typedef`) is not taken
 * there even when only it would let the parse go on. A parse records the
 * spans where a keyword token matched (`matched`), each with the rule calls it
 * matched in; a leaf of a token rule in its tree over such a span (`lexed`,
 * the rule's kind, which an alias keeps) that the keyword outranks (see
 * `tokenConflict`) makes the span keyword-only (`only`), and the input is
 * parsed again, where no token rule takes a keyword-only span the keyword
 * outranks it on. The spans only grow, so the reparses end.
 *
 * The keyword counts only where it matched in the tree's parse state: in a
 * call made, through some chain of calls up to the first, by calls that build
 * nodes each beginning a node of the tree that goes on past the span, as the
 * items a parser has in progress there. A chain through a call that began no
 * such node lexed the text before the span otherwise (Rust's `m!('"')`, whose
 * token tree also takes `'` alone and a string to the next `"`, where a later
 * `_` type is a keyword `_` token): the tree's parse never reached that state.
 * A call's result is shared by every call that made it, so every chain counts.
 *
 * An immediate token a literal (`(immediateToken (literal [))`) outranks the
 * plain literal of the same text, which a lexer so lexes only where no
 * immediate one is valid (Lean's `foo[1:2:3]`, whose `[` opens a subscript,
 * not a range applied to `foo`): the spans where an immediate literal matched
 * (`matchedImmediate`) become immediate-only (`immediates`) alike where the
 * tree took the plain literal (a `plain` leaf), whose parse state the
 * immediate one matched in.
 *
 * An immediate token that matched separator text alone at an offset
 * (`matchedSeparators`, Make's line break that ends a rule) is lexed there
 * where the tree skipped a separator in its parse state: a tree-sitter lexer
 * that completed it takes no separator transition after it, so the offset
 * skips no separator (`separators`) when the input is parsed again (Make's
 * `a:\nb\n`, whose rule ends at the line break, with no `b` prerequisite).
 */
export class KeywordLexing {
  constructor(tokens, program = null) {
    this.tokens = tokens;
    this.program = program;
    this.leads = null;
    this.only = new Set();
    this.matched = new Map();
    this.immediates = new Set();
    this.matchedImmediate = new Map();
    this.separators = new Set();
    this.matchedSeparators = new Map();
    this.joined = new Map();
    this.matchedJoins = new Map();
  }

  /** Records a separator at `position` a token in the rule call `call` begins with, read on with up to `reach`. */
  matchJoin(position, reach, call) {
    const join = this.matchedJoins.get(position);
    if (!join) this.matchedJoins.set(position, { reach, calls: new Set([call]) });
    else {
      join.reach = Math.max(join.reach, reach);
      join.calls.add(call);
    }
  }

  /** Records an immediate token of separator text alone matched at `position` in the rule call `call`. */
  matchSeparator(position, call) {
    const calls = this.matchedSeparators.get(position);
    if (!calls) this.matchedSeparators.set(position, new Set([call]));
    else calls.add(call);
  }

  /** Records a keyword or `immediate` literal token matched over `span` in the rule call `call`. */
  match(span, call, immediate = false) {
    const matched = immediate ? this.matchedImmediate : this.matched;
    const calls = matched.get(span);
    if (!calls) matched.set(span, new Set([call]));
    else calls.add(call);
  }

  // Whether an immediate token outranks the plain literal over a span.
  immediateOnly(start, end) {
    return this.immediates.has(`${start}|${end}`);
  }

  // Whether a keyword-only span's keyword outranks a token rule's leaf over it.
  outranks(leaf) {
    return this.only.has(`${leaf.start}|${leaf.end}`) && this.outranksAt(leaf);
  }

  // Whether a keyword over the span of a token rule's leaf outranks it.
  outranksAt(leaf) {
    return tokenConflict({ type: 'token', kind: null, start: leaf.start, end: leaf.end }, leaf, this.tokens) > 0;
  }

  // The FIRST sets of the program's rules, the texts of its keywords and the
  // byte length of the longest, computed once (see `inParseState`), or null
  // without a program.
  leadsOf() {
    if (!this.program) return null;
    if (!this.leads) {
      const facts = grammarFacts(this.program);
      facts.first ??= firstSets(facts.rules);
      const keywords = keywordTexts(this.program.rules);
      this.leads = { first: facts.first, keywords, longest: Math.max(0, ...[...keywords.keys()].map(({ length }) => length)) };
    }
    return this.leads;
  }

  /** Marks the spans where `root` took a token rule's leaf over a keyword; true when one is new. */
  conflicts(root) {
    let found = false;
    let reach = null;
    const pending = [root];
    while (pending.length > 0) {
      const node = pending.pop();
      if (node.type === 'node') {
        for (let index = node.children.length - 1; index >= 0; index -= 1) pending.push(node.children[index]);
        continue;
      }
      if (node.trivia && node.kind == null) {
        const join = this.matchedJoins.get(node.start);
        if (join && !this.joined.has(node.start)) {
          reach ??= treeReach(root, this.leadsOf(), this.tokens.bytes);
          const seen = new Set();
          if ([...join.calls].some((call) => builtAround(call, node, reach) && inParseState(call, node, reach, seen, this.leads))) {
            this.joined.set(node.start, join.reach);
            found = true;
          }
        }
        const calls = this.matchedSeparators.get(node.start);
        if (!calls || this.separators.has(node.start)) continue;
        reach ??= treeReach(root, this.leadsOf(), this.tokens.bytes);
        const seen = new Set();
        if (![...calls].some((call) => builtAround(call, node, reach) && inParseState(call, node, reach, seen, this.leads))) continue;
        this.separators.add(node.start);
        found = true;
        continue;
      }
      const span = `${node.start}|${node.end}`;
      let [matched, only] = [this.matched, this.only];
      if (node.plain) [matched, only] = [this.matchedImmediate, this.immediates];
      else if (node.lexed === undefined) continue;
      if (!matched.has(span) || only.has(span)) continue;
      if (!node.plain && !this.outranksAt({ type: 'token', kind: node.lexed, start: node.start, end: node.end })) continue;
      reach ??= treeReach(root, this.leadsOf(), this.tokens.bytes);
      const seen = new Set();
      if (![...matched.get(span)].some((call) => inParseState(call, node, reach, seen, this.leads))) continue;
      only.add(span);
      found = true;
    }
    this.matched = new Map();
    this.matchedImmediate = new Map();
    this.matchedSeparators = new Map();
    this.matchedJoins = new Map();
    return found;
  }
}

// The starts of the leaves of a tree that are not trivia, in order, and for
// each the farthest end of a node that begins with that leaf. A token the
// external scanner scanned of no width does not count: the parser scanned it
// before its lexer, in the parse state the next leaf is lexed in (JavaScript's
// automatic semicolon before a line break and a keyword `class`). `last`
// holds the end of each leaf, by its start, `rules` the farthest end of a
// node of each rule, by its start and rule, and `keywords` the text of each
// token leaf of the input `bytes` that is a keyword of the `leads` (see
// `KeywordLexing.leadsOf`), by its start. `widthless` holds the offsets of
// the tokens the external scanner scanned of no width.
function treeReach(root, leads = null, bytes = null) {
  const starts = [];
  const ends = new Map();
  const last = new Map();
  const rules = new Map();
  const keywords = new Map();
  const widthless = new Set();
  let open = [];
  const pending = [root];
  while (pending.length > 0) {
    const node = pending.pop();
    if (isTrivia(node)) continue;
    if (node.type !== 'node' && node.start === node.end && scannedToken(node) === 1) {
      widthless.add(node.start);
      continue;
    }
    if (node.type === 'node') {
      open.push(node.end);
      const key = `${node.start}|${node.rule}`;
      rules.set(key, Math.max(rules.get(key) ?? -1, node.end));
      for (let index = node.children.length - 1; index >= 0; index -= 1) pending.push(node.children[index]);
      continue;
    }
    starts.push(node.start);
    last.set(node.start, Math.max(last.get(node.start) ?? -1, node.end));
    const text = node.type === 'token' && node.kind == null && leads && bytes && node.end - node.start <= leads.longest
      ? leads.keywords.get(String.fromCharCode(...bytes.subarray(node.start, node.end))) : undefined;
    if (text !== undefined) keywords.set(node.start, text);
    if (open.length > 0) ends.set(node.start, Math.max(ends.get(node.start) ?? -1, ...open));
    open = [];
  }
  return { starts, ends, last, rules, keywords, widthless };
}

// Whether a keyword matched in `call` was in the parse state of the tree's
// `leaf` over its span (see `KeywordLexing`): whether some chain of the calls
// that made it, up to the first, holds no call that builds a node the tree
// does not have in progress there. `seen` holds the calls already searched,
// and `leads` the FIRST sets of the rules, or null.
function inParseState(call, leaf, reach, seen, leads = null) {
  const pending = [call];
  while (pending.length > 0) {
    const current = pending.pop();
    if (current === null) return true;
    if (seen.has(current)) continue;
    seen.add(current);
    if (current.builds && current.position < leaf.start) {
      const low = firstStart(reach.starts, current.position);
      const first = reach.starts[low];
      if (first !== undefined && first < leaf.start && (reach.ends.get(first) ?? -1) < leaf.end) continue;
      // A call that begins inside a leaf of the tree lexed that leaf's text
      // otherwise (Rocq's `[` of a list where the tree has the token `=[`).
      const previous = reach.starts[low - 1];
      if (previous !== undefined && reach.last.get(previous) > current.position) continue;
      // A node of the call's rule that the tree closes before a leaf
      // preceding `leaf` is no longer in progress there: the call matched
      // the keyword on a parse that read that leaf otherwise (Rocq's
      // `match ... end > 0 end`, where the first `end` closes the match and
      // the second is an identifier), or by a token the external scanner
      // scanned of no width before it, which the parser shifted before it
      // lexed `leaf` (TypeScript's automatic semicolon before a line break
      // and an identifier `as`).
      const own = reach.rules.get(`${current.position}|${current.nodeKind}`);
      if (own !== undefined && own < leaf.start && (reach.starts[firstStart(reach.starts, own)] < leaf.start || reach.widthless.has(own))) continue;
      // A call whose rule begins with no keyword the tree took where the
      // call began read that keyword's text otherwise, as a token a lexer
      // never lexes where the keyword is valid (TypeScript's identifier
      // `return` before an `as` expression, where the tree's `return as`
      // returns the identifier `as`).
      const keyword = first !== undefined && first < leaf.start ? reach.keywords.get(first) : undefined;
      if (keyword !== undefined && leads?.first.get(current.name)?.has(`literal ${keyword}`) === false) continue;
    }
    pending.push(...current.parents);
  }
  return false;
}

// Whether the node the rule call `call` builds, or the first call that
// made it that builds one, through some chain, is a node of the tree around
// `leaf` (see `KeywordLexing`): a call whose node the tree lacks lexed in a
// parse state the tree never had (Make's `list` of a target `-include`,
// where the tree has the `-include` of a directive).
function builtAround(call, leaf, reach) {
  const pending = [call];
  const seen = new Set();
  while (pending.length > 0) {
    const current = pending.pop();
    if (current === null) return true;
    if (seen.has(current)) continue;
    seen.add(current);
    if (current.builds) {
      if ((reach.rules.get(`${current.position}|${current.nodeKind}`) ?? -1) > leaf.start) return true;
      continue;
    }
    pending.push(...current.parents);
  }
  return false;
}

// The index of the first of the ascending `starts` at or after `offset`.
function firstStart(starts, offset) {
  let [low, high] = [0, starts.length];
  while (low < high) {
    const middle = (low + high) >> 1;
    if (starts[middle] < offset) low = middle + 1;
    else high = middle;
  }
  return low;
}

/** The expectation a failed terminal records. */
export function expectationOf(expression) {
  switch (expression.kind) {
    case 'literal': return quoteText(expression.value);
    case 'literalInsensitive': return `${quoteText(expression.value)}i`;
    case 'charRange': return `${quoteText(expression.start)}..${quoteText(expression.end)}`;
    case 'charClass': return 'character class';
    case 'byteClass': return 'byte class';
    case 'any': return 'any character';
    case 'regex': return `/${expression.value}/`;
    default: return expression.kind;
  }
}
