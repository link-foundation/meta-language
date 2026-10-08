// Generate scanner policies shared by both runtimes from the pinned sources.
import { scannerFamilies } from '../scripts/scanner-families.mjs';
import { fileURLToPath } from 'node:url';
import { resolve } from 'node:path';
const quotedStops = ["'", '"', '$'];
export const DART_SCANNERS = [
  ...[
    ['template_single_quote_characters', false], ['template_double_quote_characters', false],
    ['template_single_quote_line_characters', true], ['template_double_quote_line_characters', true],
  ].map(([token, singleLine]) => ({ family: 'fragment', name: token, token, stops: [...quotedStops, '\\'], rejected: singleLine ? ['\0', '\n'] : ['\0'] })),
  { family: 'fragment', name: 'raw_template_characters', token: 'raw_template_characters', stops: quotedStops, requiredMarker: '\\' },
  { family: 'delimited', name: 'block_comment', token: 'block_comment', opening: '/*', closing: '*/', nested: true, openingLookahead: '*', rejectOpeningLookahead: true, rejected: ['\0'] },
  { family: 'delimited', name: 'documentation_block_comment', token: 'documentation_block_comment', opening: '/*', closing: '*/', nested: true, openingLookahead: '*', rejected: ['\0'] },
  { family: 'pattern-token', name: 'annotation_opening_parenthesis', token: 'annotation_opening_parenthesis', pattern: '\\(' },
];
const pathCharacters = '0-9A-Za-z+_./-';
export const NIX_SCANNERS = [
  { family: 'fragment', name: 'string_fragment', token: 'string_fragment', stops: ['"', '\\', '${'], pairedPrefixes: [{ prefix: '$', except: ['"', '\\', '{'] }] },
  { family: 'fragment', name: 'indented_string_fragment', token: 'indented_string_fragment', stops: ["''", '${'], pairedPrefixes: [{ prefix: '$', except: ["'", '{'] }] },
  { family: 'pattern-token', name: 'path_initial_plain_fragment', token: 'path_initial_plain_fragment', pattern: `[${pathCharacters}]*/[${pathCharacters}]*[0-9A-Za-z+_.-][${pathCharacters}]*` },
  { family: 'pattern-token', name: 'path_initial_interpolation_fragment', token: 'path_initial_interpolation_fragment', pattern: `[${pathCharacters}]*/[${pathCharacters}]*`, before: '\\$' },
  { family: 'pattern-token', name: 'path_fragment', token: 'path_fragment', pattern: `[${pathCharacters}]+` },
  { family: 'pattern-token', name: 'dollar_escape', token: 'dollar_escape', pattern: '\\\\', before: '\\$' },
  { family: 'pattern-token', name: 'indented_dollar_escape', token: 'indented_dollar_escape', pattern: "''(\\\\)?", before: '\\$' },
];
export const CMAKE_SCANNERS = [
  ...['argument', 'comment'].map((kind) => ({ family: 'split-counted-delimiter', name: `bracket_${kind}`, startToken: `bracket_${kind}_opening`, contentToken: `bracket_${kind}_content`, endToken: `bracket_${kind}_closing`, prefix: kind === 'comment' ? '#[' : '[', marker: '=', opening: '[', closing: ']', suffix: ']', skipWhitespace: false, skipContentWhitespace: true, allowEnd: true })),
  { family: 'pattern-token', name: 'line_comment', token: 'line_comment', pattern: '#[^\\r\\n\\x00]*', excludedAtStart: '#\\[=*\\[' },
];
export const FRAGMENT_SCANNERS = { dart: DART_SCANNERS, nix: NIX_SCANNERS, cmake: CMAKE_SCANNERS };
export function fragmentScannerListing(id) {
  if (!Object.hasOwn(FRAGMENT_SCANNERS, id)) throw new TypeError(`unknown fragment scanner ${id}`);
  let listing = scannerFamilies(FRAGMENT_SCANNERS[id]);
  if (id === 'nix') listing += '(rule path_initial_fragment token (choice unordered (ref path_initial_plain_fragment) (ref path_initial_interpolation_fragment)))\n';
  return listing;
}
if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) process.stdout.write(fragmentScannerListing(process.argv[2]));
