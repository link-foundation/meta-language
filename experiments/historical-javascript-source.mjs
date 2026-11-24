import { execFileSync } from 'node:child_process';
import { mkdtempSync, rmSync, symlinkSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = fileURLToPath(new URL('../', import.meta.url));

function prepareHistoricalJavaScriptSource(revision, frozenPaths, producerPaths) {
  const directory = mkdtempSync(join(tmpdir(), 'meta-language-javascript-source-'));
  try {
    execFileSync('git', ['diff', '--exit-code', revision, '--', ...frozenPaths], { cwd: root, stdio: 'inherit' });
    const archive = join(directory, 'source.tar');
    const paths = [...new Set(['js/src', 'js/package.json',
      ...frozenPaths.filter(path => !path.startsWith('js/src/')), ...producerPaths])];
    execFileSync('git', ['archive', '-o', archive, revision, ...paths], { cwd: root, stdio: 'inherit' });
    execFileSync('tar', ['-xf', archive, '-C', directory], { stdio: 'inherit' });
    rmSync(archive);
    symlinkSync(join(root, 'js/node_modules'), join(directory, 'js/node_modules'), 'dir');
    return directory;
  } catch (error) {
    rmSync(directory, { recursive: true, force: true });
    throw error;
  }
}

export function withHistoricalJavaScriptSource(revision, frozenPaths, run, producerPaths = []) {
  const directory = prepareHistoricalJavaScriptSource(revision, frozenPaths, producerPaths);
  try {
    return run(directory);
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
}

export async function withHistoricalJavaScriptModules(revision, frozenPaths, run, producerPaths = []) {
  const directory = prepareHistoricalJavaScriptSource(revision, frozenPaths, producerPaths);
  try {
    return await run(directory);
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
}
