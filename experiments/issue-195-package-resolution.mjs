// Pack and install the candidate in a fresh consumer with no root overrides.
// Repository overrides alone do not determine installed package resolutions.
import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { randomUUID } from 'node:crypto';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const directory = path.join(root, '.issue-195-work', `package-resolution-${randomUUID()}`);
await mkdir(directory, { recursive: true });
const packed = execFileSync('npm', ['pack', '--json', '--pack-destination', directory], {
  cwd: path.join(root, 'js'), encoding: 'utf8',
});
await writeFile(path.join(directory, 'pack.json'), packed);
const metadata = JSON.parse(packed);
const artifacts = Array.isArray(metadata) ? metadata : Object.values(metadata);
assert.equal(artifacts.length, 1, 'exactly one candidate package must be packed');
const artifact = artifacts[0];
await writeFile(path.join(directory, 'package.json'), JSON.stringify({
  name: 'dependency-resolution-consumer', private: true, version: '0.0.0',
}));
const installation = execFileSync('npm', [
  'install', '--ignore-scripts', '--no-audit', '--no-fund', path.join(directory, artifact.filename),
], { cwd: directory, encoding: 'utf8' });
await writeFile(path.join(directory, 'install.log'), installation);
const lock = JSON.parse(await readFile(path.join(directory, 'package-lock.json'), 'utf8'));
const inventory = JSON.parse(await readFile(path.join(root, 'parity/dependency-inventory.json'), 'utf8'));
const observations = inventory.items.filter(({ category, kind }) => category === 'npm' && kind === 'runtime')
  .map((item) => {
    const installed = lock.packages[`node_modules/${item.source.package}`]?.version;
    return { name: item.source.package, installed, auditedCurrent: item.current };
  });
const report = { directory, artifact, observations };
await writeFile(path.join(directory, 'resolution.json'), `${JSON.stringify(report, null, 2)}\n`);
const output = process.argv[2];
if (output) await writeFile(path.resolve(output), `${JSON.stringify(report, null, 2)}\n`);
for (const { name, installed, auditedCurrent } of observations) {
  assert.equal(installed, auditedCurrent, name);
}
console.log(`Clean candidate consumer agrees with all ${observations.length} audited runtime npm resolutions: ${directory}`);
