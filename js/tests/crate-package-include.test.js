// The crate's `include` list in rust/Cargo.toml must name every file the Rust
// sources embed with include_str! or include_bytes!, otherwise `cargo package`
// fails when it verifies the archive (the delivery-crate-pack evidence step).
import assert from 'node:assert/strict';
import { readFileSync, readdirSync } from 'node:fs';
import path from 'node:path';
import test from 'node:test';
import { fileURLToPath } from 'node:url';

const rust = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..', 'rust');

function sources(directory) {
  return readdirSync(directory, { withFileTypes: true }).flatMap((entry) => {
    const full = path.join(directory, entry.name);
    if (entry.isDirectory()) return sources(full);
    return entry.name.endsWith('.rs') ? [full] : [];
  });
}

// Cargo's include globs: `**/` spans any directories, a trailing `**` anything, `*` one name.
function patternMatcher(pattern) {
  const expression = pattern
    .split(/(\*\*\/?)/u)
    .map((part) => {
      if (part === '**/') return '(?:.*/)?';
      if (part === '**') return '.*';
      return part.replace(/[.+^${}()|[\]\\]/gu, '\\$&').replace(/\*/gu, '[^/]*');
    })
    .join('');
  return new RegExp(`^${expression}$`, 'u');
}

test('the crate package includes every file the Rust sources embed', () => {
  const manifest = readFileSync(path.join(rust, 'Cargo.toml'), 'utf8');
  const include = [...manifest.match(/^include = \[([^\]]*)\]/mu)[1].matchAll(/"([^"]+)"/gu)].map(([, pattern]) => patternMatcher(pattern));
  const embedded = sources(path.join(rust, 'src')).flatMap((file) =>
    [...readFileSync(file, 'utf8').matchAll(/include_(?:str|bytes)!\(\s*"([^"]+)"\s*\)/gu)].map(([, target]) =>
      path.relative(rust, path.resolve(path.dirname(file), target)).split(path.sep).join('/'),
    ),
  );
  assert.ok(embedded.includes('src/data/foundation-models.json'));
  assert.deepEqual(embedded.filter((file) => !include.some((pattern) => pattern.test(file))), []);
  assert.ok(include.some((pattern) => pattern.test('src/lib.rs')) && include.some((pattern) => pattern.test('vendor/tree-sitter-rust/src/parser.c')));
  assert.ok(!include.some((pattern) => pattern.test('docs/vision.md')), 'the include list stays tight');
});
