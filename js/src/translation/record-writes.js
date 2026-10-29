// Actual type/field owners remain with the ordinary checker.
import { coerce, bindLocal } from './check.js';
import { unsupported, typeError } from './diagnostics.js';
import { UNIT } from './types.js';
import { readRecordFieldWriteOperator, readRecordFieldWriteName, readRecordFieldSourceWriteRefusal, readRecordCompoundBinaryOperation,
  readRecordFieldWriteArgumentCount, readRecordFieldWriteIdentity } from './record-write-rules.js';

export function checkRecordFieldWrite(checker, node, object, env, path) {
  const operator = readRecordFieldWriteOperator(node.op);
  if (!operator) return null;
  const name = readRecordFieldWriteName(node.op);
  const field = object.type.kind === 'record' ? object.type.fields.find(field => field.name === name) : undefined;
  const refusal = readRecordFieldSourceWriteRefusal(checker.language, operator, object.type.kind, object.type.reference === true,
    object.type.open === true, Boolean(field), field?.type.kind ?? '');
  if (refusal) throw unsupported('shared record field assignment', refusal, node.span);
  if (node.args.length !== readRecordFieldWriteArgumentCount(operator) || node.args.some(value => !value)) {
    throw typeError('a field assignment must retain every actual operand', node.span);
  }
  const binary = readRecordCompoundBinaryOperation(checker.language, operator, field.type.kind);
  if (binary) {
    const inner = new Map(env), bindings = [];
    const capture = (prefix, value) => {
      let name;
      do { name = checker.freshName(prefix); } while (inner.has(name));
      bindLocal(inner, name, value.type);
      bindings.push({ name, value });
      return { k: 'var', name, type: value.type };
    };
    const owner = capture('ml_field_owner', object);
    const previous = capture('ml_field_previous', { k: 'recordField', object: owner, field: name,
      index: object.type.fields.indexOf(field), type: field.type });
    const right = node.args[0] ?? { k: 'num', value: '1', type: field.type };
    const left = { k: 'name', path: [previous.name] };
    const operation = binary === 'exponentiation'
      ? { k: 'math', op: 'exponentiateJavaScriptNumericValue', name: '**',
        args: [{ value: left, spread: false }, { value: right, spread: false }], span: node.span }
      : { k: 'binary', op: binary, left, right, span: node.span };
    const calculated = coerce(checker.expr(operation, inner, path, field.type),
    field.type, checker.language, node.span);
    let body = { k: 'method', object: owner, op: readRecordFieldWriteIdentity('=', name),
      args: [calculated], type: UNIT, span: node.span };
    for (let index = bindings.length - 1; index >= 0; index -= 1)
      body = { k: 'let', ...bindings[index], body, type: UNIT };
    return body;
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
