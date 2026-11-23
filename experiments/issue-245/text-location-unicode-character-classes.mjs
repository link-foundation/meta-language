import assert from 'node:assert/strict';
import { wordNgramOverlap } from '../../js/src/index.js';

for (const word of ['A𲎰B', 'x𖺠y', 'b𑷠c', 'a᫏b']) {
  const prefix = '🚀 ';
  const matches = wordNgramOverlap(`${prefix}${word}`, word, { size: 1 });
  assert.deepEqual(matches, [{
    words: [word],
    left: { start: prefix.length, end: prefix.length + word.length },
    right: { start: 0, end: word.length },
  }]);
  console.log(JSON.stringify(matches[0]));
}
