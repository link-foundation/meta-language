// Mechanical transport for the portable construction plans. The ordinary
// inference engine/checker retain term identity, coercion and evaluation order.
import { createSurfaceReferenceFacts } from './surface-reference-facts.js';
import { array } from './types.js';
import { typeError, unsupported } from './diagnostics.js';
import { acceptOptionalPrimitiveKind } from './optional-value-rules.js';
import {
  planCollectionConstructionChecked, planCollectionConstructionInference,
  readCollectionConstructionDiagnostic, readCollectionConstructionElement,
  readCollectionConstructionKeyAdmission,
} from './collection-construction-rules.js';

function collectionFacts(value) {
  const facts = createSurfaceReferenceFacts([value], { retainValues: true });
  return { facts, columns: [facts.texts, facts.starts, facts.counts, facts.labels, facts.targets],
    identity: readCollectionConstructionElement(facts.starts, facts.counts, facts.targets, facts.root, 0) };
}

export function readCollectionConstructionContext(kind, expected) {
  const { facts, columns, identity } = collectionFacts(expected);
  const plan = planCollectionConstructionInference(...columns, kind, identity);
  return { form: plan[0], context: plan[1] === 1, first: facts.values[plan[2]], second: facts.values[plan[3]] };
}

export function inferCollectionConstruction(inference, node, env, expected) {
  const selected = readCollectionConstructionContext(node.k, expected);
  const first = selected.context ? selected.first : inference.fresh();
  if (selected.form === 1) {
    if (node.items) inference.unify(inference.expr(node.items, env, { kind: 'array', element: first }), { kind: 'array', element: first }, node.span);
    const term = { kind: 'set', element: first };
    inference.collections.set(node, term);
    return term;
  }
  const second = selected.context ? selected.second : inference.fresh();
  const pair = { kind: 'tuple', elements: [first, second] };
  if (node.entries) inference.unify(inference.expr(node.entries, env, { kind: 'array', element: pair }), { kind: 'array', element: pair }, node.span);
  const term = { kind: 'map', key: first, value: second };
  inference.collections.set(node, term);
  return term;
}

export function checkCollectionConstruction(checker, node, env, path, expected) {
  const want = node.type ? checker.resolveType(node.type, path, node.span) : expected;
  const selected = readCollectionConstructionContext(node.k, want);
  const element = selected.form === 1 ? selected.first : { kind: 'tuple', elements: [selected.first, selected.second] };
  const items = checker.referenceValue(checker.expr(selected.form === 1 ? node.items : node.entries, env, path, selected.context ? array(element) : undefined));
  const { facts, columns, identity } = collectionFacts(items.type);
  const plan = planCollectionConstructionChecked(...columns, node.k, identity);
  if (plan[0]) throw typeError(readCollectionConstructionDiagnostic(plan[0], 'message'), node.span);
  const first = facts.values[plan[1]], second = facts.values[plan[2]];
  const error = readCollectionConstructionKeyAdmission(node.k, acceptOptionalPrimitiveKind(first.kind));
  if (error) throw unsupported(readCollectionConstructionDiagnostic(error, 'construct'), readCollectionConstructionDiagnostic(error, 'message'), node.span);
  return selected.form === 1 ? { k: 'set', items, type: { kind: 'set', element: first } }
    : { k: 'map', entries: items, type: { kind: 'map', key: first, value: second } };
}
