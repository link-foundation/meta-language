// Surface-only wrappers have no trusted metadata escape hatch. Their checked
// result contains only the established Method/Coalesce/Index operations.
import { typeError } from './diagnostics.js';
import { acceptJavaScriptArrayBindingField, acceptJavaScriptArrayBindingOffset, acceptJavaScriptArrayBindingOffsetField } from './array-binding-rules.js';

export function checkJavaScriptArrayBindingWrapper(node) {
  const plain = value => value && typeof value === 'object' && !Array.isArray(value)
    && (Object.getPrototypeOf(value) === Object.prototype || Object.getPrototypeOf(value) === null);
  if (!plain(node) || !Object.hasOwn(node, 'k') || Object.getOwnPropertySymbols(node).length) throw typeError('array binding needs a plain owned surface record');
  const constructor = Object.getOwnPropertyDescriptor(node, 'k');
  if (!Object.hasOwn(constructor, 'value') || typeof constructor.value !== 'string') throw typeError('array binding needs its actual owned constructor');
  const kind = constructor.value;
  for (const field of Object.getOwnPropertyNames(node)) {
    const descriptor = Object.getOwnPropertyDescriptor(node, field);
    if (!descriptor.enumerable || !Object.hasOwn(descriptor, 'value') || !acceptJavaScriptArrayBindingField(kind, field)) {
      throw typeError('array binding contains an unrepresented field', node.span);
    }
  }
  if (Object.hasOwn(node, 'parenthesized') && typeof node.parenthesized !== 'boolean') throw typeError('array binding parentheses need an actual Boolean', node.span);
  const operands = kind === 'arrayBindingRest' ? ['object', 'start'] : ['value', 'fallback'];
  for (const field of operands) {
    if (!Object.hasOwn(node, field) || !plain(node[field]) || typeof node[field].k !== 'string') throw typeError('array binding needs every actual expression operand', node.span);
  }
  if (kind === 'arrayBindingDefault' && node.value.k !== 'index') throw typeError('array binding default needs its actual indexed element read', node.span);
  if (kind === 'arrayBindingRest') {
    const start = node.start;
    for (const field of Reflect.ownKeys(start)) {
      const descriptor = Object.getOwnPropertyDescriptor(start, field);
      if (!acceptJavaScriptArrayBindingOffsetField(field) || !descriptor.enumerable || !Object.hasOwn(descriptor, 'value')) throw typeError('array rest start contains an unrepresented field', node.span);
    }
    const literal = start.k === 'num' && Object.hasOwn(start, 'value') && typeof start.value === 'string'
      && (!Object.hasOwn(start, 'negative') || typeof start.negative === 'boolean');
    const value = literal ? Number(start.value) * (start.negative ? -1 : 1) : NaN;
    if (!acceptJavaScriptArrayBindingOffset(literal, Number.isSafeInteger(value), value < 0)) throw typeError('array rest start needs its actual nonnegative safe integer literal', node.span);
  }
}
