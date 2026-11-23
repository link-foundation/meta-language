import { readFileSync } from 'node:fs';
import { rangeValue } from './text-location-graphemes.js';

let classes;

function readCharacterClasses() {
  const source = readFileSync(new URL('./data/text-location-character-classes.lino', import.meta.url), 'utf8');
  return source.trim().split('\n').slice(1).map(line => {
    const [value, first, last] = line.slice(1, -1).split(' ');
    return { value, first: Number.parseInt(first, 16), last: Number.parseInt(last, 16) };
  });
}

export function textLocationCharacterClass(value) {
  const codePoint = value.codePointAt(0);
  if (codePoint < 128) {
    if (/^[0-9A-Za-z_]/u.test(value)) return 'word';
    return value[0] === '-' ? 'dash' : undefined;
  }
  const ranges = classes ??= readCharacterClasses();
  return rangeValue(ranges, codePoint);
}
