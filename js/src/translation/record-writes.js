// Actual type/field owners remain with the ordinary checker.
import { coerce } from './check.js';
import { unsupported, typeError } from './diagnostics.js';
import { UNIT } from './types.js';
import { readRecordFieldWriteOperator, readRecordFieldWriteName, readRecordFieldWriteRefusal,
  readRecordFieldWriteArgumentCount, readRecordFieldWriteIdentity } from './record-write-rules.js';

export function checkRecordFieldWrite(checker, node, object, env, path) {
  const operator = readRecordFieldWriteOperator(node.op);
  if (!operator) return null;
  const name = readRecordFieldWriteName(node.op);
  const field = object.type.kind === 'record' ? object.type.fields.find(field => field.name === name) : undefined;
  const refusal = readRecordFieldWriteRefusal(operator, object.type.kind, object.type.reference === true,
    object.type.open === true, Boolean(field), field?.type.kind ?? '');
  if (refusal) throw unsupported('shared record field assignment', refusal, node.span);
  if (node.args.length !== readRecordFieldWriteArgumentCount(operator) || node.args.some(value => !value)) {
    throw typeError('a field assignment must retain every actual operand', node.span);
  }
  const args = node.args.map(argument => coerce(checker.expr(argument, env, path, field.type), field.type,
    checker.language, argument.span ?? node.span));
  return { k: 'method', object, op: node.op, args, type: UNIT, span: node.span };
}

export function checkSourceRecordFieldWrite(checker, node, env, path) {
  if (typeof node.field !== 'string' || node.index) throw unsupported('shared indexed assignment',
    'indexed assignment needs live slots, holes, length and property-key semantics', node.span);
  const object = checker.expr(node.object, env, path, undefined);
  const operation = readRecordFieldWriteIdentity(node.op, node.field);
  if (!operation) throw unsupported('shared record field assignment', 'this operator needs its complete state operation', node.span);
  return checkRecordFieldWrite(checker, { ...node, op: operation, args: node.value ? [node.value] : [] }, object, env, path);
}
