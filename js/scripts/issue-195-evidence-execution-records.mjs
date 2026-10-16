import { readFile } from 'node:fs/promises';
import path from 'node:path';
import { readStructuredDataRecords } from '../src/structured-data-records.js';
import { stageFiles } from './issue-195-evidence-stages.mjs';
import { isJavaScriptSuitePartitionName } from './issue-195-javascript-suite-partitions.mjs';
import { isRustSuitePartitionName } from './issue-195-rust-suite-partitions.mjs';

/** Read all reported stage scopes while retaining the original duplicate gate. */
export async function readEvidenceStageExecutionRecords({ records, stageNames, knownTestIds, resultsDirectory, stageErrors }) {
  const known = new Set(knownTestIds);
  const seen = new Set();
  const accepted = [];
  const recordedStages = [...records.keys()].filter(name => stageNames.includes(name) || isJavaScriptSuitePartitionName(name) || isRustSuitePartitionName(name));
  for (const name of recordedStages) {
    let contents;
    try {
      contents = await readFile(path.join(resultsDirectory, stageFiles(name).observations), 'utf8');
    } catch (error) {
      if (error.code === 'ENOENT') continue;
      throw error;
    }
    const problems = [];
    const namespaces = new Set();
    contents.split(/\r?\n/u).forEach((line, index) => {
      if (!line) return;
      let record;
      try {
        const namespace = /^\(([^/\s]+)\/record:/u.exec(line)?.[1];
        if (namespaces.has(namespace)) throw new Error('duplicate record namespace');
        const decoded = readStructuredDataRecords(line);
        if (decoded.length !== 1) throw new Error('one complete record per line is required');
        namespaces.add(namespace);
        [record] = decoded;
      } catch (error) {
        problems.push(`invalid execution record at line ${index + 1}: ${error.message}`);
        return;
      }
      if (!record || typeof record !== 'object') {
        problems.push(`execution record at line ${index + 1} is not an object`);
        return;
      }
      if (!known.has(record.testId)) {
        problems.push(`execution record has unknown testId ${record.testId}`);
        return;
      }
      const key = `${record.testId}\u0000${record.assertionId}\u0000${record.fixtureId}`;
      if (seen.has(key)) {
        problems.push(`duplicate execution record ${key.replaceAll('\u0000', ' ')}`);
        return;
      }
      seen.add(key);
      accepted.push(record);
    });
    if (problems.length > 0) stageErrors.push({ stage: name, error: problems.join('; ') });
  }
  return accepted;
}
