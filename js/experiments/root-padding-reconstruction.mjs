// Parses every draft source padded with leading and trailing whitespace
// through the public API and reports whether reconstruction is byte-exact.
import { LinkNetwork } from '../src/index.js';
import { DRAFTS } from './default-cst-source-drafts.mjs';

for (const [name, draft] of Object.entries(DRAFTS)) {
  const text = ` \n ${draft.source} \n `;
  const rebuilt = LinkNetwork.parse(text, name).reconstructText();
  console.log(`${rebuilt === text ? 'exact' : 'LOST '} ${name}${rebuilt === text ? '' : `: ${JSON.stringify(rebuilt.slice(0, 8))}…${JSON.stringify(rebuilt.slice(-8))}`}`);
}
