// Portable-core types. Numeric types keep their source semantics apart:
// `nat` and `int` are unbounded, `fixed` is a Rust machine integer whose
// arithmetic aborts on overflow. Data types are named by their qualified path.

export const NAT = Object.freeze({ kind: 'nat' });
export const INT = Object.freeze({ kind: 'int' });
export const BOOL = Object.freeze({ kind: 'bool' });
export const STRING = Object.freeze({ kind: 'string' });
export const UNIT = Object.freeze({ kind: 'unit' });

export function fixed(bits, signed) {
  return Object.freeze({ kind: 'fixed', bits, signed });
}

export function data(name) {
  return Object.freeze({ kind: 'data', name });
}

export function typeKey(type) {
  switch (type.kind) {
    case 'fixed':
      return `${type.signed ? 'i' : 'u'}${type.bits}`;
    case 'data':
      return `data:${type.name}`;
    default:
      return type.kind;
  }
}

export function sameType(left, right) {
  return typeKey(left) === typeKey(right);
}

export function isNumeric(type) {
  return type.kind === 'nat' || type.kind === 'int' || type.kind === 'fixed';
}

/** True when every value of the type is a non-negative integer. */
export function isNatural(type) {
  return type.kind === 'nat' || (type.kind === 'fixed' && !type.signed);
}

export function fixedBounds(type) {
  const bits = BigInt(type.bits);
  return type.signed
    ? { min: -(1n << (bits - 1n)), max: (1n << (bits - 1n)) - 1n }
    : { min: 0n, max: (1n << bits) - 1n };
}

const RUST_FIXED = /^(u|i)(8|16|32|64|128|size)$/u;

export function rustFixedType(name) {
  const match = RUST_FIXED.exec(name);
  if (!match) return undefined;
  return fixed(match[2] === 'size' ? 64 : Number(match[2]), match[1] === 'i');
}
