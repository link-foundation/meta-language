// Decisions for fresh local array construction. These facts describe actual
// lexical allocations and uses; they do not authorize a general heap update.
import { readRequiredNumberFact } from './fact-column-rules.js';

/** @param {string} statement @param {string} expression @param {string} operation @param {string} receiver @param {number} pathLength @param {number} argumentsCount @returns {boolean} */
export function acceptOwnedArrayAppendShape(statement, expression, operation, receiver, pathLength, argumentsCount) {
  return statement === 'expr' && expression === 'method' && operation === 'push'
    && receiver === 'name' && pathLength === 1 && argumentsCount > 0;
}

/** @param {string} statement @param {string} expression @param {string} operation @param {string} receiver @param {number} pathLength @param {number} argumentsCount @returns {boolean} */
export function acceptOwnedArrayRemovalShape(statement, expression, operation, receiver, pathLength, argumentsCount) {
  return statement === 'expr' && expression === 'method' && operation === 'pop'
    && receiver === 'name' && pathLength === 1 && argumentsCount === 0;
}

/** @returns {number[]} */
export function readOwnedArrayRemovalSliceBounds() {
  return [0, -1];
}

/** @param {string} context @param {string} operation @param {boolean} reader @returns {boolean} */
export function acceptOwnedArrayRead(context, operation, reader) {
  if (context === 'index-receiver' || context === 'length-receiver' || context === 'iteration-receiver') return true;
  if (context === 'argument') return reader;
  return context === 'method-receiver' && (operation === 'slice' || operation === 'includes'
    || operation === 'indexOf' || operation === 'lastIndexOf' || operation === 'join');
}

/** @param {string} kind @returns {boolean} */
export function acceptOwnedArrayReaderResult(kind) {
  return kind === 'float' || kind === 'fixed' || kind === 'int' || kind === 'nat'
    || kind === 'bool' || kind === 'string' || kind === 'unit';
}

/**
 * An escape before an append is unsafe. A later escape is unsafe when a
 * surrounding loop can reach the append again without a new allocation.
 * Return transfers are absent from this list because they leave the function.
 * @param {number[]} allocationLoops @param {number[]} appendLoops
 * @param {number[]} escapeLoops @param {number[]} escapeStarts
 * @param {number[]} escapeCounts @param {number[]} escapePositions
 * @param {number} position @returns {boolean}
 */
export function acceptOwnedArrayAppendUses(allocationLoops, appendLoops, escapeLoops, escapeStarts, escapeCounts, escapePositions, position) {
  for (let escape = 0; escape < escapePositions.length; escape += 1) {
    if (readRequiredNumberFact(escapePositions, escape) < position) return false;
    const start = readRequiredNumberFact(escapeStarts, escape);
    const end = start + readRequiredNumberFact(escapeCounts, escape);
    for (let edge = start; edge < end; edge += 1) {
      const loop = readRequiredNumberFact(escapeLoops, edge);
      let appendInside = false;
      let allocationInside = false;
      for (let index = 0; index < appendLoops.length; index += 1) {
        if (readRequiredNumberFact(appendLoops, index) === loop) appendInside = true;
      }
      for (let index = 0; index < allocationLoops.length; index += 1) {
        if (readRequiredNumberFact(allocationLoops, index) === loop) allocationInside = true;
      }
      if (appendInside && !allocationInside) return false;
    }
  }
  return true;
}

/**
 * A complete RHS read walk must still record every actual unsafe use before
 * this shape can omit the assignment escape. Existing escapes are retained.
 * This decision never creates an owner or blesses a parameter/foreign array.
 * @param {string} expression @param {string} operation @param {string} receiver
 * @param {number} pathLength @param {boolean} sameOwner
 * @param {number} argumentsCount @returns {boolean}
 */
export function acceptOwnedArraySelfSliceRebinding(expression, operation, receiver, pathLength, sameOwner, argumentsCount) {
  if (expression === 'copySlice') return operation === '' && receiver === 'name'
    && pathLength === 1 && sameOwner && argumentsCount === 0;
  return expression === 'method' && operation === 'slice' && receiver === 'name'
    && pathLength === 1 && sameOwner
    && (argumentsCount === 0 || argumentsCount === 1 || argumentsCount === 2);
}

/**
 * A consuming array use may move its actual lexical value only when no
 * reachable continuation reads that same binding. Edges describe evaluation
 * order; alternatives join their actual continuation rather than each other.
 * @param {number} position @param {number[]} bindings @param {number[]} starts
 * @param {number[]} counts @param {number[]} successors @returns {boolean}
 */
export function acceptOwnedArrayFinalUse(position, bindings, starts, counts, successors) {
  const binding = readRequiredNumberFact(bindings, position);
  const pending = [];
  const seen = [];
  const start = readRequiredNumberFact(starts, position);
  const end = start + readRequiredNumberFact(counts, position);
  for (let edge = start; edge < end; edge += 1) pending.push(readRequiredNumberFact(successors, edge));
  let cursor = 0;
  while (cursor < pending.length) {
    const event = readRequiredNumberFact(pending, cursor);
    cursor += 1;
    if (seen.includes(event)) continue;
    seen.push(event);
    if (readRequiredNumberFact(bindings, event) === binding) return false;
    const nextStart = readRequiredNumberFact(starts, event);
    const nextEnd = nextStart + readRequiredNumberFact(counts, event);
    for (let edge = nextStart; edge < nextEnd; edge += 1) pending.push(readRequiredNumberFact(successors, edge));
  }
  return true;
}

/** A parser-owned closed lexical for-of block supplies the exact source
 * declaration and loop identities. Ordinary aliases supply no such owner.
 * @param {string} block @param {boolean} inline @param {number} statements
 * @param {string} binding @param {string} expression @param {string} receiver
 * @param {number} pathLength @param {string} following
 * @param {boolean} parserOwned @param {boolean} fresh @returns {boolean} */
export function acceptOwnedArrayIterationBorrowShape(block, inline, statements, binding, expression, receiver, pathLength, following, parserOwned, fresh) {
  return block === 'block' && !inline && statements === 2 && binding === 'const'
    && expression === 'forOfIterable' && receiver === 'name' && pathLength === 1
    && following === 'for' && parserOwned && fresh;
}

/** Scoped borrows end only when their actual closed source block leaves.
 * Every unknown alias/capture/call remains a permanent ordinary escape.
 * @param {number} position @param {number[]} starts @param {number[]} ends
 * @returns {boolean} */
export function acceptOwnedArrayBorrowedAppend(position, starts, ends) {
  if (!(position >= 0 && position % 1 === 0) || starts.length !== ends.length) return false;
  for (let at = 0; at < starts.length; at += 1) {
    const start = readRequiredNumberFact(starts, at), end = readRequiredNumberFact(ends, at);
    if (!(start >= 0 && start % 1 === 0 && end > start && end % 1 === 0)) return false;
    if (start <= position && position < end) return false;
  }
  return true;
}
