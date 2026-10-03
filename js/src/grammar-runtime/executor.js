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
// The verdict of the precedence filter on a result whose right operand is a
// node of one part: valid unless a result ends before the operand (see
// `lonePending`).
const LONE = Symbol('lone operand');

/** The step budget of one parse, shared with every embedded-language executor. */
export function stepBudget(options, length) {
  return { steps: 0, limit: options.stepLimit ?? 100_000 + 1000 * length };
}

// The repair cost of a MISSING leaf; an ERROR leaf costs the bytes it skips.
const MISSING_COST = 2;

function makeResult(end, state, children = NO_CHILDREN, dynamic = 0, cost = 0) {
  return { end, state, children, dynamic, precedence: null, tail: null, ambiguous: false, cost };
}

function resultKey(result) {
  return `${result.end}|${result.state.key}`;
}

// Adds a result to a deduplicating map: of two results with the same end
// and state, the lower repair cost wins, then, under `(matching longest)`
// (when `tokens` holds the token ranks and the input bytes), the tokens a
// lexer prefers (a lexer decides them before any parse does), then the
// shift or reduction an LR parser keeps by precedence, then the higher
// dynamic precedence; on a tie the first stays and,
// without repairs and unless both build the same trees, is marked ambiguous
// (as a copy, since results are shared through the memo).
function addResult(results, result, tokens = null) {
  const key = resultKey(result);
  const existing = results.get(key);
  if (!existing || result.cost < existing.cost) {
    results.set(key, result);
    return;
  }
  if (result.cost > existing.cost) return;
  const order = tokens ? preferredTokens(result, existing, tokens) || shiftOrder(result, existing) : 0;
  // A trace, off unless a probe sets the array (see
  // experiments/native-order-trace.mjs): every decided pair and its order.
  globalThis.__orderTrace?.push([result, existing, order]);
  if (order > 0 || (order === 0 && result.dynamic > existing.dynamic)) results.set(key, result);
  else if (order === 0 && result.dynamic === existing.dynamic && existing.cost === 0 && !existing.ambiguous && !sameOutput(result.children, existing.children)) {
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
  // `$(...);*`, whose `;` is a separator only after `$(` was shifted).
  // `pending` is the end of that leaf, `decided` the lookahead's start.
  let pending = Infinity;
  let decided = Infinity;
  const lookahead = (tree) => {
    if (decided === Infinity && !isTrivia(tree) && firstLeafStart(tree) >= pending) decided = firstLeafStart(tree);
  };
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
      const lone = a.type === 'node' ? b : a;
      if (lone.type !== 'node' && !isTrivia(lone)) pending = Math.min(pending, lone.end);
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
      lookahead(a);
      lookahead(b);
      return Math.min(a.start, b.start) > decided ? 0 : tokenConflict(a, b, tokens);
    } else {
      lookahead(a);
      skip(left);
      skip(right);
      skipped = [[], []];
    }
  }
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
  const rank = (leaf.kind !== null ? tokens.kinds.get(leaf.kind) : tokens.literals.get(textOf(tokens.bytes, leaf.start, leaf.end))) ?? null;
  if (leaf.priority === undefined) return rank;
  return { specificity: 0, order: Infinity, ...rank, priority: leaf.priority };
}

// Two leaves that differ in end or kind: 1 when a lexer prefers `a`, -1 when
// it prefers `b`, 0 when it cannot tell. The ranks decide only between two
// tokens at one offset; otherwise the longer leaf wins.
function tokenConflict(a, b, tokens) {
  const first = a.start === b.start ? tokenRank(a, tokens) : null;
  const second = a.start === b.start ? tokenRank(b, tokens) : null;
  if (first && second && first.priority !== second.priority) return first.priority > second.priority ? 1 : -1;
  if (a.end !== b.end) return a.end > b.end ? 1 : -1;
  if (!first || !second || sameRank(first, second)) return 0;
  if (first.specificity !== second.specificity) return first.specificity > second.specificity ? 1 : -1;
  return first.order < second.order ? 1 : -1;
}

// Whether two token ranks are the same, as one token's.
function sameRank(a, b) {
  return a.priority === b.priority && a.specificity === b.specificity && a.order === b.order;
}

// Whether two precedences are the same, or both none.
function samePrecedence(a, b) {
  if (!a || !b) return !a && !b;
  return a.level === b.level && a.associativity === b.associativity;
}

// Which of two results over the same text an LR parser keeps when it decides
// a shift-reduce conflict by precedence, as tree-sitter does when the grammar
// is generated: they are walked in order, skipping the subtrees both share,
// to the first node that the two build from one offset but end apart. The
// shorter one was reduced where the longer one shifted on, and as the item in
// progress is the node's own rule, its precedence in the two decides: the
// higher level wins and, on equal levels, the associativity of the reduced
// node, right to shift and left to reduce. 1 when `result` is kept, -1 when
// `existing` is, 0 when neither.
function shiftOrder(result, existing) {
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
  const enter = (stack, node) => {
    skip(stack);
    stack.push([node.children, 0, node]);
  };
  for (;;) {
    const a = peek(left);
    const b = peek(right);
    if (a === null || b === null) return 0;
    if (a === b || (a.type !== 'node' && b.type !== 'node' && a.start === b.start && a.end === b.end)) {
      skip(left);
      skip(right);
      continue;
    }
    if (a.type === 'node' && b.type === 'node' && a.start === b.start) {
      const pair = chainPair(a, b);
      if (pair && pair[0].end !== pair[1].end) {
        const [long, short, sign] = pair[0].end > pair[1].end ? [...pair, 1] : [pair[1], pair[0], -1];
        return sign * shiftPreferred(long, short);
      }
      // The same node reduced on in two ways (Rust's `m!(x);` in a block, a
      // macro invocation that `_expression_except_range` reduces under
      // `(precedence 1 none (ref macro_invocation))` and
      // `_declaration_statement` of level 0) conflicts at its end: the higher
      // level it was reduced with wins.
      if (pair) {
        const first = pair[0].reduced?.level ?? 0;
        const second = pair[1].reduced?.level ?? 0;
        if (first !== second) return first > second ? 1 : -1;
      }
    }
    const lone = loneReduction(a, b) || -loneReduction(b, a);
    if (lone !== 0) return lone;
    const reduced = extraReduction(a, b, right) || -extraReduction(b, a, left);
    if (reduced !== 0) return reduced;
    if (a.type === 'node' && b.type === 'node' && a.start === b.start) {
      const parted = chainConflict(a, b);
      if (parted !== 0) return parted;
    }
    if (a.type !== 'node' && b.type !== 'node') return 0;
    if (a.type === 'node') enter(left, a);
    if (b.type === 'node') enter(right, b);
  }
}

// Which of two results an LR parser keeps when one of them reduced a token
// alone, to a silent rule of level 0 or under a precedence (Rust's
// `(precedence -1 none (literal $))`), where the other shifted on in a node
// `a` that begins with the token (Rust's `$x:expr` binding of level 1): the
// innermost such node is the item in progress, and its precedence against the
// token's decides, as in `shiftPreferred`. A token that ends a silent rule
// reduced under a precedence (Rust's `_let_chain`, `let ... && c` of level 3
// left before the `&&` of `c && d`) is reduced with that rule's precedence.
// 1 when `a`'s result is kept, -1 when `b`'s is, 0 when neither.
function loneReduction(a, b) {
  if (a.type !== 'node' || b.type !== 'token') return 0;
  let progress = a;
  for (;;) {
    const first = progress.children.find((child) => !isTrivia(child));
    if (!first) return 0;
    if (first.type !== 'node') {
      if (!sameTree(first, b)) return 0;
      break;
    }
    progress = first;
  }
  const none = { level: 0, associativity: 'none' };
  const shifted = progress.precedence ?? none;
  const reduced = b.reduced ?? b.precedence ?? none;
  if (shifted.level !== reduced.level) return shifted.level > reduced.level ? 1 : -1;
  if (reduced.associativity === 'right') return 1;
  if (reduced.associativity === 'left') return -1;
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
// goes on, the associativity of the reduced node. 1 when `a`'s result is
// kept, -1 when the other is, 0 when neither.
function extraReduction(a, b, stack) {
  if (a.type !== 'node') return 0;
  let parent = a;
  for (;;) {
    const first = parent.children.find((child) => !isTrivia(child));
    if (!first) return 0;
    if (sameTree(first, b)) break;
    if (first.type !== 'node') return 0;
    parent = first;
  }
  const [siblings, index, container] = stack[stack.length - 1];
  const next = siblings.slice(index).filter((child) => !isTrivia(child));
  const own = parent.children.filter((child) => !isTrivia(child));
  if (own.length > next.length || own.some((child, at) => !sameTree(child, next[at]))) return 0;
  const none = { level: 0, associativity: 'none' };
  const mine = reduction(parent);
  const other = container?.precedence ?? none;
  if (mine.level !== other.level) return mine.level > other.level ? 1 : -1;
  if (container && container.end > parent.end) {
    if (mine.associativity === 'left') return 1;
    if (mine.associativity === 'right') return -1;
  }
  return 0;
}

// Whether two subtrees are the same tree over the same text, whichever of
// them holds the white space around it.
function sameTree(a, b) {
  if (a === b) return true;
  if (a.type !== b.type || a.kind !== b.kind) return false;
  if (a.type !== 'node') return a.start === b.start && a.end === b.end;
  const first = a.children.filter((child) => !isTrivia(child));
  const second = b.children.filter((child) => !isTrivia(child));
  return first.length === second.length && first.every((child, index) => sameTree(child, second[index]));
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
// one offset, as the pair of its nodes, or null.
function chainPair(a, b) {
  const other = leftmostChain(b);
  for (const node of leftmostChain(a)) {
    const match = other.find((candidate) => candidate.kind === node.kind && candidate.start === node.start);
    if (match) return [node, match];
  }
  return null;
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
function chainConflict(a, b) {
  const first = leftmostChain(a);
  const second = leftmostChain(b);
  const ends = (chain) => new Set(chain.map((node) => node.end));
  const [mine, theirs] = [ends(first), ends(second)];
  const parted = [...mine].filter((end) => !theirs.has(end)).concat([...theirs].filter((end) => !mine.has(end)));
  if (parted.length === 0) return 0;
  const end = Math.min(...parted);
  const [reducing, shifting, sign] = mine.has(end) ? [first, second, -1] : [second, first, 1];
  const short = reducing.findLast((node) => node.end === end);
  const long = shifting.findLast((node) => node.end > end);
  if (!long) return 0;
  const own = short.children.filter((child) => !isTrivia(child));
  const next = long.children.filter((child) => !isTrivia(child));
  if (own.length >= next.length || own.some((child, at) => !sameTree(child, next[at]))) return 0;
  return sign * shiftPreferred(long, short);
}

// 1 when the shift that built `long` is preferred to the reduction that
// ended `short` (the same node kind from the same offset), -1 when the
// reduction is, 0 when the precedences cannot tell. The shift is in the
// innermost node of `long` that goes on past the end of `short`. When that
// node began with `short`, as a binary expression whose left operand is the
// expression a statement is of, the long result reduced that operand to a
// silent rule where the short one reduced its node: the two reductions
// conflict instead, and a silent rule's reduction is of level 0.
function shiftPreferred(long, short) {
  const none = { level: 0, associativity: 'none' };
  const begin = firstLeafStart(short);
  let progress = long;
  for (;;) {
    const inner = progress.children.find((child) => child.type === 'node' && child.start < short.end && child.end > short.end);
    if (!inner) break;
    if (firstLeafStart(inner) === begin) {
      const reduced = reduction(short);
      return reduced.level === 0 ? 0 : -Math.sign(reduced.level);
    }
    progress = inner;
  }
  const shifted = progress.precedence ?? none;
  const reduced = reducedBefore(short, progress);
  if (shifted.level !== reduced.level) return shifted.level > reduced.level ? 1 : -1;
  if (reduced.associativity === 'right') return 1;
  if (reduced.associativity === 'left') return -1;
  return 0;
}

// The precedence a node is reduced with: the innermost one over its last
// part, as a generated parser takes the precedence of a production's last
// step (Rust's `let` condition, whose value is `(precedence 3 left (ref
// expression))`, reduces before the `&&` of a binary expression of level 3),
// else none of level 0.
function reduction(node) {
  return node.tail ?? node.precedence ?? { level: 0, associativity: 'none' };
}

// The precedence `short` was reduced with where the shift in `progress` went
// on instead: that of the innermost node along its rightmost chain whose
// last part is the first part of `progress`, as the production a generated
// parser completes at the conflict (Rust's `let bar = || baz && quux`, where
// the closure of level -1 ends with `baz`, not the `let` condition), or the
// precedence a token ending a silent rule keeps (see `loneReduction`); else
// the precedence `short` reduces with.
function reducedBefore(short, progress) {
  const first = progress.children.find((child) => !isTrivia(child));
  for (let node = short; first && node.type === 'node';) {
    const meaningful = node.children.filter((child) => !isTrivia(child));
    const last = meaningful[meaningful.length - 1];
    if (!last) break;
    if (sameTree(last, first)) return (last.type === 'token' && last.reduced) || reduction(node);
    node = last;
  }
  return reduction(short);
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
    case 'capture': case 'precedence': case 'dynamicPrecedence': case 'alias': case 'repeat1': return nullable(expression.item);
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
    this.shiftMemo = new Map();
    this.owners = null;
    this.edgeMemo = new Map();
    this.inExtra = false;
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
    // The scan past each failing element at a repair point (see elementFailed).
    this.repairMemo = new Map();
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
    const results = [makeResult(start, state, placed, 0, MISSING_COST)];
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
  // (white space) are trivia, as no extra nests in another.
  skipTrivia(position, state) {
    const { trivia } = this.program;
    if (trivia.length === 0) return { end: position, leaves: NO_CHILDREN };
    const key = `${position}|${state.key}|${this.inExtra}`;
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
        if (this.inExtra && item.kind !== null) continue;
        const end = this.quietly(() => longestResult(this.evaluate(item.expression, cursor, state, true))?.end ?? -1);
        if (end > best) {
          best = end;
          bestKind = item.kind;
        }
      }
      if (best === cursor) break;
      const extra = this.extraNode(bestKind, cursor, best, state);
      leaves.push(extra?.node ?? { type: 'token', kind: bestKind, start: cursor, end: best, trivia: true });
      cursor = extra?.end ?? best;
    }
    const skipped = { end: cursor, leaves };
    this.triviaMemo.set(key, skipped);
    return skipped;
  }

  // The node an extra of a rule that builds one makes of its text, parsed as
  // syntax, as a tree-sitter extra of a rule that is no token is a node with
  // its children (Rust's doc comments); null for any other extra.
  // Under `(matching longest)` the parse is the one with the tokens a lexer
  // prefers, which may end before the longest (Rust's `////` is a comment
  // without a doc marker); otherwise the one that ends at `end`. Gives the
  // node and its end.
  extraNode(kind, start, end, state) {
    if (kind === null || this.program.rules.get(kind)?.kind !== 'normal') return null;
    this.inExtra = true;
    try {
      const results = this.quietly(() => this.evaluate({ kind: 'ref', name: kind }, start, state, false))
        .filter((result) => result.cost === 0 && result.children.some((child) => child.type === 'node'));
      let best = null;
      for (const result of results) {
        if (!this.longestTokens) {
          if (result.end === end) best ??= result;
          continue;
        }
        const order = best === null ? 1 : preferredTokens(result, best, this.longestTokens);
        if (order > 0 || (order === 0 && result.end > best.end)) best = result;
      }
      const node = best?.children.find((child) => child.type === 'node');
      return node ? { node: { ...node, trivia: true }, end: best.end } : null;
    } finally {
      this.inExtra = false;
    }
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
      return this.elementFailed(start, leaves, state, missingOf(expression), expression, (cursor) => this.terminal(expression, cursor, state, false));
    }
    const children = inToken ? NO_CHILDREN : [...leaves, { type: 'token', kind: separatorRunKind(expression, start, end), start, end }];
    return [makeResult(end, state, children)];
  }

  // A leaf over the longest match of `item` in token context: token(),
  // immediateToken() and longest() alternatives build on it.
  // A token under a lexical precedence keeps its level on the leaf, as the
  // token's rank where it is matched (see `tokenRank`).
  tokenLeaf(item, start, leaves, state, inToken, kind) {
    const best = longestResult(this.evaluate(item, start, state, true));
    if (!best) return [];
    const leaf = item.kind === 'lexicalPrecedence'
      ? { type: 'token', kind, start, end: best.end, priority: item.level }
      : { type: 'token', kind, start, end: best.end };
    const children = inToken ? NO_CHILDREN : [...leaves, leaf];
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
        const scanned = expression.item.kind === 'ref' && this.program.externalTokens.has(expression.item.name);
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
  // never built.
  continuation(item, left, inToken) {
    if (!this.repairPoints?.has(left.end)) return this.evaluate(item, left.end, left.state, inToken);
    const last = left.children.findLast((child) => !isSeparator(child));
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
    const conflicts = (child, side) => {
      if (child.type !== 'node' || !child.precedence) return false;
      const inner = child.precedence.level;
      if (inner > level || (inner === level && associativity === side)) return false;
      if (side === 'right' && this.lexedShift(child.rule)) return false;
      if (!this.reachesOwner(expression, child.rule, side)) return false;
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
      if (meaningful.length < 2 || conflicts(meaningful[0], 'left')) return meaningful.length < 2;
      const right = conflicts(meaningful[meaningful.length - 1], 'right');
      return right === LONE ? LONE : !right;
    };
    let results = inToken
      ? this.evaluate(expression.item, position, state, inToken)
      : this.filtered(expression.item, position, state, valid);
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
      addResult(found, result, this.longestTokens);
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
    while (body && ['precedence', 'dynamicPrecedence', 'capture'].includes(body.kind)) body = body.item;
    if (body?.kind === 'seq' && body.items.length > 1 && !nullable(body.items[0])) {
      shifts = (this.leadPriority({ kind: 'seq', items: body.items.slice(1) }, new Set()) ?? 0) > 0;
    }
    this.shiftMemo.set(kind, shifts);
    return shifts;
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
      case 'capture': case 'precedence': case 'dynamicPrecedence': case 'alias':
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
    if (!this.owners) {
      this.owners = new Map();
      const walk = (item, name) => {
        if (!item || typeof item !== 'object') return;
        if (item.kind === 'precedence' && !this.owners.has(item)) this.owners.set(item, name);
        walk(item.item, name);
        if (Array.isArray(item.items)) for (const child of item.items) walk(child, name);
      };
      for (const [name, rule] of this.program.rules) walk(rule.expression, name);
    }
    const owner = this.owners.get(expression);
    const rule = this.program.rules.get(kind);
    if (owner === undefined || rule?.nodeKind !== kind) return true;
    const key = `${kind}|${side}`;
    let names = this.edgeMemo.get(key);
    if (!names) {
      names = new Set();
      this.edgeRules(rule.expression, side, names);
      this.edgeMemo.set(key, names);
    }
    return names.has(owner);
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
      case 'capture': case 'precedence': case 'dynamicPrecedence': case 'alias':
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
      return this.elementFailed(start, leaves, state, { kind: null }, expression, (cursor) => this.longest(expression, cursor, state, false));
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
    // The seed grows while a pass reaches a new end or settles an end on
    // another tree: a left operand ranked anew (Rust's `impl A + B + C`,
    // where `bounded_type` over `impl A + B` outranks `impl` over `A + B`)
    // changes the trees grown from it, so the next pass grows them again; at
    // most one pass per end changes a tree, for an order that is not
    // transitive. A pass grows only the results the pass before added or
    // changed: the trees grown from the others are already merged, so a
    // chain of n operators takes n passes of one seed each, not n of n.
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
        addResult(merged, result, this.longestTokens);
        const kept = merged.get(key);
        if (kept === existing) continue;
        // A tie with a tree of an earlier pass is the ambiguity the rule body
        // marks when both trees meet in one pass.
        if (kept !== result) merged.set(key, this.ambiguousResult(rule, existing, inToken));
        renewed.add(key);
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
        const acted = this.runAction(rule, result, scratch, position);
        if (!acted) continue;
        if (!(expected && acted.ambiguous) && !acted.tail) {
          built.push(acted);
          continue;
        }
        // The rule is one part of the rule that refers to it, which reduces
        // with the precedence around that part, not inside it; a token the
        // rule ends with keeps the precedence the rule reduces with, for the
        // conflict with a shift after it (see `loneReduction`).
        const changes = { ambiguous: expected ? false : acted.ambiguous, tail: null };
        const meaningful = acted.tail ? acted.children.filter((child) => !isTrivia(child)) : [];
        const last = meaningful[meaningful.length - 1];
        if (last?.type === 'token' && !samePrecedence(last.reduced ?? last.precedence, acted.tail)) {
          changes.children = acted.children.map((child) => (child === last ? { ...child, reduced: acted.tail } : child));
        }
        built.push(copyResult(acted, changes));
        continue;
      }
      const node = shareChildren({
        type: 'node', kind: rule.nodeKind, rule: rule.nodeKind, start: position, end: result.end,
        precedence: result.precedence, tail: result.tail, ambiguous: result.ambiguous,
      }, result);
      const acted = this.runAction(rule, result, node, position);
      if (acted) built.push(copyResult(acted, { children: [node], tail: null, ambiguous: false }));
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
      return this.elementFailed(start, leaves, state, { kind: name }, name, null);
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
      const failed = { ok: false, farthest: this.farthest, expected: [...this.expected].sort(), elementFarthest: this.elementFarthest };
      if (this.repairPoints) failed.partial = partial ? this.root(startRule, partial.repaired, false) : this.errorRoot(startRule);
      return failed;
    }
    // The complete results end apart, before their trailing trivia, so they
    // are ranked here as addResult ranks results with one end: the lower
    // cost, then the tokens a lexer prefers and the shift or reduction an LR
    // parser keeps, then the higher dynamic precedence. Repaired results of equal cost are not ambiguities.
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
    const root = this.root(startRule, chosen, tied && chosen.result.cost === 0);
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
    const order = this.longestTokens ? preferredTokens(whole(a), whole(b), this.longestTokens) || shiftOrder(whole(a), whole(b)) : 0;
    if (order !== 0) return order;
    return Math.sign(a.result.dynamic - b.result.dynamic);
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
