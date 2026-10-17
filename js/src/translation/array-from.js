// Mechanical transport preserves the original inference terms and aliases.
import { createSurfaceReferenceFacts } from './surface-reference-facts.js';
import { STRING } from './types.js';
import {
  planArrayFromCallbackConstraints, planArrayFromExpectedElement, planArrayFromInferenceInputs,
  planArrayFromIterableType, planArrayFromMapCallback, readArrayFromElement, readArrayFromCallbackError,
} from './array-from-rules.js';

function arrayFromFacts(values) {
  const facts = createSurfaceReferenceFacts(values, { retainValues: true });
  return { facts, columns: [facts.texts, facts.starts, facts.counts, facts.labels, facts.targets],
    ids: values.map((_, index) => readArrayFromElement(facts.starts, facts.counts, facts.targets, facts.root, index)) };
}

export function readArrayFromIterableType(object) {
  const { facts, columns, ids } = arrayFromFacts([object]);
  const plan = planArrayFromIterableType(...columns, ids[0]);
  if (!plan.length) return null;
  const element = plan[0] === 1 ? STRING : plan[0] === 3
    ? { kind: 'tuple', elements: [facts.values[plan[1]], facts.values[plan[2]]] }
    : facts.values[plan[1]];
  return { form: plan[0], element };
}

export function readArrayFromExpectedElement(expected) {
  const { facts, columns, ids } = arrayFromFacts([expected]);
  const plan = planArrayFromExpectedElement(...columns, ids[0]);
  return { selected: plan[0] === 1, element: facts.values[plan[1]] };
}

export function readArrayFromInferenceInputs(args, expected) {
  const { facts, columns, ids } = arrayFromFacts([args, expected]);
  const plan = planArrayFromInferenceInputs(...columns, ids[0], ids[1]);
  return { selected: plan[0] === 1, element: facts.values[plan[1]], object: facts.values[plan[2]], mapper: facts.values[plan[3]] };
}

export function readArrayFromCallbackConstraints(term, declared, params, ret) {
  const { facts, columns, ids } = arrayFromFacts([term, declared, params, ret]);
  const plan = planArrayFromCallbackConstraints(...columns, ...ids);
  if (plan[0] === -1) return { error: readArrayFromCallbackError(declared.params.length, params.length), pairs: [] };
  if (plan[0] === 1) return { error: '', pairs: [[facts.values[plan[1]], { kind: 'function', params: facts.values[plan[2]], ret: facts.values[plan[3]] }]] };
  const pairs = [];
  for (let at = 1; at < plan.length; at += 2) pairs.push([facts.values[plan[at]], facts.values[plan[at + 1]]]);
  return { error: '', pairs };
}

export function readArrayFromMapCallback(callback, params) {
  const { facts, columns, ids } = arrayFromFacts([callback, params]);
  const plan = planArrayFromMapCallback(...columns, ...ids);
  const pairs = [];
  for (let at = 2; at < plan.length; at += 2) pairs.push([facts.values[plan[at]], facts.values[plan[at + 1]]]);
  return { status: plan[0], ret: facts.values[plan[1]], pairs };
}
