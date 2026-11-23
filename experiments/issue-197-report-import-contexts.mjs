// Compare the import contexts implied by two report schedules without
// translating the source catalog. Pass the two prefixes used for the output
// of --list --shard K/8, with each file named <prefix><K>.txt.
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { posix } from 'node:path';
import { readModuleHeaderBindings } from '../js/scripts/generate-self-translation-report.mjs';

const [before, after] = process.argv.slice(2);
assert.ok(before && after, 'provide the two report listing prefixes');

function readSchedule(prefix) {
  return Array.from({ length: 8 }, (_, index) =>
    readFileSync(`${prefix}${index + 1}.txt`, 'utf8').trim().split('\n').filter(Boolean).sort());
}

const original = readSchedule(before);
const proposed = readSchedule(after);
const modules = original.flat().sort();
assert.equal(new Set(modules).size, modules.length);
assert.deepEqual(proposed.flat().sort(), modules);
const headers = new Map(modules.map(module => [module, readModuleHeaderBindings(readFileSync(module, 'utf8'))]));

function importContexts(schedule) {
  const reports = new Map();
  for (const shard of schedule) {
    const visiting = new Set();
    const contexts = new Map();
    function context(module) {
      if (contexts.has(module)) return contexts.get(module);
      const imports = new Map();
      visiting.add(module);
      for (const { requested, specifier } of headers.get(module).imports) {
        if (!specifier || !/^\.\.?\//u.test(specifier)) continue;
        const provider = posix.normalize(posix.join(posix.dirname(module), specifier));
        const header = headers.get(provider);
        if (!header || visiting.has(provider)) continue;
        if (requested.some(binding => header.exports.includes(binding))) {
          context(provider);
          imports.set(specifier, provider);
        } else {
          imports.set(specifier, null);
        }
      }
      visiting.delete(module);
      contexts.set(module, imports);
      return imports;
    }
    for (const module of shard) {
      const dependencies = new Map();
      function visit(provider) {
        if (dependencies.has(provider)) return;
        const imports = context(provider);
        dependencies.set(provider, [...imports]);
        for (const target of imports.values()) {
          if (target !== null) visit(target);
        }
      }
      visit(module);
      reports.set(module, [...dependencies]);
    }
  }
  return reports;
}

const former = importContexts(original);
const current = importContexts(proposed);
for (const module of modules) assert.deepEqual(current.get(module), former.get(module), module);
console.log(`Compared import contexts for all ${modules.length} modules.`);
