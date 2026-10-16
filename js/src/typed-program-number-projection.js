// A checked numeric coercion has no independent Surface constructor. Only its
// actual arithmetic parent may ask the existing checker to derive it again.
import { readOptionalNumberOperation } from './translation/optional-value-rules.js';
import { acceptProgramDerivedNumberOperand } from './typed-program-number-projection-rules.js';

export function readLinkedProgramNumberOperand(parent, field, value) {
  if (value?.k !== 'nullishNumber') return value;
  const fail = () => { throw new TypeError('a linked derived number operand has inconsistent fields, types or arithmetic context'); };
  const float = type => type !== null && typeof type === 'object' && !Array.isArray(type)
    && Reflect.ownKeys(type).length === 1 && type.kind === 'float';
  if (!acceptProgramDerivedNumberOperand(parent.k, parent.op, field) || !float(parent.type) || !float(parent.domain)) fail();
  const keys = Reflect.ownKeys(value);
  if (keys.length !== 3 || keys.some(key => !['k', 'arg', 'type'].includes(key)
    || !Object.hasOwn(Object.getOwnPropertyDescriptor(value, key), 'value')) || !float(value.type)) fail();
  const child = value.arg;
  if (child === null || typeof child !== 'object' || Array.isArray(child)
    || readOptionalNumberOperation('JavaScript', child.type?.kind ?? '', child.type?.element?.kind ?? '') !== 'number') fail();
  return child;
}
