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
  if (context === 'index-receiver' || context === 'length-receiver') return true;
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
