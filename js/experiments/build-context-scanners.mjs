// Shared scanner descriptors for the pinned CSS, PowerShell and Erlang sources.
import { scannerFamilies } from '../scripts/scanner-families.mjs';
import { fileURLToPath } from 'node:url';
import { resolve } from 'node:path';
export const CSS_SCANNERS = [
  { family: 'context-token', name: 'descendant_selection', token: 'descendant_operator', requireWhitespace: true,
    immediateCharacters: ['#', '.', '[', '-', '*'], immediateRanges: [['a', 'z'], ['A', 'Z'], ['0', '9']],
    contextPrefix: ':', rejectContextWhitespace: true, target: '{', stops: [';', '}'], blockedTokens: ['error_recovery'] },
  { family: 'context-token', name: 'pseudo_class_selection', token: 'pseudo_class_selector_colon', opening: ':',
    rejectAfter: [':'], target: '{', stops: [';', '}'], allowEnd: true, comments: true, advanceBeforeCheck: true,
    blockedTokens: ['error_recovery'] },
];
export const POWERSHELL_SCANNERS = [
  { family: 'lookahead-boundary', name: 'statement_boundary', token: 'statement_terminator', before: ['}', ';', ')', '\n', '\0'] },
];
const erlangWhitespace = [...Array.from({ length: 32 }, (_, index) => String.fromCodePoint(index + 1)), ...Array.from({ length: 33 }, (_, index) => String.fromCodePoint(index + 128))];
// The Erlang scanner is Apache-2.0, copyright Meta Platforms, Inc. and
// affiliates; its pinned source and license accompany the grammar register.
export const ERLANG_SCANNERS = [
  { family: 'line-counted-delimiter', name: 'multiple_quote_string', token: 'multiple_quote_string', whitespace: erlangWhitespace, minimum: 3, countModulo: 65536 },
  { family: 'line-counted-delimiter', name: 'multiple_quote_sigil_string', token: 'multiple_quote_sigil_string', whitespace: erlangWhitespace, minimum: 3, countModulo: 65536, prefix: '~', optionalPrefixCharacters: ['s', 'S', 'b', 'B'] },
];
export const CONTEXT_SCANNERS = { css: CSS_SCANNERS, powershell: POWERSHELL_SCANNERS, erlang: ERLANG_SCANNERS };
export function contextScannerListing(id) {
  const descriptors = CONTEXT_SCANNERS[id];
  if (!descriptors) throw new TypeError(`unknown context scanner ${id}`);
  const guard = id === 'css' ? 'error_recovery' : id === 'erlang' ? 'error_sentinel' : null;
  return scannerFamilies(descriptors) + (guard ? `(scanner rejection_guard (tokens ${guard}) (operations fail))\n` : '');
}
if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) process.stdout.write(contextScannerListing(process.argv[2]));
