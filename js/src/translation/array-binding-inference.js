// Original-term transport only. The shared plan chooses endpoints; the common
// unifier retains variable identity, occurs checks and ordinary type errors.
import { createSurfaceReferenceFacts } from './surface-reference-facts.js';
import { planJavaScriptArrayDefaultInference } from './array-binding-rules.js';

export function readJavaScriptArrayDefaultConstraints(resolved, fallback, result) {
  const facts = createSurfaceReferenceFacts({ resolved, fallback, result }, { retainValues: true });
  const child = name => {
    for (let at = facts.starts[facts.root]; at < facts.starts[facts.root] + facts.counts[facts.root]; at += 1) {
      if (facts.labels[at] === name) return facts.targets[at];
    }
    return -1;
  };
  const plan = planJavaScriptArrayDefaultInference(facts.texts, facts.starts, facts.counts, facts.labels, facts.targets, child('resolved'), child('fallback'), child('result'));
  if (!plan.length) return null;
  if (plan[0] < 0) return { refused: true, pairs: [] };
  const pairs = [];
  for (let at = 0; at < plan.length; at += 2) pairs.push([facts.values[plan[at]], facts.values[plan[at + 1]]]);
  return { refused: false, pairs };
}
