// Portable-core types. Numeric types keep their source semantics apart:
// `nat` and `int` are unbounded, `fixed` is a Rust machine integer whose
// arithmetic aborts on overflow, `float` is a JavaScript Number (an IEEE-754
// binary64). Data types are named by their qualified path.

export const NAT = Object.freeze({ kind: 'nat' });
export const INT = Object.freeze({ kind: 'int' });
export const BOOL = Object.freeze({ kind: 'bool' });
export const STRING = Object.freeze({ kind: 'string' });
export const UNIT = Object.freeze({ kind: 'unit' });
export const FLOAT = Object.freeze({ kind: 'float' });
// The lines a program has printed, newest first: only the targets whose
// programs are pure carry them, as a value (see output.js).
export const OUTPUT = Object.freeze({ kind: 'output' });

export function fixed(bits, signed) {
  return Object.freeze({ kind: 'fixed', bits, signed });
}

export function data(name) {
  return Object.freeze({ kind: 'data', name });
}

/** A JavaScript array whose elements all have one type; the portable core never mutates it. */
export function array(element) {
  return Object.freeze({ kind: 'array', element });
}

/** Absence keeps the distinction between JavaScript null and undefined. */
export function option(element, absence = 'both') {
  return Object.freeze({ kind: 'option', element, absence });
}

export function tuple(elements) {
  return Object.freeze({ kind: 'tuple', elements: Object.freeze([...elements]) });
}

/** Fields are named; their declaration order is not part of a record's shape. */
export function record(fields, open = false) {
  return Object.freeze({ kind: 'record', fields: Object.freeze(fields.map((field) => Object.freeze({ ...field }))), ...(open && { open }) });
}

export function map(key, value) {
  return Object.freeze({ kind: 'map', key, value });
}

export function set(element) {
  return Object.freeze({ kind: 'set', element });
}

export function callable(params, ret) {
  return Object.freeze({ kind: 'function', params: Object.freeze([...params]), ret });
}

export function typeKey(type) {
  switch (type.kind) {
    case 'fixed':
      return `${type.signed ? 'i' : 'u'}${type.bits}`;
    case 'data':
      return `data:${type.name}`;
    case 'array':
      return `array:${typeKey(type.element)}`;
    case 'option':
      return `option:${type.absence ?? 'both'}:${typeKey(type.element)}`;
    case 'tuple':
      return `tuple:[${type.elements.map(typeKey).join(',')}]`;
    case 'record':
      return `record:${type.open ? 'open:' : ''}${JSON.stringify([...type.fields].sort((a, b) => a.name < b.name ? -1 : a.name > b.name ? 1 : 0).map(({ name, type: fieldType }) => [name, typeKey(fieldType)]))}`;
    case 'map':
      return `map:${typeKey(type.key)}:${typeKey(type.value)}`;
    case 'set':
      return `set:${typeKey(type.element)}`;
    case 'function':
      return `function:(${type.params.map(typeKey).join(',')}):${typeKey(type.ret)}`;
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

export function isFloat(type) {
  return type.kind === 'float';
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
