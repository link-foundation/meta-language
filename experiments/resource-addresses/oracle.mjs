// Future CI producer, intentionally unexecuted during the full source draft.
import { createHash } from 'node:crypto';
import { readProjectData } from '../../js/scripts/project-data.mjs';
import { structuredDataToLino } from '../../js/src/structured-data-links.js';
import { importResourceDescriptionNTriples } from '../../js/src/resource-description-ntriples.js';
const [major, minor] = process.versions.node.split('.').map(Number);
if (!(major === 22 && minor >= 15 || major === 24)) throw new RangeError('uncovered Node ' + process.versions.node + '; >=22.15 <23 or >=24 <25');
const fixture = readProjectData(new URL('../../parity/fixtures/resource-description-addresses.lino', import.meta.url));
process.stdout.write(structuredDataToLino(fixture.valid.map(value => {
  const source = '<' + value + '> <urn:predicate> "value" .\n';
  const graph = importResourceDescriptionNTriples(source, { sourceIdentifier: 'address-fixture' });
  return { value, root: graph.root.asU64(), syntaxRoot: graph.syntaxRoot.asU64(),
    sha256: createHash('sha256').update(graph.network.toLino()).digest('hex') };
})));
