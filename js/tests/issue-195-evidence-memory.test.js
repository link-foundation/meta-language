// Resource regressions for issue #199. Children run with bounded heaps and
// report peak RSS so typed arrays and WebAssembly allocations count too.
import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { mkdtemp, readFile, rm } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { test } from 'node:test';

import { ndjson, parityDigests, PARITY_ARTIFACT_FILES, writeParityDigestArtifacts } from '../scripts/issue-195-parity-evidence.mjs';
import { recordIssue195DirectiveObservation } from './support/issue-195-observations.js';

const observe = (assertion, name) => recordIssue195DirectiveObservation(
  'I195-RESOURCE-BOUNDED-MEMORY-REGRESSIONS', [assertion], name,
);

const moduleUrl = (relative) => JSON.stringify(new URL(relative, import.meta.url).href);

function boundedRun(source, heapMiB = 64, rssMiB = 256) {
  const run = spawnSync(process.execPath, [
    `--max-old-space-size=${heapMiB}`, '--expose-gc', '--input-type=module', '-e', source,
  ], { encoding: 'utf8', timeout: 120_000 });
  assert.equal(run.status, 0, run.error?.message ?? run.stderr);
  const result = JSON.parse(run.stdout);
  assert.ok(result.peakMiB < rssMiB, `peak RSS ${result.peakMiB} MiB, limit ${rssMiB} MiB`);
  return result;
}

test('loading the public package stays within a capped heap and RSS without compiling grammars', () => {
  const { loaded } = boundedRun(`
    const { loadedGrammarIds } = await import(${moduleUrl('../src/programming-language-parser.js')});
    await import(${moduleUrl('../src/index.js')});
    console.log(JSON.stringify({ loaded: loadedGrammarIds(), peakMiB: process.resourceUsage().maxRSS / 1024 }));
  `);
  assert.deepEqual(loaded, []);
  observe('lazyGrammarsBoundedHeapAndPeakRss', 'loading the public package stays within a capped heap and RSS without compiling grammars');
});

test('multi-megabyte Unicode source offsets fit a capped heap and RSS', () => {
  const { checkpoints, end, offset } = boundedRun(`
    const { sourceBoundaries } = await import(${moduleUrl('../src/source-boundaries.js')});
    const source = 'aé😀\\n'.repeat(1_000_000);
    const index = sourceBoundaries(source);
    console.log(JSON.stringify({ checkpoints: index.checkpointCount, end: index.get(source.length),
      offset: index.offsetOf(8_000_000), peakMiB: process.resourceUsage().maxRSS / 1024 }));
  `);
  assert.equal(checkpoints, 62_501);
  assert.deepEqual(end, { byte: 8_000_000, row: 1_000_000, column: 0 });
  assert.equal(offset, 5_000_000);
  observe('sourceOffsetsBoundedHeapAndPeakRss', 'multi-megabyte Unicode source offsets fit a capped heap and RSS');
});

test('repeated Markdown inline parsing stays within a capped heap and RSS', () => {
  const { setups, deleted, retainedGrowthMiB } = boundedRun(`
    const { Parser, Tree } = await import(${moduleUrl('../node_modules/web-tree-sitter/web-tree-sitter.js')});
    let setups = 0;
    let deleted = 0;
    const setLanguage = Parser.prototype.setLanguage;
    Parser.prototype.setLanguage = function (...args) { setups++; return setLanguage.apply(this, args); };
    const remove = Tree.prototype.delete;
    Tree.prototype.delete = function (...args) { deleted++; return remove.apply(this, args); };
    const { parseProgrammingLanguage } = await import(${moduleUrl('../src/programming-language-parser.js')});
    const source = Array.from({ length: 400 }, (_, i) => 'Paragraph *' + i + '* with \\x60code\\x60.\\n').join('\\n');
    const retained = [];
    for (let round = 0; round < 4; round++) {
      const result = parseProgrammingLanguage(source, 'Markdown');
      if (!result.tree) throw new Error('Markdown parse produced no tree');
      // First-use background compilation has a transient RSS peak. Compare
      // retained allocations only after it settles, while maxRSS still
      // measures that compilation peak against the process ceiling.
      if (round === 0 || round === 3) await new Promise((resolve) => setTimeout(resolve, 1500));
      global.gc();
      retained.push(process.memoryUsage().rss);
    }
    console.log(JSON.stringify({ setups, deleted, retainedGrowthMiB: (retained.at(-1) - retained[0]) / 2 ** 20,
      peakMiB: process.resourceUsage().maxRSS / 1024 }));
  // The peak ceiling includes compiling both grammars. Retained growth after
  // the first parse separately guards repeated parser and tree allocations.
  `, 96, 1024);
  assert.equal(setups, 5, 'four block parsers share one inline parser');
  assert.equal(deleted, 4 * 401, 'each block and inline tree is deleted');
  assert.ok(retainedGrowthMiB < 128, `retained growth ${retainedGrowthMiB} MiB`);
  observe('inlineParserBoundedHeapAndPeakRss', 'repeated Markdown inline parsing stays within a capped heap and RSS');
});

test('streamed parity artifacts preserve the existing digest and difference format exactly', async () => {
  const directory = await mkdtemp(path.join(tmpdir(), 'parity-format-'));
  try {
    const observations = {
      javascript: { schemaVersion: 1, positive: [{ x: 1 }, { x: 2 }], negative: [] },
      rust: { schemaVersion: 2, positive: [{ x: 1 }], negative: [] },
    };
    const expected = parityDigests(observations);
    await writeParityDigestArtifacts(directory, observations);
    for (const name of ['digests', 'differences']) {
      assert.equal(await readFile(path.join(directory, PARITY_ARTIFACT_FILES[name]), 'utf8'), ndjson(expected[name]));
    }
  } finally {
    await rm(directory, { recursive: true, force: true });
  }
});

test('parity artifacts larger than the heap stream even when every entry differs', () => {
  const { bytes, entries, digests } = boundedRun(`
    const { mkdtemp, rm, stat } = await import('node:fs/promises');
    const { createReadStream } = await import('node:fs');
    const { createInterface } = await import('node:readline');
    const { tmpdir } = await import('node:os');
    const path = await import('node:path');
    const { writeParityDigestArtifacts, PARITY_ARTIFACT_FILES } = await import(${moduleUrl('../scripts/issue-195-parity-evidence.mjs')});
    const directory = await mkdtemp(path.join(tmpdir(), 'parity-memory-'));
    try {
      const javascript = { positive: Array(768).fill({ source: 'j'.repeat(96 * 1024) }) };
      const rust = { positive: Array(768).fill({ source: 'r'.repeat(96 * 1024) }) };
      await writeParityDigestArtifacts(directory, { javascript, rust });
      const differences = path.join(directory, PARITY_ARTIFACT_FILES.differences);
      let entries = 0;
      for await (const line of createInterface({ input: createReadStream(differences), crlfDelay: Infinity })) {
        const record = JSON.parse(line);
        if (record.section !== 'positive' || record.index !== entries || record.javascript.source[0] !== 'j' || record.rust.source[0] !== 'r') {
          throw new Error('incorrect streamed difference');
        }
        entries++;
      }
      let digests = 0;
      for await (const line of createInterface({ input: createReadStream(path.join(directory, PARITY_ARTIFACT_FILES.digests)), crlfDelay: Infinity })) {
        const record = JSON.parse(line);
        if (record.section === 'positive') {
          if (record.index !== digests || record.javascript === record.rust || !/^[a-f0-9]{64}$/.test(record.javascript)) {
            throw new Error('incorrect streamed digest');
          }
          digests++;
        }
      }
      console.log(JSON.stringify({ bytes: (await stat(differences)).size, entries, digests,
        peakMiB: process.resourceUsage().maxRSS / 1024 }));
    } finally {
      await rm(directory, { recursive: true, force: true });
    }
  `);
  assert.ok(bytes > 128 * 1024 * 1024, `artifact size ${bytes}`);
  assert.equal(entries, 768);
  assert.equal(digests, 768);
  observe('parityArtifactsBoundedHeapAndPeakRss', 'parity artifacts larger than the heap stream even when every entry differs');
});
