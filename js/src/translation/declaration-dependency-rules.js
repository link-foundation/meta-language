// Candidate dependency scheduling only. Actual complete bodies, lexical
// owners, types and target emission remain mandatory in the host checker.
import { readRequiredNumberFact } from './fact-column-rules.js';

/** @param {number[]} values @param {number} value @returns {boolean} */
function containsDeclarationDependency(values, value) {
  for (const previous of values) if (previous === value) return true;
  return false;
}

/** Dense adjacency facts must describe exactly their actual ordered edges.
 * @param {number[]} starts @param {number[]} counts @param {number[]} targets
 * @returns {boolean} */
export function acceptDeclarationDependencyFacts(starts, counts, targets) {
  if (starts.length !== counts.length) return false;
  let offset = 0;
  for (let node = 0; node < starts.length; node += 1) {
    const start = readRequiredNumberFact(starts, node);
    const count = readRequiredNumberFact(counts, node);
    if (start !== offset || count < 0 || count % 1 !== 0 || count > targets.length - offset) return false;
    offset += count;
  }
  if (offset !== targets.length) return false;
  for (const target of targets) {
    if (!(target >= 0 && target < starts.length) || target % 1 !== 0) return false;
  }
  return true;
}

/** Iterative two-pass SCC partition. Each result is its component's least
 * original node index. No recursion, matrix, source rewriting or callbacks.
 * The reverse walk scans the retained edges, so memory is O(V+E), and the
 * simple immutable fact implementation takes O(V*(V+E)) comparisons.
 * @param {number[]} starts @param {number[]} counts @param {number[]} targets
 * @returns {number[]} */
export function readDeclarationDependencyComponents(starts, counts, targets) {
  if (!acceptDeclarationDependencyFacts(starts, counts, targets)) {
    throw new RangeError('invalid declaration dependency facts');
  }
  let visited = [];
  for (let node = 0; node < starts.length; node += 1) visited.push(0);
  let finished = [];
  for (let root = 0; root < starts.length; root += 1) {
    if (readRequiredNumberFact(visited, root) === 1) continue;
    let pending = [root];
    while (pending.length > 0) {
      const instruction = readRequiredNumberFact(pending, pending.length - 1);
      pending = pending.slice(0, pending.length - 1);
      if (instruction < 0) finished.push(-instruction - 1);
      else if (readRequiredNumberFact(visited, instruction) === 0) {
        visited = visited.slice(0, instruction).concat([1]).concat(visited.slice(instruction + 1));
        pending.push(-instruction - 1);
        const start = readRequiredNumberFact(starts, instruction);
        let edge = start + readRequiredNumberFact(counts, instruction);
        while (edge > start) {
          edge -= 1;
          const target = readRequiredNumberFact(targets, edge);
          if (readRequiredNumberFact(visited, target) === 0) pending.push(target);
        }
      }
    }
  }
  let assigned = [];
  for (let node = 0; node < starts.length; node += 1) assigned.push(0);
  let vertices = [];
  let leaders = [];
  let position = finished.length;
  while (position > 0) {
    position -= 1;
    const root = readRequiredNumberFact(finished, position);
    if (readRequiredNumberFact(assigned, root) === 1) continue;
    let pending = [root];
    let members = [];
    let leader = root;
    let at = 0;
    while (at < pending.length) {
      const current = readRequiredNumberFact(pending, at);
      at += 1;
      if (readRequiredNumberFact(assigned, current) === 1) continue;
      assigned = assigned.slice(0, current).concat([1]).concat(assigned.slice(current + 1));
      members.push(current);
      if (current < leader) leader = current;
      for (let source = 0; source < starts.length; source += 1) {
        if (readRequiredNumberFact(assigned, source) === 1) continue;
        const start = readRequiredNumberFact(starts, source);
        const end = start + readRequiredNumberFact(counts, source);
        for (let edge = start; edge < end; edge += 1) {
          if (readRequiredNumberFact(targets, edge) === current && !containsDeclarationDependency(pending, source)) pending.push(source);
        }
      }
    }
    for (const member of members) {
      vertices.push(member);
      leaders.push(leader);
    }
  }
  let result = [];
  for (let node = 0; node < starts.length; node += 1) {
    for (let at = 0; at < vertices.length; at += 1) {
      if (readRequiredNumberFact(vertices, at) === node) result.push(readRequiredNumberFact(leaders, at));
    }
  }
  return result;
}

/** Initial profile changes only ordinary synchronous hoisted declarations.
 * @param {number} count @param {boolean} ordinary @param {boolean} unique
 * @returns {boolean} */
export function acceptRecursiveDeclarationGroup(count, ordinary, unique) {
  return count > 1 && count % 1 === 0 && ordinary && unique;
}

/** A group publishes only after the whole original group and every original
 * member have actually passed; counts come from actual checked owners.
 * @param {number} members @param {number} declarations @param {number} signatures
 * @param {boolean} flat @param {boolean} emitted @param {boolean} matching
 * @returns {boolean} */
export function acceptRecursiveDeclarationPublication(members, declarations, signatures, flat, emitted, matching) {
  return members > 1 && members % 1 === 0 && declarations === members && signatures === members && flat && emitted && matching;
}

/** Module effects and unknown binders cannot be silently omitted by either
 * new atomic grouping or an existing partial-scope retry. A wider form needs
 * the existing parser/checker to check the COMPLETE original module first.
 * @param {string[]} forms @param {boolean} completeChecked @returns {boolean} */
export function acceptDeclarationModuleScope(forms, completeChecked) {
  if (completeChecked) return true;
  for (const form of forms) {
    if (form !== 'comment' && form !== 'named function declaration'
      && form !== 'named import' && form !== 'primitive literal binding'
      && form !== 'empty class declaration') return false;
  }
  return true;
}

/** A signature describes a binding only after the actual module surface is
 * certified and that original name has exactly one actual declaration/import
 * owner. An isolated checked body alone cannot supply module ownership.
 * @param {boolean} scopeKnown @param {number} ownerCount @returns {boolean} */
export function acceptDeclarationSignatureBinding(scopeKnown, ownerCount) {
  return scopeKnown && ownerCount === 1;
}
