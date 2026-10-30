// Input declarations only. Never obtain expected values by running a kernel.
import { writeProjectData } from './project-data.mjs';
if (process.argv.slice(2).join(' ') !== '--draft') throw new TypeError('use --draft to author fixture declarations');
const symbols = ['&', '|', '^', '<<', '>>', '~'];
const operations = symbols.map((symbol, index) => ({ symbol, name: 'compute' + index,
  source: '/** @param {bigint} left @param {bigint} right @returns {bigint} */\nexport function compute' + index
    + '(left,right){return ' + (symbol === '~' ? '~left' : 'left ' + symbol + ' right') + ';}\n',
  assignmentName: symbol === '~' ? '' : 'assign' + index,
  assignmentSource: symbol === '~' ? '' : '/** @param {bigint} left @param {bigint} right @returns {bigint} */\nexport function assign' + index
    + '(left,right){left ' + symbol + '= right;return left;}\n' }));
const valueSources = ['0n','1n','-1n','2n','-2n','7n','-7n','4294967296n','-4294967296n',
  '9007199254740993n','-9007199254740993n','18446744073709551615n','-18446744073709551616n',
  '340282366920938463463374607431768211455n','-340282366920938463463374607431768211456n'];
const shiftSources = ['-129n','-65n','-33n','-32n','-1n','0n','1n','31n','32n','33n','64n','65n','129n'];
const source = operations.map(row => row.source + row.assignmentSource).join('\n');
writeProjectData(new URL('../../parity/fixtures/big-integer-bitwise.lino', import.meta.url), {
  schemaVersion: 1, model: 'exact signed BigInt bitwise operator product 1', operations, valueSources, shiftSources, source,
  rightShiftSaturation: { left: '-340282366920938463463374607431768211456n', right: '999999999999999999999999999999999n' },
  kernelSource: source + '\n' + operations.map(row => 'console.log(' + row.name + '(-9007199254740993n,5n));').join('\n') + '\n',
  versions: { JavaScript: 'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0', Rust: '>=1.90.0 <1.100.0; edition 2024',
    Lean: '>=4.34.1 <4.35.0', Rocq: 'core >=9.3.0 <9.4.0; standard library >=9.2.0 <9.3.0' },
  refusals: [
    '/** @returns {bigint} */\nexport function compute(){return 1n >>> 1n;}\n',
    '/** @returns {bigint} */\nexport function compute(){return 1n & 1;}\n',
    '/** @returns {number} */\nexport function compute(){return 1 & 1n;}\n',
  ],
  boundary: 'Declared exact input products, independent original source operators, both linked codecs, complete four-target emission, compound local assignments and huge arithmetic-right-shift saturation. No execution observation. Implicit coercion, faithful mixed-type errors, unsigned BigInt shift errors and host resource-error equivalence remain independent obligations.',
});
