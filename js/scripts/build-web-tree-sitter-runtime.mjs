#!/usr/bin/env node
// Rebuilds js/src/vendor/web-tree-sitter/tree-sitter.wasm.gz, the web-tree-sitter
// runtime patched to read UTF-8 input (utf8-input.patch). The published runtime
// always parses UTF-16, and tree-sitter charges error recovery per skipped byte,
// so on malformed input it chose different recoveries than the native runtime
// (Lean `import` gave `(MISSING identifier)` instead of `(ERROR)`). With UTF-8
// input both runtimes compute the same costs, offsets and columns.
//
// The build first compiles the unpatched sources and requires the result to
// equal the wasm shipped in the pinned npm package, so the only difference in
// the vendored runtime is the patch. Requires git and Docker (emscripten image).
//
//   node js/scripts/build-web-tree-sitter-runtime.mjs          # rebuild
//   node js/scripts/build-web-tree-sitter-runtime.mjs --check  # verify lock only
import { createHash } from 'node:crypto';
import { execFile } from 'node:child_process';
import { mkdir, readFile, rm, writeFile } from 'node:fs/promises';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { promisify } from 'node:util';
import { gunzipSync, gzipSync } from 'node:zlib';

import { CONTAINER_LABEL } from '../../scripts/lib/cache-classes.mjs';
import { makeScratchDirectory } from '../../scripts/lib/scratch.mjs';

const run = promisify(execFile);
const root = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const vendorDir = join(root, 'js/src/vendor/web-tree-sitter');
const lockPath = join(vendorDir, 'runtime-lock.json');
const packagePath = join(root, 'js/node_modules/web-tree-sitter');

const SOURCE = Object.freeze({
  upstream: 'tree-sitter/tree-sitter',
  version: '0.25.10',
  revision: 'da6fe9beb4f7f67beb75914ca8e0d48ae48d6406',
  emscripten: '4.0.4',
  patch: 'js/src/vendor/web-tree-sitter/utf8-input.patch',
});

// xtask/src/build_wasm.rs at the pinned revision.
const EXPORTED_RUNTIME_METHODS = [
  'AsciiToString', 'stringToUTF8', 'UTF8ToString', 'lengthBytesUTF8',
  'stringToUTF16', 'loadWebAssemblyModule', 'getValue', 'setValue',
];

const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

async function compile(checkout, output) {
  const exported = [
    await readFile(join(checkout, 'lib/src/wasm/stdlib-symbols.txt'), 'utf8'),
    await readFile(join(checkout, 'lib/binding_web/lib/exports.txt'), 'utf8'),
  ].join('').replaceAll('"', '').split('\n').filter(Boolean).map((line) => `_${line}`).join('')
    .replace(/,$/u, '');
  await mkdir(output, { recursive: true });
  await run('docker', [
    // The label lets scripts/clean-caches.mjs find the container if it outlives an interrupted run.
    'run', '--rm', '--label', CONTAINER_LABEL, '--volume', `${checkout}:/src`, '--volume', `${output}:/out`,
    '--user', String(process.getuid()), '--workdir', '/src', `emscripten/emsdk:${SOURCE.emscripten}`,
    'emcc', '-O3', '--minify', '0', '-gsource-map', '--source-map-base', '.', '-fno-exceptions', '-std=c11',
    '-s', 'WASM=1', '-s', 'MODULARIZE=1', '-s', 'INITIAL_MEMORY=33554432', '-s', 'ALLOW_MEMORY_GROWTH=1',
    '-s', 'SUPPORT_BIG_ENDIAN=1', '-s', 'MAIN_MODULE=2', '-s', 'FILESYSTEM=0', '-s', 'NODEJS_CATCH_EXIT=0',
    '-s', 'NODEJS_CATCH_REJECTION=0', '-s', `EXPORTED_FUNCTIONS=${exported}`,
    '-s', `EXPORTED_RUNTIME_METHODS=${EXPORTED_RUNTIME_METHODS.join(',')}`,
    '-D', 'fprintf(...)=', '-D', 'NDEBUG=', '-D', '_POSIX_C_SOURCE=200112L', '-D', '_DEFAULT_SOURCE=',
    '-D', '_DARWIN_C_SOURCE=', '-I', 'lib/src', '-I', 'lib/include',
    '--js-library', 'lib/binding_web/lib/imports.js', '--pre-js', 'lib/binding_web/lib/prefix.js',
    '-s', 'EXPORT_ES6=1', '-o', '/out/tree-sitter.mjs', 'lib/src/lib.c', 'lib/binding_web/lib/tree-sitter.c',
  ], { maxBuffer: 64 * 1024 * 1024 });
  return readFile(join(output, 'tree-sitter.wasm'));
}

async function packageRuntime() {
  const metadata = JSON.parse(await readFile(join(packagePath, 'package.json'), 'utf8'));
  return { version: metadata.version, wasm: await readFile(join(packagePath, 'tree-sitter.wasm')) };
}

/** Verifies the vendored runtime against its lock and the installed package. */
export async function checkRuntimeLock() {
  const problems = [];
  const lock = JSON.parse(await readFile(lockPath, 'utf8'));
  const installed = await packageRuntime();
  for (const key of ['upstream', 'version', 'revision', 'emscripten', 'patch']) {
    if (lock[key] !== SOURCE[key]) problems.push(`lock ${key} is ${lock[key]}, the build pins ${SOURCE[key]}`);
  }
  if (installed.version !== lock.version) {
    problems.push(`web-tree-sitter ${installed.version} is installed, the runtime is built from ${lock.version}`);
  }
  if (sha256(installed.wasm) !== lock.upstreamWasmSha256) {
    problems.push('the installed web-tree-sitter wasm differs from the locked unpatched build');
  }
  if (sha256(await readFile(join(root, SOURCE.patch))) !== lock.patchSha256) problems.push('patch digest mismatch');
  const wasm = gunzipSync(await readFile(join(vendorDir, 'tree-sitter.wasm.gz')));
  if (sha256(wasm) !== lock.wasmSha256) problems.push('vendored runtime wasm digest mismatch');
  return problems;
}

async function main() {
  if (process.argv.includes('--check')) {
    const problems = await checkRuntimeLock();
    if (problems.length) {
      console.error(problems.join('\n'));
      process.exit(1);
    }
    console.log(`web-tree-sitter runtime lock matches ${SOURCE.version} with ${SOURCE.patch}`);
    return;
  }
  const work = makeScratchDirectory('web-tree-sitter-');
  try {
    const checkout = join(work, 'tree-sitter');
    await run('git', ['clone', '--quiet', '--depth', '1', '--branch', `v${SOURCE.version}`,
      `https://github.com/${SOURCE.upstream}`, checkout]);
    const { stdout: head } = await run('git', ['-C', checkout, 'rev-parse', 'HEAD']);
    if (head.trim() !== SOURCE.revision) throw new Error(`v${SOURCE.version} is ${head.trim()}, not ${SOURCE.revision}`);

    const installed = await packageRuntime();
    const upstream = await compile(checkout, join(work, 'upstream'));
    if (installed.version !== SOURCE.version || sha256(upstream) !== sha256(installed.wasm)) {
      throw new Error(`the unpatched build does not reproduce web-tree-sitter ${installed.version}`);
    }
    await run('git', ['-C', checkout, 'apply', join(root, SOURCE.patch)]);
    const patched = await compile(checkout, join(work, 'patched'));
    await writeFile(join(vendorDir, 'tree-sitter.wasm.gz'), gzipSync(patched, { level: 9 }));
    await writeFile(join(vendorDir, 'LICENSE'), await readFile(join(checkout, 'LICENSE')));
    const lock = {
      description: 'web-tree-sitter runtime patched to parse UTF-8 input, as the native runtime does.',
      ...SOURCE,
      patchSha256: sha256(await readFile(join(root, SOURCE.patch))),
      upstreamWasmSha256: sha256(upstream),
      wasmSha256: sha256(patched),
      license: 'LICENSE',
    };
    await writeFile(lockPath, `${JSON.stringify(lock, null, 2)}\n`);
    console.log(`built the UTF-8 web-tree-sitter ${SOURCE.version} runtime (${lock.wasmSha256})`);
  } finally {
    await rm(work, { recursive: true, force: true });
  }
}

if (process.argv[1] === fileURLToPath(import.meta.url)) await main();
