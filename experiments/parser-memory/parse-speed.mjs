// Times repeated parses of large Rust and JavaScript sources, to compare the
// parse speed of grammar code kept on V8's baseline compiler with tiered code.
import { readFileSync } from 'node:fs';
import { LinkNetwork } from '../../js/src/index.js';

const sources = [
  ['Rust', readFileSync(new URL('../../rust/src/tree_sitter_adapter.rs', import.meta.url), 'utf8')],
  ['JavaScript', readFileSync(new URL('../../js/src/programming-language-parser.js', import.meta.url), 'utf8')],
];
for (const [language, source] of sources) {
  const times = [];
  for (let round = 0; round < 8; round += 1) {
    const started = performance.now();
    LinkNetwork.parse(source, language);
    times.push(Math.round(performance.now() - started));
  }
  console.log(language, source.length, 'chars', times.join(' '), 'ms', Math.round(process.memoryUsage().rss / 1048576), 'MB');
}
