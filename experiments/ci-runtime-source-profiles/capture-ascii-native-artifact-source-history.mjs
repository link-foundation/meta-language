import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { asciiHostPin } from '../../js/scripts/ascii-text-predicate-host-history.mjs';
import { checkProjectData, readProjectData, writeProjectData } from '../../js/scripts/project-data.mjs';
import { readBootstrapStructuredDataLino } from '../../scripts/lib/structured-data-bootstrap.mjs';

const root = new URL('../../', import.meta.url);
const commit = 'c29d2b626cd05c209c814cc78cf654668ed8d29c';
const path = 'parity/self-translation/ascii-text-predicate-rules-native.lino';
const nativePath = 'ascii_text_predicate_rules/part_2.rs';
const git = (...args) => execFileSync('git', ['--no-replace-objects', ...args], { cwd: fileURLToPath(root) });
const previous = git('show', `${commit}:${path}`);
const current = readFileSync(new URL(path, root));
const previousArtifact = readBootstrapStructuredDataLino(previous.toString('utf8'));
const currentArtifact = readProjectData(new URL(path, root));
const previousPart = previousArtifact.files.find(row => row.path === nativePath);
const currentPart = currentArtifact.files.find(row => row.path === nativePath);
const sourceFile = 'rust/src/translation/' + nativePath;
assert.equal(previousPart.source, git('show', `${commit}:${sourceFile}`).toString('utf8'));
assert.equal(currentPart.source, readFileSync(new URL(sourceFile, root), 'utf8'));
assert.deepEqual(previousArtifact, { ...currentArtifact, files: currentArtifact.files.map(row => row === currentPart ? previousPart : row) });
const patch = git('diff', '--no-ext-diff', '--no-textconv', '--no-color', '--unified=0', commit, '--', sourceFile).toString('utf8');
const previousLines = previousPart.source.split('\n');
const currentLines = currentPart.source.split('\n');
const edits = [];
for (const range of patch.matchAll(/^@@ -(\d+)(?:,(\d+))? \+(\d+)(?:,(\d+))? @@/gmu)) {
  const [, oldLine, oldCount = '1', newLine, newCount = '1'] = range;
  const oldStart = Number(oldLine) - (oldCount === '0' ? 0 : 1);
  const newStart = Number(newLine) - (newCount === '0' ? 0 : 1);
  const block = (lines, start, count) => Number(count) === 0 ? '' : lines.slice(start, start + Number(count)).join('\n') + '\n';
  edits.push({ start: block(currentLines, 0, newStart).length,
    from: block(currentLines, newStart, newCount), to: block(previousLines, oldStart, oldCount) });
}
assert.ok(edits.length > 0);
let restored = currentPart.source;
for (const { start, from, to } of edits.toReversed()) {
  assert.equal(restored.slice(start, start + from.length), from);
  restored = restored.slice(0, start) + to + restored.slice(start + from.length);
}
assert.equal(restored, previousPart.source);
const packet = { schemaVersion: 1, commit, path, nativePath, previous: asciiHostPin(previous), current: asciiHostPin(current),
  previousPart: asciiHostPin(previousPart.source), currentPart: asciiHostPin(currentPart.source), edits };
const output = new URL('parity/self-translation/ascii-native-artifact-source-history.lino', root);
const args = process.argv.slice(2);
assert.ok(args.length <= 1 && args.every(argument => argument === '--check'));
if (args.includes('--check')) checkProjectData(output, packet);
else writeProjectData(output, packet);
console.log(`Preserved the exact ASCII native artifact through ${edits.length} reviewed source intervals.`);
