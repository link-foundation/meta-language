#!/usr/bin/env node
import { access, readFile } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const manifestPath = path.join(root, 'parity', 'language-features.json');
const manifestOnly = process.argv.includes('--manifest-only');

const manifest = JSON.parse(await readFile(manifestPath, 'utf8'));
const errors = [];

if (!Array.isArray(manifest.operationFamilies) || manifest.operationFamilies.length === 0) {
  errors.push('manifest.operationFamilies must list the required API operation families');
}

// Module-level parity gate: every `pub mod` exported from the Rust crate root
// must be classified in manifest.rustModules so the public Rust surface can
// never silently grow past the JavaScript port. This is the guard that closes
// the gap that let issue #166 slip (a Rust-only module with no manifest row).
const featureIds = new Set((manifest.features ?? []).map((feature) => feature.id));
const featureById = new Map((manifest.features ?? []).map((feature) => [feature.id, feature]));
const rustModules = manifest.rustModules ?? {};
const libRsPath = path.join(root, 'rust', 'src', 'lib.rs');

let publicModules = [];
try {
  const libRs = await readFile(libRsPath, 'utf8');
  publicModules = [...libRs.matchAll(/^\s*pub mod (\w+);/gm)].map((match) => match[1]);
} catch {
  errors.push(`could not read rust/src/lib.rs to enumerate the public module surface`);
}

if (publicModules.length === 0 && errors.length === 0) {
  errors.push('rust/src/lib.rs declared no `pub mod` entries; the module gate cannot run');
}

for (const moduleName of publicModules) {
  const entry = rustModules[moduleName];
  if (!entry) {
    errors.push(
      `Rust module \`${moduleName}\` is a \`pub mod\` in rust/src/lib.rs but is not classified in manifest.rustModules (add a "ported" row that names its feature, or a "rust-only" row with a reason)`,
    );
    continue;
  }
  if (entry.parity === 'ported') {
    if (!entry.feature) {
      errors.push(`rustModules.${moduleName} is "ported" but does not name a feature row`);
    } else if (!featureIds.has(entry.feature)) {
      errors.push(
        `rustModules.${moduleName} references unknown feature \`${entry.feature}\` (no such row in features[])`,
      );
    } else if (featureById.get(entry.feature).javascript?.status !== 'implemented') {
      errors.push(
        `rustModules.${moduleName} is "ported" but feature \`${entry.feature}\` JavaScript status is not implemented`,
      );
    }
  } else if (entry.parity === 'rust-only') {
    if (typeof entry.reason !== 'string' || entry.reason.trim().length === 0) {
      errors.push(`rustModules.${moduleName} is "rust-only" but does not justify it with a reason`);
    }
  } else {
    errors.push(`rustModules.${moduleName}.parity must be "ported" or "rust-only", got ${entry.parity}`);
  }
}

// No stale classifications: every rustModules key must still be a real pub mod.
const publicModuleSet = new Set(publicModules);
for (const moduleName of Object.keys(rustModules)) {
  if (!publicModuleSet.has(moduleName)) {
    errors.push(
      `manifest.rustModules lists \`${moduleName}\`, which is no longer a \`pub mod\` in rust/src/lib.rs (remove the stale entry)`,
    );
  }
}

for (const feature of manifest.features ?? []) {
  if (!feature.id) {
    errors.push('feature row is missing id');
    continue;
  }

  for (const language of ['rust', 'javascript']) {
    const cell = feature[language];
    if (!cell) {
      errors.push(`${feature.id} is missing ${language} cell`);
      continue;
    }
    if (feature.required && cell.status !== 'implemented') {
      errors.push(`${feature.id} ${language} status must be implemented`);
    }
    if (!Array.isArray(cell.evidence) || cell.evidence.length === 0) {
      errors.push(`${feature.id} ${language} cell must include evidence`);
      continue;
    }
    for (const evidence of cell.evidence) {
      try {
        await access(path.join(root, evidence));
      } catch {
        errors.push(`${feature.id} ${language} evidence does not exist: ${evidence}`);
      }
    }
  }
}

if (!manifestOnly) {
  const jsModule = await import(pathToFileURL(path.join(root, 'js', 'src', 'index.js')));
  const actualOperations = jsModule.API_OPERATIONS.map((entry) => entry.name()).sort();
  const expectedOperations = [...manifest.operationFamilies].sort();
  if (JSON.stringify(actualOperations) !== JSON.stringify(expectedOperations)) {
    errors.push(
      `JavaScript API operations ${actualOperations.join(', ')} do not match manifest ${expectedOperations.join(', ')}`,
    );
  }

  for (const entry of jsModule.API_OPERATIONS) {
    for (const style of jsModule.ApiStyle.ALL) {
      const cell = entry.style(style);
      if (!cell) {
        errors.push(`${entry.name()} missing ${style} JavaScript style cell`);
      } else if (!cell.fixture?.value) {
        errors.push(`${entry.name()} ${style} fixture must be named or explained`);
      }
    }
  }
}

if (errors.length > 0) {
  for (const error of errors) {
    console.error(`parity: ${error}`);
  }
  process.exit(1);
}

console.log(
  `parity: checked ${manifest.features.length} feature rows and ${publicModules.length} Rust public modules`,
);
