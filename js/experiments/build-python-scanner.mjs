// Port the pinned tree-sitter-python 0.25.0 scanner through reusable,
// executable Links data. The source and scanner hashes live in sources.json.
import { scannerFamilies } from '../scripts/scanner-families.mjs';
export const pythonScannerDescriptors = [
  { family: 'indentation', name: 'layout', newlineToken: 'newline', indentToken: 'indent', dedentToken: 'dedent', interpolationVariable: 'strings_inside', stringStartToken: 'string_start', stringContentToken: 'string_fragment', commentLiterals: ['except'] },
  { family: 'prefixed-quoted', name: 'strings', startToken: 'string_start', contentToken: 'string_fragment', endToken: 'string_end', interpolationEscapeToken: 'escape_interpolation', interpolationVariable: 'strings_inside', recoveryTokens: ['indent'] },
];
export const pythonScanner = scannerFamilies(pythonScannerDescriptors);
if (import.meta.url === new URL(process.argv[1], 'file:').href) process.stdout.write(pythonScanner);
