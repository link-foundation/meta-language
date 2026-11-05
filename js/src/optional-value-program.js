import { readFileSync } from 'node:fs';
import { LinkNetwork } from './network.js';
import { decodeTypedProgramLinks } from './typed-program-links.js';
import { recheckLinkedTypedProgram } from './typed-program-checked-projection.js';

/** Read the full editable source-derived optional value and Boolean conversion definition graph. */
export function createOptionalValueProgramLinks() {
  const network = LinkNetwork.fromLino(readFileSync(new URL('./data/optional-value-program.lino', import.meta.url), 'utf8'));
  const roots = network.links().filter(link => link.metadata().language === 'meta-language typed program'
    && link.metadata().term === 'typed program' && link.metadata().definition === '1');
  if (roots.length !== 1) throw new TypeError('the optional value and Boolean conversion program needs one actual schema root');
  const root = roots[0].id();
  recheckLinkedTypedProgram(decodeTypedProgramLinks(network, root), { wholeProgram: true });
  return { network, root };
}
