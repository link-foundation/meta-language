#!/usr/bin/env node
// Generates the issue 195 generative fixtures under parity/fixtures/issue-195-generative/:
// for JavaScript, Lean, Rocq and Rust, inputs derived from the conformance cases by a
// seeded PRNG (js/tests/support/generative.js) — property compositions, fuzz mutations,
// metamorphic pairs and edit sequences — plus the kept reproducers of reproducers.json,
// each with the concrete syntax tree the native tree-sitter CLI prints for it. The
// meta-language runtimes never produce these trees: js/tests/issue-195-generative.test.js
// and rust/tests/unit/issue_195_generative.rs compare their public networks with them.
//
//   TREE_SITTER_CLI=tree-sitter node js/scripts/generate-issue-195-generative.mjs [--seed TEXT]
//   TREE_SITTER_CLI=tree-sitter node js/scripts/generate-issue-195-generative.mjs --check
//
// The same seed always generates the same inputs; `--check` regenerates with the
// recorded seed and fails when a fixture differs.
import { createHash } from 'node:crypto';
import { execFile } from 'node:child_process';
import { mkdir, readFile, rm, writeFile } from 'node:fs/promises';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { promisify } from 'node:util';

import { cargoLockVersions } from './build-vendored-grammars.mjs';
import { cliGrammarDirectory, cliOutput } from './generate-default-cst-expectations.mjs';
import { LANGUAGES } from './generate-issue-195-conformance.mjs';
import { cliCstToLines, parseCstLines } from '../tests/support/cst-lines.js';
import { COUNTS, DEFAULT_SEED, createRandom, generateInputs, relationHolds, seedNumber, seedSources } from '../tests/support/generative.js';

import { makeScratchDirectory } from '../../scripts/lib/scratch.mjs';

const run = promisify(execFile);
const root = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const fixtureDir = join(root, 'parity/fixtures/issue-195-generative');
const lockPath = join(root, 'js/src/vendor/grammars/grammar-lock.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const relative = (path) => path.slice(root.length + 1);
const cleanOf = (cst) => !parseCstLines(cst).some((node) => node.hasError || node.error || node.missing);

/** Runs `tasks` with at most `limit` in flight, keeping their order. */
async function pool(tasks, limit = 8) {
  const results = new Array(tasks.length);
  let next = 0;
  await Promise.all(Array.from({ length: Math.min(limit, tasks.length) }, async () => {
    while (next < tasks.length) {
      const index = next;
      next += 1;
      results[index] = await tasks[index]();
    }
  }));
  return results;
}

function formatCases(value) {
  const lines = value.cases.map((entry) => `    ${JSON.stringify(entry)}`);
  const { cases, ...header } = value;
  const head = JSON.stringify(header, null, 2).replace(/\n\}$/u, '');
  return `${head},\n  "cases": [\n${lines.join(',\n')}\n  ]\n}\n`;
}

async function generate(treeSitter, seed) {
  const lock = JSON.parse(await readFile(lockPath, 'utf8'));
  const version = (await run(treeSitter, ['--version'])).stdout.trim();
  if (version !== lock.treeSitterCli) throw new Error(`${treeSitter} is ${version}; the grammar lock pins ${lock.treeSitterCli}`);
  const versions = await cargoLockVersions();
  const reproducerBytes = await readFile(join(fixtureDir, 'reproducers.json'));
  const reproducers = JSON.parse(reproducerBytes.toString('utf8'));
  const scratch = makeScratchDirectory('issue-195-generative-');
  process.env.TREE_SITTER_LIBDIR = join(scratch, 'lib');
  const outputs = new Map();
  const manifest = {
    description: 'The issue 195 generative fixtures: property compositions, fuzz mutations, metamorphic pairs and edit sequences generated from the conformance inputs by a seeded PRNG, and kept reproducers, each with the tree the native tree-sitter CLI prints for it (the independent oracle).',
    generator: 'js/scripts/generate-issue-195-generative.mjs',
    prng: 'mulberry32 seeded with FNV-1a of `<seed>:<language>` (js/tests/support/generative.js, rust/tests/unit/generative_support.rs)',
    seed,
    seedNumber: seedNumber(seed),
    // The first outputs of the PRNG per language stream, which both suites reproduce.
    prngVectors: Object.fromEntries(LANGUAGES.map(({ name }) => {
      const random = createRandom(`${seed}:${name}`);
      return [`${seed}:${name}`, [random.next(), random.next(), random.next(), random.next()]];
    })),
    counts: COUNTS,
    inputs: {
      'reproducers.json': sha256(reproducerBytes),
      'issue-195-conformance/manifest.json': sha256(await readFile(join(root, 'parity/fixtures/issue-195-conformance/manifest.json'))),
    },
    oracle: {
      tool: version,
      command: 'tree-sitter parse --cst <file> (cwd: the pinned grammar)',
      format: 'Canonical CST lines, as in parity/fixtures/issue-195-conformance/manifest.json.',
    },
    relations: {
      'prepend-blank-lines': 'Two leading newlines move every node down two rows and keep kinds, fields, nesting, columns and flags, for clean trees that do not start with U+FEFF; the fixtures record whether the oracle keeps it (relationHolds).',
      crlf: 'LF to CRLF keeps kinds, fields, nesting, flags and start points.',
    },
    languages: {},
  };
  try {
    for (const language of LANGUAGES) {
      const directory = await cliGrammarDirectory(language.id, versions, scratch);
      const cli = (text, name) => cliOutput(treeSitter, directory, text, scratch, `${language.id}-${name}`)
        .then((output) => cliCstToLines(output, text));
      const seeds = seedSources(language.name);
      const inputs = generateInputs(language.name, seeds, seed);
      for (const reproducer of reproducers.cases.filter((entry) => entry.language === language.name)) {
        inputs.push({ id: `reproducer/${reproducer.id}`, kind: 'reproducer', source: reproducer.source });
      }
      const cases = await pool(inputs.map((entry) => async () => {
        const name = entry.id.replace(/\//gu, '-');
        const cst = await cli(entry.source, name);
        const result = { ...entry, clean: cleanOf(cst), cst };
        if (entry.kind === 'metamorphic') {
          result.variantCst = await cli(entry.variant, `${name}-variant`);
          result.relationHolds = relationHolds(entry.relation, cst, result.variantCst);
        }
        if (entry.kind === 'edit') {
          result.steps = [];
          for (const [position, step] of entry.steps.entries()) {
            const stepCst = await cli(step.source, `${name}-${position}`);
            result.steps.push({ start: step.start, end: step.end, replacement: step.replacement, clean: cleanOf(stepCst), cst: stepCst });
          }
        }
        return result;
      }));
      const text = formatCases({ language: language.name, seed, cases });
      const file = `${language.id}.json`;
      outputs.set(join(fixtureDir, file), text);
      const counts = {};
      for (const entry of cases) counts[entry.kind] = (counts[entry.kind] ?? 0) + 1;
      const holding = cases.filter((entry) => entry.kind === 'metamorphic' && entry.relationHolds).length;
      manifest.languages[language.name] = {
        file,
        sha256: sha256(text),
        seedSources: seeds.length,
        cases: counts,
        malformed: cases.filter((entry) => !entry.clean).length,
        relationsHolding: holding,
      };
    }
  } finally {
    await rm(scratch, { recursive: true, force: true });
  }
  outputs.set(join(fixtureDir, 'manifest.json'), `${JSON.stringify(manifest, null, 2)}\n`);
  return outputs;
}

async function main() {
  const args = process.argv.slice(2);
  const treeSitter = process.env.TREE_SITTER_CLI ?? 'tree-sitter';
  const seedIndex = args.indexOf('--seed');
  let seed = seedIndex >= 0 ? args[seedIndex + 1] : DEFAULT_SEED;
  if (args.includes('--check')) {
    seed = JSON.parse(await readFile(join(fixtureDir, 'manifest.json'), 'utf8')).seed;
  }
  const outputs = await generate(treeSitter, seed);
  if (args.includes('--check')) {
    const stale = [];
    for (const [path, content] of outputs) {
      const current = await readFile(path).catch(() => null);
      if (!current || !current.equals(Buffer.from(content))) stale.push(relative(path));
    }
    if (stale.length) {
      console.error(`stale generative fixtures (run node js/scripts/generate-issue-195-generative.mjs):\n  ${stale.join('\n  ')}`);
      process.exit(1);
    }
    console.log(`issue 195 generative fixtures match (${outputs.size} files)`);
    return;
  }
  for (const [path, content] of outputs) {
    await mkdir(dirname(path), { recursive: true });
    await writeFile(path, content);
  }
  console.log(`wrote ${outputs.size} files to ${relative(fixtureDir)}`);
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  await main();
}
