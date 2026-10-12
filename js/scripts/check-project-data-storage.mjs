#!/usr/bin/env node
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { checkProjectData, readProjectData } from './project-data.mjs';
import { validateProjectDataStorage } from './project-data-storage.mjs';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const registryFile = resolve(root, 'parity/project-data-json-exceptions.lino');
const registry = readProjectData(registryFile);
checkProjectData(registryFile, registry);
const { files, problems } = await validateProjectDataStorage(root, registry);
if (problems.length) {
  for (const problem of problems) console.error(problem);
  process.exitCode = 1;
} else console.log(`Project data source storage: checked ${files.length} exact retained JSON boundary files.`);
