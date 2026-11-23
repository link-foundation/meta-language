/** Normalize a JSON number into a sign, decimal position and significant digits. */
export function jsonNumberSpelling(source) {
  const negative = source.startsWith('-');
  const [mantissa, exponent = '0'] = source.slice(Number(negative)).split(/[eE]/u, 2);
  const [whole, fraction = ''] = mantissa.split('.', 2);
  const coefficient = whole + fraction;
  const first = coefficient.search(/[1-9]/u);
  if (first === -1) return { sign: 0, position: 0, digits: '' };
  let last = coefficient.length;
  while (coefficient[last - 1] === '0') last -= 1;
  return {
    sign: negative ? -1 : 1,
    position: whole.length - first + Number(exponent),
    digits: coefficient.slice(first, last),
  };
}

export function isIntegerSpelling(number) {
  return number.position >= number.digits.length;
}

export function compareNumberSpellings(left, right) {
  if (left.sign !== right.sign) return Math.sign(left.sign - right.sign);
  if (left.position !== right.position) return left.position < right.position ? -left.sign : left.sign;
  if (left.digits === right.digits) return 0;
  return left.digits < right.digits ? -left.sign : left.sign;
}
