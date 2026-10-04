// Arithmetic the portable core translates, beside items it carries.
import { strict as assert } from 'node:assert';

/**
 * The sum of `a` and `b`.
 * @param {number} a
 * @param {number} b
 * @returns {number}
 */
export function add(a, b) {
  return a + b;
}

/**
 * Whether `name` names Rust.
 * @param {string} name
 * @returns {boolean}
 */
export function isRust(name) {
  return name === 'Rust';
}

export class Box {
  constructor(value) {
    this.value = value;
  }
}

assert.equal(add(1, 2), 3);
