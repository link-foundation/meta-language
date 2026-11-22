#!/usr/bin/env node
// Rebuilds js/src/vendor/grammars/*.wasm from the exact grammar crates pinned
// in rust/Cargo.lock, so both runtimes parse with byte-identical generated
// parsers. The oracle grammars of the languages a native grammar parses are
// built into js/oracles/grammars, which the npm package does not ship. Requires `cargo fetch` (for crate sources) and the tree-sitter CLI
// pinned in the grammar lock, which downloads the WASI SDK used for WebAssembly.
//
//   node js/scripts/build-vendored-grammars.mjs [--only id,id] [--jobs N]
//   node js/scripts/build-vendored-grammars.mjs --only id,id --refresh-external-assets
//   node js/scripts/build-vendored-grammars.mjs --check   # verify lock only
//   node js/scripts/build-vendored-grammars.mjs --notice  # refresh licenses and NOTICE.md
//   node js/scripts/build-vendored-grammars.mjs --regenerate  # after a CLI update: replace the
//                                                             # vendored parsers patched grammars generate
import { createHash } from 'node:crypto';
import { gunzipSync, gzipSync } from 'node:zlib';
import { execFile } from 'node:child_process';
import { cp, mkdir, readFile, rm, writeFile, readdir } from 'node:fs/promises';
import { existsSync } from 'node:fs';
import { homedir } from 'node:os';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { promisify } from 'node:util';

import { makeScratchDirectory } from '../../scripts/lib/scratch.mjs';
import { ORACLE_GRAMMAR_DIRECTORY, PACKAGED_GRAMMAR_DIRECTORY, grammarDirectory, grammarFile } from './grammar-files.mjs';
import { externalGrammarDirectory, readExternalGrammarAsset } from './external-grammar-cache.mjs';
import { readProjectData, resolveProjectDataPath, writeProjectData } from './project-data.mjs';
import { compileGrammarWebAssembly, compressGrammarWebAssembly } from './compile-grammar-webassembly.mjs';

export { ensureGrammarConfig } from './compile-grammar-webassembly.mjs';

const run = promisify(execFile);
const root = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const vendorDir = join(root, PACKAGED_GRAMMAR_DIRECTORY);
const oracleDir = join(root, ORACLE_GRAMMAR_DIRECTORY);
const lockPath = join(vendorDir, 'grammar-lock.json');
const rustLockPath = join(root, 'rust/src/data/grammar-lock.json');
const externalAssetPath = join(root, 'parity/grammars/external-assets.lino');

/**
 * Grammar id -> Rust crate and the parser directory inside the crate, or the
 * generated parser vendored under rust/vendor (compiled by rust/build.rs).
 * `oracle` marks the tree-sitter oracle of a language a native grammar parses:
 * a Rust development dependency (the CSV parser is not compiled at all) whose
 * WebAssembly build the npm package does not ship. `wasmPatch` changes only
 * the hand-written scanner of the WebAssembly build of a crate, never its
 * generated parser or the Rust build.
 */
export const GRAMMAR_SOURCES = Object.freeze({
  agda: { crate: 'tree-sitter-agda', dir: '.', oracle: true },
  bash: { crate: 'tree-sitter-bash', dir: '.' },
  c: { crate: 'tree-sitter-c', dir: '.', oracle: true },
  cmake: {
    oracle: true,
    vendored: 'rust/vendor/tree-sitter-cmake',
    upstream: 'uyha/tree-sitter-cmake',
    version: 'v0.7.5',
    revision: 'e997bd0b275ca525ce9befecedf5299031183661',
    // The external scanner starts with no bracket open instead of reading
    // uninitialised memory, so error recovery is the same on every platform.
    patch: 'rust/vendor/tree-sitter-cmake/scanner-state.patch',
    dir: '.',
  },
  cpp: { crate: 'tree-sitter-cpp', dir: '.', oracle: true },
  csv: {
    vendored: 'rust/vendor/tree-sitter-csv',
    upstream: 'tree-sitter-grammars/tree-sitter-csv',
    version: 'f6bf6e35eb0b95fbadea4bb39cb9709507fcb181',
    revision: 'f6bf6e35eb0b95fbadea4bb39cb9709507fcb181',
    // RFC 4180: unquoted fields contain no quotes, so a stray or unterminated
    // quote is a parse error instead of text.
    patch: 'rust/vendor/tree-sitter-csv/rfc4180-quotes.patch',
    oracle: true,
    dir: 'csv',
  },
  csharp: { crate: 'tree-sitter-c-sharp', dir: '.' },
  css: { crate: 'tree-sitter-css', dir: '.', oracle: true },
  dart: { crate: 'tree-sitter-dart', dir: '.', oracle: true },
  diff: { crate: 'tree-sitter-diff', dir: '.', oracle: true },
  dtd: { crate: 'tree-sitter-xml', dir: 'dtd', oracle: true },
  elixir: { crate: 'tree-sitter-elixir', dir: '.' },
  elm: { crate: 'tree-sitter-elm', dir: '.' },
  erlang: { crate: 'tree-sitter-erlang', dir: '.', oracle: true },
  go: { crate: 'tree-sitter-go', dir: '.', oracle: true },
  graphql: { crate: 'tree-sitter-graphql', dir: '.', oracle: true },
  groovy: { crate: 'tree-sitter-groovy', dir: '.', oracle: true },
  haskell: { crate: 'tree-sitter-haskell', dir: '.' },
  hcl: { crate: 'tree-sitter-hcl', dir: '.', oracle: true },
  html: { crate: 'tree-sitter-html', dir: '.', oracle: true },
  ini: { crate: 'tree-sitter-ini', dir: '.', oracle: true },
  java: { crate: 'tree-sitter-java', dir: '.', oracle: true },
  javascript: { crate: 'tree-sitter-javascript', dir: '.', oracle: true },
  json: { crate: 'tree-sitter-json', dir: '.', oracle: true },
  json5: { crate: 'tree-sitter-json5-orchard', dir: '.', oracle: true },
  kotlin: { crate: 'tree-sitter-kotlin-ng', dir: '.' },
  lean: {
    oracle: true,
    vendored: 'rust/vendor/tree-sitter-lean',
    upstream: 'wvhulle/tree-sitter-lean',
    version: 'bd942cd2795016239be02b3b3d5ef635645ddd38',
    revision: 'bd942cd2795016239be02b3b3d5ef635645ddd38',
    // The tree-sitter-lean4 0.3.0 crate's sources, vendored so the development oracle build
    // compiles them with MSVC's /utf-8: the crate's build script does not, and
    // its non-ASCII node names then are not UTF-8 on Windows.
    dir: '.',
  },
  lua: { crate: 'tree-sitter-lua', dir: '.', oracle: true },
  make: { crate: 'tree-sitter-make', dir: '.', oracle: true },
  markdown: { crate: 'tree-sitter-md-025', dir: 'tree-sitter-markdown' },
  markdown_inline: { crate: 'tree-sitter-md-025', dir: 'tree-sitter-markdown-inline' },
  matlab: { crate: 'tree-sitter-matlab', dir: '.' },
  nix: { crate: 'tree-sitter-nix', dir: '.', oracle: true },
  ocaml: { crate: 'tree-sitter-ocaml', dir: 'grammars/ocaml' },
  ocaml_interface: { crate: 'tree-sitter-ocaml', dir: 'grammars/interface' },
  odin: { crate: 'tree-sitter-odin', dir: '.', oracle: true },
  pascal: { crate: 'tree-sitter-pascal', dir: '.', oracle: true },
  perl: { crate: 'ts-parser-perl', dir: '.' },
  php: { crate: 'tree-sitter-php', dir: 'php' },
  powershell: { crate: 'tree-sitter-powershell', dir: '.', oracle: true },
  proto: { crate: 'tree-sitter-proto', dir: '.', oracle: true },
  python: { crate: 'tree-sitter-python', dir: '.', oracle: true },
  r: { crate: 'tree-sitter-r', dir: '.', oracle: true },
  racket: { crate: 'tree-sitter-racket', dir: '.', oracle: true },
  regex: { crate: 'tree-sitter-regex', dir: '.', oracle: true },
  ruby: { crate: 'tree-sitter-ruby', dir: '.' },
  rocq: {
    oracle: true,
    vendored: 'rust/vendor/tree-sitter-rocq',
    upstream: 'aruzdh/tree-sitter-rocq',
    version: '300fe33fc299c30f736fd56d8ef8a28b08acd4e6',
    revision: '300fe33fc299c30f736fd56d8ef8a28b08acd4e6',
    // Translated-program constructs (Recdef's `Function`, mutual and
    // redefining `Ltac`, `first`/`solve`, Ltac `let ... in`) and the
    // vernacular, term and Ltac syntax the pinned Rocq stdlib files use.
    patch: 'rust/vendor/tree-sitter-rocq/meta-language.patch',
    dir: '.',
  },
  rust: {
    vendored: 'rust/vendor/tree-sitter-rust',
    upstream: 'tree-sitter/tree-sitter-rust',
    version: 'v0.24.2',
    revision: '77a3747266f4d621d0757825e6b11edcbf991ca5',
    // Syntax real published crates use that the release reports as errors:
    // `~` and bare `$` in macro token trees, unit structs with a where
    // clause, attributes on struct-pattern fields and later tuple elements,
    // `as T <= e`, unit types in where predicates, turbofish on functions
    // named like primitive types, the 2015 `try!` macro and cargo script
    // frontmatter.
    patch: 'rust/vendor/tree-sitter-rust/meta-language.patch',
    oracle: true,
    dir: '.',
  },
  scala: { crate: 'tree-sitter-scala', dir: '.' },
  scheme: { crate: 'tree-sitter-scheme', dir: '.', oracle: true },
  solidity: { crate: 'tree-sitter-solidity', dir: '.', oracle: true },
  sql: { crate: 'tree-sitter-sequel', dir: '.', oracle: true },
  // The 0.7.4 scanner exits through stderr when allocation fails, which a
  // Wasm parser cannot import; the Wasm build traps instead.
  swift: { crate: 'tree-sitter-swift', dir: '.', wasmPatch: 'js/scripts/grammar-patches/tree-sitter-swift-wasm.patch' },
  toml: { crate: 'tree-sitter-toml-ng', dir: '.', oracle: true },
  tsx: { crate: 'tree-sitter-typescript', dir: 'tsx', oracle: true },
  typescript: { crate: 'tree-sitter-typescript', dir: 'typescript', oracle: true },
  vb: { crate: 'tree-sitter-vb-dotnet', dir: '.', oracle: true },
  xml: { crate: 'tree-sitter-xml', dir: 'xml', oracle: true },
  yaml: { crate: 'tree-sitter-yaml', dir: '.' },
  zig: { crate: 'tree-sitter-zig', dir: '.', oracle: true },
});

const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

export async function cargoLockVersions() {
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
    if (existsSync(path)) return (await readFile(path, 'utf8')).trimEnd() + '\n';
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

export async function grammarSource(id, versions) {
  const source = GRAMMAR_SOURCES[id];
  if (source.vendored) {
    return {
      ...source,
      crateDir: externalGrammarDirectory(source.vendored),
      parser: gunzipSync(readExternalGrammarAsset(`${source.vendored}/src/parser.c.gz`)),
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
// one Rust compiles; with `regenerate`, a regenerated parser and its headers
// replace the vendored ones instead (after a CLI update).
export async function checkoutUpstream(source, checkout, treeSitter, { regenerate = false } = {}) {
  await run('git', ['clone', '--quiet', `https://github.com/${source.upstream}`, checkout]);
  await run('git', ['-C', checkout, 'checkout', '--quiet', source.revision]);
  if (!treeSitter) return;
  if (source.patch) {
    await run('git', ['-C', checkout, 'apply', join(root, source.patch)]);
    // A patch that only fixes the hand-written scanner keeps the committed
    // generated parser; one that changes the grammar regenerates it.
    const { stdout: changed } = await run('git', ['-C', checkout, 'diff', '--name-only']);
    if (!changed.trim().split('\n').every((path) => path.endsWith('src/scanner.c'))) {
      await run(treeSitter, ['generate'], { cwd: join(checkout, source.dir) });
      if (regenerate) await replaceVendoredParser(source, join(checkout, source.dir, 'src'));
    }
  }
  const upstream = await readFile(join(checkout, source.dir, 'src/parser.c'));
  if (sha256(upstream) !== sha256(source.parser)) {
    const patched = source.patch ? ` with ${source.patch}` : '';
    throw new Error(`${source.upstream}@${source.revision}${patched} parser.c differs from ${source.vendored}`);
  }
}

// Writes a regenerated parser (deterministically compressed) and the generated
// headers the vendored copy keeps, so rust/build.rs compiles them.
async function replaceVendoredParser(source, generated) {
  const parser = await readFile(join(generated, 'parser.c'));
  await writeFile(join(externalGrammarDirectory(`${source.vendored}/src`), 'parser.c.gz'), gzipSync(parser, { level: 9 }));
  const headers = externalGrammarDirectory(`${source.vendored}/src/tree_sitter`);
  for (const header of await readdir(headers)) {
    if (existsSync(join(generated, 'tree_sitter', header))) await cp(join(generated, 'tree_sitter', header), join(headers, header));
  }
  source.parser = parser;
}

/** Rewrites every grammar's license file from its pinned source. */
async function refreshLicenses(lock, versions) {
  for (const id of Object.keys(lock.grammars)) {
    const source = await grammarSource(id, versions);
    const work = makeScratchDirectory(`grammar-${id}-`);
    try {
      let dir = source.crateDir;
      if (source.vendored) {
        dir = join(work, 'crate');
        await checkoutUpstream(source, dir);
      }
      const license = await licenseText(dir);
      if (license) await writeFile(join(root, grammarFile(lock.grammars[id], `${id}.LICENSE`)), license);
      lock.grammars[id].license = license ? `${id}.LICENSE` : null;
    } finally {
      await rm(work, { recursive: true, force: true });
    }
  }
}

// Applies a crate's WebAssembly-only patch to the scratch copy; it may touch
// only the hand-written scanner, so the parser stays the one Rust compiles.
async function applyWasmPatch(source, crate) {
  const patch = await readFile(join(root, source.wasmPatch), 'utf8');
  const touched = [...patch.matchAll(/^\+\+\+ b\/(.+)$/gmu)].map((match) => match[1]);
  if (touched.length === 0 || !touched.every((path) => path === join(source.dir, 'src/scanner.c').replace(/^\.\//u, ''))) {
    throw new Error(`${source.wasmPatch} may patch only ${source.dir}/src/scanner.c, not ${touched.join(', ')}`);
  }
  await run('git', ['apply', join(root, source.wasmPatch)], { cwd: crate });
}

async function externalBuild(source, id, cli, wasm, archive) {
  const crate = `${source.crate}-${source.version}`;
  const lock = await readFile(join(root, 'rust/Cargo.lock'), 'utf8');
  const entry = lock.split('[[package]]').find(block => block.includes(`\nname = "${source.crate}"\n`)
    && block.includes(`\nversion = "${source.version}"\n`)
    && block.includes('\nsource = "registry+https://github.com/rust-lang/crates.io-index"\n'));
  const checksum = /^checksum = "([a-f0-9]{64})"$/mu.exec(entry ?? '')?.[1];
  if (!checksum) throw new Error(`Rust does not lock an authenticated crates.io source for ${crate}`);
  const url = `https://static.crates.io/crates/${source.crate}/${crate}.crate`;
  const response = await fetch(url);
  if (!response.ok) throw new Error(`failed to download ${url}: HTTP ${response.status}`);
  const bytes = Buffer.from(await response.arrayBuffer());
  if (sha256(bytes) !== checksum) throw new Error(`crate archive checksum mismatch: ${crate}`);
  return {
    group: 'wasm', bytes: archive.length, sha256: sha256(archive), encoding: 'gzip',
    decodedBytes: wasm.length, decodedSha256: sha256(wasm),
    build: { crate, directory: source.dir, id, cli, url, bytes: bytes.length, sha256: checksum },
  };
}

async function buildOne(id, versions, treeSitter, options) {
  const source = await grammarSource(id, versions);
  // Build from a scratch copy so the cargo registry sources stay untouched.
  const work = makeScratchDirectory(`grammar-${id}-`);
  try {
    if (source.vendored) {
      // The CLI needs the full grammar tree, so build from the pinned upstream
      // checkout and require its parser to equal the one Rust compiles.
      await checkoutUpstream(source, join(work, 'crate'), treeSitter, options);
    } else {
      await cp(source.crateDir, join(work, 'crate'), { recursive: true });
      if (source.wasmPatch) await applyWasmPatch(source, join(work, 'crate'));
    }
    const wasm = await compileGrammarWebAssembly(work, join('crate', source.dir), id, [treeSitter]);
    const archive = compressGrammarWebAssembly(wasm);
    if (!options.refreshAssets) {
      const destination = externalGrammarDirectory(grammarDirectory(source));
      await writeFile(join(destination, `${id}.wasm.gz`), archive);
    }
    const license = await licenseText(source.vendored ? join(work, 'crate') : source.crateDir);
    if (license) await writeFile(join(root, grammarFile(source, `${id}.LICENSE`)), license);
    const entry = {
      id,
      ...(source.oracle ? { oracle: true } : {}),
      ...(source.vendored
        ? {
            upstream: source.upstream,
            vendored: source.vendored,
            ...(source.patch ? { patch: source.patch } : {}),
          }
        : { crate: source.crate, directory: source.dir, ...(source.wasmPatch ? { wasmPatch: source.wasmPatch } : {}) }),
      version: source.version,
      parserSha256: sha256(source.parser),
      wasmSha256: sha256(wasm),
      license: license ? `${id}.LICENSE` : null,
    };
    const asset = options.refreshAssets
      ? { ...await externalBuild(source, id, options.cli, wasm, archive), upstream: entry }
      : undefined;
    return { entry, asset };
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
    if (Boolean(entry.oracle) !== Boolean(source.oracle)) {
      problems.push(`${id}: lock records oracle ${Boolean(entry.oracle)}, the source has ${Boolean(source.oracle)}`);
    }
    if ((entry.wasmPatch ?? null) !== (source.wasmPatch ?? null)) {
      problems.push(`${id}: lock records wasm patch ${entry.wasmPatch ?? 'none'}, the source has ${source.wasmPatch ?? 'none'}`);
    }
    if ((entry.patch ?? null) !== (source.patch ?? null)) {
      problems.push(`${id}: lock records patch ${entry.patch ?? 'none'}, the source has ${source.patch ?? 'none'}`);
    }
    const { parser } = await grammarSource(id, versions);
    if (sha256(parser) !== entry.parserSha256) {
      problems.push(`${id}: parser.c digest differs from the Rust build input`);
    }
    const wasm = gunzipSync(readExternalGrammarAsset(grammarFile(source, `${id}.wasm.gz`)));
    if (sha256(wasm) !== entry.wasmSha256) problems.push(`${id}: wasm digest mismatch`);
  }
  return problems;
}

function noticeRows(entries) {
  return entries.map((entry) => {
    const source = entry.crate
      ? `crate \`${entry.crate}\` ${entry.version}${entry.directory === '.' ? '' : ` (\`${entry.directory}\`)`}${entry.wasmPatch ? ` with [\`${entry.wasmPatch.split('/').pop()}\`](../../../../${entry.wasmPatch})` : ''}`
      : `\`${entry.upstream}\` ${entry.version}${entry.patch ? ` with [\`${entry.patch.split('/').pop()}\`](${entry.oracle ? '../../..' : '../../../..'}/${entry.patch})` : ''} (vendored in \`${entry.vendored}\`)`;
    const license = entry.license ? `[\`${entry.license}\`](${entry.license})` : 'see upstream';
    return `| \`${entry.id}.wasm.gz\` | ${source} | \`${entry.parserSha256}\` | \`${entry.wasmSha256}\` | ${license} |`;
  });
}

/** Writes the NOTICE.md of the packaged grammars and of the oracle grammars. */
export async function writeNotice(lock) {
  const entries = Object.values(lock.grammars);
  const table = (oracle) => `| File | Source | parser.c SHA-256 | wasm SHA-256 | License |
| --- | --- | --- | --- | --- |
${noticeRows(entries.filter((entry) => Boolean(entry.oracle) === oracle)).join('\n')}

Every grammar is distributed under its upstream license (MIT unless the
linked license file states otherwise).
`;
  const build = `\`node js/scripts/build-vendored-grammars.mjs\` with tree-sitter CLI
${lock.treeSitterCli.split(' ')[1]} and verified against \`grammar-lock.json\` by
\`--check\`. Each file is a zero-mtime gzip of the \`.wasm\` whose SHA-256 is
listed; the parser digest is of the generated \`src/parser.c\` (after the
listed patch, if any).`;
  const notice = `# Vendored tree-sitter grammars

These WebAssembly grammars are compiled from exactly the generated parsers the
Rust crate links (\`rust/Cargo.lock\` crates or \`rust/vendor\`), so both
runtimes parse with the same grammar revision. They are rebuilt by
${build}

The tree-sitter oracles of the languages a native grammar parses are
development files this package does not ship (\`js/oracles/grammars\` in the
repository).

${table(false)}`;
  const oracles = `# Oracle tree-sitter grammars

The languages these grammars describe are parsed by the native grammars of
\`parity/grammars/native\`. The tree-sitter grammars stay as their oracles: the
fixture generators and the tests compare the native trees with theirs, so they
are development files the npm package does not ship (Rust links them as
development dependencies). Their \`grammar-lock.json\` entries have
\`"oracle": true\`. They are compiled from exactly the generated parsers of
\`rust/Cargo.lock\` crates or \`rust/vendor\`, and rebuilt by
${build}

${table(true)}`;
  await writeFile(join(vendorDir, 'NOTICE.md'), notice);
  await mkdir(oracleDir, { recursive: true });
  await writeFile(join(oracleDir, 'NOTICE.md'), oracles);
}

/** The lock ships in both packages: npm under src/vendor, the crate under src/data. */
async function writeLock(lock) {
  writeProjectData(lockPath, lock);
  writeProjectData(rustLockPath, lock);
  await writeNotice(lock);
}

async function main() {
  const args = process.argv.slice(2);
  const versions = await cargoLockVersions();
  const previous = existsSync(resolveProjectDataPath(lockPath)) ? readProjectData(lockPath) : null;

  if (args.includes('--check')) {
    const problems = await checkLock(previous, versions);
    if (!existsSync(resolveProjectDataPath(rustLockPath))
      || JSON.stringify(readProjectData(rustLockPath)) !== JSON.stringify(previous)) {
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
  const cliVersion = (await run(treeSitter, ['--version'])).stdout.trim().split(/\s+/u).slice(0, 2).join(' ');
  const cli = /^tree-sitter (\d+\.\d+\.\d+)$/u.exec(cliVersion)?.[1];
  if (!cli) throw new Error(`expected an exact Tree-sitter CLI version, received ${cliVersion}`);
  const refreshAssets = args.includes('--refresh-external-assets');

  await mkdir(vendorDir, { recursive: true });
  await mkdir(oracleDir, { recursive: true });
  const ids = Object.keys(GRAMMAR_SOURCES).filter((id) => !only || only.has(id));
  if (refreshAssets && (!only || ids.length !== only.size || ids.some((id) => {
    const source = GRAMMAR_SOURCES[id];
    return !source.crate || source.wasmPatch;
  }))) throw new Error('select unpatched registry grammars with --only when refreshing external assets');
  const grammars = { ...(previous?.grammars ?? {}) };
  const external = refreshAssets ? readProjectData(externalAssetPath) : null;
  const assets = new Map();
  const queue = [...ids];
  const failures = [];
  await Promise.all(
    Array.from({ length: Math.max(1, jobs) }, async () => {
      while (queue.length) {
        const id = queue.shift();
        const started = Date.now();
        try {
          const { entry, asset } = await buildOne(id, versions, treeSitter, {
            regenerate: args.includes('--regenerate'), refreshAssets, cli,
          });
          grammars[id] = entry;
          if (asset) assets.set(grammarFile(entry, `${id}.wasm.gz`), asset);
          console.log(`built ${id} in ${((Date.now() - started) / 1000).toFixed(1)}s`);
        } catch (error) {
          failures.push(id);
          console.error(`failed ${id}: ${error.stderr ?? error.message}`);
        }
      }
    }),
  );

  if (failures.length) {
    console.error(`failed grammars: ${failures.join(', ')}`);
    process.exit(1);
  }
  if (external) {
    external.previousAssets ??= {};
    for (const [path, asset] of [...assets].sort(([a], [b]) => a.localeCompare(b))) {
      const prior = external.assets[path];
      if (prior && prior.sha256 !== asset.sha256) (external.previousAssets[path] ??= []).push(prior);
      external.assets[path] = asset;
    }
    writeProjectData(externalAssetPath, external);
  }
  const sorted = Object.fromEntries(Object.entries(grammars).sort(([a], [b]) => a.localeCompare(b)));
  const lock = {
    description:
      'Grammar crates pinned by rust/Cargo.lock and their WebAssembly builds used by the JavaScript runtime.',
    treeSitterCli: cliVersion,
    treeSitterRuntime: {
      rust: versions.get('tree-sitter'),
      javascript: JSON.parse(await readFile(join(root, 'js/node_modules/web-tree-sitter/package.json'), 'utf8')).version,
    },
    grammars: sorted,
  };
  await writeLock(lock);
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  await main();
}
