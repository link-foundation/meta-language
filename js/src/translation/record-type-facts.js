// Project actual shared type objects and actual variable bindings. All child
// selection and reachability decisions remain in the automatically ported rule.
import { createSurfaceReferenceFacts } from './surface-reference-facts.js';
import { readRecordTypeOwnerPresence } from './record-value-rules.js';

export function containsRecordTypeOwner(term, owner, bindings) {
  const facts = createSurfaceReferenceFacts([term, owner, ...Array.from(bindings)], { retainValues: true });
  const roots = facts.targets.slice(facts.starts[facts.root], facts.starts[facts.root] + facts.counts[facts.root]);
  const numbers = facts.values.map(value => typeof value === 'number' ? value : NaN);
  return readRecordTypeOwnerPresence(facts.texts, numbers, facts.starts, facts.counts,
    facts.labels, facts.targets, roots.slice(2), roots[0], roots[1]);
}
