// Checked operand allocation; shared rules own admission and operation roles.
import { coerce } from './check.js';
import { unsupported } from './diagnostics.js';
import { BOOL, FLOAT, UNIT } from './types.js';
import { readReferenceMethodResult, readReferenceMethodRequiredArguments,
  readReferenceMethodArgumentRole, acceptReferenceMethodKeyKind } from './reference-method-rules.js';
import { readReferenceMethodRefusal } from './reference-method-rules.js';

export function checkReferenceMethod(checker, node, object, env, path) {
  const kind = object.type.kind, operation = node.op;
  const refusal = object.type.reference === true ? readReferenceMethodRefusal(kind, operation) : '';
  if (refusal) throw unsupported('mutable collection iteration', refusal, node.span);
  const result = object.type.reference === true ? readReferenceMethodResult(kind, operation) : '';
  if (!result) return null;
  const key = kind === 'map' ? object.type.key : object.type.element;
  if ((kind === 'map' || kind === 'set') && !acceptReferenceMethodKeyKind(key.kind)) {
    throw unsupported('shared collection key', 'this key needs its explicit SameValueZero and identity encoding', node.span);
  }
  if (node.args.length < readReferenceMethodRequiredArguments(kind, operation)) {
    throw unsupported('shared collection method arguments', 'missing arguments need explicit undefined key and value domains', node.span);
  }
  const args = node.args.map((argument, index) => {
    if (!argument) throw unsupported('shared collection method argument', 'every source argument needs its actual expression', node.span);
    const role = readReferenceMethodArgumentRole(kind, operation, index);
    const type = role === 'key' ? key : role === 'value' ? object.type.value : role === 'element' ? object.type.element : undefined;
    const value = checker.expr(argument, env, path, type);
    return type ? coerce(value, type, checker.language, argument.span ?? node.span) : value;
  });
  const type = result === 'receiver' ? object.type : result === 'length' ? FLOAT : result === 'boolean' ? BOOL
    : result === 'unit' ? UNIT : { kind: 'option', element: result === 'optional element' ? object.type.element : object.type.value, absence: 'undefined' };
  return { k: 'method', object, op: operation, args, type, span: node.span };
}
