// Future CI producer. Never run during the complete source draft phase.
import { createHash } from 'node:crypto';
import { inferRegularGrammar } from '../../js/src/grammar-inference.js';
import { readProjectData } from '../../js/scripts/project-data.mjs';
import { structuredDataToLino } from '../../js/src/structured-data-links.js';
const [major, minor] = process.versions.node.split('.').map(Number);
if (!(major === 22 && minor >= 15 || major === 24)) throw new RangeError('uncovered Node ' + process.versions.node + '; >=22.15 <23 or >=24 <25');
const fixtures = readProjectData(new URL('../../parity/fixtures/regular-grammar-inference.lino', import.meta.url));
const argument = process.argv[2];
if (process.argv.length !== 3 || !/^(?:0|[1-9][0-9]*)$/u.test(argument ?? '')) throw new TypeError('provide one actual fixture index');
const fixture = fixtures.cases[Number(argument)];
if (!fixture) throw new RangeError('unknown fixture index');
const model = inferRegularGrammar(fixture.positives, fixture.negatives, fixture.options);
process.stdout.write(structuredDataToLino({ name: fixture.name, alphabet: model.alphabet, table: model.table,
  grammar: model.grammar.normalized(), root: model.root.asU64(),
  archiveSha256: createHash('sha256').update(model.network.toLino()).digest('hex') }));
