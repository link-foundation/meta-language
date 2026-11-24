import assert from 'node:assert/strict';
import { locate } from '../../js/src/index.js';

const options = { normalize: ['nfkc'] };
for (const [spelling, quote] of [['ﾡￂ', '가'], ['꟱', 'S']]) {
  const prefix = '🚀 ';
  const matches = locate(`${prefix}${spelling} together`, quote, options);
  assert.deepEqual(matches, [{
    start: prefix.length,
    end: prefix.length + spelling.length,
    text: spelling,
    normalizations: ['nfkc'],
    distance: 0,
    edits: [],
  }]);
  console.log(JSON.stringify(matches[0]));
}
