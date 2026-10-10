// Generate R's raw literals and contextual bracket scopes as shared scanner data.
import { scannerFamilies } from '../scripts/scanner-families.mjs';
import { fileURLToPath } from 'node:url';
import { resolve } from 'node:path';

export const R_SCANNERS = [{
  family: 'scoped-layout', name: 'scoped_layout', startToken: 'start',
  newlineToken: 'newline', separatorToken: 'semicolon', continuationToken: 'external_else',
  recoveryToken: 'error_sentinel',
  pairs: [
    { opening: '(', closing: ')', openToken: 'external_open_parenthesis', closeToken: 'external_close_parenthesis', ignoreNewlines: true, allowContinuation: false },
    { opening: '{', closing: '}', openToken: 'external_open_brace', closeToken: 'external_close_brace', ignoreNewlines: false, allowContinuation: true },
    { opening: '[[', closing: ']]', openToken: 'external_open_double_bracket', closeToken: 'external_close_double_bracket', ignoreNewlines: true, allowContinuation: false },
    { opening: '[', closing: ']', openToken: 'external_open_bracket', closeToken: 'external_close_bracket', ignoreNewlines: true, allowContinuation: false },
  ],
}, {
  family: 'quoted-counted', name: 'raw_string', startToken: 'raw_string_open',
  contentToken: 'raw_string_content', endToken: 'raw_string_close', maximum: 255,
}];

if (process.argv[1] && fileURLToPath(import.meta.url) === resolve(process.argv[1])) {
  process.stdout.write(scannerFamilies(R_SCANNERS));
}
