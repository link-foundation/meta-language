import { spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';

const digest = value => createHash('sha256').update(value).digest('hex');

function runGit(checkout, arguments_, input) {
  const result = spawnSync('git', ['-C', checkout, ...arguments_], {
    input,
    maxBuffer: 2 * 1024 * 1024,
    env: { ...process.env, GIT_NO_LAZY_FETCH: '1' },
  });
  if (result.status !== 0) throw new Error(result.error?.message ?? result.stderr.toString());
  return result.stdout;
}

export function git(checkout, ...arguments_) {
  return runGit(checkout, arguments_);
}

export function readCorpusInput(checkout, row) {
  const input = git(checkout, 'cat-file', 'blob', row.oid);
  if (input.length !== row.size) throw new Error('corpus length mismatch ' + row.path);
  return { ...row, sha256: digest(input), input };
}

export function readSupplementalCorpus(root, source, inputLimit) {
  const checkout = root + '/' + source.directory;
  // Enumerate names without opening the blobs. In a filtered checkout,
  // ls-tree -l fetches every file under a path before the corpus filter runs.
  const tree = git(checkout, 'ls-tree', '-rz', source.revision, '--', ...source.paths).toString('utf8');
  const rows = tree.split('\0').filter(Boolean).flatMap(record => {
    const [, oid, path] = /^100(?:644|755) blob ([0-9a-f]{40})\t([^\t]+)$/u.exec(record) ?? [];
    if (!path || !source.pathPattern.test(path)) return [];
    return [{ corpus: source.name, path: source.directory + '/' + path, oid, label: 'unlabelled' }];
  });
  if (rows.length === 0) throw new Error('empty pinned corpus ' + source.name);
  const identities = [...new Set(rows.map(row => row.oid))];
  const promisor = spawnSync('git', ['-C', checkout, 'config', '--bool', '--get', 'remote.origin.promisor'], {
    encoding: 'utf8',
  });
  if (promisor.status !== 0 && promisor.status !== 1) {
    throw new Error(promisor.error?.message ?? promisor.stderr);
  }
  const input = identities.join('\n') + '\n';
  if (promisor.stdout.trim() === 'true') {
    // Request only the selected paths' objects in one transfer. Their sizes
    // determine the sample order and the exclusion ledger.
    runGit(checkout, ['fetch', '--no-tags', '--no-write-fetch-head', '--stdin', 'origin'], input);
  }
  const sizes = new Map();
  const objectSizes = runGit(checkout, ['cat-file', '--batch-check=%(objectname) %(objecttype) %(objectsize)'],
    input).toString('utf8').trimEnd().split('\n');
  for (const line of objectSizes) {
    const [oid, type, size] = line.split(' ');
    if (type !== 'blob' || !/^\d+$/u.test(size)) throw new Error('unrecognized corpus object ' + line);
    sizes.set(oid, Number(size));
  }
  for (const row of rows) {
    row.size = sizes.get(row.oid);
    if (row.size === undefined) throw new Error('missing pinned corpus input ' + row.path);
  }
  rows.sort((a, b) => b.size - a.size || (a.path < b.path ? -1 : 1));
  let selected = 0;
  return rows.map(row => {
    if (row.size > inputLimit) return { ...row, limit: 'input bytes' };
    if (selected === source.sampleLimit) return { ...row, limit: 'sample count' };
    selected += 1;
    return readCorpusInput(checkout, row);
  });
}
