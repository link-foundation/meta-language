import { readFileSync } from 'node:fs';
import { LinkNetwork } from './network.js';
import { decodeTypedProgramLinks } from './typed-program-links.js';
import { recheckLinkedTypedProgram } from './typed-program-checked-projection.js';

/** Read the full editable source-derived JavaScript primitive comparison definition graph. */
export function createJavaScriptComparisonProgramLinks() {
  const network = LinkNetwork.fromLino(readFileSync(new URL('./data/javascript-comparison-program.lino', import.meta.url), 'utf8'));
  const roots = network.links().filter(link => link.metadata().language === 'meta-language typed program'
    && link.metadata().term === 'typed program' && link.metadata().definition === '1');
  if (roots.length !== 1) throw new TypeError('the JavaScript primitive comparison program needs one actual schema root');
  const root = roots[0].id();
  recheckLinkedTypedProgram(decodeTypedProgramLinks(network, root), { wholeProgram: true });
  return { network, root };
}
