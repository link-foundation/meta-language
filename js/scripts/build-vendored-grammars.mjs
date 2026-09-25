#!/usr/bin/env node
// Rebuilds js/src/vendor/grammars/*.wasm from the exact grammar crates pinned
// in rust/Cargo.lock, so both runtimes parse with byte-identical generated
// parsers. Requires `cargo fetch` (for crate sources), the tree-sitter CLI
// 0.25.10 and Docker or emcc (used by `tree-sitter build --wasm`).
//
//   node js/scripts/build-vendored-grammars.mjs [--only id,id] [--jobs N]
//   node js/scripts/build-vendored-grammars.mjs --check   # verify lock only
//   node js/scripts/build-vendored-grammars.mjs --notice  # refresh licenses and NOTICE.md
import { createHash } from 'node:crypto';
import { gunzipSync, gzipSync } from 'node:zlib';
import { execFile } from 'node:child_process';
import { cp, mkdir, mkdtemp, readFile, rm, writeFile, readdir } from 'node:fs/promises';
import { existsSync } from 'node:fs';
import { homedir, tmpdir } from 'node:os';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { promisify } from 'node:util';

const run = promisify(execFile);
const root = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const vendorDir = join(root, 'js/src/vendor/grammars');
const lockPath = join(vendorDir, 'grammar-lock.json');
const rustLockPath = join(root, 'rust/src/data/grammar-lock.json');

/**
 * Grammar id -> Rust crate and the parser directory inside the crate, or the
 * generated parser vendored under rust/vendor (compiled by rust/build.rs).
 */
export const GRAMMAR_SOURCES = Object.freeze({
  c: { crate: 'tree-sitter-c', dir: '.' },
  cpp: { crate: 'tree-sitter-cpp', dir: '.' },
  csv: {
    vendored: 'rust/vendor/tree-sitter-csv',
    upstream: 'tree-sitter-grammars/tree-sitter-csv',
    version: 'f6bf6e35eb0b95fbadea4bb39cb9709507fcb181',
    revision: 'f6bf6e35eb0b95fbadea4bb39cb9709507fcb181',
    // RFC 4180: unquoted fields contain no quotes, so a stray or unterminated
    // quote is a parse error instead of text.
    patch: 'rust/vendor/tree-sitter-csv/rfc4180-quotes.patch',
    dir: 'csv',
  },
  csharp: { crate: 'tree-sitter-c-sharp', dir: '.' },
  css: { crate: 'tree-sitter-css', dir: '.' },
  dtd: { crate: 'tree-sitter-xml', dir: 'dtd' },
  go: { crate: 'tree-sitter-go', dir: '.' },
  graphql: { crate: 'tree-sitter-graphql', dir: '.' },
  html: { crate: 'tree-sitter-html', dir: '.' },
  ini: { crate: 'tree-sitter-ini', dir: '.' },
  java: { crate: 'tree-sitter-java', dir: '.' },
  javascript: { crate: 'tree-sitter-javascript', dir: '.' },
  json: { crate: 'tree-sitter-json', dir: '.' },
  json5: { crate: 'tree-sitter-json5-orchard', dir: '.' },
  kotlin: { crate: 'tree-sitter-kotlin-ng', dir: '.' },
  lean: { crate: 'tree-sitter-lean4', dir: '.' },
  lua: { crate: 'tree-sitter-lua', dir: '.' },
  markdown: { crate: 'tree-sitter-md-025', dir: 'tree-sitter-markdown' },
  markdown_inline: { crate: 'tree-sitter-md-025', dir: 'tree-sitter-markdown-inline' },
  pascal: { crate: 'tree-sitter-pascal', dir: '.' },
  perl: { crate: 'ts-parser-perl', dir: '.' },
  php: { crate: 'tree-sitter-php', dir: 'php' },
  proto: { crate: 'tree-sitter-proto', dir: '.' },
  python: { crate: 'tree-sitter-python', dir: '.' },
  r: { crate: 'tree-sitter-r', dir: '.' },
  ruby: { crate: 'tree-sitter-ruby', dir: '.' },
  rocq: {
    vendored: 'rust/vendor/tree-sitter-rocq',
    upstream: 'aruzdh/tree-sitter-rocq',
    version: '300fe33fc299c30f736fd56d8ef8a28b08acd4e6',
    revision: '300fe33fc299c30f736fd56d8ef8a28b08acd4e6',
    dir: '.',
  },
  rust: { crate: 'tree-sitter-rust', dir: '.' },
  scala: { crate: 'tree-sitter-scala', dir: '.' },
  sql: { crate: 'tree-sitter-sequel', dir: '.' },
  swift: { crate: 'tree-sitter-swift', dir: '.' },
  toml: { crate: 'tree-sitter-toml-ng', dir: '.' },
  tsx: { crate: 'tree-sitter-typescript', dir: 'tsx' },
  typescript: { crate: 'tree-sitter-typescript', dir: 'typescript' },
  vb: { crate: 'tree-sitter-vb-dotnet', dir: '.' },
  xml: { crate: 'tree-sitter-xml', dir: 'xml' },
  yaml: { crate: 'tree-sitter-yaml', dir: '.' },
});

// zlib writes a zero mtime, so identical wasm always yields identical archives.
const compressWasm = (wasm) => gzipSync(wasm, { level: 9 });
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

async function cargoLockVersions() {
  const lock = await readFile(join(root, 'rust/Cargo.lock'), 'utf8');
  const versions = new Map();
  for (const block of lock.split('[[package]]')) {
    const name = /^name = "([^"]+)"/m.exec(block)?.[1];
    const version = /^version = "([^"]+)"/m.exec(block)?.[1];
    if (name && version && !/^source = "git/m.test(block)) versions.set(name, version);
  }
  return versions;
}

async function registrySourceDir(crate, version) {
  const registry = join(process.env.CARGO_HOME ?? join(homedir(), '.cargo'), 'registry/src');
  for (const index of await readdir(registry)) {
    const candidate = join(registry, index, `${crate}-${version}`);
    if (existsSync(candidate)) return candidate;
  }
  throw new Error(`${crate} ${version} is not in the cargo registry; run cargo fetch in rust/`);
}

const MIT_TERMS = `Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
`;

// Published grammar crates often ship only a `license = "MIT"` manifest
// field. The MIT terms are then reproduced with the manifest's authors so the
// notice travels with the compiled grammar.
async function licenseText(crateDir) {
  for (const name of ['LICENSE', 'LICENSE-MIT', 'LICENSE.md', 'LICENSE.txt']) {
    const path = join(crateDir, name);
    if (existsSync(path)) return readFile(path, 'utf8');
  }
  const manifestPath = join(crateDir, 'Cargo.toml');
  if (!existsSync(manifestPath)) return null;
  const manifest = await readFile(manifestPath, 'utf8');
  const field = (key) => new RegExp(`^${key}\\s*=\\s*"([^"]*)"`, 'm').exec(manifest)?.[1];
  if (field('license') !== 'MIT') return null;
  const authors = /^authors\s*=\s*\[([^\]]*)\]/m.exec(manifest)?.[1]
    .split(',').map((author) => author.trim().replace(/^"|"$/g, '')).filter(Boolean) ?? [];
  const holders = authors.length ? authors.join(', ') : `the ${field('name')} authors`;
  return `MIT License

The \`${field('name')}\` Cargo manifest declares \`license = "MIT"\` but ships
no license file; this notice reproduces the MIT terms for its authors.
Repository: ${field('repository') ?? 'not declared'}

Copyright (c) ${holders}

${MIT_TERMS}`;
}

// Some published crates omit tree-sitter.json, which the CLI needs to name the
// wasm module. The exported `tree_sitter_<name>` symbol is authoritative.
async function ensureGrammarConfig(grammarDir) {
  if (existsSync(join(grammarDir, 'tree-sitter.json'))) return;
  const parser = await readFile(join(grammarDir, 'src/parser.c'), 'utf8');
  const name = /(?:TS_PUBLIC|extern)[^\n]*\btree_sitter_(\w+)\s*\(\s*void\s*\)/.exec(parser)?.[1];
  if (!name) throw new Error(`cannot find the exported language symbol in ${grammarDir}`);
  const config = {
    grammars: [{ name, scope: `source.${name}`, path: '.', 'file-types': [] }],
    metadata: { version: '0.0.0' },
  };
  await writeFile(join(grammarDir, 'tree-sitter.json'), JSON.stringify(config));
}

// The Docker emcc build mounts only the grammar's `src`, so a scanner that
// includes a sibling grammar's shared header (TypeScript/TSX use
// `../../common/scanner.h`) gets a byte-identical local copy instead.
async function localizeSharedIncludes(grammarDir) {
  const scannerPath = join(grammarDir, 'src/scanner.c');
  if (!existsSync(scannerPath)) return;
  const scanner = await readFile(scannerPath, 'utf8');
  let localized = scanner;
  for (const [directive, relative] of scanner.matchAll(/#include "(\.\.\/[^"]+)"/g)) {
    const name = `shared_${relative.split('/').pop()}`;
    await cp(join(grammarDir, 'src', relative), join(grammarDir, 'src', name));
    localized = localized.replace(directive, `#include "${name}"`);
  }
  if (localized !== scanner) await writeFile(scannerPath, localized);
}

async function grammarSource(id, versions) {
  const source = GRAMMAR_SOURCES[id];
  if (source.vendored) {
    return {
      ...source,
      crateDir: join(root, source.vendored),
      parser: gunzipSync(await readFile(join(root, source.vendored, 'src/parser.c.gz'))),
    };
  }
  const version = versions.get(source.crate);
  if (!version) throw new Error(`${source.crate} is not pinned in rust/Cargo.lock`);
  const crateDir = await registrySourceDir(source.crate, version);
  return {
    ...source,
    version,
    crateDir,
    parser: await readFile(join(crateDir, source.dir, 'src/parser.c')),
  };
}

// Clones the pinned upstream revision. With `treeSitter`, it also applies the
// grammar's local patch, regenerates the parser and requires it to equal the
// one Rust compiles.
async function checkoutUpstream(source, checkout, treeSitter) {
  await run('git', ['clone', '--quiet', `https://github.com/${source.upstream}`, checkout]);
  await run('git', ['-C', checkout, 'checkout', '--quiet', source.revision]);
  if (!treeSitter) return;
  if (source.patch) {
    await run('git', ['-C', checkout, 'apply', join(root, source.patch)]);
    await run(treeSitter, ['generate'], { cwd: join(checkout, source.dir) });
  }
  const upstream = await readFile(join(checkout, source.dir, 'src/parser.c'));
  if (sha256(upstream) !== sha256(source.parser)) {
    const patched = source.patch ? ` with ${source.patch}` : '';
    throw new Error(`${source.upstream}@${source.revision}${patched} parser.c differs from ${source.vendored}`);
  }
}

/** Rewrites every grammar's license file from its pinned source. */
async function refreshLicenses(lock, versions) {
  for (const id of Object.keys(lock.grammars)) {
    const source = await grammarSource(id, versions);
    const work = await mkdtemp(join(tmpdir(), `grammar-${id}-`));
    try {
      let dir = source.crateDir;
      if (source.vendored) {
        dir = join(work, 'crate');
        await checkoutUpstream(source, dir);
      }
      const license = await licenseText(dir);
      if (license) await writeFile(join(vendorDir, `${id}.LICENSE`), license);
      lock.grammars[id].license = license ? `${id}.LICENSE` : null;
    } finally {
      await rm(work, { recursive: true, force: true });
    }
  }
}

async function buildOne(id, versions, treeSitter) {
  const source = await grammarSource(id, versions);
  // Docker can only mount writable, non-hidden paths, so build from a copy.
  const work = await mkdtemp(join(tmpdir(), `grammar-${id}-`));
  try {
    if (source.vendored) {
      // The CLI needs the full grammar tree, so build from the pinned upstream
      // checkout and require its parser to equal the one Rust compiles.
      await checkoutUpstream(source, join(work, 'crate'), treeSitter);
    } else {
      await cp(source.crateDir, join(work, 'crate'), { recursive: true });
    }
    const grammarDir = join(work, 'crate', source.dir);
    await localizeSharedIncludes(grammarDir);
    await ensureGrammarConfig(grammarDir);
    const output = join(work, `${id}.wasm`);
    await run(treeSitter, ['build', '--wasm', '-o', output, grammarDir], {
      cwd: work,
      maxBuffer: 64 * 1024 * 1024,
    });
    const wasm = await readFile(output);
    await writeFile(join(vendorDir, `${id}.wasm.gz`), compressWasm(wasm));
    const license = await licenseText(source.vendored ? join(work, 'crate') : source.crateDir);
    if (license) await writeFile(join(vendorDir, `${id}.LICENSE`), license);
    return {
      id,
      ...(source.vendored
        ? {
            upstream: source.upstream,
            vendored: source.vendored,
            ...(source.patch ? { patch: source.patch } : {}),
          }
        : { crate: source.crate, directory: source.dir }),
      version: source.version,
      parserSha256: sha256(source.parser),
      wasmSha256: sha256(wasm),
      license: license ? `${id}.LICENSE` : null,
    };
  } finally {
    await rm(work, { recursive: true, force: true });
  }
}

/** Verifies every grammar is locked to the Rust build input and its wasm. */
async function checkLock(lock, versions) {
  const problems = [];
  const extra = Object.keys(lock?.grammars ?? {}).filter((id) => !GRAMMAR_SOURCES[id]);
  for (const id of extra) problems.push(`${id}: locked but not a known grammar source`);
  for (const [id, source] of Object.entries(GRAMMAR_SOURCES)) {
    const entry = lock?.grammars?.[id];
    const pinned = source.vendored ? source.version : versions.get(source.crate);
    if (!entry) {
      problems.push(`${id}: missing from grammar lock`);
      continue;
    }
    if (entry.version !== pinned) {
      problems.push(`${id}: lock has ${entry.version}, the Rust build pins ${pinned}`);
      continue;
    }
    if ((entry.patch ?? null) !== (source.patch ?? null)) {
      problems.push(`${id}: lock records patch ${entry.patch ?? 'none'}, the source has ${source.patch ?? 'none'}`);
    }
    const { parser } = await grammarSource(id, versions);
    if (sha256(parser) !== entry.parserSha256) {
      problems.push(`${id}: parser.c digest differs from the Rust build input`);
    }
    const wasm = gunzipSync(await readFile(join(vendorDir, `${id}.wasm.gz`)));
    if (sha256(wasm) !== entry.wasmSha256) problems.push(`${id}: wasm digest mismatch`);
  }
  return problems;
}

async function writeNotice(lock) {
  const rows = Object.values(lock.grammars).map((entry) => {
    const source = entry.crate
      ? `crate \`${entry.crate}\` ${entry.version}${entry.directory === '.' ? '' : ` (\`${entry.directory}\`)`}`
      : `\`${entry.upstream}\` ${entry.version}${entry.patch ? ` with [\`${entry.patch.split('/').pop()}\`](../../../../${entry.patch})` : ''} (vendored in \`${entry.vendored}\`)`;
    const license = entry.license ? `[\`${entry.license}\`](${entry.license})` : 'see upstream';
    return `| \`${entry.id}.wasm.gz\` | ${source} | \`${entry.parserSha256}\` | \`${entry.wasmSha256}\` | ${license} |`;
  });
  const notice = `# Vendored tree-sitter grammars

These WebAssembly grammars are compiled from exactly the generated parsers the
Rust crate links (\`rust/Cargo.lock\` crates or \`rust/vendor\`), so both
runtimes parse with the same grammar revision. They are rebuilt by
\`node js/scripts/build-vendored-grammars.mjs\` with tree-sitter CLI
${lock.treeSitterCli.split(' ')[1]} and verified against \`grammar-lock.json\` by
\`--check\`. Each file is a zero-mtime gzip of the \`.wasm\` whose SHA-256 is
listed; the parser digest is of the generated \`src/parser.c\` (after the
listed patch, if any).

| File | Source | parser.c SHA-256 | wasm SHA-256 | License |
| --- | --- | --- | --- | --- |
${rows.join('\n')}

Every grammar is distributed under its upstream license (MIT unless the
linked license file states otherwise).
`;
  await writeFile(join(vendorDir, 'NOTICE.md'), notice);
}

/** The lock ships in both packages: npm under src/vendor, the crate under src/data. */
async function writeLock(lock) {
  const text = `${JSON.stringify(lock, null, 2)}\n`;
  await writeFile(lockPath, text);
  await writeFile(rustLockPath, text);
  await writeNotice(lock);
}

async function main() {
  const args = process.argv.slice(2);
  const versions = await cargoLockVersions();
  const previous = existsSync(lockPath) ? JSON.parse(await readFile(lockPath, 'utf8')) : null;

  if (args.includes('--check')) {
    const problems = await checkLock(previous, versions);
    if (!existsSync(rustLockPath) || (await readFile(rustLockPath, 'utf8')) !== (await readFile(lockPath, 'utf8'))) {
      problems.push('rust/src/data/grammar-lock.json differs from the npm grammar lock');
    }
    if (problems.length) {
      console.error(problems.join('\n'));
      process.exit(1);
    }
    console.log(`grammar lock matches the Rust build for ${Object.keys(GRAMMAR_SOURCES).length} grammars`);
    return;
  }
  if (args.includes('--notice')) {
    await refreshLicenses(previous, versions);
    await writeLock(previous);
    return;
  }

  const onlyIndex = args.indexOf('--only');
  const only = onlyIndex >= 0 ? new Set(args[onlyIndex + 1].split(',')) : null;
  const jobsIndex = args.indexOf('--jobs');
  const jobs = jobsIndex >= 0 ? Number(args[jobsIndex + 1]) : 3;
  const treeSitter = process.env.TREE_SITTER_CLI ?? 'tree-sitter';
  const { stdout: cliVersion } = await run(treeSitter, ['--version']);

  await mkdir(vendorDir, { recursive: true });
  const ids = Object.keys(GRAMMAR_SOURCES).filter((id) => !only || only.has(id));
  const grammars = { ...(previous?.grammars ?? {}) };
  const queue = [...ids];
  const failures = [];
  await Promise.all(
    Array.from({ length: Math.max(1, jobs) }, async () => {
      while (queue.length) {
        const id = queue.shift();
        const started = Date.now();
        try {
          grammars[id] = await buildOne(id, versions, treeSitter);
          console.log(`built ${id} in ${((Date.now() - started) / 1000).toFixed(1)}s`);
        } catch (error) {
          failures.push(id);
          console.error(`failed ${id}: ${error.stderr ?? error.message}`);
        }
      }
    }),
  );

  const sorted = Object.fromEntries(Object.entries(grammars).sort(([a], [b]) => a.localeCompare(b)));
  const lock = {
    description:
      'Grammar crates pinned by rust/Cargo.lock and their WebAssembly builds used by the JavaScript runtime.',
    treeSitterCli: cliVersion.trim(),
    treeSitterRuntime: {
      rust: versions.get('tree-sitter'),
      javascript: JSON.parse(await readFile(join(root, 'js/node_modules/web-tree-sitter/package.json'), 'utf8')).version,
    },
    grammars: sorted,
  };
  await writeLock(lock);
  if (failures.length) {
    console.error(`failed grammars: ${failures.join(', ')}`);
    process.exit(1);
  }
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  await main();
}
