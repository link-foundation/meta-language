import { GrammarBuilder as Expression } from '../grammar.js';
import { exclusiveIntegerBoundary } from './json-schema-integer-policy.js';

export const numericLimitKeywords = ['minimum', 'maximum', 'exclusiveMinimum', 'exclusiveMaximum'];

const { charRange, seq, choice, literal, repeat, repeat0, optional, empty } = Expression;
const digit = charRange('0', '9');
const digits = (first, last) => first === last ? literal(String(first)) : charRange(String(first), String(last));
const alternatives = expressions => expressions.length ? choice(...expressions) : null;

function fixedWidth(low, high) {
  const first = Number(low[0]);
  const last = Number(high[0]);
  const width = low.length - 1;
  if (width === 0) return digits(first, last);
  if (first === last) return seq(digits(first, last), fixedWidth(low.slice(1), high.slice(1)));
  const expressions = [seq(digits(first, first), fixedWidth(low.slice(1), '9'.repeat(width)))];
  if (last - first > 1) expressions.push(seq(digits(first + 1, last - 1), repeat(digit, width, width)));
  expressions.push(seq(digits(last, last), fixedWidth('0'.repeat(width), high.slice(1))));
  return choice(...expressions);
}

function unsignedIntegerRange(minimum, maximum) {
  const widths = [];
  const low = String(minimum);
  const high = String(maximum ?? minimum);
  for (let width = low.length; width <= high.length; width += 1) {
    const start = width === low.length ? low : `1${'0'.repeat(width - 1)}`;
    const end = maximum !== null && width === high.length ? high : '9'.repeat(width);
    widths.push(fixedWidth(start, end));
  }
  if (maximum === null) widths.push(seq(charRange('1', '9'), repeat(digit, high.length)));
  return choice(...widths);
}

export function signedIntegerRange(first, last) {
  const parts = [];
  if (first === null || first <= 0) parts.push(seq(literal('-'), unsignedIntegerRange(last === null ? 0 : Math.max(0, -last), first === null ? null : -first)));
  if (last === null || last >= 0) parts.push(unsignedIntegerRange(Math.max(0, first ?? 0), last));
  return choice(...parts);
}

export function integerBounds(rules) {
  return [
    Math.max(Math.ceil(rules.minimum ?? -Infinity), Math.floor(rules.exclusiveMinimum ?? -Infinity) + 1),
    Math.min(Math.floor(rules.maximum ?? Infinity), Math.ceil(rules.exclusiveMaximum ?? Infinity) - 1),
  ];
}

export function exclusiveIntegerRange(intervals) {
  const boundaries = new Map();
  const changeAt = (point, change) => boundaries.set(point, (boundaries.get(point) ?? 0) + change);
  let coverage = 0;
  for (const [first, last] of intervals) {
    if (first > last) continue;
    if (first !== -Infinity && (!Number.isSafeInteger(first) || !Number.isSafeInteger(first - 1))) return undefined;
    if (last !== Infinity && (!Number.isSafeInteger(last) || !Number.isSafeInteger(last + 1))) return undefined;
    if (first === -Infinity) coverage += 1;
    else changeAt(first, 1);
    if (last !== Infinity) changeAt(last + 1, -1);
  }
  const expressions = [];
  let first = null;
  for (const [point, change] of [...boundaries].sort(([left], [right]) => left - right)) {
    const next = coverage + change;
    const boundary = exclusiveIntegerBoundary(coverage, next);
    if (boundary < 0) expressions.push(signedIntegerRange(first, point - 1));
    if (boundary > 0) first = point;
    coverage = next;
  }
  if (coverage === 1) expressions.push(signedIntegerRange(first, null));
  return alternatives(expressions);
}

function decimalParts({ value, inclusive }) {
  const [mantissa, exponent = '0'] = String(value).split('e');
  const [whole, tail = ''] = mantissa.split('.');
  const position = whole.length + Number(exponent);
  const combined = whole + tail;
  const fraction = position < 0 ? '0'.repeat(-position) + combined : combined.slice(position);
  return { whole: Math.floor(value), fraction: fraction.replace(/0+$/u, ''), inclusive };
}

function acceptsZeroes(bound, index, lower) {
  if (!bound) return true;
  const zero = [...bound.fraction.slice(index)].every(value => value === '0');
  return zero ? bound.inclusive : !lower;
}

function fractionDigits(lower, upper, index, consumed) {
  if (!lower && !upper) return repeat(digit, consumed ? 0 : 1);
  const expressions = [];
  if (consumed && acceptsZeroes(lower, index, true) && acceptsZeroes(upper, index, false)) expressions.push(empty());
  if (index >= Math.max(lower?.fraction.length ?? 0, upper?.fraction.length ?? 0)) {
    if (upper) {
      if (upper.inclusive && (!lower || lower.inclusive)) return repeat(literal('0'), consumed ? 0 : 1);
      return null;
    }
    if (lower.inclusive) return repeat(digit, consumed ? 0 : 1);
    return seq(repeat0(literal('0')), charRange('1', '9'), repeat0(digit));
  }
  const first = lower ? Number(lower.fraction[index] ?? '0') : 0;
  const last = upper ? Number(upper.fraction[index] ?? '0') : 9;
  if (first > last) return null;
  const lowerSuffix = lower ? fractionDigits(lower, first === last ? upper : null, index + 1, true) : null;
  if (lowerSuffix) expressions.push(seq(digits(first, first), lowerSuffix));
  if (upper && (!lower || first !== last)) {
    const upperSuffix = fractionDigits(null, upper, index + 1, true);
    if (upperSuffix) expressions.push(seq(digits(last, last), upperSuffix));
  }
  const innerFirst = first + Number(Boolean(lower));
  const innerLast = last - Number(Boolean(upper));
  if (innerFirst <= innerLast) expressions.push(seq(digits(innerFirst, innerLast), repeat0(digit)));
  return alternatives(expressions);
}

function fractionalSuffix(lower, upper) {
  const decimals = fractionDigits(lower, upper, 0, false);
  const dot = decimals && seq(literal('.'), decimals);
  if (acceptsZeroes(lower, 0, true) && acceptsZeroes(upper, 0, false)) return dot ? optional(dot) : empty();
  return dot;
}

function unsignedDecimalRange(lower, upper) {
  const first = lower && decimalParts(lower);
  const last = upper && decimalParts(upper);
  const expressions = [];
  const lowInteger = first ? first.whole + 1 : 0;
  const highInteger = last ? last.whole - 1 : null;
  if (highInteger === null || lowInteger <= highInteger) {
    expressions.push(seq(unsignedIntegerRange(lowInteger, highInteger), fractionalSuffix(null, null)));
  }
  if (first) {
    const suffix = fractionalSuffix(first, last?.whole === first.whole ? last : null);
    if (suffix) expressions.push(seq(literal(String(first.whole)), suffix));
  }
  if (last && first?.whole !== last.whole) {
    const suffix = fractionalSuffix(null, last);
    if (suffix) expressions.push(seq(literal(String(last.whole)), suffix));
  }
  return alternatives(expressions);
}

function narrower(inclusive, exclusive, lower) {
  if (inclusive === undefined && exclusive === undefined) return null;
  if (inclusive !== undefined && (exclusive === undefined || (lower ? inclusive > exclusive : inclusive < exclusive))) {
    return { value: inclusive, inclusive: true };
  }
  return { value: exclusive, inclusive: false };
}

function decimalLimits(rules) {
  return [narrower(rules.minimum, rules.exclusiveMinimum, true),
    narrower(rules.maximum, rules.exclusiveMaximum, false)];
}

function emptyDecimalInterval(first, last) {
  return first !== null && last !== null
    && (first.value > last.value || (first.value === last.value && !(first.inclusive && last.inclusive)));
}

function decimalRange(first, last) {
  if ([first, last].some(bound => bound && (Math.abs(bound.value) >= Number.MAX_SAFE_INTEGER ||
      decimalParts({ ...bound, value: Math.abs(bound.value) }).fraction.length > 128))) return undefined;
  if (emptyDecimalInterval(first, last)) return null;
  const mirrored = bound => ({ value: -bound.value, inclusive: bound.inclusive });
  const negative = !first || first.value < 0 || (first.value === 0 && first.inclusive)
    ? unsignedDecimalRange(last && last.value <= 0 ? mirrored(last) : null, first && mirrored(first)) : null;
  const positive = !last || last.value > 0 || (last.value === 0 && last.inclusive)
    ? unsignedDecimalRange(first && first.value >= 0 ? first : null, last) : null;
  return alternatives([negative && seq(literal('-'), negative), positive].filter(Boolean));
}

/** Return undefined for decimal bounds outside the finite provider grammar budget. */
export function signedDecimalRange(rules) {
  return decimalRange(...decimalLimits(rules));
}

export function exclusiveDecimalRange(branchLimits) {
  const boundaries = new Map();
  function changeAt(value, at, after) {
    const change = boundaries.get(value) ?? { at: 0, after: 0 };
    change.at += at;
    change.after += after;
    boundaries.set(value, change);
  }
  let coverage = 0;
  for (const rules of branchLimits) {
    const [first, last] = decimalLimits(rules);
    if (emptyDecimalInterval(first, last)) continue;
    if (first) changeAt(first.value, Number(first.inclusive), 1);
    else coverage += 1;
    if (last) changeAt(last.value, last.inclusive ? 0 : -1, -1);
  }
  const intervals = [];
  let first = null;
  for (const [value, change] of [...boundaries].sort(([left], [right]) => left - right)) {
    // An endpoint can belong to a different number of branches than either side.
    const at = coverage + change.at;
    const after = coverage + change.after;
    if (coverage === 1 && at !== 1) intervals.push([first, { value, inclusive: false }]);
    if (coverage !== 1 && at === 1) first = { value, inclusive: true };
    if (at === 1 && after !== 1) intervals.push([first, { value, inclusive: true }]);
    if (at !== 1 && after === 1) first = { value, inclusive: false };
    coverage = after;
  }
  if (coverage === 1) intervals.push([first, null]);
  if (intervals.length === 1 && intervals[0].every(bound => bound === null)) return Expression.ref('number');
  const expressions = intervals.map(([first, last]) => decimalRange(first, last));
  return expressions.includes(undefined) ? undefined : alternatives(expressions.filter(Boolean));
}
