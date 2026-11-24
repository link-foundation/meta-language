// Future CI producer; no behavioral invocation is allowed in the draft phase.
import { runInNewContext } from 'node:vm';
import { readProjectData } from '../../js/scripts/project-data.mjs';
import { structuredDataToLino } from '../../js/src/structured-data-links.js';
import { selfTranslate } from '../../js/src/self-translation.js';
const [major, minor] = process.versions.node.split('.').map(Number);
if (!(major === 22 && minor >= 15 || major === 24)) throw new RangeError('uncovered Node ' + process.versions.node + '; >=22.15 <23 or >=24 <25');
const fixture = readProjectData(new URL('../../parity/fixtures/array-state-methods.lino', import.meta.url));
const source = fixture.cases.map((row, index) => row.source.replace('function compute(', 'function compute' + index + '(')).join('\n');
const translated = selfTranslate(source, 'JavaScript', 'Rust', { semanticTranslation: true, retainDiagnostics: true });
if (!translated.items.every(item => ['translated', 'comment', 'provenance'].includes(item.status))) throw new TypeError('every real fixture function must translate');
const expected = fixture.cases.map(row => String(runInNewContext(row.source.replace(/^export /gmu, '') + '\ncompute();', {}, { timeout: 1000 })));
process.stdout.write(structuredDataToLino({ source, code: translated.code, expected }));
