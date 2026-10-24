// Transport actual checked operand shapes to the complete shared decision.
import { acceptReferenceArrayProjectionIndex, readReferenceArrayProjection,
  readReferenceArrayProjectionSuffix, renderReferenceArrayProjection } from './reference-array-projection-rules.js';

function acceptsIndex(expression) {
  if (!expression) return false;
  const children = expression.k === 'binary' ? [expression.left, expression.right]
    : expression.k === 'unary' || expression.k === 'cast' ? [expression.arg]
      : expression.k === 'math' ? expression.args : [];
  return acceptReferenceArrayProjectionIndex(expression.k, expression.type.kind, children.map(acceptsIndex));
}
export function emitReferenceArrayProjection(emitter, expression) {
  const array = expression.array, reference = array?.reference;
  const operation = readReferenceArrayProjection(expression.k, array?.k ?? '', reference?.type.kind ?? '',
    reference?.type.reference === true, expression.type.kind, acceptsIndex(expression.index));
  if (!operation) return null;
  const receiver = emitter.expr(reference), position = expression.index ? emitter.expr(expression.index) : '';
  emitter.temporaries = readReferenceArrayProjectionSuffix(emitter.temporaries + 1, [receiver, position]);
  if (operation === 'length') {
    if (expression.type.kind !== 'float') emitter.usesBig = true;
  } else {
    emitter.usesArray = true;
    if (operation === 'optionalIndex') emitter.usesStandardArray = true;
    else if (!expression.provenInBounds) emitter.state.arrayRead();
  }
  return renderReferenceArrayProjection(operation, receiver, position, expression.index?.type.kind ?? '',
    expression.type.kind, String(emitter.temporaries));
}
