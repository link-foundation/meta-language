// Mechanical term transport for the portable optional constraint plans.
// Solving the returned pairs remains the responsibility of the common unifier.
import { createSurfaceReferenceFacts } from './surface-reference-facts.js';
import {requireJavaScriptAbsenceOnlyValue} from './absence-only-rules.js';
import {
  planInferenceTermIdentity, planOptionalAnnotationConstraints, planOptionalAdditionalAbsence, planOptionalCoalescingConstraints, planOptionalFieldInference,
  planOptionalJoinConstraints, planOptionalTypeConstraints,
} from './optional-inference-rules.js';

function optionalFacts(value) {
  return createSurfaceReferenceFacts(value, { retainValues: true });
}

function optionalChild(facts, name) {
  for (let at = facts.starts[facts.root]; at < facts.starts[facts.root] + facts.counts[facts.root]; at += 1) {
    if (facts.labels[at] === name) return facts.targets[at];
  }
  return -1;
}

function optionalArrays(facts) {
  return [facts.texts, facts.starts, facts.counts, facts.labels, facts.targets];
}

export function readOptionalTypeConstraints(left, right) {
  const facts = optionalFacts({ left, right });
  const plan = planOptionalTypeConstraints(...optionalArrays(facts), optionalChild(facts, 'left'), optionalChild(facts, 'right'));
  if (!plan.length) return null;
  return { pair: [facts.values[plan[0]], facts.values[plan[1]]], absenceOwners: plan.slice(2).filter(identity => identity >= 0).map(identity => facts.values[identity]) };
}

export function readOptionalCoalescingConstraints(left, resolved, right, force = false) {
  if(requireJavaScriptAbsenceOnlyValue('JavaScript',resolved.kind,resolved.element?.kind??''))return [right,right];
  const facts = optionalFacts({ left, resolved, right });
  const plan = planOptionalCoalescingConstraints(...optionalArrays(facts), optionalChild(facts, 'left'), optionalChild(facts, 'resolved'), optionalChild(facts, 'right'), force);
  if (!plan.length) return null;
  const second = facts.values[plan[2]];
  return [facts.values[plan[1]], plan[0] === 1 ? { kind: 'option', element: second, absence: 'both' } : second];
}

export function readOptionalFieldInference(object, resolved) {
  const facts = optionalFacts({ object, resolved });
  const plan = planOptionalFieldInference(...optionalArrays(facts), optionalChild(facts, 'object'), optionalChild(facts, 'resolved'));
  return { element: facts.values[plan[0]], bind: facts.values[plan[1]] };
}

export function readOptionalJoinConstraints(left, right, resolvedLeft, resolvedRight, current) {
  const facts = optionalFacts({ left, right, resolvedLeft, resolvedRight, current });
  const plan = planOptionalJoinConstraints(...optionalArrays(facts), optionalChild(facts, 'left'), optionalChild(facts, 'right'), optionalChild(facts, 'resolvedLeft'), optionalChild(facts, 'resolvedRight'), optionalChild(facts, 'current'));
  const selected = facts.values[plan[1]];
  return {
    pair: plan[0] === 0 ? [facts.values[plan[2]], facts.values[plan[3]]] : null,
    value: plan[0] === 2 ? { kind: 'option', element: selected, absence: 'undefined' } : selected,
  };
}

export function readOptionalAdditionalAbsence(actual, declared) {
  const facts = optionalFacts({ actual, declared });
  const plan = planOptionalAdditionalAbsence(...optionalArrays(facts), optionalChild(facts, 'actual'), optionalChild(facts, 'declared'));
  const pairs = [];
  for (let at = 1; at < plan.length; at += 2) pairs.push([facts.values[plan[at]], facts.values[plan[at + 1]]]);
  return { additional: plan[0] === 1, pairs };
}


export function readInferenceTermIdentity(left, right) {
  const facts = optionalFacts({ left, right });
  const plan = planInferenceTermIdentity(...optionalArrays(facts), optionalChild(facts, 'left'), optionalChild(facts, 'right'));
  if (!plan.length) return null;
  const pairs = [];
  for (let at = 1; at < plan.length; at += 2) pairs.push([facts.values[plan[at]], facts.values[plan[at + 1]]]);
  return pairs;
}


export function acceptOptionalAnnotationTypes(actual, declared) {
  const facts = optionalFacts({ actual, declared });
  const plan = planOptionalAnnotationConstraints(...optionalArrays(facts), optionalChild(facts, 'actual'), optionalChild(facts, 'declared'));
  if (!plan.length) return false;
  for (let at = 1; at < plan.length; at += 2) if (!acceptOptionalAnnotationTypes(facts.values[plan[at]], facts.values[plan[at + 1]])) return false;
  return true;
}
