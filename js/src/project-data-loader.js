// Runtime packages read the same structured Links data as authoring tools.
import { existsSync, readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { readStructuredDataLino } from './structured-data-links.js';

export function readRuntimeProjectData(file) {
  let path = file instanceof URL ? fileURLToPath(file) : resolve(file);
  if (path.endsWith('.json') || path.endsWith('.lino')) {
    const stem = path.slice(0, -5);
    const json = `${stem}.json`;
    const linked = `${stem}.lino`;
    if (existsSync(json) && existsSync(linked)) throw new Error(`project data exists in both JSON and Links Notation: ${path}`);
    if (path.endsWith('.json') && existsSync(linked)) path = linked;
  }
  const source = readFileSync(path, 'utf8');
  return path.endsWith('.lino') ? readStructuredDataLino(source) : JSON.parse(source);
}
