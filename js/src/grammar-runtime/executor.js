// The native grammar executor: it interprets a loaded program (load.js) over
// a byte string and builds the lossless concrete syntax tree. Generalized
// matching keeps every result an expression can produce, deduplicated by end
// offset and parser state; PEG matching keeps at most one. Rule calls are
// memoized, left recursion grows a seed to a fixpoint, and the nesting depth,
// the step count and the memo size are bounded, so a hostile input ends in a
// rejection instead of a stack overflow or a runaway parse. Automatic
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

/** Thrown when the rule nesting exceeds `maxDepth`; the driver turns it into an `ERROR` root. */
export class NestingTooDeep extends Error {}

/** Thrown when a parse runs out of its step budget. */
export class StepLimitReached extends Error {}

const DEFAULT_MAX_DEPTH = 1000;
const DEFAULT_MEMO_LIMIT = 1_000_000;
const NO_CHILDREN = Object.freeze([]);

/** The step budget of one parse, shared with every embedded-language executor. */
export function stepBudget(options, length) {
  return { steps: 0, limit: options.stepLimit ?? 100_000 + 1000 * length };
}

// The repair cost of a MISSING leaf; an ERROR leaf costs the bytes it skips.
const MISSING_COST = 2;

function makeResult(end, state, children = NO_CHILDREN, dynamic = 0, cost = 0) {
  return { end, state, children, dynamic, precedence: null, ambiguous: false, cost };
}

function resultKey(result) {
  return `${result.end}|${result.state.key}`;
}

// Adds a result to a deduplicating map: of two results with the same end
// and state, the lower repair cost wins, then, under `(matching longest)`
// (when `tokens` holds the token ranks and the input bytes), the tokens a
// lexer prefers (a lexer decides them before any parse does), then the
// higher dynamic precedence; on a tie the first stays and,
// without repairs, is marked ambiguous (as a copy, since results are shared
// through the memo).
function addResult(results, result, tokens = null) {
  const key = resultKey(result);
  const existing = results.get(key);
  if (!existing || result.cost < existing.cost) {
    results.set(key, result);
    return;
  }
  if (result.cost > existing.cost) return;
  const order = tokens ? preferredTokens(result, existing, tokens) : 0;
  if (order > 0 || (order === 0 && result.dynamic > existing.dynamic)) results.set(key, result);
  else if (order === 0 && result.dynamic === existing.dynamic && existing.cost === 0 && !existing.ambiguous) results.set(key, copyResult(existing, { ambiguous: true }));
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
  const covers = (leaf, trivia) => trivia.some((item) => item.kind === null && item.start === leaf.start && leaf.end >= item.end);
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
    if (a === null || b === null) return 0;
    if (a === b) {
      skip(left);
      skip(right);
      skipped = [[], []];
    } else if (a.type === 'node' || b.type === 'node') {
      if (a.type === 'node') enter(left, a);
      if (b.type === 'node') enter(right, b);
    } else if (a.trivia || b.trivia) {
      if (a.trivia) skipped[0].push(a);
      if (b.trivia) skipped[1].push(b);
      if (a.trivia) skip(left);
      if (b.trivia) skip(right);
    } else if (a.start < b.start && covers(a, skipped[1])) {
      return 1;
    } else if (b.start < a.start && covers(b, skipped[0])) {
      return -1;
    } else if (a.end !== b.end || a.kind !== b.kind) {
      return tokenConflict(a, b, tokens);
    } else {
      skip(left);
      skip(right);
      skipped = [[], []];
    }
  }
}

// The rank of a token leaf, or null for another leaf or an unranked token.
function tokenRank(leaf, tokens) {
  if (leaf.type !== 'token') return null;
  if (leaf.kind !== null) return tokens.kinds.get(leaf.kind) ?? null;
  return tokens.literals.get(textOf(tokens.bytes, leaf.start, leaf.end)) ?? null;
}

// Two leaves that differ in end or kind: 1 when a lexer prefers `a`, -1 when
// it prefers `b`, 0 when it cannot tell. The ranks decide only between two
// tokens at one offset; otherwise the longer leaf wins.
function tokenConflict(a, b, tokens) {
  const first = a.start === b.start ? tokenRank(a, tokens) : null;
  const second = a.start === b.start ? tokenRank(b, tokens) : null;
  if (first && second && first.priority !== second.priority) return first.priority > second.priority ? 1 : -1;
  if (a.end !== b.end) return a.end > b.end ? 1 : -1;
  if (!first || !second || first === second) return 0;
  if (first.specificity !== second.specificity) return first.specificity > second.specificity ? 1 : -1;
  return first.order < second.order ? 1 : -1;
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

// A copy of a result with `changes`, its children still shared and lazy (a
// spread would flatten them).
function copyResult(result, changes) {
  const copy = {
    end: result.end, state: result.state, dynamic: result.dynamic,
    precedence: result.precedence, ambiguous: result.ambiguous, cost: result.cost,
  };
  if (!('children' in changes)) shareChildren(copy, result);
  return Object.assign(copy, changes);
}

function joinResults(left, right, inToken) {
  const joined = {
    end: right.end,
    state: right.state,
    dynamic: left.dynamic + right.dynamic,
    precedence: null,
    ambiguous: left.ambiguous || right.ambiguous,
    cost: left.cost + right.cost,
  };
  if (inToken) {
    joined.children = NO_CHILDREN;
    return joined;
  }
  const leftCount = childCount(left);
  const rightCount = childCount(right);
  if (rightCount === 0) return shareChildren(joined, left);
  if (leftCount === 0) return shareChildren(joined, right);
  return shareChildren(joined, { [CHAIN]: { left: partOf(left), right: partOf(right), length: leftCount + rightCount, flat: null } });
}

function longestResult(results) {
  let best = null;
  for (const result of results) if (!best || result.end > best.end) best = result;
  return best;
}

// The union of kind sets, or null when any is unknown.
function union(sets) {
  if (sets.some((set) => set === null)) return null;
  return new Set(sets.flatMap((set) => [...set]));
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
// text for a literal, else none.
// A literal that took the separators after it (see `beforeSeparator`) is
// still that literal, as a tree-sitter lexer names it: its leaf is an
// anonymous alias of the literal (`'\n` over `\n\n`). Any other terminal
// leaf is named by its text.
function separatorRunKind(expression, start, end) {
  return expression.kind === 'literal' && end - start > encodeText(expression.value).length ? `'${expression.value}` : null;
}

function missingOf(expression) {
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
    this.longestTokens = program.tokenRanks ? { ...program.tokenRanks, bytes } : null;
    this.depth = 0;
    this.memo = new Map();
    this.memoLimit = options.memoLimit ?? DEFAULT_MEMO_LIMIT;
    this.callStack = [];
    this.triviaMemo = new Map();
    this.operandMemo = new Map();
    // The repair points where a continuation after a MISSING leaf is open.
    this.chained = new Set();
    this.scannerMemo = new Map();
    this.embedMemo = new Map();
    this.farthest = begin;
    this.expected = new Set();
    this.suppressed = 0;
    this.view = null;
    // Automatic recovery: the offsets where a failing element is repaired,
    // and the farthest offset where an element failed without a repair.
    this.repairPoints = null;
    this.elementFarthest = -1;
    // Under `(matching longest)`, the keyword lexing of the parse (see
    // `KeywordLexing`), or none.
    this.keywords = null;
  }

  step() {
    this.budget.steps += 1;
    if (this.budget.steps > this.budget.limit) throw new StepLimitReached();
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
  // offset as an ERROR leaf.
  elementFailed(start, leaves, state, missing, retry) {
    if (this.suppressed > 0) return [];
    if (!this.repairPoints?.has(start)) {
      if (start > this.elementFarthest) this.elementFarthest = start;
      return [];
    }
    const results = [makeResult(start, state, [...leaves, { type: 'missing', start, end: start, ...missing }], 0, MISSING_COST)];
    for (let cursor = start; cursor < this.end;) {
      cursor += decodeAt(this.bytes, cursor, this.end).length;
      const found = this.quietly(() => retry(cursor));
      if (found.length === 0) continue;
      const error = { type: 'error', start, end: cursor };
      for (const result of found) {
        results.push(copyResult(result, { children: [...leaves, error, ...result.children], cost: result.cost + cursor - start }));
      }
      break;
    }
    return results;
  }

  // Skips trivia: repeatedly the longest match of any trivia expression
  // allowed in the current mode. Returns the new offset and the trivia leaves.
  skipTrivia(position, state) {
    const { trivia } = this.program;
    if (trivia.length === 0) return { end: position, leaves: NO_CHILDREN };
    const key = `${position}|${state.key}`;
    const cached = this.triviaMemo.get(key);
    if (cached) return cached;
    const mode = state.modes[state.modes.length - 1];
    const leaves = [];
    let cursor = position;
    for (;;) {
      let best = cursor;
      let bestKind = null;
      for (const item of trivia) {
        if (item.modes && !item.modes.includes(mode)) continue;
        const end = this.quietly(() => longestResult(this.evaluate(item.expression, cursor, state, true))?.end ?? -1);
        if (end > best) {
          best = end;
          bestKind = item.kind;
        }
      }
      if (best === cursor) break;
      leaves.push({ type: 'token', kind: bestKind, start: cursor, end: best, trivia: true });
      cursor = best;
    }
    const skipped = { end: cursor, leaves };
    this.triviaMemo.set(key, skipped);
    return skipped;
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
    if (inToken || !this.longestTokens) return starts;
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
  beforeSeparator(expression, start, leaves) {
    const covers = (leaf, from) => leaf.kind === null && leaf.start === from && this.matchTerminal(expression, from) >= leaf.end;
    const at = leaves.findIndex((leaf) => covers(leaf, leaf.start));
    if (at < 0) return { start, end: -1, leaves };
    let end = this.matchTerminal(expression, leaves[at].start);
    for (let index = at + 1; index < leaves.length && covers(leaves[index], end); index += 1) end = this.matchTerminal(expression, end);
    return { start: leaves[at].start, end, leaves: leaves.slice(0, at) };
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

  terminal(expression, position, state, inToken) {
    let { end: start, leaves } = this.terminalStart(position, state, inToken);
    let end = -1;
    if (this.longestTokens && leaves.length > 0) ({ start, end, leaves } = this.beforeSeparator(expression, start, leaves));
    if (end < 0) end = this.matchTerminal(expression, start);
    if (end < 0) {
      this.fail(start, expectationOf(expression));
      if (inToken) return [];
      return this.elementFailed(start, leaves, state, missingOf(expression), (cursor) => this.terminal(expression, cursor, state, false));
    }
    const children = inToken ? NO_CHILDREN : [...leaves, { type: 'token', kind: separatorRunKind(expression, start, end), start, end }];
    return [makeResult(end, state, children)];
  }

  // A leaf over the longest match of `item` in token context: token(),
  // immediateToken() and longest() alternatives build on it.
  tokenLeaf(item, start, leaves, state, inToken, kind) {
    const best = longestResult(this.evaluate(item, start, state, true));
    if (!best) return [];
    const children = inToken ? NO_CHILDREN : [...leaves, { type: 'token', kind, start, end: best.end }];
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
        return this.evaluate(expression.item, position, state, inToken).map((result) => (inToken ? result : copyResult(result, {
          children: result.children.map((child) => (isTrivia(child) ? child : { ...child, field: expression.label })),
        })));
      case 'alias': return this.alias(expression, position, state, inToken);
      case 'precedence': return this.precedence(expression, position, state, inToken);
      case 'dynamicPrecedence':
        return this.evaluate(expression.item, position, state, inToken)
          .map((result) => copyResult(result, { dynamic: result.dynamic + expression.level }));
      case 'lexicalPrecedence': return this.evaluate(expression.item, position, state, inToken);
      case 'longest': return this.longest(expression, position, state, inToken);
      case 'token': case 'immediateToken': {
        const starts = expression.kind === 'token'
          ? [this.terminalStart(position, state, inToken)]
          : this.immediateStarts(position, state, inToken);
        const found = new Map();
        const keyword = this.keywords !== null && !inToken && isKeyword(expression.item);
        for (const { end, leaves } of starts) {
          for (const result of this.tokenLeaf(expression.item, end, leaves, state, inToken, null)) {
            if (keyword) this.keywords.matched.add(`${end}|${result.end}`);
            addResult(found, result, this.longestTokens);
          }
        }
        const results = [...found.values()];
        const { end: start, leaves } = starts[0];
        if (results.length > 0 || inToken) return results;
        return this.elementFailed(start, leaves, state, missingOf(expression.item), (cursor) => this.evaluate(expression, cursor, state, false));
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
  // never built.
  continuation(item, left, inToken) {
    if (!this.repairPoints?.has(left.end)) return this.evaluate(item, left.end, left.state, inToken);
    const last = left.children[left.children.length - 1];
    if (last?.type !== 'missing' || last.start !== left.end) return this.evaluate(item, left.end, left.state, inToken);
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
  // invalid one that reached the same end first.
  sequence(items, position, state, inToken, keep = null) {
    let current = [makeResult(position, state)];
    for (const [index, item] of items.entries()) {
      const next = new Map();
      const last = index === items.length - 1;
      for (const left of current) {
        for (const right of this.continuation(item, left, inToken)) {
          const joined = joinResults(left, right, inToken);
          if (last && keep && !keep(joined)) continue;
          addResult(next, joined, this.longestTokens);
        }
      }
      current = [...next.values()];
      if (this.peg) current = current.slice(0, 1);
      if (current.length === 0) return current;
    }
    return current;
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
    for (const item of expression.items) {
      for (const result of this.evaluate(item, position, state, inToken)) addResult(results, result, this.longestTokens);
    }
    return [...results.values()];
  }

  repetition(item, min, max, position, state, inToken) {
    const join = (left, right) => joinResults(left, right, inToken);
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
    let frontier = [makeResult(position, state)];
    for (let count = 0; frontier.length > 0; count += 1) {
      if (count >= min) {
        const fresh = [];
        for (const result of frontier) {
          addResult(results, result, this.longestTokens);
          if (results.get(resultKey(result)) === result) fresh.push(result);
        }
        frontier = fresh;
      }
      if (max !== null && count >= max) break;
      const next = new Map();
      for (const left of frontier) {
        for (const right of this.continuation(item, left, inToken)) {
          if (zeroWidth(left, right)) {
            // Zero-width iterations can pad up to the minimum once.
            if (count < min) addResult(results, join(left, right), this.longestTokens);
            continue;
          }
          addResult(next, join(left, right), this.longestTokens);
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
        return copyResult(result, { children: result.children.map((child) => (child === meaningful[0] ? renamed(child, expression.name) : child)) });
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
    const { level, associativity } = expression;
    const tag = { level, associativity };
    const valid = (result) => {
      if (allowed(result)) return true;
      this.fail(result.end, 'precedence');
      return false;
    };
    const conflicts = (child, side) => {
      if (child.type !== 'node' || !child.precedence) return false;
      const inner = child.precedence.level;
      if (inner > level || (inner === level && associativity === side)) return false;
      const kinds = this.operandKinds(expression)[side];
      if (kinds === null) return true;
      const facing = child.children.filter((grandchild) => !isTrivia(grandchild));
      const edge = side === 'left' ? facing[facing.length - 1] : facing[0];
      return edge !== undefined && kinds.has(edge.kind);
    };
    const allowed = (result) => {
      const meaningful = result.children.filter((child) => !isTrivia(child));
      if (meaningful.length < 2) return true;
      return !conflicts(meaningful[0], 'left') && !conflicts(meaningful[meaningful.length - 1], 'right');
    };
    const results = inToken
      ? this.evaluate(expression.item, position, state, inToken)
      : this.filtered(expression.item, position, state, valid);
    return results.map((result) => copyResult(result, { precedence: tag }));
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
        if (!rule) return null;
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
      case 'capture': case 'precedence': case 'dynamicPrecedence':
      case 'optional': case 'repeat0': case 'repeat1': case 'repeat': return this.unitKinds(expression.item, visiting, side);
      case 'alias': return new Set([expression.name]);
      default: return null;
    }
  }

  // The results of `expression` that `keep` accepts, filtered before a
  // sequence or an unordered choice merges results of the same end and state.
  filtered(expression, position, state, keep) {
    if (expression.kind === 'seq' && expression.items.length > 0) return this.sequence(expression.items, position, state, false, keep);
    if (expression.kind === 'choice' && !expression.ordered && !this.peg) {
      this.step();
      const results = new Map();
      for (const item of expression.items) {
        for (const result of this.filtered(item, position, state, keep)) addResult(results, result, this.longestTokens);
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
      return this.elementFailed(start, leaves, state, { kind: null }, (cursor) => this.longest(expression, cursor, state, false));
    }
    const kind = best.item.kind === 'ref' ? (this.program.rules.get(best.item.name)?.nodeKind ?? best.item.name) : null;
    const children = inToken ? NO_CHILDREN : [...leaves, { type: 'token', kind, start, end: best.result.end }];
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

  /** A rule call or an external token, memoized with left-recursion growth. */
  reference(name, position, state, inToken) {
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
      if (!known.evaluating) return known.results;
      // Left recursion: answer with the current seed and mark every call
      // between the two as depending on it, so none of them is memoized.
      known.leftRecursive = true;
      for (let index = this.callStack.length - 1; index >= 0 && this.callStack[index] !== known; index -= 1) {
        this.callStack[index].involved = true;
      }
      return known.seed;
    }
    const entry = { evaluating: true, leftRecursive: false, involved: false, seed: [], results: null };
    this.memo.set(key, entry);
    this.callStack.push(entry);
    this.depth += 1;
    try {
      if (this.depth > this.maxDepth) throw new NestingTooDeep();
      let results = this.ruleBody(rule, position, state, inToken);
      if (entry.leftRecursive) results = this.grow(entry, rule, position, state, inToken, results);
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
    let current = new Map(first.map((result) => [resultKey(result), result]));
    for (;;) {
      entry.seed = [...current.values()];
      const merged = new Map(current);
      for (const result of this.ruleBody(rule, position, state, inToken)) merged.set(resultKey(result), result);
      const grew = merged.size > current.size;
      current = merged;
      if (!grew) break;
    }
    return [...current.values()];
  }

  ruleBody(rule, position, state, inToken) {
    if (rule.kind === 'token' || rule.kind === 'atomic') {
      const { end: start, leaves } = this.terminalStart(position, state, inToken);
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
        built.push(copyResult(acted, { children: inToken ? NO_CHILDREN : [...leaves, leaf], precedence: null, ambiguous: false }));
      }
      if (built.length > 0) return built;
      this.fail(start, rule.nodeKind);
      if (inToken) return built;
      return this.elementFailed(start, leaves, state, { kind: rule.nodeKind }, (cursor) => this.ruleBody(rule, cursor, state, false));
    }
    const results = this.evaluate(rule.expression, position, state, inToken);
    const built = [];
    for (const result of results) {
      if (rule.kind === 'silent' || inToken) {
        const scratch = { type: 'node', kind: rule.nodeKind, start: position, end: result.end, children: result.children };
        const acted = this.runAction(rule, result, scratch, position);
        if (acted) built.push(acted);
        continue;
      }
      const node = shareChildren({
        type: 'node', kind: rule.nodeKind, rule: rule.nodeKind, start: position, end: result.end,
        precedence: result.precedence, ambiguous: result.ambiguous,
      }, result);
      const acted = this.runAction(rule, result, node, position);
      if (acted) built.push(copyResult(acted, { children: [node], ambiguous: false }));
    }
    return built;
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
  scannerToken(name, position, state, inToken) {
    const { end: start, leaves } = this.terminalStart(position, state, inToken);
    const key = `${name}|${start}|${state.key}`;
    let scanned = this.scannerMemo.get(key);
    if (scanned === undefined) {
      scanned = this.runScanner(name, start, state);
      this.scannerMemo.set(key, scanned);
    }
    if (!scanned) {
      this.fail(start, name);
      if (inToken) return [];
      return this.elementFailed(start, leaves, state, { kind: name }, (cursor) => this.scannerToken(name, cursor, state, false));
    }
    const children = inToken ? NO_CHILDREN : [
      ...leaves,
      ...scanned.skipped,
      { type: 'token', kind: name, start: scanned.tokenStart, end: scanned.end },
    ];
    return [makeResult(scanned.end, scanned.state, children)];
  }

  runScanner(name, start, state) {
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
      column: () => columnOf(this.bytes, cursor, this.begin),
      atEnd: () => cursor >= this.end,
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
   * reaches farthest, with the rest of the input as an ERROR leaf.
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
      const rest = [...trailing.leaves, { type: 'error', start: trailing.end, end: this.end }];
      const repaired = { result: copyResult(result, { cost: result.cost + this.end - trailing.end }), trailing: rest };
      if (this.repairPoints.has(trailing.end)) complete.push(repaired);
      else {
        if (trailing.end > this.elementFarthest) this.elementFarthest = trailing.end;
        if (!partial || trailing.end > partial.end || (trailing.end === partial.end && repaired.result.cost < partial.repaired.result.cost)) {
          partial = { end: trailing.end, repaired };
        }
      }
    }
    if (complete.length === 0) {
      const failed = { ok: false, farthest: this.farthest, expected: [...this.expected].sort(), elementFarthest: this.elementFarthest };
      if (this.repairPoints) failed.partial = partial ? this.root(startRule, partial.repaired, false) : this.errorRoot(startRule);
      return failed;
    }
    let chosen = complete[0];
    for (const candidate of complete) if (candidate.result.cost < chosen.result.cost) chosen = candidate;
    // Repaired results of equal cost are not ambiguities.
    return { ok: true, root: this.root(startRule, chosen, chosen.result.cost === 0 && complete.length > 1) };
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

/**
 * Keyword lexing under `(matching longest)`. A tree-sitter lexer lexes a
 * keyword wherever the parse state admits it, before any parse goes on, so a
 * token rule's leaf over the same text (an identifier `typedef`) is not taken
 * there even when only it would let the parse go on. A parse records the
 * spans where a keyword token matched (`matched`); a leaf of a token rule in
 * its tree over such a span (`lexed`, the rule's kind, which an alias keeps)
 * that the keyword outranks (see `tokenConflict`) makes the span keyword-only
 * (`only`), and the input is parsed again, where no token rule takes a
 * keyword-only span the keyword outranks it on. The spans only grow, so the
 * reparses end.
 */
export class KeywordLexing {
  constructor(tokens) {
    this.tokens = tokens;
    this.only = new Set();
    this.matched = new Set();
  }

  // Whether a keyword-only span's keyword outranks a token rule's leaf over it.
  outranks(leaf) {
    return this.only.has(`${leaf.start}|${leaf.end}`) && this.outranksAt(leaf);
  }

  // Whether a keyword over the span of a token rule's leaf outranks it.
  outranksAt(leaf) {
    return tokenConflict({ type: 'token', kind: null, start: leaf.start, end: leaf.end }, leaf, this.tokens) > 0;
  }

  /** Marks the spans where `root` took a token rule's leaf over a keyword; true when one is new. */
  conflicts(root) {
    let found = false;
    const pending = [root];
    while (pending.length > 0) {
      const node = pending.pop();
      if (node.type === 'node') {
        for (let index = node.children.length - 1; index >= 0; index -= 1) pending.push(node.children[index]);
        continue;
      }
      if (node.lexed === undefined) continue;
      const span = `${node.start}|${node.end}`;
      if (!this.matched.has(span) || this.only.has(span)) continue;
      if (!this.outranksAt({ type: 'token', kind: node.lexed, start: node.start, end: node.end })) continue;
      this.only.add(span);
      found = true;
    }
    this.matched = new Set();
    return found;
  }
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
