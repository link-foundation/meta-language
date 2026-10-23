// Host allocation of existing surface terms; all arithmetic is in the shared plan.
import { unsupported } from './diagnostics.js';
import { planFixedIntegerCastExpression } from './fixed-integer-cast-rules.js';
import { FLOAT, INT } from './types.js';

export function readJavaScriptFixedIntegerCast(parser, name, args, where) {
  if (name !== 'BigInt.asIntN' && name !== 'BigInt.asUintN') return null;
  const width = args[0];
  if (args.length !== 2 || width?.k !== 'num' || width.type !== FLOAT || width.negative) {
    throw unsupported(name, 'the width must be a supported constant Number and the value must have BigInt type', where);
  }
  const plan = planFixedIntegerCastExpression(Number(width.value), name === 'BigInt.asIntN');
  if (!plan.length) throw unsupported(name, 'a width of 8, 16, 32, 64 or 128 is represented; other widths need an integer-domain extension', where);
  const names = [];
  while (names.length < 3) {
    if (!Number.isSafeInteger(parser.recordParameterOrdinal + 1)) throw unsupported(name, 'fresh binding names exhausted their safe ordinal range', where);
    const candidate = 'fixed_integer_cast_value' + parser.recordParameterOrdinal++;
    if (!parser.recordParameterNames.has(candidate)) {
      parser.recordParameterNames.add(candidate); names.push(candidate);
    }
  }
  const terms = [];
  for (let offset = 0; offset < plan.length; offset += 5) {
    const [kind, value, first, second, third] = plan.slice(offset, offset + 5);
    const term = (index) => terms[Number(index)];
    if (kind === 'input') terms.push(args[1]);
    else if (kind === 'integer') terms.push({ k: 'num', value, type: INT, span: where });
    else if (kind === 'reference') terms.push({ k: 'name', path: [names[Number(value)]], span: where });
    else if (kind === 'binary') terms.push({ k: 'binary', op: value, left: term(first), right: term(second), span: where });
    else if (kind === 'conditional') terms.push({ k: 'if', cond: term(first), then: term(second), else: term(third), span: where });
    else if (kind === 'let') terms.push({ k: 'let', name: names[Number(value)], value: term(first), body: term(second), span: where });
    else throw unsupported(name, 'the shared construction instruction must allocate an existing integer surface term', where);
  }
  return terms[terms.length - 1];
}
