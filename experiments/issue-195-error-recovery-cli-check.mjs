// Parses every error-recovery case with the pinned tree-sitter CLI, once as
// UTF-8 and once with --encoding utf16-le, and prints both S-expressions next to
// the fixture's `expected` and `utf16Tree`.
// Usage: TREE_SITTER_CLI=/path/to/tree-sitter node experiments/issue-195-error-recovery-cli-check.mjs
import { execFile } from 'node:child_process';
import { mkdtempSync, readFileSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { promisify } from 'node:util';
import { cargoLockVersions } from '../js/scripts/build-vendored-grammars.mjs';
import { cliGrammarDirectory } from '../js/scripts/generate-default-cst-expectations.mjs';

const run = promisify(execFile);
const treeSitter = process.env.TREE_SITTER_CLI ?? 'tree-sitter';
const fixture = JSON.parse(readFileSync(new URL('../parity/fixtures/issue-195-conformance/error-recovery.json', import.meta.url), 'utf8'));
const scratch = mkdtempSync(join(tmpdir(), 'error-recovery-'));
process.env.TREE_SITTER_LIBDIR = join(scratch, 'lib');
const versions = await cargoLockVersions();
const directories = new Map();
const strip = (tree) => tree.replace(/ \[\d+, \d+\] - \[\d+, \d+\]/gu, '').replace(/\s+/gu, ' ').replace(/\( /gu, '(').replace(/ \)/gu, ')').trim();
async function parse(dir, file, encoding) {
  const args = ['parse', file, ...(encoding ? ['--encoding', encoding] : [])];
  const { stdout } = await run(treeSitter, args, { cwd: dir, env: { ...process.env, NO_COLOR: '1' } }).catch((error) => error);
  return strip(stdout.split('\n').filter((line) => !line.startsWith(file)).join('\n'));
}
// The plain S-expression leaves MISSING nodes out of the tree (the CLI reports
// them on the summary line), so `utf16Tree` is compared without them.
// A missing named node prints as a plain node and a missing token not at all.
const unmark = (tree) => tree.replace(/ \(MISSING "[^"]*"\)/gu, '').replace(/\(MISSING ([^()\s]+)\)/gu, '($1)');
let mismatches = 0;
for (const entry of fixture.cases) {
  const id = entry.language.toLowerCase();
  if (!directories.has(id)) directories.set(id, await cliGrammarDirectory(id, versions, scratch));
  const file = join(scratch, `${entry.id}.src`);
  writeFileSync(file, entry.source);
  const utf8 = await parse(directories.get(id), file);
  const bytes = Buffer.from(entry.source, 'utf16le');
  const file16 = join(scratch, `${entry.id}.utf16`);
  writeFileSync(file16, bytes);
  const utf16 = await parse(directories.get(id), file16, 'utf16-le');
  const ok = utf8 === entry.expected && utf16 === unmark(entry.utf16Tree);
  if (!ok) mismatches += 1;
  console.log(`${ok ? 'ok  ' : 'DIFF'} ${entry.id}\n  utf8  ${utf8}\n  want  ${entry.expected}\n  utf16 ${utf16}\n  want  ${entry.utf16Tree}`);
}
console.log(`${fixture.cases.length - mismatches}/${fixture.cases.length} cases agree`);
