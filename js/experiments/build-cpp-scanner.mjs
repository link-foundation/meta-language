// Generate the pinned C++ raw string scanner through the common text-state family.
import { readFileSync, writeFileSync } from 'node:fs';
import { scannerFamilies } from '../scripts/scanner-families.mjs';
export const descriptors = [{ family: 'remembered-content', name: 'raw_string', delimiterToken: 'raw_string_boundary_marker', contentToken: 'raw_string_content', delimiterPattern: '[^\\s\\\\(]{1,15}', closingPrefix: ')', closingSuffix: '"', allowEmpty: true, allowEnd: true }];
export const listing = scannerFamilies(descriptors);
const output = new URL('../../parity/grammars/scanners/cpp.lino', import.meta.url);
if (process.argv.includes('--check')) { if (readFileSync(output, 'utf8') !== listing) throw Error('C++ scanner generation drift'); }
else { writeFileSync(output, listing); console.log('generated C++ scanner grammar data'); }
