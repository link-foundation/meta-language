import { readFileSync } from 'node:fs';
import { documentScalarIsValid } from './document-text-policy.js';
import { documentProjectionRules } from './document-projection-rules.js';

const namedXmlReferences = new Map([
  ['amp;', '&'], ['lt;', '<'], ['gt;', '>'], ['quot;', '"'], ['apos;', "'"],
]);
const namedReferenceWidth = 32;
let namedHtmlReferences;

function htmlReferences() {
  namedHtmlReferences ??= new Map(readFileSync(
    new URL('./data/document-character-references.lino', import.meta.url), 'utf8',
  ).trimEnd().split('\n').map(line => {
    const [name, ...numbers] = line.slice('(namedCharacterReference '.length, -1).split(' ');
    return [name.replace('%3B', ';'), String.fromCodePoint(...numbers.map(Number))];
  }));
  return namedHtmlReferences;
}

/** Reads a character reference beginning at the given ampersand. */
export function documentCharacterReference(source, start, syntax) {
  if (source[start + 1] === '#') return numericReference(source, start, syntax);
  const names = syntax === 'xml' ? namedXmlReferences : htmlReferences();
  let end = start + 1;
  const limit = Math.min(end + namedReferenceWidth, source.length);
  while (end < limit && /[A-Za-z0-9]/u.test(source[end])) end += 1;
  if (source[end] === ';') end += 1;
  while (end > start + 1) {
    const name = source.slice(start + 1, end);
    const text = names.get(name);
    if (text !== undefined && (syntax === 'html' || name.endsWith(';'))) return { text, end };
    end -= 1;
  }
  return null;
}

function numericReference(source, start, syntax) {
  let end = start + 2;
  const hexadecimal = source[end] === 'x' || source[end] === 'X';
  if (hexadecimal) end += 1;
  const first = end;
  let number = 0;
  while (end < source.length) {
    const code = source.charCodeAt(end);
    const digit = code >= 48 && code <= 57 ? code - 48 : hexadecimal && (code | 32) >= 97 && (code | 32) <= 102
      ? (code | 32) - 87 : -1;
    if (digit < 0) break;
    number = Math.min(number * (hexadecimal ? 16 : 10) + digit, 0x110000);
    end += 1;
  }
  if (end === first || (syntax !== 'html' && source[end] !== ';')) return null;
  if (source[end] === ';') end += 1;
  const valid = documentScalarIsValid(number);
  if (!valid && syntax === 'xml') return null;
  const replacement = syntax === 'xml' ? undefined : documentProjectionRules('numericReferenceReplacement')[number - 0x80];
  const scalar = !valid ? 0xfffd : replacement === undefined ? number : Number(replacement);
  return { text: String.fromCodePoint(scalar), end };
}
