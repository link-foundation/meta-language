import assert from 'node:assert/strict';

export function measureSequentialTestFiles(log, assigned, sources) {
  const owners = new Map();
  for (const file of assigned) {
    owners.set(file.slice(3), file);
    for (const match of sources.get(file).toString().matchAll(/\btest\(\s*(['"])([^\n]*?)\1\s*,/gu)) {
      const title = match[2];
      owners.set(title, owners.has(title) && owners.get(title) !== file ? null : file);
    }
  }

  const samples = new Map();
  let beginning, start, end, currentFile, title, lastResult;
  for (const line of log.split('\n')) {
    const time = Date.parse(line.slice(0, 28));
    if (line.endsWith('TAP version 13')) start = beginning = time;
    if (start === undefined) continue;
    const subtest = /# Subtest: (.*)$/u.exec(line);
    if (subtest) title = subtest[1];
    if (/Z (?:not )?ok \d+ - /u.test(line)) {
      const file = owners.get(title);
      if (file && currentFile !== file) {
        if (currentFile) {
          assert.ok(assigned.indexOf(currentFile) < assigned.indexOf(file));
          samples.set(currentFile, Math.max(1, lastResult - beginning));
          beginning = lastResult;
        }
        currentFile = file;
      }
      lastResult = time;
      title = undefined;
    }
    if (/Z # duration_ms [0-9.]+$/u.test(line)) end = time;
  }
  assert.ok(start < end && currentFile, 'the complete TAP stream has an identifiable last file');
  samples.set(currentFile, Math.max(1, end - beginning));
  return { samples, testMilliseconds: end - start };
}
