// Generate the pinned TOML scanner as shared executable grammar data.
import { scannerFamilies } from '../scripts/scanner-families.mjs';
import { fileURLToPath } from 'node:url';
import { resolve } from 'node:path';
export const TOML_SCANNERS = [
  { family: 'line-boundary', name: 'line_boundary', token: 'line_ending_or_end_of_file' },
  ...[['basic', '"'], ['literal', "'"]].map(([kind, delimiter]) => ({
    family: 'delimiter-run', name: `multiple_line_${kind}_string`,
    contentToken: `multiple_line_${kind}_string_content`,
    endToken: `multiple_line_${kind}_string_end`, delimiter, count: 3,
  })),
];
if (process.argv[1] && fileURLToPath(import.meta.url) === resolve(process.argv[1])) process.stdout.write(scannerFamilies(TOML_SCANNERS));
