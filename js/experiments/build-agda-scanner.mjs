// Port the pinned Agda scanner's indentation stack and pending layout queue
// to shared Links operations; no host-language scanner is loaded at runtime.
import { parseGrammarLinks, renderGrammarLinks } from '../src/grammar-links.js';
import { fileURLToPath } from 'node:url';
import { resolve } from 'node:path';
const integer = n => `(integer ${n})`;
const value = name => `(variable ${name})`;
const set = (name, n) => `(set ${name} ${n})`;
const branch = (condition, yes, no = '') => `(if ${condition} (then ${yes})${no ? ` (else ${no})` : ''})`;
const valid = name => `(valid ${name})`;
const stack = 'agda_indents';
const queue = 'agda_layout_tokens';
const top = `(top ${stack})`;
const depth = `(depth ${stack})`;
const width = value('agda_width');
const pending = value('agda_pending_dedents');
const newline = `(push ${queue} ${integer(0)})`;
const dedent = `${newline} (push ${queue} ${integer(2)})`;
const indent = `(push ${stack} ${width}) (push ${queue} ${integer(1)})`;
export function agdaScanner() {
  const whitespace = '(class plain (char %20) (char %09) (char %0D) (char %0A))';
  const greater = `(greater ${width} ${top})`;
  const less = `(less ${width} ${top})`;
  const afterNewline = branch(greater, branch(valid('indent'), indent), branch(valid('newline'), newline));
  const withoutNewline = branch(greater, branch(valid('indent'), indent), branch(less, `(pop ${stack}) (while (all (greater ${depth} ${integer(1)}) ${less}) (do (pop ${stack}) ${set('agda_pending_dedents', `(add ${pending} ${integer(1)})`)})) ${branch(valid('dedent'), dedent, set('agda_pending_dedents', `(add ${pending} ${integer(1)})`))}`));
  const eof = branch(`(all ${valid('dedent')} (greater ${depth} ${integer(1)}))`, `(pop ${stack}) ${dedent}`, branch(valid('newline'), newline));
  const scan = `${set('agda_skipped_newline', integer(0))} (while (next ${whitespace}) (do ${branch('(next (literal %0A))', set('agda_skipped_newline', integer(1)))} (skip ${whitespace}))) ${set('agda_width', 'column')} (while (greater ${width} ${integer(65535)}) (do ${set('agda_width', `(subtract ${width} ${integer(65536)})`)})) ${branch('atEnd', eof, branch(`(equal ${value('agda_skipped_newline')} ${integer(1)})`, afterNewline, withoutNewline))}`;
  const fill = branch(`(all ${valid('dedent')} (greater ${pending} ${integer(0)}))`, `${set('agda_pending_dedents', `(subtract ${pending} ${integer(1)})`)} ${dedent}`, scan);
  const emit = ['newline', 'indent', 'dedent'].map((name, index) => branch(`(equal (top ${queue}) ${integer(index)})`, `(pop ${queue}) (emit ${name})`)).join(' ');
  const operations = `${branch(`(equal ${depth} ${integer(0)})`, `(push ${stack} ${integer(0)})`)} ${branch(`(equal (depth ${queue}) ${integer(0)})`, fill)} ${branch(`(equal (depth ${queue}) ${integer(0)})`, 'fail')} ${emit} fail`;
  const declaration = `(scanner agda_layout (tokens newline indent dedent) (operations ${operations}))`;
  return renderGrammarLinks(parseGrammarLinks(`(grammar (start unused))\n${declaration}\n(rule unused normal (literal x))\n`)).split('\n').filter(line => line.startsWith('(scanner ')).join('\n') + '\n';
}
if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) process.stdout.write(agdaScanner());
