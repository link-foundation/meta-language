import { readUnitAlternativeExpressionForm } from '../translation/frontend-rules.js';

// Project the grammar's whole-rule alternatives into the context vectors
// consumed by the shared precedence decision. Grammar rule order identifies
// the same contexts for every vector; source spellings do not decide sharing.
export function completeRuleContexts(program, name) {
  const parents = new Map([...program.rules.keys()].map(rule => [rule, new Set()]));
  for (const [owner, rule] of program.rules) {
    const pending = [rule.expression];
    while (pending.length > 0) {
      const expression = pending.pop();
      const form = readUnitAlternativeExpressionForm(expression.kind);
      if (form === 1) parents.get(expression.name)?.add(owner);
      else if (form === 2) pending.push(...expression.items);
      else if (form === 3) pending.push(expression.item);
    }
  }
  const reached = new Set();
  const pending = [name];
  while (pending.length > 0) {
    const current = pending.pop();
    if (reached.has(current)) continue;
    reached.add(current);
    pending.push(...parents.get(current) ?? []);
  }
  return [...program.rules.keys()].map(rule => reached.has(rule));
}
