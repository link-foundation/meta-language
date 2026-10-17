// Thin IEEE bit representation boundary. Decimal decisions are shared rules.
import { acceptBinaryDoublePrecisionFormatModel, renderBinaryDoublePrecisionSpecial,
  renderFiniteBinaryDoublePrecision } from './issue-199-decimal-number-rules.mjs';

export function formatBinaryDoublePrecisionValue(value, model = 'ecmascript-number-text') {
  if (typeof value !== 'number') throw new TypeError('binary double precision formatting requires a numeric value');
  if (!acceptBinaryDoublePrecisionFormatModel(model)) throw new TypeError(`unsupported binary double precision format model: ${model}`);
  const view = new DataView(new ArrayBuffer(8));
  view.setFloat64(0, value, false);
  const high = view.getUint32(0, false);
  const low = view.getUint32(4, false);
  const negative = high >= 0x80000000;
  const biasedExponent = (high >>> 20) & 0x7ff;
  const fraction = BigInt(high & 0xfffff) * 4294967296n + BigInt(low);
  if (biasedExponent === 0x7ff) return renderBinaryDoublePrecisionSpecial(model, fraction === 0n ? 'infinity' : 'not-a-number', negative);
  const mantissa = biasedExponent === 0 ? fraction : fraction + 4503599627370496n;
  const exponent = biasedExponent === 0 ? -1074n : BigInt(biasedExponent - 1075);
  const text = renderFiniteBinaryDoublePrecision(mantissa, exponent, negative, model);
  if (text === '') throw new RangeError('the declared binary double precision interval has no supported decimal representation');
  return text;
}
