// Ordinary typed Links definitions, including the complete recursive closure.
import { readFileSync } from 'node:fs';
import { LinkNetwork } from './network.js';
import { decodeTypedProgramLinks } from './typed-program-links.js';
import { recheckLinkedTypedProgram } from './typed-program-checked-projection.js';

/** Reconstruct a fresh editable modular integer definition graph. */
export function createFixedIntegerCastProgramLinks() {
  const network = LinkNetwork.fromLino(readFileSync(new URL('./data/fixed-integer-cast-program.lino', import.meta.url), 'utf8'));
  const roots = network.links().filter(link => link.metadata().language === 'meta-language typed program'
    && link.metadata().term === 'typed program' && link.metadata().definition === '1');
  if (roots.length !== 1) throw new TypeError('the fixed integer cast program needs one actual schema root');
  const root = roots[0].id();
  recheckLinkedTypedProgram(decodeTypedProgramLinks(network, root), { wholeProgram: true });
  return { network, root };
}
