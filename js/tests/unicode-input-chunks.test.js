import assert from 'node:assert/strict';
import { test } from 'node:test';

import { LinkNetwork, LinkType } from '../src/index.js';

// The JavaScript runtime hands the parser UTF-8 input in chunks of at most 4096 bytes, and
// web-tree-sitter copies each chunk into a 5119-unit buffer; these sources place a four-byte
// astral character (a surrogate pair in UTF-16) on every alignment around both boundaries.
const PADDINGS = [...Array(31).keys()].flatMap((offset) => [4070 + offset, 5095 + offset]);
const ASTRAL_CASES = [
  ['JavaScript', (padding) => `const ${'a'.repeat(padding)}𝓝 = 1;\n`, '𝓝'],
  ['Rust', (padding) => `//${' '.repeat(padding)}\nconst C: char = '𐲝';\n`, "'𐲝'"],
  ['Lean', (padding) => `def s : String := "${'a'.repeat(padding)}😀"\n`, '😀'],
  ['Rocq', (padding) => `Definition s := "${'a'.repeat(padding)}😀".\n`, '😀'],
];

test('astral characters straddling a parser input chunk boundary stay whole', () => {
  for (const [language, build, expected] of ASTRAL_CASES) {
    for (const padding of PADDINGS) {
      const source = build(padding);
      const network = LinkNetwork.parse(source, language);
      const label = `${language} with ${padding} padding code units`;
      assert.equal(network.reconstructText(), source, label);
      assert.ok(network.verifyFullMatch().isClean(), label);
      const texts = network.links()
        .filter((link) => link.metadata().linkType === LinkType.Syntax)
        .map((link) => {
          const range = link.metadata().span?.byteRange;
          return range ? Buffer.from(source).subarray(range.start, range.end).toString('utf8') : '';
        });
      assert.ok(texts.some((text) => text.includes(expected)), label);
      assert.ok(texts.every((text) => !text.includes('�')), label);
    }
  }
});
