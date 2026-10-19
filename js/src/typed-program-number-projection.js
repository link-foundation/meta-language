// Eligible arithmetic operands retain their original guarded unwrapping.
// Other parents preserve the wrapper for the shared Surface conversion plan;
// the whole-program checker must rederive its actual coercion and types.
import { readOptionalNumberOperation } from './translation/optional-value-rules.js';
import { acceptProgramDerivedNumberOperand } from './typed-program-number-projection-rules.js';

export function readLinkedProgramNumberOperand(parent, field, value) {
  if (value?.k !== 'nullishNumber') return value;
  if (!acceptProgramDerivedNumberOperand(parent.k, parent.op, field)) return value;
  const fail = () => { throw new TypeError('a linked derived number operand has inconsistent fields, types or arithmetic context'); };
  const float = type => type !== null && typeof type === 'object' && !Array.isArray(type)
    && Reflect.ownKeys(type).length === 1 && type.kind === 'float';
  if (!float(parent.type) || !float(parent.domain)) fail();
  const keys = Reflect.ownKeys(value);
  if (keys.length !== 3 || keys.some(key => !['k', 'arg', 'type'].includes(key)
    || !Object.hasOwn(Object.getOwnPropertyDescriptor(value, key), 'value')) || !float(value.type)) fail();
  const child = value.arg;
  if (child === null || typeof child !== 'object' || Array.isArray(child)
    || readOptionalNumberOperation('JavaScript', child.type?.kind ?? '', child.type?.element?.kind ?? '') !== 'number') fail();
  return child;
}
