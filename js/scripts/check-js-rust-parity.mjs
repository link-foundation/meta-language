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

console.log(`parity: checked ${manifest.features.length} feature rows`);
