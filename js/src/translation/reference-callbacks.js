// Conservative source reachability proof for the existing snapshot backend.
// A writing/unknown callback needs the live slot interpreter instead.
import { readReferenceCallbackEffect, readReferenceCallbackOperandFields } from './reference-method-rules.js';
import { readProgramExpressionOperandFields } from '../typed-program-link-rules.js';

export function acceptReadOnlyReferenceCallback(callback, declarations) {
  if (!callback || !['lambda', 'functionRef'].includes(callback.k)) return false;
  const pending = [callback], seen = new WeakSet(), functions = new Set();
  let count = 0;
  while (pending.length) {
    const node = pending.pop();
    if (!node || typeof node !== 'object' || seen.has(node)) continue;
    seen.add(node);
    if (Array.isArray(node)) { for (const value of node) pending.push(value); continue; }
    if (typeof node.k !== 'string') {
      // Checked record entries and match arms contain these actual operands;
      // pattern/type/source metadata does not count as executable expressions.
      if (node.value && typeof node.value === 'object') pending.push(node.value);
      if (node.body && typeof node.body === 'object') pending.push(node.body);
      continue;
    }
    if (++count > 100000) return false;
    const role = readReferenceCallbackEffect(node.k ?? '', node.op ?? '');
    if (role === 1) return false;
    if (role === 2 && !functions.has(node.fn)) {
      const entry = declarations.get(node.fn);
      if (entry?.k !== 'fn' || !entry.body) return false;
      functions.add(node.fn); pending.push(entry.body);
    }
    if (role === 3 && !['lambda', 'functionRef'].includes(node.fn?.k)) return false;
    for (const field of readReferenceCallbackOperandFields(node.k, readProgramExpressionOperandFields(node.k)).split(' ')) if (field && node[field]) pending.push(node[field]);
  }
  return true;
}
