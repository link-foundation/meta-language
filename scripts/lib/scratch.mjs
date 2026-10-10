// Temporary scratch directories that scripts/clean-caches.mjs can reclaim.
//
// A script that clones, builds or generates into the OS temporary directory
// creates its directory here. The marker names this worktree and the creating
// process, so a later cleanup removes the directory only for this worktree and
// only after that process has exited, for example when it crashed or was
// interrupted before its own `finally` ran.
import { mkdtempSync, realpathSync, writeFileSync } from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { SCRATCH_MARKER } from './cache-classes.mjs';
import { processIdentity } from './process-identity.mjs';

const repositoryRoot = realpathSync(path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..'));

/** Creates `<tmp>/<prefix>XXXXXX` with a scratch marker and returns its path. */
export function makeScratchDirectory(prefix, label = prefix.replace(/-+$/u, '')) {
  const directory = mkdtempSync(path.join(os.tmpdir(), prefix));
  writeFileSync(
    path.join(directory, SCRATCH_MARKER),
    `${JSON.stringify({ root: repositoryRoot, ...processIdentity(), label, createdAt: new Date().toISOString() })}\n`,
  );
  return directory;
}
