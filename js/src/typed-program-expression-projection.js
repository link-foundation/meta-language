// Host JSON boundary adapter. Projection instructions and admission decisions
// are shared data; the existing checker still owns types and semantics.
import { TYPED_PROGRAM_LINKS_SCHEMA } from './typed-program-links.js';
import { acceptProgramExpressionOperation, readProgramExpressionProjection } from './typed-program-link-rules.js';

export function projectLinkedProgramExpression(value) {
  const plans = TYPED_PROGRAM_LINKS_SCHEMA.expressionProjections;
  if (Object.hasOwn(TYPED_PROGRAM_LINKS_SCHEMA.expressionOperations, value?.k)
    && !acceptProgramExpressionOperation(value.k, value.op)) throw new TypeError('the linked program contains an unrecognized expression operation');
  let key = value?.k;
  const base = plans[key];
  if (base?.variant) {
    const discriminator = base.variant.reduce((current, field) => current?.[field], value);
    const variant = `${key}:${discriminator}`;
    if (readProgramExpressionProjection(variant)) key = variant;
  }
  if (!readProgramExpressionProjection(key ?? '') || !plans[key].construct) throw new TypeError(`the linked expression ${value?.k} has no surface projection`);
  const plan = plans[key];
  const result = { k: plan.construct, ...(value.span !== undefined ? { span: value.span } : {}) };
  const operation = plan.renames?.[value.op] ?? value.op;
  for (const [target, mode, source, right] of plan.fields) {
    const child = value[source];
    if (mode === 'copy') { if (child !== undefined) result[target] = child; }
    else if (mode === 'expression') result[target] = projectLinkedProgramExpression(child);
    else if (mode === 'expressions') result[target] = child.map(projectLinkedProgramExpression);
    else if (mode === 'name-path') result[target] = [child];
    else if (mode === 'function-path') result[target] = { k: 'name', path: [child] };
    else if (mode === 'array-items') result[target] = child.map((item) => ({ value: projectLinkedProgramExpression(item), spread: plan.spreadOperations?.includes(value.op) ?? false }));
    else if (mode === 'spread-pair') result[target] = [child, value[right]].map((item) => ({ value: projectLinkedProgramExpression(item), spread: true }));
    else if (mode === 'element-type') result[target] = child.element;
    else if (mode === 'binding-type') result[target] = child.type;
    else if (mode === 'integer-type') result[target] = child.kind !== 'float';
    else if (mode === 'operation') result[target] = operation;
    else if (mode === 'operation-name') result[target] = `Math.${operation}`;
    else if (mode === 'constant') result[target] = source;
    else throw new TypeError(`unknown shared projection instruction ${mode}`);
  }
  return result;
}
