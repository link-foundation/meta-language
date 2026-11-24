import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync } from 'node:fs';
import { selfTranslate } from '../src/self-translation.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { DecoratorSet } from '../src/decorators.js';

const archive = process.argv[2];
const check = process.argv.includes('--check');
const outputs = [
  new URL('../src/data/pest/built-ins.lino', import.meta.url),
  new URL('../../rust/src/data/pest-built-ins.lino', import.meta.url),
];
const policySource = readFileSync(new URL('../src/grammar-importers/pest-policy.js', import.meta.url), 'utf8');
const policyOutput = new URL('../../rust/src/grammar/import/pest_policy.rs', import.meta.url);
const pestVersion = '2.9.2';
assert.equal(createHash('sha256').update(readFileSync(archive)).digest('hex'),
  '45d3aca230fad2e6f6317ca0a72724338c4960cb97168a85cdee66df4a9a21a8');

const extract = (file) => execFileSync('tar', ['-xOzf', archive, `pest-${pestVersion}/src/unicode/${file}.rs`], {
  encoding: 'utf8', maxBuffer: 8 * 1024 * 1024,
});
const word = (character) => Array.from(Buffer.from(character), byte => (
  (byte >= 48 && byte <= 57) || (byte >= 65 && byte <= 90) || (byte >= 97 && byte <= 122)
    ? String.fromCharCode(byte) : `%${byte.toString(16).toUpperCase().padStart(2, '0')}`
)).join('');

function characterSet(ranges) {
  if (!ranges.length) return '(not (empty))';
  return `(class plain ${ranges.map(([first, last]) => {
    const start = word(String.fromCodePoint(first));
    return first === last ? `(char ${start})` : `(range ${start} ${word(String.fromCodePoint(last))})`;
  }).join(' ')})`;
}

function intervalTable(property) {
  const tables = Object.fromEntries(Array.from(property.matchAll(/(tree\d_level\d): &\[([^\]]*)\]/gu), ([, name, entries]) => [
    name, entries.trim().split(/\s*,\s*/u).filter(Boolean).map(entry => BigInt(entry)),
  ]));
  assert.equal(Object.keys(tables).length, 6);
  function maskFor(first) {
    if (first < 0x800) return tables.tree1_level1[first >> 6];
    if (first < 0x10000) {
      const leaf = tables.tree2_level1[(first >> 6) - 0x20];
      return leaf === undefined ? 0n : tables.tree2_level2[Number(leaf)];
    }
    const branch = tables.tree3_level1[(first >> 12) - 0x10];
    if (branch === undefined) return 0n;
    const leaf = tables.tree3_level2[Number(branch) * 64 + ((first >> 6) & 63)];
    return tables.tree3_level3[Number(leaf)];
  }
  const ranges = [];
  function append(first, last) {
    const previous = ranges.at(-1);
    if (previous?.[1] === first - 1) previous[1] = last;
    else ranges.push([first, last]);
  }
  for (let first = 0; first < 0x110000; first += 64) {
    if (first >= 0xd800 && first < 0xe000) continue;
    let mask = maskFor(first);
    if (mask === 0xffffffffffffffffn) {
      append(first, first + 63);
      continue;
    }
    for (let bit = 0; mask > 0n; bit += 1, mask >>= 1n) {
      if ((mask & 1n) === 1n) append(first + bit, first + bit);
    }
  }
  return ranges;
}

const builtin = new Map();
const ascii = {
  ASCII_DIGIT: [['0', '9']], ASCII_NONZERO_DIGIT: [['1', '9']], ASCII_BIN_DIGIT: [['0', '1']],
  ASCII_OCT_DIGIT: [['0', '7']], ASCII_HEX_DIGIT: [['0', '9'], ['a', 'f'], ['A', 'F']],
  ASCII_ALPHA_LOWER: [['a', 'z']], ASCII_ALPHA_UPPER: [['A', 'Z']],
  ASCII_ALPHA: [['a', 'z'], ['A', 'Z']], ASCII_ALPHANUMERIC: [['0', '9'], ['a', 'z'], ['A', 'Z']],
  ASCII: [['\0', '\x7f']],
};
for (const [name, ranges] of Object.entries(ascii)) {
  builtin.set(name, characterSet(ranges.map(pair => pair.map(character => character.codePointAt(0)))));
}
builtin.set('NEWLINE', '(choice ordered (literal %0A) (literal %0D%0A) (literal %0D))');
builtin.set('SOI', '(predicate (empty) (atStart))');
builtin.set('EOI', '(not (any))');

const definitions = extract('mod');
for (const family of ['binary', 'category', 'script']) {
  const names = new RegExp(`static ${family.toUpperCase()}_PROPERTY_NAMES = \\[([^\\]]+)\\]`, 'u').exec(definitions)[1];
  const entries = names.replace(/\/\/[^\n]*/gu, '');
  const declared = new Set(family === 'binary'
    ? entries.match(/\b[A-Z][A-Z_0-9]+\b/gu)
    : Array.from(entries.matchAll(/\("[^"]+",\s*([A-Z_0-9]+)\)/gu), match => match[1]));
  for (const [, name, table] of extract(family).matchAll(/pub const ([A-Z_0-9]+):[^\n]+TrieSet \{([\s\S]+?)\n\};/gu)) {
    if (declared.delete(name)) builtin.set(name, characterSet(intervalTable(table)));
  }
  assert.deepEqual([...declared], [], `unresolved pest ${family} properties`);
}
const file = ['(grammar (start ASCII_DIGIT))', ...Array.from(builtin, ([name, expr]) => (
  `(rule ${name} silent ${expr})`
))].join('\n') + '\n';
for (const output of outputs) {
  if (check) assert.equal(readFileSync(output, 'utf8'), file, `pest built-in rules are stale in ${output.pathname}`);
  else writeFileSync(output, file);
}
const translation = selfTranslate(policySource, 'JavaScript', 'Rust');
assert.ok(translation.items.every(item => item.status === 'translated'));
assert.equal(selfTranslate(translation.code, 'Rust', 'JavaScript').code, policySource);
const policyDefinitions = emitRust(checkProgram(parseJavaScript(policySource))).definitions.join('\n\n');
const decorators = DecoratorSet.fromLino(readFileSync(new URL('../../parity/self-translation/pest-decorators.lino', import.meta.url), 'utf8'));
const { source: styledPolicy } = decorators.decorate('emitter', { format: 'Rust', source: policyDefinitions });
const policy = `// Generated from the JavaScript pest importer policy.\n${styledPolicy}\n`;
if (check) assert.equal(readFileSync(policyOutput, 'utf8'), policy, 'pest implicit trivia policy is stale');
else writeFileSync(policyOutput, policy);
console.log(`${builtin.size} pest rules from pest ${pestVersion}`);
