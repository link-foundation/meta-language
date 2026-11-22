// Actual type/field owners remain with the ordinary checker.
import { coerce, bindLocal } from './check.js';
import { unsupported, typeError } from './diagnostics.js';
import { UNIT } from './types.js';
import {javaScriptConstructorFieldValueType, readJavaScriptOwnedRecordField} from './javascript-constructor-presence.js';
import { readRecordFieldWriteOperator, readRecordFieldWriteName, readRecordFieldSourceWriteRefusal, readRecordCompoundBinaryOperation,
  readOptionalRecordCompoundBinaryOperation, readRecordFieldWriteArgumentCount, readRecordFieldWriteIdentity } from './record-write-rules.js';

export function checkRecordFieldWrite(checker, node, object, env, path) {
  const operator = readRecordFieldWriteOperator(node.op);
  if (!operator) return null;
  const name = readRecordFieldWriteName(node.op);
  const field = object.type.kind === 'record' ? object.type.fields.find(field => field.name === name) : undefined;
  const type=field ? javaScriptConstructorFieldValueType(field) : undefined;
  const optionalBinary = field?.optional && type.kind === 'option'
    ? readOptionalRecordCompoundBinaryOperation(checker.language, operator, type.element.kind, type.absence) : '';
  if(field?.optional&&name==='__proto__')throw unsupported('shared record field assignment',
    'a potentially inherited prototype setter requires the actual prototype operation',node.span);
  const refusal = readRecordFieldSourceWriteRefusal(checker.language, operator, object.type.kind, object.type.reference === true,
    object.type.open === true, Boolean(field), optionalBinary ? type.element.kind : type?.kind ?? '');
  if (refusal) throw unsupported('shared record field assignment', refusal, node.span);
  if (node.args.length !== readRecordFieldWriteArgumentCount(operator) || node.args.some(value => !value)) {
    throw typeError('a field assignment must retain every actual operand', node.span);
  }
  const binary = optionalBinary || readRecordCompoundBinaryOperation(checker.language, operator, type.kind);
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
    const previous = capture('ml_field_previous', readJavaScriptOwnedRecordField(checker.referenceValue(owner), field));
    const operandType = optionalBinary ? type.element : type;
    let right = node.args[0] ?? { k: 'num', value: '1', type: operandType };
    const left = { k: 'name', path: [previous.name] };
    if (optionalBinary && operandType.kind === 'string') {
      const operand = checker.expr(right, inner, path, undefined);
      if (operand.type.kind !== 'string') throw unsupported('optional String field addition',
        'an absent left operand needs an actual String right operand to produce a String field value', node.span);
      right = { k: 'name', path: [capture('ml_field_right', operand).name] };
    }
    const operation = binary === 'exponentiation'
      ? { k: 'math', op: 'exponentiateJavaScriptNumericValue', name: '**',
        args: [{ value: left, spread: false }, { value: right, spread: false }], span: node.span }
      : { k: 'binary', op: binary, left, right, span: node.span };
    const calculated = coerce(checker.expr(operation, inner, path, operandType), type, checker.language, node.span);
    let body = { k: 'method', object: owner, op: readRecordFieldWriteIdentity('=', name),
      args: [calculated], type: UNIT, span: node.span };
    for (let index = bindings.length - 1; index >= 0; index -= 1)
      body = { k: 'let', ...bindings[index], body, type: UNIT };
    return body;
  }
  const args = node.args.map(argument => coerce(checker.expr(argument, env, path, type), type,
    checker.language, argument.span ?? node.span));
  return { k: 'method', object, op: node.op, args, type: UNIT, span: node.span };
}

export function checkSourceRecordFieldWrite(checker, node, env, path) {
  const literalKey = checker.language === 'JavaScript' && node.index?.k === 'str';
  if (node.index ? !literalKey : typeof node.field !== 'string') throw unsupported('shared indexed assignment',
    'indexed assignment needs live slots, holes, length and property-key semantics', node.span);
  const object = checker.expr(node.object, env, path, undefined);
  if (literalKey && object.type.kind !== 'record') throw unsupported('shared indexed assignment',
    'indexed assignment needs live slots, holes, length and property-key semantics', node.span);
  const operation = readRecordFieldWriteIdentity(node.op, literalKey ? node.index.value : node.field);
  if (!operation) throw unsupported('shared record field assignment', 'this operator needs its complete state operation', node.span);
  return checkRecordFieldWrite(checker, { ...node, op: operation, args: node.value ? [node.value] : [] }, object, env, path);
}
