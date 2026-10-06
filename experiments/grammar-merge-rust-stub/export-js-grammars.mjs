// Writes the fixture sources, imported by the JavaScript pest importer and
// serialized, so the stub probe can run the Rust merge on the same IR.
import { readFileSync, writeFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { importPest, serializeGrammar } from '../../js/src/index.js';

const here = path.dirname(fileURLToPath(import.meta.url));
const fixture = JSON.parse(readFileSync(path.join(here, '../../parity/fixtures/grammar-merge.json'), 'utf8'));
const exported = {
  sources: fixture.sources.map((source) => ({ ...source, grammar: serializeGrammar(importPest(source.text)) })),
  upstreamChange: serializeGrammar(importPest(fixture.upstreamChange.text)),
  upstreamChangeText: fixture.upstreamChange.text,
};
writeFileSync(process.argv[2] ?? '/tmp/grammar-merge-js-grammars.json', JSON.stringify(exported, null, 2));
