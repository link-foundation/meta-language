import { readProjectData, writeProjectData } from '../../js/scripts/project-data.mjs';

const fixtures = [
  ['^[A-Z]{2}$', [
    ['"AX"', true], [String.raw`"\u0041\u0058"`, true],
    [String.raw`"\u0041\u0078"`, false], [String.raw`"AX\n"`, false],
  ]],
  ['^a*$', [
    ['""', true], [String.raw`"a\u0061a"`, true], ['"b"', false],
  ]],
  [String.raw`^["\\\n]$`, [
    [String.raw`"\""`, true], [String.raw`"\u0022"`, true],
    [String.raw`"\\"`, true], [String.raw`"\u005C"`, true],
    [String.raw`"\n"`, true], [String.raw`"\u000a"`, true],
    [String.raw`"\t"`, false], ['""', false],
  ]],
  ['^[🧠-🧪]$', [
    ['"🧠"', true], [String.raw`"\uD83e\uDDE9"`, true],
    [String.raw`"\ud83e\uddea"`, true], [String.raw`"\ud83e\uddeb"`, false],
  ]],
  [String.raw`^(?:a|b?){2,3}z$`, [
    ['"z"', true], ['"abbz"', true], [String.raw`"\u0062az"`, true],
    ['"aaaaz"', false], ['"xxz"', false],
  ]],
  [String.raw`^a|[^\d]z$`, [
    ['"ab"', true], [String.raw`"🍎\n\u007a"`, true], ['"9z"', false],
    ['"x9z"', false], ['"xza"', false],
  ]],
  [String.raw`\d{2}\.\w`, [
    ['"🍎19.X!"', true], [String.raw`"\u0031\u0030\u002e_"`, true],
    ['"١٩.X"', false], ['"19.é"', false],
  ]],
  [String.raw`^.$`, [
    [String.raw`"\uD83D\uDE00"`, true], [String.raw`"\t"`, true],
    [String.raw`"\u2028"`, false], [String.raw`"\r"`, false],
  ]],
];

const file = new URL('../../parity/fixtures/json-schema-and-recovery.lino', import.meta.url);
const data = readProjectData(file);
const patterns = fixtures.map(([pattern, cases]) => [{ type: 'string', pattern }, cases, []]);
if (!patterns.every(([schema]) => data.grammars.some(([existing]) => existing.pattern === schema.pattern))) {
  data.grammars.push(...patterns);
  writeProjectData(file, data);
}
