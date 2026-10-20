#!/usr/bin/env node
// Transport receipts bind retained compiler feedback to the exact initial
// translation file partition. They are not test execution observations.
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { copyFileSync, existsSync, lstatSync, mkdirSync, readFileSync, readdirSync, realpathSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { readProjectData, writeProjectData } from './project-data.mjs';
import { partitionGroupFiles, partitionTests, testFiles } from './test-groups.mjs';

const ROOT = fileURLToPath(new URL('../../', import.meta.url));
const PARTS = 3;
const RECEIPT = 'translation-test-feedback-partition.lino';
const KIND = 'initial JavaScript translation feedback partition';
const SOURCES = ['issue-195-artifacts/compiler-feedback', 'issue-195-results/artifacts/decimal-format-draft-native.lino'];

function commitIdentity(commit) {
  if (!/^[0-9a-f]{40}$/u.test(commit ?? '')) throw new Error('feedback commit must be an exact Git commit');
  return commit;
}

function partitionFiles(root, shard, partitions = PARTS) {
  const match = /^([1-9][0-9]*)\/([1-9][0-9]*)$/u.exec(shard ?? '');
  if (!Number.isSafeInteger(partitions) || partitions < 1 || !match || Number(match[2]) !== partitions) {
    throw new Error(partitions === PARTS ? 'feedback shard must be one of 1/3, 2/3, 3/3'
      : 'feedback shard must use the explicitly selected positive partition count');
  }
  const groups = partitionTests(testFiles(path.join(root, 'js'))).groups;
  return partitionGroupFiles(groups.translation, shard).map(file => `js/tests/${file}`);
}

function filesWithin(directory, prefix = '') {
  const result = [];
  for (const name of readdirSync(directory).sort()) {
    const relative = prefix ? `${prefix}/${name}` : name;
    const file = path.join(directory, name);
    const stat = lstatSync(file);
    if (stat.isSymbolicLink()) throw new Error(`feedback symbolic link is not a payload: ${relative}`);
    if (stat.isDirectory()) result.push(...filesWithin(file, relative));
    else if (stat.isFile()) result.push(relative);
    else throw new Error(`feedback non-file payload: ${relative}`);
  }
  return result;
}

function payloadPath(relative) {
  if (typeof relative !== 'string' || relative.includes('\\') || relative.includes('\0') ||
      relative.split('/').some(part => !part || part === '.' || part === '..') ||
      !(relative.startsWith(`${SOURCES[0]}/`) || relative === SOURCES[1])) {
    throw new Error(`unexpected feedback payload path: ${relative}`);
  }
  return relative;
}

function targetDirectory(root, output) {
  if (typeof output !== 'string' || !output) throw new Error('feedback output directory is required');
  const target = path.resolve(root, output);
  if (!target.startsWith(`${realpathSync(root)}${path.sep}`) ||
      target.startsWith(`${path.join(root, '.git')}${path.sep}`) || target === path.join(root, '.git') ||
      SOURCES.some(source => target === path.join(root, source) || target.startsWith(`${path.join(root, source)}${path.sep}`))) {
    throw new Error('feedback output must be a directory inside its checkout');
  }
  let current = root;
  for (const part of path.relative(root, target).split(path.sep)) {
    current = path.join(current, part);
    try {
      if (lstatSync(current).isSymbolicLink()) throw new Error('feedback output cannot traverse a symbolic link');
    } catch (error) {
      if (error.code !== 'ENOENT') throw error;
    }
  }
  if (existsSync(target) && readdirSync(target).length) throw new Error('feedback output directory is not empty');
  return target;
}

function fingerprint(file) {
  const bytes = readFileSync(file);
  return { bytes: bytes.length, sha256: createHash('sha256').update(bytes).digest('hex') };
}

export function captureTranslationTestFeedbackPartition({ root = ROOT, outputDirectory, commit, shard, partitions = PARTS }) {
  root = realpathSync(root);
  commitIdentity(commit);
  const files = partitionFiles(root, shard, partitions);
  const output = targetDirectory(root, outputDirectory);
  const payloads = [];
  for (const source of SOURCES) {
    const original = path.join(root, source);
    if (!existsSync(original)) continue;
    const stat = lstatSync(original);
    if (stat.isSymbolicLink()) throw new Error(`feedback source cannot be a symbolic link: ${source}`);
    if (!stat.isDirectory() && !stat.isFile()) throw new Error(`feedback source is not a regular file or directory: ${source}`);
    const entries = stat.isDirectory() ? filesWithin(original).map(file => `${source}/${file}`) : [source];
    for (const relative of entries) {
      payloadPath(relative);
      const destination = path.join(output, relative);
      mkdirSync(path.dirname(destination), { recursive: true });
      copyFileSync(path.join(root, relative), destination);
      payloads.push({ path: relative, ...fingerprint(destination) });
    }
  }
  payloads.sort((left, right) => left.path < right.path ? -1 : left.path > right.path ? 1 : 0);
  mkdirSync(output, { recursive: true });
  const receipt = { schemaVersion: 1, kind: KIND, commit, group: 'translation', shard, files, payloads };
  writeProjectData(path.join(output, RECEIPT), receipt, { format: 'lino' });
  return receipt;
}

export function mergeTranslationTestFeedbackPartitions({ root = ROOT, inputDirectory, outputDirectory, commit, partitions = PARTS }) {
  root = realpathSync(root);
  commitIdentity(commit);
  if (typeof inputDirectory !== 'string' || !inputDirectory) throw new Error('feedback input directory is required');
  const input = path.resolve(root, inputDirectory);
  partitionFiles(root, `1/${partitions}`, partitions);
  const expected = Array.from({ length: partitions }, (_, index) => `translation-test-feedback-${commit}-${index + 1}-of-${partitions}`);
  assert.deepEqual(readdirSync(input).sort(), [...expected].sort(), 'feedback partitions must be present exactly once with no foreign parts');
  const payloads = new Map();
  for (const [index, name] of expected.entries()) {
    const directory = path.join(input, name);
    const stat = lstatSync(directory);
    if (stat.isSymbolicLink() || !stat.isDirectory()) throw new Error(`feedback partition is not an owned directory: ${name}`);
    const actualFiles = filesWithin(directory);
    const receipt = readProjectData(path.join(directory, RECEIPT), { format: 'lino' });
    assert.deepEqual(Object.keys(receipt).sort(), ['schemaVersion', 'kind', 'commit', 'group', 'shard', 'files', 'payloads'].sort(), 'unexpected feedback receipt fields');
    const shard = `${index + 1}/${partitions}`;
    assert.deepEqual([receipt.schemaVersion, receipt.kind, receipt.commit, receipt.group, receipt.shard],
      [1, KIND, commit, 'translation', shard], 'feedback partition provenance does not match this checkout');
    assert.deepEqual(receipt.files, partitionFiles(root, shard, partitions), 'feedback partition file scope does not match this checkout');
    if (!Array.isArray(receipt.payloads)) throw new Error('feedback payload inventory must be a list');
    const seen = new Set();
    for (const payload of receipt.payloads) {
      assert.deepEqual(Object.keys(payload).sort(), ['path', 'bytes', 'sha256'].sort(), 'unexpected feedback payload fields');
      const relative = payloadPath(payload.path);
      if (seen.has(relative)) throw new Error(`duplicate feedback payload in ${shard}: ${relative}`);
      seen.add(relative);
      const source = path.join(directory, relative);
      const actual = fingerprint(source);
      assert.deepEqual(actual, { bytes: payload.bytes, sha256: payload.sha256 }, `feedback payload differs from its receipt: ${relative}`);
      const prior = payloads.get(relative);
      if (prior && !readFileSync(prior).equals(readFileSync(source))) throw new Error(`conflicting feedback payload across shards: ${relative}`);
      payloads.set(relative, source);
    }
    assert.deepEqual(actualFiles.sort(), [RECEIPT, ...seen].sort(), 'feedback partition contains unregistered payloads');
  }
  if (!payloads.size) throw new Error('the complete translation feedback collection has no files');
  const output = targetDirectory(root, outputDirectory);
  for (const [relative, source] of [...payloads].sort(([left], [right]) => left < right ? -1 : left > right ? 1 : 0)) {
    const destination = path.join(output, relative);
    mkdirSync(path.dirname(destination), { recursive: true });
    copyFileSync(source, destination);
  }
  return { commit, partitions, files: [...payloads.keys()].sort() };
}

function main(argv) {
  const [mode, ...args] = argv;
  const options = {};
  for (let index = 0; index < args.length; index += 2) {
    const key = args[index];
    if (!['--root', '--out', '--commit', '--shard', '--inputs', '--partitions'].includes(key) || Object.hasOwn(options, key) || !args[index + 1]) {
      throw new Error('feedback usage: capture --commit SHA --shard K/N --out DIR [--partitions N]; merge --commit SHA --inputs DIR --out DIR [--partitions N]');
    }
    options[key] = args[index + 1];
  }
  if (options['--partitions'] !== undefined && !/^[1-9][0-9]*$/u.test(options['--partitions'])) throw new Error('feedback partition count must be a positive integer');
  const shared = { root: options['--root'] ?? ROOT, outputDirectory: options['--out'], commit: options['--commit'],
    partitions: options['--partitions'] === undefined ? PARTS : Number(options['--partitions']) };
  if (mode === 'capture' && !options['--inputs']) {
    const receipt = captureTranslationTestFeedbackPartition({ ...shared, shard: options['--shard'] });
    console.log(`translation feedback: captured ${receipt.payloads.length} files for ${receipt.shard} at ${receipt.commit}`);
  } else if (mode === 'merge' && !options['--shard']) {
    const merged = mergeTranslationTestFeedbackPartitions({ ...shared, inputDirectory: options['--inputs'] });
    console.log(`translation feedback: merged ${merged.files.length} files from ${merged.partitions} exact-commit partitions`);
  } else throw new Error('feedback mode must be capture or merge with its exact scope arguments');
}

if (process.argv[1] && pathToFileURL(path.resolve(process.argv[1])).href === import.meta.url) {
  try { main(process.argv.slice(2)); }
  catch (error) { console.error(error.message); process.exitCode = 1; }
}
