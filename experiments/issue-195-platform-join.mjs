// Replays the supported-platform join over downloaded CI platform reports.
// Usage: node experiments/issue-195-platform-join.mjs REPORT.json...
import { createHash } from 'node:crypto';
import { readFile } from 'node:fs/promises';
import { supportedPlatformCoverage } from '../js/scripts/issue-195-delivery.mjs';

const corpusBytes = await readFile(new URL('../parity/fixtures/issue-195-evidence.json', import.meta.url));
const context = {
  corpus: JSON.parse(corpusBytes),
  corpusSha256: createHash('sha256').update(corpusBytes).digest('hex'),
  version: JSON.parse(await readFile(new URL('../js/package.json', import.meta.url))).version,
};
const reports = await Promise.all(process.argv.slice(2).map(async (file) => JSON.parse(await readFile(file))));
const expected = { npm: reports[0].npm.checksum.expectedSha256, crate: reports[0].crate.checksum.expectedSha256 };
for (const sides of [['npm'], ['crate'], ['npm', 'crate']]) {
  console.log(sides.join('+'), JSON.stringify(supportedPlatformCoverage(reports, sides, expected, context)));
}
