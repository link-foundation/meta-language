// Mechanical transport and materialization for the closed callable rules.
// Algorithms receive only primitive fact arrays, never callbacks or host ASTs.
import { createSurfaceReferenceFacts } from './surface-reference-facts.js';
import { planCallableCaptureBindings } from './callable-emission-rules.js';
import {
  planCallableApplicationConstraints, planCallableClosureScope, planCallableFunctionType,
  planCallableLambdaInference, planCallableParameterOwnership, planCallableParameterTypes,
  planCallableStatementCaptures, planCallableSurfaceParameters, planCallableTypeConstraints, readCallableCaptureConflicts,
  readCallableFreeReferences, readCallableFunctionChildren, readCallableStatementUses,
  planCallableMutableCells, planCallableCellElement, planCallableCellOptionBinding, planCallableCellReadInference,
  planCallableSourceRenaming, readCallableRenamingConstant, readCallableRenamingField,
} from './callable-source-rules.js';

function factsFor(value) {
  return createSurfaceReferenceFacts(value, { retainValues: true });
}

function child(facts, node, name) {
  for (let at = facts.starts[node]; at < facts.starts[node] + facts.counts[node]; at += 1) {
    if (facts.labels[at] === name) return facts.targets[at];
  }
  return -1;
}

function arrays(facts) {
  return [facts.forms, facts.texts, facts.starts, facts.counts, facts.labels, facts.targets];
}

export function readCallableStatementUsage(statements, declared) {
  const facts = factsFor(statements);
  const events = readCallableStatementUses(...arrays(facts), facts.root, [...declared]);
  const free = new Set(), assigned = new Set();
  for (let at = 0; at < events.length; at += 2) {
    if (events[at] === 0) free.add(facts.values[events[at + 1]]);
    if (events[at] === 1) assigned.add(facts.values[events[at + 1]]);
  }
  return { free, assigned };
}

export function readCallableClosureConflicts(statements) {
  const facts = factsFor(statements);
  const plan = readCallableCaptureConflicts(...arrays(facts), facts.root);
  const conflicts = [];
  for (let at = 0; at < plan.length; at += 2) conflicts.push({ closure: facts.values[plan[at]], capture: facts.values[plan[at + 1]] });
  return conflicts;
}

export function readCallableCapturedBindings(body, names) {
  const facts = factsFor(body);
  const references = readCallableFreeReferences(...arrays(facts), facts.root, names, true);
  const captured = new Map();
  for (const reference of planCallableCaptureBindings(facts.texts, facts.starts, facts.counts, facts.labels, facts.targets, references)) {
    const node = facts.values[reference];
    captured.set(node.name, node.type);
  }
  return captured;
}

export function readCallableParameterTypes(parameters, expected) {
  const facts = factsFor({ parameters, expected });
  const plan = planCallableParameterTypes(...arrays(facts), child(facts, facts.root, 'parameters'), child(facts, facts.root, 'expected'));
  const result = [];
  for (let at = 0; at < plan.length; at += 3) result.push({ parameter: facts.values[plan[at]], type: facts.values[plan[at + 1]], declared: plan[at + 2] === 1 });
  return result;
}

export function constructCallableFunctionType(parameters, ret) {
  const facts = factsFor({ parameters, ret });
  const plan = planCallableFunctionType(facts.starts, facts.counts, facts.labels, facts.targets, child(facts, facts.root, 'parameters'), child(facts, facts.root, 'ret'));
  return { kind: 'function', params: plan.slice(1).map(identity => facts.values[identity]), ret: facts.values[plan[0]] };
}

export function readCallableFunctionConstraints(left, right) {
  const facts = factsFor({ left, right });
  const plan = planCallableTypeConstraints(facts.starts, facts.counts, facts.labels, facts.targets, child(facts, facts.root, 'left'), child(facts, facts.root, 'right'));
  if (plan.length === 1 && plan[0] === -1) return null;
  const pairs = [];
  for (let at = 0; at < plan.length; at += 2) pairs.push([facts.values[plan[at]], facts.values[plan[at + 1]]]);
  return pairs;
}

export function readCallableApplicationConstraints(fn, args, candidate) {
  const facts = factsFor({ fn, args, candidate });
  const plan = planCallableApplicationConstraints(facts.texts, facts.starts, facts.counts, facts.labels, facts.targets, child(facts, facts.root, 'fn'), child(facts, facts.root, 'args'), child(facts, facts.root, 'candidate'));
  const pairs = [];
  for (let at = 1; at < plan.length; at += 2) pairs.push([facts.values[plan[at]], facts.values[plan[at + 1]]]);
  return { ret: plan[0] === -1 ? undefined : facts.values[plan[0]], pairs };
}

export function readCallableLambdaInference(lambda, expected) {
  const facts = factsFor({ lambda, expected });
  const plan = planCallableLambdaInference(...arrays(facts), child(facts, facts.root, 'lambda'), child(facts, facts.root, 'expected'));
  const params = [];
  for (let at = 2; at < plan.length; at += 3) params.push({ parameter: facts.values[plan[at]], type: facts.values[plan[at + 1]], declared: plan[at + 2] === 1 });
  return { ret: facts.values[plan[0]], declaredRet: plan[1] === 1, params };
}

export function readCallableOwnedParameters(lambda) {
  const facts = factsFor(lambda);
  const plan = planCallableParameterOwnership(facts.starts, facts.counts, facts.labels, facts.targets, facts.root);
  const result = [];
  for (let at = 0; at < plan.length; at += 3) result.push({ owner: facts.values[plan[at]], index: plan[at + 1], parameter: facts.values[plan[at + 2]] });
  return result;
}

export function readCallableTypeChildren(type) {
  const facts = factsFor(type);
  return readCallableFunctionChildren(facts.starts, facts.counts, facts.labels, facts.targets, facts.root).map(identity => facts.values[identity]);
}

export function readCallableClosureScope(outer, tokens) {
  const facts = factsFor({ outer: { locals: [...outer.locals], tdz: [...outer.tdz], mutable: [...outer.mutable] }, tokens });
  const plan = planCallableClosureScope(facts.texts, facts.starts, facts.counts, facts.labels, facts.targets, child(facts, facts.root, 'outer'), child(facts, facts.root, 'tokens'));
  let at = 3;
  const locals = new Set(plan.slice(at, at + plan[0]).map(identity => facts.values[identity]));
  at += plan[0];
  const tdz = new Set(plan.slice(at, at + plan[1]).map(identity => facts.values[identity]));
  at += plan[1];
  const mutable = new Set(plan.slice(at, at + plan[2]).map(identity => facts.values[identity]));
  return { locals, tdz, mutable };
}

export function constructCallableSurfaceParameters(tokens) {
  const facts = factsFor(tokens);
  const plan = planCallableSurfaceParameters(facts.starts, facts.counts, facts.labels, facts.targets, facts.root);
  const params = [];
  for (let at = 0; at < plan.length; at += 3) params.push({ name: facts.values[plan[at]], type: null, span: { start: facts.values[plan[at + 1]], end: facts.values[plan[at + 2]] } });
  return params;
}

export function readCallableStatementCaptures(statements, parameters, outer) {
  const facts = factsFor(statements);
  const plan = planCallableStatementCaptures(...arrays(facts), facts.root, parameters, [...outer]);
  return {
    assignedCaptures: plan.slice(1, 1 + plan[0]).map(identity => facts.values[identity]),
    captures: plan.slice(1 + plan[0]).map(identity => facts.values[identity]),
  };
}

export function readCallableMutableCellNames(statements) {
  const facts = factsFor(statements);
  return new Set(planCallableMutableCells(...arrays(facts), facts.root).map(identity => facts.values[identity]));
}

export function readCallableCellElement(node, expected) {
  const facts = factsFor({ node, expected });
  const plan = planCallableCellElement(...arrays(facts), child(facts, facts.root, 'node'), child(facts, facts.root, 'expected'));
  return { type: facts.values[plan[0]], declared: plan[1] === 1 };
}

export function readCallableCellReadInference(cell, resolved) {
  const facts = factsFor({ cell, resolved });
  const plan = planCallableCellReadInference(facts.texts, facts.starts, facts.counts, facts.labels, facts.targets, child(facts, facts.root, 'cell'), child(facts, facts.root, 'resolved'));
  return { element: facts.values[plan[0]], bind: facts.values[plan[1]] };
}

export function readCallableCellOptionBinding(resolved, value) {
  const facts = factsFor({ resolved, value });
  const plan = planCallableCellOptionBinding(facts.texts, facts.starts, facts.counts, facts.labels, facts.targets, child(facts, facts.root, 'resolved'), child(facts, facts.root, 'value'));
  return plan.length ? { variable: facts.values[plan[0]], value: facts.values[plan[1]] } : null;
}

export function renameCallableSourceExpression(node, scope) {
  const facts = factsFor({ node, scope });
  const plan = planCallableSourceRenaming(...arrays(facts), child(facts, facts.root, 'node'), child(facts, facts.root, 'scope'));
  let position = 0;
  function materialize() {
    const operation = plan[position++];
    if (operation === 0) return facts.values[plan[position++]];
    if (operation === 3) return readCallableRenamingConstant(plan[position++]);
    if (operation === 1) {
      const count = plan[position++];
      return Array.from({ length: count }, materialize);
    }
    if (operation !== 2) throw new TypeError('the callable reconstruction plan has an unknown instruction');
    const source = plan[position++];
    const count = plan[position++];
    const result = {};
    if (source >= 0) {
      for (let edge = facts.starts[source]; edge < facts.starts[source] + facts.counts[source]; edge += 1) result[facts.labels[edge]] = facts.values[facts.targets[edge]];
    }
    for (let at = 0; at < count; at += 1) {
      const field = plan[position++];
      const label = field < 0 ? readCallableRenamingField(-field - 1) : facts.labels[field];
      if (!label) throw new TypeError('the callable reconstruction plan has an unknown field');
      result[label] = materialize();
    }
    return result;
  }
  const result = materialize();
  if (position !== plan.length) throw new TypeError('the callable reconstruction plan has trailing instructions');
  return result;
}
