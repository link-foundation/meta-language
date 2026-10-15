import { checkProjectData, writeProjectData } from './project-data.mjs';

const fixture = {
  schemaVersion: 1,
  reference: 'https://doc.rust-lang.org/reference/tokens.html#character-escapes',
  ascii: Array.from({ length: 128 }, (_, code) => code),
  nonAscii: Array.from({ length: 128 }, (_, index) => index + 128),
  invalidEscapes: ['\\b', '\\f', '\\v', '\\{', '\\}', '\\q', '\\X41', '\\x', '\\x0', '\\xG0', '\\x0G', '\\x_0', '\\x0_', '\\x{41}', '\\u0041'],
  suffixEscapes: ['\\x414', '\\x7ff', '\\00', '\\08'],
  suffixValues: ['A4', '\x7ff', '\x000', '\x008'],
  simpleEscapes: ['\\n', '\\r', '\\t', '\\0', '\\\\', '\\"', "\\'"],
  simpleValues: ['\n', '\r', '\t', '\0', '\\', '"', "'"],
  isolatedCarriageReturns: ['a\rb', '\r', '\ra', 'a\r'],
};
const file = new URL('../../parity/fixtures/rust-cooked-string-escapes.lino', import.meta.url);
if (process.argv.includes('--check')) checkProjectData(file, fixture);
else writeProjectData(file, fixture);
console.log('Rust cooked strings: 128 ASCII values, 128 non-ASCII refusals, exact spelling controls');
