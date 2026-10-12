import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { mkdirSync, mkdtempSync, rmSync, symlinkSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import test from 'node:test';
import { collectProjectDataStorageFiles, validateProjectDataStorage } from '../scripts/project-data-storage.mjs';
import { readProjectData, writeProjectData } from '../scripts/project-data.mjs';

async function scratch(body) {
  const root = mkdtempSync(join(tmpdir(), 'meta-storage-audit-'));
  try { await body(root); } finally { rmSync(root, { recursive: true, force: true }); }
}

function put(root, file, value) {
  mkdirSync(join(root, file, '..'), { recursive: true });
  writeFileSync(join(root, file), value);
}

function registry(path, boundary = 'JSON source program', provenance) {
  return { schemaVersion: 1, exceptions: [{ path, boundary, reason: 'the consumer imports this JSON source',
    consumer: { path: 'source.js', contains: "with { type: 'json' }" }, ...(provenance ? { provenance } : {}) }] };
}

test('storage audit inspects the working tree and rejects new unregistered JSON and persisted streams', () => scratch(async root => {
  put(root, 'source.js', "import data from './source.json' with { type: 'json' };\n");
  put(root, 'source.json', '{"value":1}\n');
  const exceptions = registry('source.json');
  assert.deepEqual((await validateProjectDataStorage(root, exceptions)).problems, []);
  put(root, 'untracked/new.json', '{}\n');
  put(root, 'vendor/metadata.jsonl', '{}\n');
  put(root, 'output.ndjson', '{}\n');
  put(root, 'uppercase.JSON', '{}\n');
  const result = await validateProjectDataStorage(root, exceptions);
  assert.deepEqual(result.files, ['output.ndjson', 'source.json', 'untracked/new.json', 'uppercase.JSON', 'vendor/metadata.jsonl']);
  assert.equal(result.problems.filter(problem => problem.startsWith('unregistered')).length, 4);
  assert.ok(result.problems.some(problem => problem.endsWith('vendor/metadata.jsonl')), 'vendor is not a blanket exception');
}));

test('registered protocols require unchanged consumer evidence and exact raw byte provenance', () => scratch(async root => {
  const bytes = '{"sha":"pinned","value":"α"}\r\n';
  put(root, 'source.js', "import data from './source.json' with { type: 'json' };\n");
  put(root, 'source.json', bytes);
  const exceptions = registry('source.json', 'external protocol snapshot', {
    bytes: Buffer.byteLength(bytes), sha256: createHash('sha256').update(bytes).digest('hex'), source: 'captured external API response',
  });
  assert.deepEqual((await validateProjectDataStorage(root, exceptions)).problems, []);
  put(root, 'source.json', bytes.replace('pinned', 'edited'));
  assert.ok((await validateProjectDataStorage(root, exceptions)).problems.includes('raw JSON protocol provenance changed: source.json'));
  put(root, 'source.json', bytes);
  put(root, 'source.js', 'const data = 1;\n');
  assert.ok((await validateProjectDataStorage(root, exceptions)).problems.some(problem => problem.startsWith('JSON storage consumer evidence changed')));
  delete exceptions.exceptions[0].provenance;
  assert.ok((await validateProjectDataStorage(root, exceptions)).problems.some(problem => problem.startsWith('raw JSON protocol provenance needs')));
}));

test('exception declarations cannot use globs, traversal, duplicates, missing files or unknown boundaries', () => scratch(async root => {
  put(root, 'source.js', "with { type: 'json' }\n");
  put(root, 'source.json', '{}\n');
  const exceptions = registry('source.json');
  exceptions.exceptions.push(exceptions.exceptions[0], { ...exceptions.exceptions[0], path: '../escape.json' },
    { ...exceptions.exceptions[0], path: 'absent.json' }, { ...exceptions.exceptions[0], path: '*.json' },
    { ...exceptions.exceptions[0], path: 'unknown.json', boundary: 'history' });
  const problems = (await validateProjectDataStorage(root, exceptions)).problems;
  assert.ok(problems.includes('duplicate JSON storage exception: source.json'));
  assert.ok(problems.some(problem => problem.startsWith('every JSON storage exception needs')));
  assert.ok(problems.includes('JSON storage exception has no source file: absent.json'));
  assert.ok(problems.includes('JSON storage exception has an unknown boundary: unknown.json'));
}));

test('only disposable operational directories are skipped and symbolic links cannot conceal source storage', () => scratch(async root => {
  for (const folder of ['.git', 'node_modules', 'rust/target', '.external-grammars', '.issue-195-work']) put(root, `${folder}/external.json`, '{}\n');
  put(root, 'vendor/source.json', '{}\n');
  put(root, 'examples/target/source.json', '{}\n');
  symlinkSync(join(root, 'vendor'), join(root, 'alias'));
  assert.deepEqual((await collectProjectDataStorageFiles(root)).files, ['examples/target/source.json', 'vendor/source.json']);
  const problems = (await validateProjectDataStorage(root, { schemaVersion: 1, exceptions: [] })).problems;
  assert.ok(problems.includes('source storage audit cannot follow a symbolic link: alias'));
  assert.ok(problems.some(problem => problem.endsWith('vendor/source.json')));
}));

test('the exception registry is itself structured LiNo and duplicate formats are rejected', () => scratch(async root => {
  put(root, 'source.js', "with { type: 'json' }\n");
  put(root, 'source.json', '{}\n');
  const exceptions = registry('source.json');
  writeProjectData(join(root, 'exceptions.lino'), exceptions);
  assert.deepEqual(readProjectData(join(root, 'exceptions.lino')), exceptions);
  assert.deepEqual((await validateProjectDataStorage(root, exceptions)).problems, []);
  writeProjectData(join(root, 'source.lino'), {});
  assert.ok((await validateProjectDataStorage(root, exceptions)).problems.includes('JSON exception also has LiNo storage: source.json'));
}));

test('runtime coordination records cannot exempt source files and require actual writer/reader evidence', () => scratch(async root => {
  put(root, 'protocol.js', 'function writeIdentity() { return JSON.stringify({ pid: 1 }); }\nfunction readIdentity() { return JSON.parse(text); }\n');
  const interprocess = ['active cache lease', 'cache cleanup owner lock', 'scratch owner marker'];
  const records = [...interprocess, 'explicit JSON cleanup report'].map(id => ({
    id, kind: interprocess.includes(id) ? 'older peer interprocess JSON' : 'explicit JSON command interface',
    format: 'UTF-8 JSON with a final newline', location: 'runtime-owned state outside the source exception list',
    compatibilityReason: 'older active readers parse the JSON identity', replacementCondition: 'all participating readers upgrade before the coordinated switch',
    writer: { path: 'protocol.js', function: 'writeIdentity', contains: 'JSON.stringify' },
    readers: [{ path: 'protocol.js', function: 'readIdentity', contains: 'JSON.parse' }],
    ...(interprocess.includes(id) ? {} : { selector: { flag: '--json', requiresExplicitRequest: true } }),
  }));
  const exceptions = { schemaVersion: 1, exceptions: [], runtimeProtocols: records };
  assert.deepEqual((await validateProjectDataStorage(root, exceptions)).problems, []);
  put(root, 'protocol.js', 'function writeIdentity() { return structuredDataToLino({ pid: 1 }); }\nfunction readIdentity() { return JSON.parse(text); }\nfunction other() { return JSON.stringify({ pid: 2 }); }\n');
  assert.ok((await validateProjectDataStorage(root, exceptions)).problems.some(problem => problem.startsWith('runtime JSON protocol source evidence changed')),
    'another function retaining JSON output cannot impersonate the declared protocol writer');
  put(root, 'protocol.js', 'function writeIdentity() { return JSON.stringify({ pid: 1, brace: "}", template: `}` }); }\nfunction readIdentity() { return JSON.parse(text); }\n');
  assert.deepEqual((await validateProjectDataStorage(root, exceptions)).problems, [], 'opaque source tokens preserve the writer context');
  put(root, 'new-own-report.json', '{}\n');
  assert.ok((await validateProjectDataStorage(root, exceptions)).problems.some(problem => problem.endsWith('new-own-report.json')));
  records[0].writer.contains = 'an invented consumer';
  assert.ok((await validateProjectDataStorage(root, exceptions)).problems.some(problem => problem.startsWith('runtime JSON protocol source evidence changed')));
  records[3].selector.requiresExplicitRequest = false;
  assert.ok((await validateProjectDataStorage(root, exceptions)).problems.some(problem => problem.startsWith('JSON command output requires')));
  records.push({ ...records[0], id: 'all cache reports' });
  assert.ok((await validateProjectDataStorage(root, exceptions)).problems.includes('unrecognized runtime JSON protocol: all cache reports'));
}));
