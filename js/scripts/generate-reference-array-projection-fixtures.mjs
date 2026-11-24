// Author all source/expectation declarations without invoking any product.
import { writeProjectData } from './project-data.mjs';
if (process.argv.slice(2).join(' ') !== '--draft') throw new TypeError('use --draft');
const inputs = [
  ['live length', 'const values=[1,2,3];values.push(4);return values.length;', '4'],
  ['literal read', 'const values=[6,7];values.push(8);return (values[1]??0)+values.length;', '10'],
  ['pure arithmetic index', 'const values=[6,7];values.push(8);return values[Math.floor(3/2)]??0;', '7'],
  ['variable read', 'const values=[6,7];values.push(8);const position=1;return (values[position]??0)+values.length;', '10'],
  ['negative zero', 'const values=[5];values.push(6);return values[-0]??0;', '5'],
  ['fractional index', 'const values=[5];values.push(6);return values[0.5]??11;', '11'],
  ['negative index', 'const values=[5];values.push(6);return values[-1]??12;', '12'],
  ['NaN index', 'const values=[5];values.push(6);const position=0/0;return values[position]??13;', '13'],
  ['infinite index', 'const values=[5];values.push(6);const position=1/0;return values[position]??14;', '14'],
  ['BigInt index', 'const values=[8,9];values.push(10);return values[1n]??0;', '9'],
  ['negative BigInt index', 'const values=[8,9];values.push(10);return values[-1n]??16;', '16'],
  ['receiver evaluated once', 'const calls=[];const receiver=()=>{calls.push(1);return [4,5];};return (receiver()[1]??0)*10+calls.length;', '51'],
  ['key evaluated once after receiver', 'const values=[7];const calls=[];const key=()=>{calls.push(1);values.push(8);return 1;};return (values[key()]??0)*10+calls.length;', '81'],
  ['key replaces current elements', 'const values=[1];const key=()=>{values.fill(9);return 0;};return values[key()]??0;', '9'],
  ['key shrinks current array', 'const values=[1,2];const key=()=>{values.pop();return 1;};return values[key()]??17;', '17'],
  ['key rebinds variable after receiver capture', 'let values=[1];const key=()=>{values=[2];return 0;};return values[key()]??0;', '1'],
  ['projection temporary hygiene', 'const ml_reference_array1=3,ml_reference_position1=4,ml_reference_projection1=5;const values=[6];values.push(7);return (values[0]??0)+ml_reference_array1+ml_reference_position1+ml_reference_projection1;', '18'],
  ['shallow element identity', 'const values=[[3]];values.push([4]);const item=values[0]??[];item.push(5);return (values[0]??[]).length;', '2'],
  ['borrow released before later mutation', 'const values=[1,2];values.push(3);const first=values[0]??0;values.push(4);return first*10+values.length;', '14'],
  ['borrow released before sibling call', 'const values=[1,2];values.push(3);return (values[1]??0)+values.push(4);', '6'],
];
const cases = inputs.map(([name, body, value], index) => ({ name, source: 'export function compute(){' + body + '}',
  entry: 'referenceArrayProjectionCase' + index, value, abort: null }));
cases.push({ name: 'receiver failure precedes key', entry: 'referenceArrayProjectionReceiverFailure', value: null, abort: 'receiver first',
  source: '/** @returns {number[]} */ function failArrayProjectionReceiver(){throw new Error("receiver first");}\n/** @returns {number} */ function failArrayProjectionLaterKey(){throw new Error("key later");}\nexport function compute(){return failArrayProjectionReceiver()[failArrayProjectionLaterKey()]??0;}' });
cases.push({ name: 'key failure precedes lookup', entry: 'referenceArrayProjectionKeyFailure', value: null, abort: 'key first',
  source: '/** @returns {number} */ function failArrayProjectionKey(){throw new Error("key first");}\nexport function compute(){const values=[1];values.push(2);return values[failArrayProjectionKey()]??0;}' });
writeProjectData(new URL('../../parity/fixtures/reference-array-projections.lino', import.meta.url), {
  schemaVersion: 1, model: 'shared array scalar projection without payload copies 1', sourceLanguage: 'JavaScript', cases, refusals: [],
  source: cases.map(row => row.source.replace('function compute(', 'function ' + row.entry + '(')).join('\n'),
  versions: { JavaScript: 'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0; ECMAScript modules', Rust: '>=1.90.0 <1.100.0; edition 2024' },
  sourceBasis: 'https://tc39.es/ecma262/2025/multipage/ecmascript-language-expressions.html#sec-property-accessors ; evaluate receiver, then key, then GetValue; aliases retain the original receiver identity.',
  boundary: 'Future strict original Node/emitted JavaScript/native Rust values and aborts, exact paired definition/program graphs, text/binary codecs and four emissions. Separate selection/rendering controls retain the existing native payload-copy path for effectful typed keys. No observations.' });
