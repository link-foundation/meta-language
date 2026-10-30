// Ordinary source/type ownership stays with the existing checker and emitters.
import { typeError } from './diagnostics.js';
import { sameType } from './types.js';
import { readRustPrimitiveBitwiseArity, acceptRustPrimitiveBitwiseType,
  readRustPrimitiveBitwiseIntegerOperation, renderRustPrimitiveBitwiseIntegerArgument,
  renderRustPrimitiveBitwiseExpression } from './rust-primitive-bitwise-rules.js';
import { renderBigIntegerBitwiseExpression } from './big-integer-bitwise-rules.js';
import { readFixedIntegerCastModulus } from './fixed-integer-cast-rules.js';

export function checkRustPrimitiveBitwise(checker, node, env, path, expected) {
  const arity = readRustPrimitiveBitwiseArity(node.op);
  if (checker.language !== 'Rust' || node.args.length !== arity || node.args.some(argument => argument.spread)) throw typeError('a Rust bit operation requires its actual unspread primitive operands', node.span);
  let left = checker.expr(node.args[0].value, env, path, expected, true);
  let right = arity === 2 ? checker.expr(node.args[1].value, env, path, left.type.kind === 'literal' ? expected : left.type, true) : null;
  if (left.type.kind === 'literal') left = checker.expr(node.args[0].value, env, path, right && right.type.kind !== 'literal' ? right.type : expected ?? checker.defaultNumber());
  if (right?.type.kind === 'literal') right = checker.expr(node.args[1].value, env, path, left.type);
  if (!acceptRustPrimitiveBitwiseType(left.type.kind, left.type.bits ?? 0) || right && !sameType(left.type, right.type)) throw typeError('a Rust bit operation requires matching actual Boolean or fixed integer types', node.span);
  return { k: 'math', op: node.op, args: right ? [left, right] : [left], type: left.type };
}

export function emitRustPrimitiveBitwise(target, expression, emit) {
  const type = expression.type, args = expression.args.map(emit);
  const integer = type.kind === 'fixed' && (target === 'Lean' || target === 'Rocq')
    ? renderBigIntegerBitwiseExpression(target, readRustPrimitiveBitwiseIntegerOperation(expression.op),
      args.map(argument => renderRustPrimitiveBitwiseIntegerArgument(target, type.signed, argument))) : '';
  return renderRustPrimitiveBitwiseExpression(target, expression.op, type.kind, type.bits ?? 0,
    type.signed === true, args, integer, type.kind === 'fixed' ? readFixedIntegerCastModulus(type.bits) : '');
}
