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
import { columnOf, decodeAt, quoteText, textOf, utf16View } from './text.js';

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
// and state, the lower repair cost wins, then the higher dynamic precedence;
// on a tie the first stays and, without repairs, is marked ambiguous (as a
// copy, since results are shared through the memo).
function addResult(results, result) {
  const key = resultKey(result);
  const existing = results.get(key);
  if (!existing || result.cost < existing.cost) results.set(key, result);
  else if (result.cost > existing.cost) return;
  else if (result.dynamic > existing.dynamic) results.set(key, result);
  else if (result.dynamic === existing.dynamic && existing.cost === 0 && !existing.ambiguous) results.set(key, { ...existing, ambiguous: true });
}

function joinResults(left, right, inToken) {
  return {
    end: right.end,
    state: right.state,
    children: inToken ? NO_CHILDREN : left.children.concat(right.children),
    dynamic: left.dynamic + right.dynamic,
    precedence: null,
    ambiguous: left.ambiguous || right.ambiguous,
    cost: left.cost + right.cost,
  };
}

function longestResult(results) {
  let best = null;
  for (const result of results) if (!best || result.end > best.end) best = result;
  return best;
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
    this.depth = 0;
    this.memo = new Map();
    this.memoLimit = options.memoLimit ?? DEFAULT_MEMO_LIMIT;
    this.callStack = [];
    this.triviaMemo = new Map();
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
        results.push({ ...result, children: [...leaves, error, ...result.children], cost: result.cost + cursor - start });
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
    const { end: start, leaves } = this.terminalStart(position, state, inToken);
    const end = this.matchTerminal(expression, start);
    if (end < 0) {
      this.fail(start, expectationOf(expression));
      if (inToken) return [];
      return this.elementFailed(start, leaves, state, missingOf(expression), (cursor) => this.terminal(expression, cursor, state, false));
    }
    const children = inToken ? NO_CHILDREN : [...leaves, { type: 'token', kind: null, start, end }];
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
        return this.evaluate(expression.item, position, state, inToken).map((result) => (inToken ? result : {
          ...result,
          children: result.children.map((child) => (isTrivia(child) ? child : { ...child, field: expression.label })),
        }));
      case 'alias': return this.alias(expression, position, state, inToken);
      case 'precedence': return this.precedence(expression, position, state, inToken);
      case 'dynamicPrecedence':
        return this.evaluate(expression.item, position, state, inToken)
          .map((result) => ({ ...result, dynamic: result.dynamic + expression.level }));
      case 'lexicalPrecedence': return this.evaluate(expression.item, position, state, inToken);
      case 'longest': return this.longest(expression, position, state, inToken);
      case 'token': case 'immediateToken': {
        const { end: start, leaves } = expression.kind === 'token'
          ? this.terminalStart(position, state, inToken)
          : { end: position, leaves: NO_CHILDREN };
        const results = this.tokenLeaf(expression.item, start, leaves, state, inToken, null);
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

  // `keep`, when given, filters the complete sequences before they are
  // deduplicated, so a precedence filter never loses a valid parse to an
  // invalid one that reached the same end first.
  sequence(items, position, state, inToken, keep = null) {
    let current = [makeResult(position, state)];
    for (const [index, item] of items.entries()) {
      const next = new Map();
      const last = index === items.length - 1;
      for (const left of current) {
        for (const right of this.evaluate(item, left.end, left.state, inToken)) {
          const joined = joinResults(left, right, inToken);
          if (last && keep && !keep(joined)) continue;
          addResult(next, joined);
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
      for (const result of this.evaluate(item, position, state, inToken)) addResult(results, result);
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
    // not extended again (it is the same continuation) but marks ambiguity.
    const results = new Map();
    let frontier = [makeResult(position, state)];
    for (let count = 0; frontier.length > 0; count += 1) {
      if (count >= min) {
        const fresh = [];
        for (const result of frontier) {
          const key = resultKey(result);
          if (results.has(key)) addResult(results, result);
          else {
            results.set(key, result);
            fresh.push(result);
          }
        }
        frontier = fresh;
      }
      if (max !== null && count >= max) break;
      const next = new Map();
      for (const left of frontier) {
        for (const right of this.evaluate(item, left.end, left.state, inToken)) {
          if (zeroWidth(left, right)) {
            // Zero-width iterations can pad up to the minimum once.
            if (count < min) addResult(results, join(left, right));
            continue;
          }
          addResult(next, join(left, right));
        }
      }
      frontier = [...next.values()];
    }
    return [...results.values()];
  }

  alias(expression, position, state, inToken) {
    return this.evaluate(expression.item, position, state, inToken).map((result) => {
      if (inToken) return result;
      const meaningful = result.children.filter((child) => !isTrivia(child));
      if (meaningful.length === 1) {
        return { ...result, children: result.children.map((child) => (child === meaningful[0] ? renamed(child, expression.name) : child)) };
      }
      const node = {
        type: 'node', kind: expression.name, rule: expression.name, start: position, end: result.end,
        children: result.children, ambiguous: result.ambiguous,
      };
      return { ...result, children: [node], ambiguous: false };
    });
  }

  // Precedence and associativity filter the binary-shaped results: a
  // leftmost or rightmost child node of lower precedence, or of equal
  // precedence on the side the associativity forbids, invalidates a result.
  precedence(expression, position, state, inToken) {
    const { level, associativity } = expression;
    const tag = { level, associativity };
    const valid = (result) => {
      if (allowed(result)) return true;
      this.fail(result.end, 'precedence');
      return false;
    };
    const allowed = (result) => {
      const meaningful = result.children.filter((child) => !isTrivia(child));
      if (meaningful.length < 2) return true;
      const first = meaningful[0];
      const last = meaningful[meaningful.length - 1];
      if (first.type === 'node' && first.precedence) {
        const inner = first.precedence.level;
        if (inner < level || (inner === level && associativity !== 'left')) return false;
      }
      if (last.type === 'node' && last.precedence) {
        const inner = last.precedence.level;
        if (inner < level || (inner === level && associativity !== 'right')) return false;
      }
      return true;
    };
    const results = inToken
      ? this.evaluate(expression.item, position, state, inToken)
      : this.filtered(expression.item, position, state, valid);
    return results.map((result) => ({ ...result, precedence: tag }));
  }

  // The results of `expression` that `keep` accepts, filtered before a
  // sequence or an unordered choice merges results of the same end and state.
  filtered(expression, position, state, keep) {
    if (expression.kind === 'seq' && expression.items.length > 0) return this.sequence(expression.items, position, state, false, keep);
    if (expression.kind === 'choice' && !expression.ordered && !this.peg) {
      this.step();
      const results = new Map();
      for (const item of expression.items) {
        for (const result of this.filtered(item, position, state, keep)) addResult(results, result);
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
    // While repairing, a call made quietly (where nothing is repaired) is
    // memoized apart from the same call made in the open.
    const quiet = this.repairPoints && this.suppressed > 0 ? '|quiet' : '';
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
        const acted = this.runAction(rule, result, leaf, start);
        if (!acted) continue;
        built.push({ ...acted, children: inToken ? NO_CHILDREN : [...leaves, leaf], precedence: null, ambiguous: false });
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
      const node = {
        type: 'node', kind: rule.nodeKind, rule: rule.nodeKind, start: position, end: result.end,
        children: result.children, precedence: result.precedence, ambiguous: result.ambiguous,
      };
      const acted = this.runAction(rule, result, node, position);
      if (acted) built.push({ ...acted, children: [node], ambiguous: false });
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
      ...this.valueMachine({ ...result, children }, start, working),
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
    return { ...result, state: settleState(working) };
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
      const repaired = { result: { ...result, cost: result.cost + this.end - trailing.end }, trailing: rest };
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
