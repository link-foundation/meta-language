// Future CI producer. Do not execute during the whole requirements draft.
import { runInNewContext } from 'node:vm';
import { readProjectData } from '../../js/scripts/project-data.mjs';
import { structuredDataToLino } from '../../js/src/structured-data-links.js';
import { parseJavaScript } from '../../js/src/translation/javascript.js';
import { checkProgram } from '../../js/src/translation/check.js';
import { emitRust } from '../../js/src/translation/emit-rust.js';
import { encodeTypedProgramLinks, emitTypedProgramLinks } from '../../js/src/typed-program-links.js';
const [major, minor] = process.versions.node.split('.').map(Number);
if (!(major === 22 && minor >= 15 || major === 24)) throw new RangeError('uncovered Node ' + process.versions.node);
const fixture = readProjectData(new URL('../../parity/fixtures/big-integer-bitwise.lino', import.meta.url));
const values = fixture.valueSources.map(source => runInNewContext(source, {}, { timeout: 1000 }));
const shifts = fixture.shiftSources.map(source => runInNewContext(source, {}, { timeout: 1000 }));
const program = checkProgram(parseJavaScript(fixture.source)), packet = encodeTypedProgramLinks(program), rust = emitRust(program);
const header = rust.text.split('\n').find(line => line.startsWith('#![allow('));
if (!header) throw new TypeError('the complete library needs its real attributes');
const cases = [];
for (const operation of fixture.operations) {
  const rightValues = operation.symbol === '~' ? [0n] : operation.symbol === '<<' || operation.symbol === '>>' ? shifts : values;
  for (const name of [operation.name, operation.assignmentName].filter(Boolean)) {
    const fn = runInNewContext(fixture.source.replace(/^export /gmu, '') + '\n' + name + ';', {}, { timeout: 1000 });
    for (const left of values) for (const right of rightValues) cases.push({ name, left: String(left), right: String(right), expected: String(fn(left, right)) });
  }
}
const huge = fixture.rightShiftSaturation;
const right = runInNewContext(huge.right), left = runInNewContext(huge.left);
const fn = runInNewContext(fixture.source.replace(/^export /gmu, '') + '\ncompute4;');
cases.push({ name: 'compute4', left: String(left), right: String(right), expected: String(fn(left, right)) });
process.stdout.write(structuredDataToLino({ source: fixture.source, cases, root: packet.root.asU64(), links: packet.network.toLino(),
  emissions: Object.fromEntries(['JavaScript', 'Rust', 'Lean', 'Rocq'].map(target => [target, emitTypedProgramLinks(packet.network, packet.root, target)])),
  nativeLibrary: [header, ...rust.preludes, ...rust.definitions].join('\n\n') }));
