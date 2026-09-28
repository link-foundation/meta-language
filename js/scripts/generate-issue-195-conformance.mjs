#!/usr/bin/env node
// Generates the issue 195 four-language conformance fixtures under
// parity/fixtures/issue-195-conformance/: the upstream tree-sitter test corpora of
// JavaScript, Lean, Rocq and Rust at the pinned grammar revisions (with the local
// grammar patches the build applies), files of real projects at pinned tags, and
// the hand-authored construct, Unicode, malformed and mixed-language inputs of cases.json,
// each with the concrete syntax tree the native tree-sitter CLI prints for it
// (`tree-sitter parse --cst`, converted to canonical CST lines). The meta-language
// runtimes never produce these trees: js/tests/issue-195-conformance.test.js and
// rust/tests/unit/issue_195_conformance.rs compare their public networks with them.
//
//   TREE_SITTER_CLI=tree-sitter node js/scripts/generate-issue-195-conformance.mjs [--work DIR]
//   TREE_SITTER_CLI=tree-sitter node js/scripts/generate-issue-195-conformance.mjs --check
//
// It needs git, network access (upstream clones and project files) and the
// tree-sitter CLI the grammar lock names. `--work DIR` keeps the upstream
// checkouts for later runs.
import { createHash } from 'node:crypto';
import { execFile } from 'node:child_process';
import { existsSync } from 'node:fs';
import { mkdir, mkdtemp, readFile, readdir, rm, writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { basename, dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { promisify } from 'node:util';

import { GRAMMAR_SOURCES, cargoLockVersions, checkoutUpstream, grammarSource } from './build-vendored-grammars.mjs';
import { cliGrammarDirectory, cliOutput } from './generate-default-cst-expectations.mjs';
import { cliCstToLines, cstLinesToSexp, parseCstLines, rowOffsets } from '../tests/support/cst-lines.js';
import { normalize, parseCorpus, stripFields } from '../tests/support/cst-sexpression.js';

const run = promisify(execFile);
const root = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const fixtureDir = join(root, 'parity/fixtures/issue-195-conformance');
const lockPath = join(root, 'js/src/vendor/grammars/grammar-lock.json');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const relative = (path) => path.slice(root.length + 1);

/**
 * The four languages: the upstream repository and revision of the grammar each
 * runtime compiles (the crate or vendored parser is checked against it), and the
 * real-project files parsed as they are published.
 */
export const LANGUAGES = [
  {
    name: 'JavaScript',
    id: 'javascript',
    upstream: 'tree-sitter/tree-sitter-javascript',
    revision: '44c892e0be055ac465d5eeddae6d3e194424e7de',
    tag: 'v0.25.0',
    projects: [
      { project: 'tree-sitter/tree-sitter', tag: 'v0.25.10', path: 'cli/npm/install.js', license: 'MIT', licensePath: 'LICENSE' },
    ],
  },
  {
    name: 'Lean',
    id: 'lean',
    upstream: 'wvhulle/tree-sitter-lean',
    revision: 'bd942cd2795016239be02b3b3d5ef635645ddd38',
    tag: 'tree-sitter-lean4 0.3.0 (crates.io)',
    projects: [
      { project: 'leanprover/lean4', tag: 'v4.33.1', path: 'src/Init/Control/StateRef.lean', license: 'Apache-2.0', licensePath: 'LICENSE' },
      { project: 'leanprover/lean4', tag: 'v4.33.1', path: 'src/Init/Data/Nat/Dvd.lean', license: 'Apache-2.0', licensePath: 'LICENSE' },
    ],
  },
  {
    name: 'Rocq',
    id: 'rocq',
    upstream: GRAMMAR_SOURCES.rocq.upstream,
    revision: GRAMMAR_SOURCES.rocq.revision,
    patch: GRAMMAR_SOURCES.rocq.patch,
    projects: [
      { project: 'rocq-prover/stdlib', tag: 'V9.2.0', path: 'theories/Arith/Compare_dec.v', license: 'LGPL-2.1-only', licensePath: 'LICENSE' },
    ],
  },
  {
    name: 'Rust',
    id: 'rust',
    upstream: GRAMMAR_SOURCES.rust.upstream,
    revision: GRAMMAR_SOURCES.rust.revision,
    tag: GRAMMAR_SOURCES.rust.version,
    patch: GRAMMAR_SOURCES.rust.patch,
    projects: [
      { project: 'tree-sitter/tree-sitter', tag: 'v0.25.10', path: 'lib/binding_rust/wasm_language.rs', license: 'MIT', licensePath: 'LICENSE' },
      { project: 'tree-sitter/tree-sitter-rust', tag: 'v0.24.2', path: 'examples/weird-exprs.rs', license: 'MIT', licensePath: 'LICENSE' },
    ],
  },
];

/** Host grammar node that delimits an embedded region, per host language. */
const REGION_NODES = { Markdown: ['markdown', 'code_fence_content'], HTML: ['html', 'raw_text'] };

async function gitHead(checkout) {
  return (await run('git', ['-C', checkout, 'rev-parse', 'HEAD'])).stdout.trim();
}

/**
 * The pinned upstream checkout with the local patch applied and the parser
 * regenerated, verified to produce the parser.c the grammar lock records.
 */
async function upstreamCheckout(language, work, lock, versions, treeSitter) {
  const checkout = join(work, `tree-sitter-${language.id}`);
  if (!existsSync(checkout)) {
    if (language.patch) {
      await checkoutUpstream({ ...GRAMMAR_SOURCES[language.id], ...(await grammarSource(language.id, versions)) }, checkout, treeSitter);
    } else {
      await run('git', ['clone', '--quiet', `https://github.com/${language.upstream}`, checkout]);
      await run('git', ['-C', checkout, 'checkout', '--quiet', language.revision]);
    }
  }
  if ((await gitHead(checkout)) !== language.revision) throw new Error(`${checkout} is not at ${language.revision}`);
  const parser = sha256(await readFile(join(checkout, 'src/parser.c')));
  if (parser !== lock.grammars[language.id].parserSha256) {
    throw new Error(`${language.upstream}@${language.revision} parser.c ${parser} differs from the grammar lock`);
  }
  return checkout;
}

/** Visible named node kinds of the grammar (no hidden `_` rules or supertypes). */
async function inventory(checkout) {
  const types = JSON.parse(await readFile(join(checkout, 'src/node-types.json'), 'utf8'));
  return [...new Set(types.filter((type) => type.named && !type.type.startsWith('_') && !type.subtypes)
    .map((type) => type.type))].sort();
}

async function fetchBytes(url) {
  const response = await fetch(url);
  if (!response.ok) throw new Error(`${url}: HTTP ${response.status}`);
  return Buffer.from(await response.arrayBuffer());
}

async function resolveTag(project, tag) {
  const { stdout } = await run('git', ['ls-remote', `https://github.com/${project}`, `refs/tags/${tag}^{}`, `refs/tags/${tag}`]);
  const lines = stdout.trim().split('\n').map((line) => line.split('\t'));
  return (lines.find(([, ref]) => ref.endsWith('^{}')) ?? lines[0])[0];
}

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

/** How the CLI tree compares with the upstream author's expected S-expression. */
function upstreamStatus(test, cst) {
  if (test.attributes.includes(':error')) {
    return parseCstLines(cst).some((node) => node.error || node.missing) ? 'error-expected' : 'error-missing';
  }
  if (!test.expected) return 'no-expected';
  const expected = normalize(test.expected);
  let actual = normalize(cstLinesToSexp(cst));
  if (!/\w: /u.test(expected)) actual = stripFields(actual);
  return actual === expected ? 'match' : 'differs';
}

const isClean = (cst) => !parseCstLines(cst).some((node) => node.hasError || node.error || node.missing);

function pointOf(offsets, byte) {
  let row = offsets.length - 1;
  while (offsets[row] > byte) row -= 1;
  return { row, column: byte - offsets[row] };
}

async function languageCases(language, context) {
  const { treeSitter, directories, scratch, files, cases, errorRecovery } = context;
  const cli = (text, name, offset) => cliOutput(treeSitter, directories.get(language.id), text, scratch, `${language.id}-${name}`)
    .then((output) => cliCstToLines(output, text, offset));
  const tasks = [];
  for (const [file, text] of files.corpus) {
    parseCorpus(text).forEach((test, index) => tasks.push(async () => {
      const cst = await cli(test.source, `corpus-${file}-${index}`);
      return {
        id: `corpus/${file}/${index}`,
        kind: 'corpus',
        name: test.name,
        attributes: test.attributes,
        sourceSha256: sha256(test.source),
        clean: isClean(cst),
        upstream: upstreamStatus(test, cst),
        cst,
      };
    }));
  }
  for (const [file, bytes] of files.projects) {
    tasks.push(async () => {
      const cst = await cli(bytes.toString('utf8'), `project-${file}`);
      return { id: `project/${file}`, kind: 'project', sourceSha256: sha256(bytes), clean: isClean(cst), cst };
    });
  }
  const inline = [
    ...cases.constructs.filter((entry) => entry.language === language.name).map((entry) => ({ ...entry, kind: 'construct' })),
    ...cases.unicode.filter((entry) => entry.language === language.name).map((entry) => ({ ...entry, kind: 'unicode' })),
    ...cases.malformed.filter((entry) => entry.language === language.name).map((entry) => ({ ...entry, kind: 'malformed' })),
    ...errorRecovery.cases.filter((entry) => entry.language === language.name)
      .map((entry) => ({ id: `error-recovery/${entry.id}`, source: entry.source, kind: 'malformed' })),
  ];
  for (const entry of inline) {
    tasks.push(async () => {
      const cst = await cli(entry.source, entry.id.replace(/^error-recovery\//u, ''));
      return { id: entry.id, kind: entry.kind, source: entry.source, sourceSha256: sha256(entry.source), clean: isClean(cst), cst };
    });
  }
  for (const host of cases.mixed) {
    host.regions.forEach((region, position) => {
      if (region.language !== language.name) return;
      tasks.push(async () => {
        const offsets = rowOffsets(host.source);
        const startByte = context.regionStart(host, position);
        const endByte = startByte + Buffer.byteLength(region.text, 'utf8');
        const offset = pointOf(offsets, startByte);
        const cst = await cli(region.text, `${host.id}-${position}`, offset);
        return {
          id: `mixed/${host.id}/${position}`,
          kind: 'mixed',
          host: host.host,
          hostCase: host.id,
          startByte,
          endByte,
          sourceSha256: sha256(host.source),
          clean: isClean(cst),
          cst,
        };
      });
    });
  }
  return pool(tasks);
}

/**
 * Byte offsets of every region of every mixed host, each confirmed by the
 * host grammar: the native CLI tree of the host has a region node exactly there.
 */
async function mixedRegions(cases, treeSitter, directories, scratch) {
  const starts = new Map();
  for (const host of cases.mixed) {
    const [grammar, regionKind] = REGION_NODES[host.host];
    const cst = cliCstToLines(await cliOutput(treeSitter, directories.get(grammar), host.source, scratch, `host-${host.id}`), host.source);
    const offsets = rowOffsets(host.source);
    const nodeRanges = new Set(parseCstLines(cst).filter((node) => node.kind === regionKind)
      .map((node) => `${offsets[node.start.row] + node.start.column}:${offsets[node.end.row] + node.end.column}`));
    const bytes = Buffer.from(host.source, 'utf8');
    let from = 0;
    host.regions.forEach((region, position) => {
      const start = bytes.indexOf(Buffer.from(region.text, 'utf8'), from);
      const end = start + Buffer.byteLength(region.text, 'utf8');
      if (start < 0 || !nodeRanges.has(`${start}:${end}`)) {
        throw new Error(`${host.id}: region ${position} is not a ${host.host} ${regionKind} node`);
      }
      starts.set(`${host.id}:${position}`, start);
      from = end;
    });
  }
  return (host, position) => starts.get(`${host.id}:${position}`);
}

function formatOracle(value) {
  const lines = value.cases.map((entry) => `    ${JSON.stringify(entry)}`);
  const { cases, ...header } = value;
  const head = JSON.stringify(header, null, 2).replace(/\n\}$/u, '');
  return `${head},\n  "cases": [\n${lines.join(',\n')}\n  ]\n}\n`;
}

async function generate(work, treeSitter) {
  const lock = JSON.parse(await readFile(lockPath, 'utf8'));
  const version = (await run(treeSitter, ['--version'])).stdout.trim();
  if (version !== lock.treeSitterCli) throw new Error(`${treeSitter} is ${version}; the grammar lock pins ${lock.treeSitterCli}`);
  const versions = await cargoLockVersions();
  const scratch = await mkdtemp(join(tmpdir(), 'issue-195-conformance-'));
  process.env.TREE_SITTER_LIBDIR = join(scratch, 'lib');
  const cases = JSON.parse(await readFile(join(fixtureDir, 'cases.json'), 'utf8'));
  const errorRecovery = JSON.parse(await readFile(join(fixtureDir, 'error-recovery.json'), 'utf8'));
  const outputs = new Map();
  const manifest = {
    description: 'Provenance of the issue 195 four-language conformance fixtures: upstream tree-sitter corpora at the pinned grammar revisions, real-project files at pinned tags, and the oracle that produced every expected tree (the native tree-sitter CLI with the pinned grammars, independent of the meta-language runtimes).',
    generator: 'js/scripts/generate-issue-195-conformance.mjs',
    inputs: {
      'cases.json': sha256(await readFile(join(fixtureDir, 'cases.json'))),
      'error-recovery.json': sha256(await readFile(join(fixtureDir, 'error-recovery.json'))),
    },
    oracle: {
      tool: version,
      command: 'tree-sitter parse --cst <file> (cwd: the pinned grammar)',
      format: 'Canonical CST lines (js/tests/support/cst-lines.js): <2 spaces per depth><field: ><•>KIND SR:SC-ER:EC with zero-based rows and UTF-8 byte columns; anonymous kinds are JSON-quoted, error nodes are ERROR, missing nodes are MISSING <kind>, and • marks nodes that contain an error.',
      publicProjections: [
        'Lean: the public root `file` wraps the grammar root `module`; the comparison starts at `module`.',
        'Rocq: every `ident` leaf gains one semantic `identifier` or `primitive_type` token child; the comparison drops it.',
      ],
    },
    languages: {},
  };
  const directories = new Map();
  try {
    for (const grammar of ['markdown', 'html', ...LANGUAGES.map((language) => language.id)]) {
      directories.set(grammar, await cliGrammarDirectory(grammar, versions, scratch));
    }
    const regionStart = await mixedRegions(cases, treeSitter, directories, scratch);
    for (const language of LANGUAGES) {
      const checkout = await upstreamCheckout(language, work, lock, versions, treeSitter);
      const dir = join(fixtureDir, language.id);
      const corpusDir = join(checkout, 'test/corpus');
      const corpus = new Map();
      for (const file of (await readdir(corpusDir)).filter((name) => name.endsWith('.txt')).sort()) {
        corpus.set(file, await readFile(join(corpusDir, file), 'utf8'));
        outputs.set(join(dir, 'corpus', file), await readFile(join(corpusDir, file)));
      }
      const projects = new Map();
      const projectEntries = [];
      for (const project of language.projects) {
        const commit = await resolveTag(project.project, project.tag);
        const url = `https://raw.githubusercontent.com/${project.project}/${commit}/${project.path}`;
        const bytes = await fetchBytes(url);
        const file = basename(project.path);
        const licenseFile = `${project.project.replace('/', '-')}.LICENSE`;
        projects.set(file, bytes);
        outputs.set(join(dir, 'projects', file), bytes);
        outputs.set(join(dir, 'projects', licenseFile),
          await fetchBytes(`https://raw.githubusercontent.com/${project.project}/${commit}/${project.licensePath}`));
        projectEntries.push({
          project: project.project,
          tag: project.tag,
          commit,
          path: project.path,
          url: `https://github.com/${project.project}/blob/${commit}/${project.path}`,
          license: project.license,
          licenseFile: `${language.id}/projects/${licenseFile}`,
          file: `${language.id}/projects/${file}`,
          bytes: bytes.length,
          sha256: sha256(bytes),
        });
      }
      const oracleCases = await languageCases(language, {
        treeSitter, directories, scratch, cases, errorRecovery, regionStart, files: { corpus, projects },
      });
      const oracleText = formatOracle({ language: language.name, cases: oracleCases });
      outputs.set(join(dir, 'oracle.json'), oracleText);
      const counts = {};
      for (const entry of oracleCases) counts[entry.kind] = (counts[entry.kind] ?? 0) + 1;
      const upstream = {};
      for (const entry of oracleCases.filter((item) => item.upstream)) upstream[entry.upstream] = (upstream[entry.upstream] ?? 0) + 1;
      manifest.languages[language.name] = {
        grammar: {
          id: language.id,
          upstream: language.upstream,
          revision: language.revision,
          ...(language.tag ? { tag: language.tag } : {}),
          ...(language.patch ? { patch: language.patch } : {}),
          parserSha256: lock.grammars[language.id].parserSha256,
          license: `js/src/vendor/grammars/${lock.grammars[language.id].license}`,
        },
        corpus: {
          directory: `${language.id}/corpus`,
          url: `https://github.com/${language.upstream}/tree/${language.revision}/test/corpus`,
          files: Object.fromEntries([...corpus].map(([file, text]) => [file, sha256(text)])),
          upstreamExpectation: upstream,
        },
        projects: projectEntries,
        oracle: `${language.id}/oracle.json`,
        oracleSha256: sha256(oracleText),
        cases: counts,
        inventory: await inventory(checkout),
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
  const workIndex = args.indexOf('--work');
  const work = workIndex >= 0 ? resolve(args[workIndex + 1]) : await mkdtemp(join(tmpdir(), 'issue-195-upstream-'));
  await mkdir(work, { recursive: true });
  try {
    const outputs = await generate(work, treeSitter);
    if (args.includes('--check')) {
      const stale = [];
      for (const [path, content] of outputs) {
        const current = await readFile(path).catch(() => null);
        if (!current || !current.equals(Buffer.from(content))) stale.push(relative(path));
      }
      if (stale.length) {
        console.error(`stale conformance fixtures (run node js/scripts/generate-issue-195-conformance.mjs):\n  ${stale.join('\n  ')}`);
        process.exit(1);
      }
      console.log(`issue 195 conformance fixtures match (${outputs.size} files)`);
      return;
    }
    for (const [path, content] of outputs) {
      await mkdir(dirname(path), { recursive: true });
      await writeFile(path, content);
    }
    console.log(`wrote ${outputs.size} files to ${relative(fixtureDir)}`);
  } finally {
    if (workIndex < 0) await rm(work, { recursive: true, force: true });
  }
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  await main();
}
