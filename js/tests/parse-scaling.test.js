// Complexity guard for documents with embedded regions, the counterpart of
// rust/tests/unit/parse_scaling.rs. The same generated document is parsed at two
// sizes and the cost per byte compared: a linear parse keeps it flat, and one
// that rescans the text or the network for every region multiplies it by the
// size ratio. Resolving each region's points by walking the text from its start,
// and recording grammar provenance by scanning every link for each region, once
// made a 48 KB Markdown file with a fenced block per section take a minute.
import assert from 'node:assert/strict';
import { test } from 'node:test';

import { LinkNetwork } from '../src/index.js';

// The large input holds SIZE_RATIO times as many units as the small one.
const SIZE_RATIO = 8;
const SMALL_UNITS = 32;
// Linear parsing lands near 1, quadratic near SIZE_RATIO.
const MAX_PER_BYTE_GROWTH = 3;
const SAMPLES = 3;
const ATTEMPTS = 3;

const CASES = [
  {
    language: 'Markdown',
    unit: (index) => `## Section ${index}\n\nParagraph ${index} with <b>inline ${index}</b> HTML.\n\n\`\`\`rust\npub fn item_${index}() -> usize { ${index} }\n\`\`\`\n\n`,
  },
  {
    language: 'HTML',
    unit: (index) => `<p style="color: rgb(${index}, 0, 0)">Paragraph ${index}.</p>\n<script>let item${index} = ${index};</script>\n`,
  },
];

const sourceFor = (unit, units) => Array.from({ length: units }, (_, index) => unit(index)).join('');

function nanosPerByte(source, language) {
  let best = Infinity;
  for (let sample = 0; sample < SAMPLES; sample += 1) {
    const started = process.hrtime.bigint();
    const network = LinkNetwork.parse(source, language);
    best = Math.min(best, Number(process.hrtime.bigint() - started));
    assert.equal(network.reconstructText(), source, `${language} parse must stay lossless`);
  }
  return best / Buffer.byteLength(source);
}

for (const { language, unit } of CASES) {
  test(`${language} documents with a region in every unit parse in linear time`, () => {
    const small = sourceFor(unit, SMALL_UNITS);
    const large = sourceFor(unit, SMALL_UNITS * SIZE_RATIO);
    const regions = LinkNetwork.parse(small, language).links()
      .filter((link) => link.metadata().linkType === 'Region').length;
    assert.ok(regions >= SMALL_UNITS, `${language}: ${regions} regions`);
    nanosPerByte(small, language); // warm up the grammar and the JIT
    const reports = [];
    const passed = Array.from({ length: ATTEMPTS }).some(() => {
      const smallCost = nanosPerByte(small, language);
      const largeCost = nanosPerByte(large, language);
      const growth = largeCost / smallCost;
      reports.push(`${smallCost.toFixed(0)} ns/byte at ${small.length} bytes, ${largeCost.toFixed(0)} ns/byte at ${large.length} bytes, growth ${growth.toFixed(2)}x`);
      return growth <= MAX_PER_BYTE_GROWTH;
    });
    assert.ok(passed, `per-byte parse cost grew more than ${MAX_PER_BYTE_GROWTH}x:\n  ${reports.join('\n  ')}`);
  });
}
