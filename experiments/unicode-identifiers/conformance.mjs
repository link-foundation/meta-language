// Future complete NFC conformance runner. Never execute during drafting.
// Download the pinned official NormalizationTest.txt temporarily in CI; pass its
// exact authenticated digest. The corpus stays outside the repository.
import { readFileSync } from 'node:fs';
import { createHash } from 'node:crypto';
import { normalizeUnicodeText, SOURCE_IDENTIFIER_UNICODE_VERSION } from '../../js/src/translation/unicode-identifiers.js';
const [file, digest] = process.argv.slice(2);
if (!file || !/^[a-f0-9]{64}$/u.test(digest ?? '')) throw new TypeError('use <Unicode NormalizationTest.txt> <pinned sha256>');
const bytes = readFileSync(file), source = bytes.toString('utf8');
if (createHash('sha256').update(bytes).digest('hex') !== digest) throw new Error('Unicode conformance source digest differs');
if (!source.slice(0, 2048).includes('NormalizationTest-' + SOURCE_IDENTIFIER_UNICODE_VERSION + '.txt')) throw new Error('uncovered Unicode conformance version');
let assertions = 0;
for (const [lineIndex, line] of source.split('\n').entries()) {
  const content = line.split('#')[0].trim();
  if (!content || content.startsWith('@')) continue;
  const columns = content.split(';').slice(0, 5).map(column => column.trim().split(/\s+/u).filter(Boolean)
    .map(point => String.fromCodePoint(Number.parseInt(point, 16))).join(''));
  if (columns.length !== 5) throw new Error('incomplete Unicode conformance row ' + (lineIndex + 1));
  for (let index = 0; index < columns.length; index += 1) {
    const expected = columns[index < 3 ? 1 : 3], actual = normalizeUnicodeText(columns[index]);
    if (actual !== expected) throw new Error('Unicode NFC differs at row ' + (lineIndex + 1) + ' column ' + (index + 1));
    assertions += 1;
  }
}
if (assertions === 0) throw new Error('Unicode conformance corpus is empty');
process.stdout.write('Unicode ' + SOURCE_IDENTIFIER_UNICODE_VERSION + ': ' + assertions + ' NFC assertions\n');
