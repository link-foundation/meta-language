import { readProjectData, writeProjectData } from '../../js/scripts/project-data.mjs';

const fixtures = [
  ['date', [
    ['"2000-02-29"', true], ['"2\\u003000\\u002d02-2\\u0039"', true],
    ['"2100-02-29"', false], ['"2100-02-2\\u0039"', false],
    ['"2026-04-31"', false], ['"2026-00-01"', false],
  ]],
  ['date-time', [
    ['"2024-02-29T13:50:00+03:00"', true],
    ['"2024-02-29\\u007413\\u003A50:00.15\\u002B03:00"', true],
    ['"2024-02-29T13:50:00\\u005a"', true],
    ['"2024-02-29T24:00:00Z"', false], ['"2024-02-29T13:50:00+03:60"', false],
    ['"2024-02-29T13:50:61Z"', false],
  ]],
  ['uuid', [
    ['"123E4567-e89b-12d3-a456-426614174000"', true],
    ['"123\\u00454567\\u002De89b-12d3-a456-426614174000"', true],
    ['"123g4567-e89b-12d3-a456-426614174000"', false],
    ['"123\\u00474567-e89b-12d3-a456-426614174000"', false],
    ['"123e4567-e89b-12d3-a456-42661417400"', false],
  ]],
  ['email', [
    ['"first.last@example.org"', true], ['"first\\u002elast\\u0040example.org"', true],
    ['"\\u0022a b\\u0022\\u0040example.org"', true],
    [JSON.stringify('"a\\"b"@example.org'), true],
    ['"a@-example.org"', false], ['"a..b@example.org"', false],
    ['"\\u0022a\\u000ab\\u0022@example.org"', false],
    ['"first\\u002flast@example.org"', true], ['"first\\/last@example.org"', true],
  ]],
];

const file = new URL('../../parity/fixtures/json-schema-and-recovery.lino', import.meta.url);
const data = readProjectData(file);
const missing = fixtures.filter(([format]) => !data.grammars.some(([schema]) => schema.format === format));
if (missing.length) {
  data.grammars.push(...missing.map(([format, cases]) => [{ type: 'string', format }, cases, []]));
  writeProjectData(file, data);
}
