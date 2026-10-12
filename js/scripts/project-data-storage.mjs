// Scan source storage itself, including untracked files. Dependency installations
// and disposable build/work directories are outside the source-data boundary.
import { createHash } from 'node:crypto';
import { createReadStream } from 'node:fs';
import { readFile, readdir } from 'node:fs/promises';
import { basename, join, posix } from 'node:path';
import { tokenize } from '../src/translation/lexer.js';

const directoryNames = new Set(['.git', 'node_modules', '__pycache__', '.venv', 'venv', '.lake']);
const rootDirectories = new Set(['.external-grammars', '.grammar-cache', '.issue-195-work',
  'issue-195-results', 'issue-195-artifacts', '_opam', '_site', 'target', 'rust/target', 'rust/web/pkg']);
const boundaryKinds = new Set(['npm package manifest', 'npm dependency lock', 'external tool configuration',
  'JSON source program', 'foreign grammar importer input', 'external protocol snapshot']);
const runtimeProtocols = new Map([
  ['active cache lease', 'older peer interprocess JSON'],
  ['cache cleanup owner lock', 'older peer interprocess JSON'],
  ['scratch owner marker', 'older peer interprocess JSON'],
  ['explicit JSON cleanup report', 'explicit JSON command interface'],
]);
const formatPattern = /\.(?:json|jsonl|ndjson)$/iu;

function isRelativeFile(value) {
  return typeof value === 'string' && value.length > 0 && !/[\\\0*?\[\]{}]/u.test(value)
    && !value.startsWith('/') && value === posix.normalize(value)
    && !value.split('/').some(part => part === '..' || part === '.' || !part);
}

function omittedDirectory(path, name) {
  return directoryNames.has(name) || rootDirectories.has(path);
}

export async function collectProjectDataStorageFiles(root) {
  const files = [];
  const symbolicLinks = [];
  const pending = [''];
  while (pending.length) {
    const folder = pending.pop();
    for (const entry of await readdir(join(root, folder), { withFileTypes: true })) {
      const path = folder ? `${folder}/${entry.name}` : entry.name;
      if (omittedDirectory(path, entry.name)) continue;
      if (entry.isDirectory()) pending.push(path);
      else if (entry.isSymbolicLink()) symbolicLinks.push(path);
      else if (entry.isFile() && formatPattern.test(path)) files.push(path);
    }
  }
  return { files: files.sort(), symbolicLinks: symbolicLinks.sort() };
}

async function digestFile(path) {
  const hash = createHash('sha256');
  let bytes = 0;
  for await (const chunk of createReadStream(path)) { hash.update(chunk); bytes += chunk.length; }
  return { bytes, sha256: hash.digest('hex') };
}

async function validateRuntimeProtocols(root, entries) {
  const problems = [];
  if (entries === undefined) return problems;
  if (!Array.isArray(entries)) return ['runtime JSON protocols need an ordered records array'];
  const seen = new Set();
  const sourceContexts = new Map();
  for (const entry of entries) {
    const id = entry?.id;
    if (!runtimeProtocols.has(id) || runtimeProtocols.get(id) !== entry.kind) {
      problems.push(`unrecognized runtime JSON protocol: ${String(id)}`);
      continue;
    }
    if (seen.has(id)) problems.push(`duplicate runtime JSON protocol: ${id}`);
    seen.add(id);
    if (entry.format !== 'UTF-8 JSON with a final newline'
      || typeof entry.location !== 'string' || !entry.location.trim()
      || typeof entry.compatibilityReason !== 'string' || !entry.compatibilityReason.trim()
      || typeof entry.replacementCondition !== 'string' || !entry.replacementCondition.trim()) {
      problems.push(`runtime JSON protocol needs its exact format, location, compatibility reason and replacement condition: ${id}`);
    }
    if (entry.kind === 'explicit JSON command interface' && (entry.selector?.flag !== '--json'
      || entry.selector?.requiresExplicitRequest !== true)) problems.push(`JSON command output requires the explicit --json selector: ${id}`);
    if (!Array.isArray(entry.readers) || entry.readers.length === 0) problems.push(`runtime JSON protocol needs actual reader evidence: ${id}`);
    for (const component of [entry.writer, ...(Array.isArray(entry.readers) ? entry.readers : [])]) {
      if (!isRelativeFile(component?.path) || typeof component?.function !== 'string' || !/^[A-Za-z_][A-Za-z0-9_]*$/u.test(component.function)
        || typeof component?.contains !== 'string' || !component.contains.trim()) {
        problems.push(`runtime JSON protocol needs exact writer/reader source evidence: ${id}`);
        continue;
      }
      try {
        if (!sourceContexts.has(component.path)) sourceContexts.set(component.path,
          readFunctionContexts(await readFile(join(root, component.path), 'utf8')));
        const text = sourceContexts.get(component.path).get(component.function);
        if (!text || !text.includes(component.contains)) {
          problems.push(`runtime JSON protocol source evidence changed: ${id} (${component.path}:${component.function})`);
        }
      } catch { problems.push(`runtime JSON protocol source is missing: ${id} (${component.path})`); }
    }
  }
  for (const id of runtimeProtocols.keys()) if (!seen.has(id)) problems.push(`runtime JSON protocol record is missing: ${id}`);
  return problems;
}

// Function ownership comes from the existing source tokenizer, so braces in
// strings, regular expressions, comments and templates cannot alter the range.
function readFunctionContexts(source) {
  const { tokens } = tokenize(source, 'JavaScript');
  const contexts = new Map();
  const punct = (token, value) => token?.kind === 'punct' && token.value === value;
  for (let index = 0; index < tokens.length; index += 1) {
    if (tokens[index].kind !== 'identifier' || tokens[index].value !== 'function'
      || tokens[index + 1]?.kind !== 'identifier' || !punct(tokens[index + 2], '(')) continue;
    const name = tokens[index + 1].value;
    let cursor = index + 2;
    let depth = 0;
    do {
      if (punct(tokens[cursor], '(')) depth += 1;
      else if (punct(tokens[cursor], ')')) depth -= 1;
      cursor += 1;
    } while (cursor < tokens.length && depth);
    if (!punct(tokens[cursor], '{')) continue;
    do {
      if (punct(tokens[cursor], '{')) depth += 1;
      else if (punct(tokens[cursor], '}')) depth -= 1;
      cursor += 1;
    } while (cursor < tokens.length && depth);
    if (depth) continue;
    if (contexts.has(name)) throw new Error(`protocol source has duplicate function declarations: ${name}`);
    contexts.set(name, source.slice(tokens[index].start, tokens[cursor - 1].end));
  }
  return contexts;
}

export async function validateProjectDataStorage(root, registry) {
  const problems = [];
  const { files, symbolicLinks } = await collectProjectDataStorageFiles(root);
  if (registry?.schemaVersion !== 1 || !Array.isArray(registry.exceptions)) {
    return { files, problems: ['JSON storage exceptions need schemaVersion 1 and an ordered exceptions array'] };
  }
  problems.push(...await validateRuntimeProtocols(root, registry.runtimeProtocols));
  for (const path of symbolicLinks) problems.push(`source storage audit cannot follow a symbolic link: ${path}`);
  const present = new Set(files);
  const declared = new Set();
  for (const entry of registry.exceptions) {
    const path = entry?.path;
    if (!isRelativeFile(path) || !formatPattern.test(path)) {
      problems.push('every JSON storage exception needs one exact relative JSON/JSONL/NDJSON file path');
      continue;
    }
    if (declared.has(path)) { problems.push(`duplicate JSON storage exception: ${path}`); continue; }
    declared.add(path);
    if (!present.has(path)) problems.push(`JSON storage exception has no source file: ${path}`);
    if (!boundaryKinds.has(entry.boundary)) problems.push(`JSON storage exception has an unknown boundary: ${path}`);
    if (typeof entry.reason !== 'string' || !entry.reason.trim()) problems.push(`JSON storage exception needs its consumer's format reason: ${path}`);
    if (entry.boundary === 'npm package manifest' && basename(path) !== 'package.json') problems.push(`npm manifest exception must name package.json: ${path}`);
    if (entry.boundary === 'npm dependency lock' && basename(path) !== 'package-lock.json') problems.push(`npm lock exception must name package-lock.json: ${path}`);
    const consumer = entry.consumer;
    if (!isRelativeFile(consumer?.path) || typeof consumer?.contains !== 'string' || !consumer.contains.trim()) {
      problems.push(`JSON storage exception needs an actual consumer and literal source evidence: ${path}`);
    } else {
      try {
        const text = await readFile(join(root, consumer.path), 'utf8');
        if (!text.includes(consumer.contains)) problems.push(`JSON storage consumer evidence changed: ${path} (${consumer.path})`);
      } catch { problems.push(`JSON storage consumer is missing: ${path} (${consumer.path})`); }
    }
    if (entry.boundary === 'external protocol snapshot' || entry.boundary === 'foreign grammar importer input') {
      const provenance = entry.provenance;
      if (!Number.isSafeInteger(provenance?.bytes) || provenance.bytes < 0 || !/^[a-f0-9]{64}$/u.test(provenance?.sha256 ?? '')
        || typeof provenance?.source !== 'string' || !provenance.source.trim()) {
        problems.push(`raw JSON protocol provenance needs exact bytes, sha256 and source identity: ${path}`);
      } else if (present.has(path)) {
        const actual = await digestFile(join(root, path));
        if (actual.bytes !== provenance.bytes || actual.sha256 !== provenance.sha256) problems.push(`raw JSON protocol provenance changed: ${path}`);
      }
    }
    try {
      await readFile(join(root, path.replace(formatPattern, '.lino')));
      problems.push(`JSON exception also has LiNo storage: ${path}`);
    } catch (error) { if (error.code !== 'ENOENT') throw error; }
  }
  for (const path of files) if (!declared.has(path)) problems.push(`unregistered JSON source storage; use structured LiNo or record its required boundary: ${path}`);
  return { files, problems: problems.sort() };
}
