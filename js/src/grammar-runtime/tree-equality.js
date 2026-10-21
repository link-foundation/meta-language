// Shared decisions describe observable fields and recursive CST roles. This
// adapter reads actual trees; it does not compare executable program pointers.
import { readGrammarSelectionEqualityFields, readGrammarTreeEqualityFields,
  readGrammarTreeRecursionPlan } from '../translation/grammar-selection-rules.js';

const plans = new Map();
const selectionFields = readGrammarSelectionEqualityFields(false).split('|');
const alternativeFields = readGrammarSelectionEqualityFields(true).split('|');

function grammarTreeEqualityPlan(kind, output) {
  const key = `${kind}|${output}`;
  if (!plans.has(key)) plans.set(key, { fields: readGrammarTreeEqualityFields(kind, output).split('|'), recursion: readGrammarTreeRecursionPlan(kind, output) });
  return plans.get(key);
}

// Attributes historically ignore object key order. Linked selection and lexical
// metadata preserve order, array holes, and absent versus explicitly null keys.
function sameGrammarMetadata(first, second, ordered = true) {
  if (first === second) return true;
  if (typeof first !== 'object' || typeof second !== 'object' || first === null || second === null || Array.isArray(first) !== Array.isArray(second)) return false;
  if (Array.isArray(first) && first.length !== second.length) return false;
  const keys = Object.keys(first), other = Object.keys(second);
  return keys.length === other.length && keys.every((key, index) => (!ordered || key === other[index])
    && Object.hasOwn(second, key) && sameGrammarMetadata(first[key], second[key], ordered));
}

function sameGrammarHeader(first, second, fields) {
  return fields.every(field => {
    if (field === 'trivia' || field === 'projectionAnonymous' || field === 'literal') return Boolean(first[field]) === Boolean(second[field]);
    if (field === 'field' || field === 'language' || field === 'projectionName') return (first[field] ?? null) === (second[field] ?? null);
    if (field === 'attributes') return sameGrammarMetadata(first[field] ?? null, second[field] ?? null, false);
    if (field === 'lexical') return Object.hasOwn(first, field) === Object.hasOwn(second, field) && sameGrammarMetadata(first[field], second[field]);
    return first[field] === second[field];
  });
}

function sameGrammarSelectionMetadata(first, second, fields, recursive) {
  if (first === second) return true;
  if (!first || !second) return false;
  for (const field of fields) {
    if (Object.hasOwn(first, field) !== Object.hasOwn(second, field) || !sameGrammarMetadata(first[field], second[field])) return false;
  }
  const extra = value => Object.keys(value).filter(key => !fields.includes(key) && !['program', 'root', 'alternatives', 'sourceBytes'].includes(key));
  const own = extra(first), other = extra(second);
  if (own.length !== other.length || own.some((key, index) => key !== other[index] || !sameGrammarMetadata(first[key], second[key]))) return false;
  return recursive(first, second);
}

function sameGrammarEmbedding(first, second, fullMemo, readChildren) {
  if (Object.hasOwn(first, 'sourceChildren') !== Object.hasOwn(second, 'sourceChildren')) return false;
  if (first.sourceChildren) {
    if (first.sourceChildren.length !== second.sourceChildren.length
        || !first.sourceChildren.every((child, index) => sameGrammarTreeWith(child, second.sourceChildren[index], true, fullMemo, fullMemo, readChildren))) return false;
  }
  if (!sameGrammarTreeWith(first.root, second.root, true, fullMemo, fullMemo, readChildren)) return false;
  if (Object.hasOwn(first, 'selection') !== Object.hasOwn(second, 'selection')) return false;
  return sameGrammarSelectionMetadata(first.selection, second.selection, selectionFields, (own, other) => {
    if (Object.hasOwn(own, 'alternatives') !== Object.hasOwn(other, 'alternatives')) return false;
    const alternatives = own.alternatives, peers = other.alternatives;
    if (alternatives === peers) return true;
    if (!Array.isArray(alternatives) || !Array.isArray(peers) || alternatives.length !== peers.length) return false;
    return alternatives.every((alternative, index) => sameGrammarSelectionMetadata(alternative, peers[index], alternativeFields,
      (a, b) => sameGrammarTreeWith(a.root, b.root, true, fullMemo, fullMemo, readChildren)));
  });
}

// The optional child reader changes storage traversal only. Public array APIs
// keep their original path; parser-owned chains reuse the same equality plans.
function sameGrammarTreeWith(first, second, output, memo, fullMemo, readChildren) {
  if (first === second) return true;
  if (!first || !second || first.type !== second.type) return false;
  const plan = grammarTreeEqualityPlan(first.type, output);
  if (!sameGrammarHeader(first, second, plan.fields)) return false;
  const known = memo?.get(first)?.get(second);
  if (known !== undefined) return known;
  let same = true;
  if (plan.recursion === 'full-embedding') same = sameGrammarEmbedding(first, second, fullMemo ?? new Map(), readChildren);
  else if (plan.recursion !== 'leaf' && readChildren) {
    const meaningful = plan.recursion === 'meaningful-children' ? 'truthy-trivia' : false;
    same = sameGrammarChildSequences(readChildren(first, meaningful), readChildren(second, meaningful),
      (child, peer) => sameGrammarTreeWith(child, peer, output, memo, fullMemo, readChildren));
  } else if (plan.recursion !== 'leaf') {
    const children = node => plan.recursion === 'meaningful-children' ? node.children.filter(child => !child.trivia) : node.children;
    const own = children(first), other = children(second);
    same = own.length === other.length && own.every((child, index) => sameGrammarTreeWith(child, other[index], output, memo, fullMemo));
  }
  if (memo) {
    if (!memo.has(first)) memo.set(first, new Map());
    memo.get(first).set(second, same);
  }
  return same;
}

/** Concrete child lists, including full embedded roots and selection witnesses. */
export function sameGrammarOutput(first, second) {
  if (first === second) return true;
  const memo = new Map();
  return first.length === second.length && first.every((child, index) => sameGrammarTreeWith(child, second[index], true, memo, memo));
}

/** Historic structural node equality with complete concrete embedding equality. */
export function sameGrammarTree(first, second, memo) {
  return sameGrammarTreeWith(first, second, false, memo);
}


// Readers provide exact dense parser child sequences without forcing a cached
// flattened array. Length precedes recursion, as in the original array path.
function sameGrammarChildSequences(first, second, compare) {
  if (first.identity === second.identity) return true;
  if (first.length !== second.length) return false;
  const peers = second.values();
  for (const child of first.values()) {
    const peer = peers.next();
    if (peer.done || !compare(child, peer.value)) return false;
  }
  return peers.next().done;
}

/** Complete parser-owned output, retaining the ordinary full equality rules. */
export function sameGrammarOutputChildren(first, second, readChildren) {
  const memo = new Map();
  return sameGrammarChildSequences(readChildren(first, false), readChildren(second, false),
    (child, peer) => sameGrammarTreeWith(child, peer, true, memo, memo, readChildren));
}

/** Historical structural equality over the parser's meaningful direct children. */
export function sameGrammarMeaningfulChildren(first, second, readChildren) {
  return sameGrammarChildSequences(readChildren(first, true), readChildren(second, true),
    (child, peer) => sameGrammarTreeWith(child, peer, false, undefined, undefined, readChildren));
}

/** The same structural node comparison with the parser's complete child reader. */
export function sameGrammarTreeChildren(first, second, memo, readChildren) {
  return sameGrammarTreeWith(first, second, false, memo, undefined, readChildren);
}
