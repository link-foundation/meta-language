import assert from 'node:assert/strict';
import { locate, wordNgramOverlap } from '../../js/src/text-location.js';

const normalize = ['case'];
const cases = [
  ['🌍 ΠΛΗΡΩΜΗ ΔΟΣΗΣ', 'πληρωμη δοσης', 'ΠΛΗΡΩΜΗ ΔΟΣΗΣ'],
  ['🌿 Straße', 'STRASSE', 'Straße'],
];
for (const [source, fragment, text] of cases) {
  const start = source.indexOf(text);
  assert.deepEqual(locate(source, fragment, { normalize }), [{
    start, end: start + text.length, text, normalizations: normalize, distance: 0, edits: [],
  }]);
}
assert.deepEqual(locate('ẞ', 'S', { normalize }), []);
const left = '🌍 Straße ΔΟΣΗΣ';
const right = 'STRASSE δοσης 🌿';
assert.deepEqual(wordNgramOverlap(left, right, { size: 2, normalize }), [{
  words: ['strasse', 'δοσησ'],
  left: { start: 3, end: left.length },
  right: { start: 0, end: right.indexOf(' 🌿') },
}]);
