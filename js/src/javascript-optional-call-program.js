import { readFileSync } from 'node:fs';
import { LinkNetwork } from './network.js';
import { decodeTypedProgramLinks } from './typed-program-links.js';
import { recheckLinkedTypedProgram } from './typed-program-checked-projection.js';

/** Read the complete editable lexical optional callable construction definition. */
export function createJavaScriptOptionalCallProgramLinks() {
  const network = LinkNetwork.fromLino(readFileSync(new URL('./data/javascript-optional-call-program.lino', import.meta.url), 'utf8'));
  const roots = network.links().filter(link => link.metadata().language === 'meta-language typed program'
    && link.metadata().term === 'typed program' && link.metadata().definition === '1');
  if (roots.length !== 1) throw new TypeError('the JavaScript optional callable program needs one actual schema root');
  const root = roots[0].id();
  recheckLinkedTypedProgram(decodeTypedProgramLinks(network, root), { wholeProgram: true });
  return { network, root };
}
