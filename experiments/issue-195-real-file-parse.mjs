// Parses whole real-project source files and reports reconstruction, cleanliness and error counts.
//   node experiments/issue-195-real-file-parse.mjs LANGUAGE FILE...
import { readFileSync } from 'node:fs';
import { LinkNetwork, LinkType } from '../js/src/index.js';

const [language, ...files] = process.argv.slice(2);
for (const file of files) {
  const source = readFileSync(file, 'utf8');
  const started = performance.now();
  const network = LinkNetwork.parse(source, language);
  const syntax = network.links().filter((link) => link.metadata().linkType === LinkType.Syntax);
  const errors = syntax.filter((link) => link.metadata().flags.isError || link.metadata().flags.isMissing);
  const lines = errors.slice(0, 5).map((link) => link.metadata().span?.start.row + 1);
  console.log(`${file}: lossless=${network.reconstructText() === source} clean=${network.verifyFullMatch().isClean()} syntax=${syntax.length} errors=${errors.length} firstErrorLines=${lines.join(',')} ms=${Math.round(performance.now() - started)}`);
}
