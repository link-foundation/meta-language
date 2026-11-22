import { cp, readFile, writeFile } from 'node:fs/promises';
import { existsSync } from 'node:fs';
import { execFile } from 'node:child_process';
import { join } from 'node:path';
import { promisify } from 'node:util';
import { gzipSync } from 'node:zlib';

const run = promisify(execFile);

// The CLI needs the exported language name in crates that omit tree-sitter.json.
export async function ensureGrammarConfig(grammarDir) {
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

// Keep a byte-identical local copy of each shared scanner header.
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

export async function compileGrammarWebAssembly(work, directory, id, treeSitter) {
  const grammarDir = join(work, directory);
  await localizeSharedIncludes(grammarDir);
  await ensureGrammarConfig(grammarDir);
  if (!existsSync(join(grammarDir, 'src/grammar.json'))) {
    // The CLI only reads the name from this file.
    await writeFile(join(grammarDir, 'src/grammar.json'), JSON.stringify({ name: id, rules: {} }));
  }
  const output = 'grammar.wasm';
  const [command, ...prefix] = treeSitter;
  await run(command, [...prefix, 'build', '--wasm', '-o', output, directory], {
    cwd: work,
    maxBuffer: 64 * 1024 * 1024,
  });
  return readFile(join(work, output));
}

export function compressGrammarWebAssembly(wasm) {
  const archive = gzipSync(wasm, { level: 9 });
  archive[9] = 3; // Use the same gzip operating-system byte on every build host.
  return archive;
}
