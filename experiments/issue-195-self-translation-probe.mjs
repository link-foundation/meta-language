// Probes selfTranslate on small modules in every direction.
import { selfTranslate } from '../js/src/self-translation.js';
const js = `// Arithmetic helpers.
import { x } from './y.js';

/**
 * @param {number} a
 * @param {number} b
 * @returns {number}
 */
export function add(a, b) {
  return a + b;
}

export class Box {
  constructor(value) { this.value = value; }
}
`;
const toRust = selfTranslate(js, 'JavaScript', 'Rust');
console.log(toRust.code);
console.log(toRust.items.map((i) => `${i.term}:${i.status}:${i.reason}`).join('\n'));
const back = selfTranslate(toRust.code, 'Rust', 'JavaScript');
console.log('==== back\n' + back.code);
console.log(back.items.map((i) => `${i.term}:${i.status}:${i.reason}`).join('\n'));
console.log('same', selfTranslate(js, 'js', 'ts').code === js);
console.log('round trip identical', back.code === js);
