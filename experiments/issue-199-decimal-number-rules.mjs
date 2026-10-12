// Draft exact decimal selection; native parity and typed-model integration are pending.
// Host adapters supply IEEE significand, binary exponent and sign; every
// formatting decision below is portable checked JavaScript source.

/** @param {bigint} base @param {bigint} exponent @returns {bigint} */
export function raiseIntegerPower(base, exponent) {
  let value = 1n;
  let index = 0n;
  while (index < exponent) {
    value *= base;
    index += 1n;
  }
  return value;
}

/** @param {bigint} value @returns {bigint} */
export function countDecimalDigits(value) {
  let digits = 1n;
  while (value >= 10n) {
    value /= 10n;
    digits += 1n;
  }
  return digits;
}

/** @param {bigint} digit @returns {string} */
export function renderDecimalDigit(digit) {
  if (digit === 0n) return '0';
  if (digit === 1n) return '1';
  if (digit === 2n) return '2';
  if (digit === 3n) return '3';
  if (digit === 4n) return '4';
  if (digit === 5n) return '5';
  if (digit === 6n) return '6';
  if (digit === 7n) return '7';
  if (digit === 8n) return '8';
  if (digit === 9n) return '9';
  return '';
}

/** @param {bigint} value @param {bigint} width @returns {string} */
export function renderFixedWidthDecimal(value, width) {
  let text = '';
  while (width > 0n) {
    text = renderDecimalDigit(value % 10n) + text;
    value /= 10n;
    width -= 1n;
  }
  return text;
}

/** @param {bigint} count @returns {string} */
export function renderDecimalZeroes(count) {
  let text = '';
  let index = 0n;
  while (index < count) {
    text += '0';
    index += 1n;
  }
  return text;
}

/** @param {string} model @returns {boolean} */
export function acceptBinaryDoublePrecisionFormatModel(model) {
  return model === 'ecmascript-number-text' || model === 'node-console-number-text'
    || model === 'rust-default-binary-double-precision-text';
}

/** @param {string} model @param {string} kind @param {boolean} negative @returns {string} */
export function renderBinaryDoublePrecisionSpecial(model, kind, negative) {
  if (!acceptBinaryDoublePrecisionFormatModel(model)) return '';
  if (kind === 'not-a-number') return 'NaN';
  if (kind === 'infinity') {
    const value = model === 'rust-default-binary-double-precision-text' ? 'inf' : 'Infinity';
    return negative ? '-' + value : value;
  }
  if (kind === 'zero') return negative && model !== 'ecmascript-number-text' ? '-0' : '0';
  return '';
}

/** @param {bigint} value @param {bigint} exponent @param {boolean} negative @param {string} model @returns {string} */
export function renderShortestDecimalValue(value, exponent, negative, model) {
  const digits = countDecimalDigits(value);
  const position = digits + exponent;
  const sign = negative ? '-' : '';
  if (model !== 'rust-default-binary-double-precision-text' && (position > 21n || position <= -6n)) {
    const scale = raiseIntegerPower(10n, digits - 1n);
    const first = renderDecimalDigit(value / scale);
    const tail = digits > 1n ? '.' + renderFixedWidthDecimal(value % scale, digits - 1n) : '';
    const power = position - 1n;
    return sign + first + tail + 'e' + (power >= 0n ? '+' : '') + power.toString();
  }
  if (exponent >= 0n) return sign + renderFixedWidthDecimal(value, digits) + renderDecimalZeroes(exponent);
  if (position > 0n) {
    const scale = raiseIntegerPower(10n, -exponent);
    const integer = value / scale;
    return sign + renderFixedWidthDecimal(integer, countDecimalDigits(integer)) + '.' + renderFixedWidthDecimal(value % scale, -exponent);
  }
  return sign + '0.' + renderDecimalZeroes(-position) + renderFixedWidthDecimal(value, digits);
}

/**
 * Select the shortest integer decimal in the exact nearest-even rounding
 * interval. Rust's selected shortest-decimal model chooses the upper decimal
 * at a closest-decimal tie; ECMAScript chooses its even decimal neighbour.
 * Empty text signals an unsupported input/model to the checked host boundary.
 * @param {bigint} mantissa @param {bigint} exponent @param {boolean} negative @param {string} model @returns {string}
 */
export function renderFiniteBinaryDoublePrecision(mantissa, exponent, negative, model) {
  if (!acceptBinaryDoublePrecisionFormatModel(model)) return '';
  if (mantissa < 0n || mantissa > 9007199254740991n || exponent < -1074n || exponent > 971n) return '';
  if (mantissa === 0n) return exponent === -1074n ? renderBinaryDoublePrecisionSpecial(model, 'zero', negative) : '';
  if (exponent > -1074n && mantissa < 4503599627370496n) return '';
  const asymmetric = mantissa === 4503599627370496n && exponent > -1074n;
  const inclusive = mantissa % 2n === 0n;
  let lower = mantissa * 4n - (asymmetric ? 1n : 2n);
  let upper = mantissa * 4n + 2n;
  let exact = mantissa * 4n;
  let denominator = 1n;
  if (exponent >= 2n) {
    const scale = raiseIntegerPower(2n, exponent - 2n);
    lower *= scale;
    upper *= scale;
    exact *= scale;
  } else denominator = raiseIntegerPower(2n, 2n - exponent);

  let magnitude = 0n;
  if (exact >= denominator) {
    let scale = denominator;
    while (exact >= scale * 10n) {
      scale *= 10n;
      magnitude += 1n;
    }
  } else {
    let scale = exact;
    while (scale < denominator) {
      scale *= 10n;
      magnitude -= 1n;
    }
  }
  let precision = 1n;
  while (precision <= 17n) {
    let decimalExponent = magnitude - precision + 1n;
    let minimumNumerator = lower;
    let maximumNumerator = upper;
    let exactNumerator = exact;
    let divisor = denominator;
    if (decimalExponent >= 0n) divisor *= raiseIntegerPower(10n, decimalExponent);
    else {
      const scale = raiseIntegerPower(10n, -decimalExponent);
      minimumNumerator *= scale;
      maximumNumerator *= scale;
      exactNumerator *= scale;
    }
    let minimum = minimumNumerator / divisor;
    if (minimumNumerator % divisor !== 0n || !inclusive) minimum += 1n;
    let maximum = maximumNumerator / divisor;
    if (maximumNumerator % divisor === 0n && !inclusive) maximum -= 1n;
    if (minimum <= maximum) {
      let nearest = exactNumerator / divisor;
      const remainder = exactNumerator % divisor;
      const upperTie = model === 'rust-default-binary-double-precision-text' || nearest % 2n !== 0n;
      if (remainder * 2n > divisor || remainder * 2n === divisor && upperTie) nearest += 1n;
      if (nearest < minimum) nearest = minimum;
      if (nearest > maximum) nearest = maximum;
      if (nearest > 0n) {
        while (nearest % 10n === 0n) {
          nearest /= 10n;
          decimalExponent += 1n;
        }
        if (countDecimalDigits(nearest) <= precision) return renderShortestDecimalValue(nearest, decimalExponent, negative, model);
      }
    }
    precision += 1n;
  }
  return '';
}
