// Port the pinned HCL scanner through the shared template-context family.
// node js/experiments/build-hcl-scanner.mjs > parity/grammars/scanners/hcl.lino
import { scannerFamilies } from '../scripts/scanner-families.mjs';
import { fileURLToPath } from 'node:url';
import { resolve } from 'node:path';

export const HCL_SCANNERS = [{
  family: 'template-context', name: 'templates',
  quotedStartToken: 'quoted_template_start', quotedEndToken: 'quoted_template_end',
  contentToken: 'template_literal_chunk',
  interpolationStartToken: 'template_interpolation_start', interpolationEndToken: 'template_interpolation_end',
  directiveStartToken: 'template_directive_start', directiveEndToken: 'template_directive_end',
  delimiterToken: 'here_document_identifier', labelPattern: '[a-zA-Z0-9_-]+',
}];

if (process.argv[1] && fileURLToPath(import.meta.url) === resolve(process.argv[1])) {
  process.stdout.write(scannerFamilies(HCL_SCANNERS));
}
