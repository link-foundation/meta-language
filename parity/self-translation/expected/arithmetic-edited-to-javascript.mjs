// meta-language:self-translation:v1 source=Rust target=JavaScript sha256=d177b4d9c7c9599f02d14f77ab0b3173ea98bc9ede12cc500081e7a5bf983b17 bytes=1545

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

// meta-language:translated Rust function_item items=1 sha256=1cc70992ddbd8cb1076aad6e24efcf175ca50a343abeb47ae5f145b0df265a90
// | pub fn is_rust(name: String) -> bool {
// |     name == "Rust" || name == "rs"
// | }
export function is_rust(name) {
  return ((name === "Rust") || (name === "rs"));
}

export class Box {
  constructor(value) {
    this.value = value;
  }
}

assert.equal(add(1, 2), 3);
