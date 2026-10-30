// Preserve actual source operands; the shared rule selects source capture.
import { acceptReferenceArraySourceOperands } from './reference-array-projection-rules.js';

export function captureArraySourceOperands(checker, array, index, env) {
  if (!acceptReferenceArraySourceOperands(checker.language, array.k, array.reference?.type.kind ?? '',
    array.reference?.type.reference === true)) return { array, index, bindings: [] };
  const inner = new Map(env), bindings = [];
  for (const [prefix, value] of [['ml_array_read_value', array.reference], ['ml_array_read_index', index]]) {
    let name;
    do { name = checker.freshName(prefix); } while (inner.has(name));
    inner.set(name, value.type);
    bindings.push({ name, value });
  }
  return { bindings, array: { ...array, reference: { k: 'var', name: bindings[0].name, type: array.reference.type } },
    index: { k: 'var', name: bindings[1].name, type: index.type } };
}
export function wrapArraySourceOperands(body, bindings) {
  for (let index = bindings.length - 1; index >= 0; index -= 1)
    body = { k: 'let', name: bindings[index].name, value: bindings[index].value, body, type: body.type };
  return body;
}
