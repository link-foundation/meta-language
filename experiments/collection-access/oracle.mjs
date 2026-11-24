// Future independent CI producer. Never execute during the complete draft.
import { runInNewContext } from 'node:vm';
import { readProjectData } from '../../js/scripts/project-data.mjs';
import { structuredDataToLino } from '../../js/src/structured-data-links.js';
import { parseJavaScript } from '../../js/src/translation/javascript.js';
import { checkProgram } from '../../js/src/translation/check.js';
import { emitRust } from '../../js/src/translation/emit-rust.js';
import { encodeTypedProgramLinks, emitTypedProgramLinks } from '../../js/src/typed-program-links.js';
const [major, minor] = process.versions.node.split('.').map(Number);
if (!(major === 22 && minor >= 15 || major === 24)) throw new RangeError('uncovered Node ' + process.versions.node + '; >=22.15 <23 or >=24 <25');
const fixture = readProjectData(new URL('../../parity/fixtures/collection-access.lino', import.meta.url));
const source = fixture.cases.map((input, index) => input.source.replace('function compute(', 'function compute' + index + '(')).join('\n');
const program = checkProgram(parseJavaScript(source)), packet = encodeTypedProgramLinks(program), rust = emitRust(program);
const header = rust.text.split('\n').find(line => line.startsWith('#![allow('));
if (!header) throw new TypeError('the complete translated library needs its actual attributes');
const expected = fixture.cases.map(input => String(runInNewContext(input.source.replace(/^export /gmu, '') + '\ncompute();', {}, { timeout: 1000 })));
process.stdout.write(structuredDataToLino({ source, root: packet.root.asU64(), links: packet.network.toLino(),
  emissions: Object.fromEntries(['JavaScript', 'Rust', 'Lean', 'Rocq'].map(target => [target, emitTypedProgramLinks(packet.network, packet.root, target)])),
  nativeLibrary: [header, ...rust.preludes, ...rust.definitions].join('\n\n'), expected }));
