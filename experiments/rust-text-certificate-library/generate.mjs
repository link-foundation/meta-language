// Additive packaging experiment. The original automatically translated source,
// definitions, generic emitter defaults and native oracle stay unchanged.
// These syntax receipts concern that supported pure-decision artifact, not
// arbitrary Rust Drop/temporary-scope semantics. Actual native comparison is
// still a CI boundary; no token inventory substitutes for that observation.
import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync, mkdirSync } from 'node:fs';
import { resolve, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { readProjectData, checkProjectData, writeProjectData } from '../../js/scripts/project-data.mjs';
import { packageRustLibrary, readRustLibraryNesting } from '../../js/scripts/rust-library-packaging.mjs';
import { DecoratorSet } from '../../js/src/decorators.js';

const originalFile = new URL('../rust-text-certificate/data/translation.lino', import.meta.url);
const policyFile = new URL('./data/policy.lino', import.meta.url);
const libraryFile = new URL('./data/library.lino', import.meta.url);
const decoratorsFile = new URL('./data/decorators.lino', import.meta.url);
const sha256 = value => createHash('sha256').update(value).digest('hex');

export function createTextCertificateLibrary() {
  const bytes = readFileSync(originalFile), original = readProjectData(originalFile), policy = readProjectData(policyFile);
  if (sha256(bytes) !== policy.sourceSha256) throw new Error('the frozen complete translation packet changed');
  if (original.definitions.length !== policy.declarationCount || original.declarations.length !== policy.declarationCount) {
    throw new Error('the complete declaration inventory changed');
  }
  const packaged = packageRustLibrary(original.completeRust, policy);
  const definitions = original.definitions.map((source, index) => {
    const owners = packaged.inventory.items.filter(item => item.source.trim() === source.trim());
    if (owners.length !== 1) throw new Error(`generated definition ${index} has no unique complete item owner`);
    return { ordinal: index, originalName: original.declarations[index], itemId: owners[0].id, sha256: sha256(source) };
  });
  if (new Set(definitions.map(row => row.itemId)).size !== definitions.length) throw new Error('generated definition ownership repeats');
  const decorated = DecoratorSet.fromLino(packaged.decorators.toLino()).decorateAll('translation-rule', packaged.records);
  if (JSON.stringify(decorated) !== JSON.stringify(packaged.decorated)) throw new Error('Links decorators did not reproduce the packaged sources');
  for (const module of packaged.modules) {
    if (module.source.split('\n').length >= policy.maximumLines) throw new Error('a generated raw module exceeds the recorded line budget');
  }
  const packet = {
    model: policy.model,
    source: { file: 'experiments/rust-text-certificate/data/translation.lino', sha256: sha256(bytes),
      completeSourceSha256: sha256(original.completeRust), completeSourceBytes: Buffer.byteLength(original.completeRust) },
    modulePath: policy.modulePath, maximumRawLines: policy.maximumLines,
    definitions,
    originalHeaders: packaged.inventory.items.filter(item => item.kind === 'inner attribute')
      .map(item => ({ id: item.id, source: item.source, sha256: sha256(item.source) })),
    originalTrailingSource: packaged.inventory.trailing,
    items: packaged.observations.map(row => {
      const originalItem = packaged.inventory.items.find(item => item.id === row.id);
      const target = decorated.find(item => item.id === row.id);
      return { ...row, originalSha256: sha256(originalItem.source), sourceSha256: sha256(target.source),
        originalStructure: readRustLibraryNesting(originalItem.source), structure: readRustLibraryNesting(target.source) };
    }),
    modules: packaged.modules.map(module => ({ ...module, sha256: sha256(module.source), rawLines: module.source.split('\n').length })),
    originalStructure: readRustLibraryNesting(original.completeRust),
    structure: readRustLibraryNesting(packaged.modules.slice(1).map(module => module.source).join('\n')),
    validationBoundary: policy.validationBoundary,
  };
  return { packet, decorators: packaged.decorators.toLino() };
}

export function materializeTextCertificateLibrary(packet, directory) {
  // Filenames come from the checked packet, not caller-supplied path fragments.
  for (const module of packet.modules) {
    if (!/^(?:mod|part_\d+)\.rs$/u.test(module.file) || sha256(module.source) !== module.sha256) {
      throw new Error('a materialized library module has an invalid name or source digest');
    }
  }
  mkdirSync(directory, { recursive: true });
  for (const module of packet.modules) writeFileSync(join(directory, module.file), module.source);
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  const { packet, decorators } = createTextCertificateLibrary(), check = process.argv.includes('--check');
  if (check) {
    checkProjectData(libraryFile, packet);
    if (readFileSync(decoratorsFile, 'utf8') !== decorators) throw new Error('the recorded Rust library decorators are stale');
  } else {
    writeProjectData(libraryFile, packet); writeFileSync(decoratorsFile, decorators);
  }
  const at = process.argv.indexOf('--materialize');
  if (at >= 0) {
    if (!process.argv[at + 1] || process.argv[at + 1].startsWith('--')) throw new Error('--materialize needs an explicit output directory');
    materializeTextCertificateLibrary(packet, resolve(process.argv[at + 1]));
  }
  console.log(`Complete Rust library packet: ${packet.definitions.length} definitions, ${packet.items.length} executable items, ${packet.modules.length - 1} whole-item parts; native validation remains a CI boundary.`);
}
