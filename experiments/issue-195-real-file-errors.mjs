// Prints the source lines holding the outermost ERROR/MISSING nodes of a parsed real file.
//   node experiments/issue-195-real-file-errors.mjs LANGUAGE FILE [LIMIT]
import { readFileSync } from 'node:fs';
import { LinkNetwork, LinkType } from '../js/src/index.js';

const [language, file, limit = '40'] = process.argv.slice(2);
const source = readFileSync(file, 'utf8');
const lines = source.split('\n');
const network = LinkNetwork.parse(source, language);
const errors = network.links().filter((link) => link.metadata().linkType === LinkType.Syntax &&
  (link.metadata().flags.isError || link.metadata().flags.isMissing));
const seen = new Set();
for (const link of errors) {
  const row = link.metadata().span.start.row;
  if (seen.has(row)) continue;
  seen.add(row);
  if (seen.size > Number(limit)) break;
  const kind = link.metadata().flags.isMissing ? `MISSING ${link.metadata().term}` : 'ERROR';
  console.log(`${row + 1}: [${kind}] ${lines[row].slice(0, 150)}`);
}
