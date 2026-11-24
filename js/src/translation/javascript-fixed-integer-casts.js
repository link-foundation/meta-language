// Host allocation of existing surface terms; all arithmetic is in the shared plan.
import { unsupported } from './diagnostics.js';
import { planJavaScriptFixedIntegerCastExpression, planJavaScriptFixedIntegerArguments } from './javascript-fixed-integer-cast-plan.js';
import { FLOAT, INT } from './types.js';

export function readJavaScriptFixedIntegerCast(parser, name, args, where) {
  if (name !== 'BigInt.asIntN' && name !== 'BigInt.asUintN') return null;
  let width = args[0], sign = 1;
  for(let depth=0;depth<64&&width?.k==='unary'&&width.op==='neg';depth+=1){sign=-sign;width=width.arg;}
  if (args.length < 2 || args.length > 4096 || width?.k !== 'num' || width.type !== FLOAT) {
    throw unsupported(name, 'the width must be a supported constant Number and the value must have BigInt type', where);
  }
  const plan = planJavaScriptFixedIntegerCastExpression(Number(width.value)*(width.negative?-sign:sign), name === 'BigInt.asIntN');
  if (!plan.length) throw unsupported(name, 'constant widths whose ToIndex result is between 0 and 4096 are represented; other widths require their complete conversion and resource model', where);
  const captures = planJavaScriptFixedIntegerArguments(args.length), names = [];
  while (names.length < 3 + captures.length) {
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
    if (kind === 'input') terms.push(captures.length ? {k:'name',path:[names[3]],span:where} : args[1]);
    else if (kind === 'integer') terms.push({ k: 'num', value, type: INT, span: where });
    else if (kind === 'reference') terms.push({ k: 'name', path: [names[Number(value)]], span: where });
    else if (kind === 'binary') terms.push({ k: 'binary', op: value, left: term(first), right: term(second), span: where });
    else if (kind === 'conditional') terms.push({ k: 'if', cond: term(first), then: term(second), else: term(third), span: where });
    else if (kind === 'let') terms.push({ k: 'let', name: names[Number(value)], value: term(first), body: term(second), span: where });
    else throw unsupported(name, 'the shared construction instruction must allocate an existing integer surface term', where);
  }
  let body=terms[terms.length - 1];
  for(let slot=captures.length-1;slot>=0;slot-=1)body={k:'let',name:names[slot+3],value:args[captures[slot]],body,span:where};
  return body;
}
