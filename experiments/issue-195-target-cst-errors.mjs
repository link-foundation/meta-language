// Lists the CST error and missing nodes of translated artifacts (default: /tmp/pairs).
import { readFileSync, readdirSync } from 'node:fs';
import path from 'node:path';
import { LinkNetwork, LinkType } from '../js/src/index.js';

const languages = { mjs: 'javascript', rs: 'rust', lean: 'lean', v: 'rocq' };
const files = process.argv.slice(2);
const inputs = files.length > 0 ? files : readdirSync('/tmp/pairs').map((name) => path.join('/tmp/pairs', name));
for (const file of inputs) {
  const language = languages[file.split('.').pop()];
  const source = readFileSync(file, 'utf8');
  const network = LinkNetwork.parse(source, language);
  const report = network.verifyFullMatch();
  console.log(file, 'clean', report.isClean());
  if (report.isClean()) continue;
  const errors = network.links().filter((link) => {
    const metadata = link.metadata();
    return metadata.linkType === LinkType.Syntax && (metadata.term === 'ERROR' || metadata.isMissing);
  });
  for (const link of errors.slice(0, 8)) {
    const metadata = link.metadata();
    const span = metadata.span ?? metadata.byteRange ?? metadata;
    console.log('  ', metadata.term, JSON.stringify(span).slice(0, 160));
  }
  console.log('  report', JSON.stringify(report).slice(0, 600));
}
