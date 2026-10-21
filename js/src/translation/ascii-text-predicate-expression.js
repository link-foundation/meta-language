// Transport the shared finite sets through existing checked expression forms.
// Standard intrinsics and successful allocation remain explicit premises.
import { BOOL, STRING } from './types.js';

export function createAsciiTextPredicateExpression(parser, input, mode, scalars, span) {
  let ordinal = 0, name;
  do { name = 'ascii_predicate_points' + ordinal++; } while (parser.reservedNames.has(name));
  parser.reservedNames.add(name);
  if (mode === 'first') {
    // At most 128 standard queries, independent of the input string length.
    const alternatives = scalars.map(scalar => ({ k: 'stringTest', op: 'startsWith',
      object: { k: 'name', path: [name], span },
      search: { k: 'str', value: String.fromCharCode(scalar), span }, span }));
    const body = alternatives.length ? alternatives.reduce((left, right) => ({
      k: 'binary', op: 'or', left, right, span,
    })) : { k: 'bool', value: false, span };
    return { k: 'let', name, value: input, body, span };
  }
  let pointOrdinal = 0, pointName;
  do { pointName = 'ascii_predicate_point' + pointOrdinal++; } while (parser.reservedNames.has(pointName));
  parser.reservedNames.add(pointName);
  const callback = {
    k: 'lambda', params: [{ name: pointName, type: STRING, span }], ret: BOOL, captures: [],
    body: { k: 'stringTest', op: 'includes',
      object: { k: 'str', value: String.fromCharCode(...scalars), span },
      search: { k: 'name', path: [pointName], span }, span }, span,
  };
  // This is explicitly eager O(n) point storage; it does not claim the source
  // lazy Iterator::all allocation or failure behavior. Callback syntax is pure.
  return { k: 'let', name, value: { k: 'stringPoints', object: input, span },
    body: { k: 'method', object: { k: 'name', path: [name], span },
      op: 'every', args: [callback], span }, span };
}
