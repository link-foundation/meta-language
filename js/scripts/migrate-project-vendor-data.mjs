#!/usr/bin/env node
// A bounded automatic conversion of our three downloaded-asset manifests.
// Retained Git blobs supply the independent JSON baseline for offline checking.
import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { existsSync, readFileSync, unlinkSync, writeFileSync } from 'node:fs';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { convertStructuredData } from './convert-structured-data.mjs';
import { readProjectData, readProjectFileBytes, writeProjectData } from './project-data.mjs';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const receiptFile = resolve(root, 'parity/project-data-vendor-conversions.lino');
const files = ['js/src/vendor/grammars/grammar-lock.json', 'js/src/vendor/web-tree-sitter/runtime-lock.json',
  'rust/vendor/tree-sitter-agda-layout/source-manifest.json'];
const digest = bytes => createHash('sha256').update(bytes).digest('hex');
const baseline = (commit, file) => execFileSync('git', ['show', `${commit}:${file}`], { cwd: root, maxBuffer: 8 * 1024 * 1024 });

if (process.argv.slice(2).some(argument => argument !== '--check')) throw new Error('usage: migrate-project-vendor-data.mjs [--check]');
if (process.argv.includes('--check')) {
  const receipt = readProjectData(receiptFile);
  assert.equal(receipt.schemaVersion, 1);
  assert.deepEqual(receipt.conversions.map(entry => entry.logicalPath), files);
  for (const entry of receipt.conversions) {
    const original = baseline(entry.sourceCommit, entry.logicalPath);
    assert.equal(original.length, entry.originalBytes);
    assert.equal(digest(original), entry.originalSha256);
    const expected = Buffer.from(convertStructuredData(original.toString('utf8')));
    assert.equal(expected.length, entry.convertedBytes);
    assert.equal(digest(expected), entry.convertedSha256);
    const actual = readProjectFileBytes(resolve(root, entry.physicalPath));
    assert.ok(actual.equals(expected), `converted asset manifest differs: ${entry.logicalPath}`);
  }
} else {
  if (existsSync(receiptFile)) throw new Error('vendor conversion already recorded; use --check');
  const sourceCommit = execFileSync('git', ['rev-parse', 'HEAD'], { cwd: root, encoding: 'utf8' }).trim();
  const conversions = [];
  for (const logicalPath of files) {
    const original = readFileSync(resolve(root, logicalPath));
    assert.ok(original.equals(baseline(sourceCommit, logicalPath)), `vendor manifest has uncommitted source edits: ${logicalPath}`);
    const physicalPath = logicalPath.replace(/\.json$/u, '.lino');
    if (existsSync(resolve(root, physicalPath))) throw new Error(`duplicate vendor manifest: ${physicalPath}`);
    const converted = Buffer.from(convertStructuredData(original.toString('utf8')));
    writeFileSync(resolve(root, physicalPath), converted, { flag: 'wx' });
    assert.ok(readFileSync(resolve(root, physicalPath)).equals(converted));
    unlinkSync(resolve(root, logicalPath));
    conversions.push({ logicalPath, physicalPath, sourceCommit, originalBytes: original.length,
      originalSha256: digest(original), convertedBytes: converted.length, convertedSha256: digest(converted) });
  }
  writeProjectData(receiptFile, { schemaVersion: 1, conversions });
}
assert.deepEqual(readProjectData(resolve(root, files[0])), readProjectData(resolve(root, 'rust/src/data/grammar-lock.json')),
  'JavaScript and Rust retain the same decoded grammar acquisition manifest');
console.log('Three asset manifests match their independent ordered conversion baselines; grammar lock matches both packages.');
