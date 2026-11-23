import assert from 'node:assert/strict';
import { deflateSync } from 'node:zlib';
import { LinkNetwork, ParseConfiguration } from '../../js/src/index.js';
import { readProjectData, writeProjectData } from '../../js/scripts/project-data.mjs';

function stream(content, entries = '') {
  return `<< /Length ${Buffer.byteLength(content, 'latin1')}${entries && ' ' + entries} >>\nstream\n${content}\nendstream`;
}

function characterMap(name, mappings, { base, baseName, compressed = false, codespace = [], kind = 2 } = {}) {
  const program = [
    '/CIDInit /ProcSet findresource begin',
    '12 dict begin',
    'begincmap',
    `/CIDSystemInfo << /Registry (Adobe) /Ordering (${kind === 2 ? 'UCS' : 'Identity'}) /Supplement 0 >> def`,
    `/CMapName /${name} def`,
    `/CMapType ${kind} def`,
    ...(baseName ? [`/${baseName} usecmap`] : []),
    ...(codespace.length ? [`${codespace.length} begincodespacerange`, ...codespace, 'endcodespacerange'] : []),
    ...mappings,
    'endcmap',
    'CMapName currentdict /CMap defineresource pop',
    'end',
    'end',
  ].join('\n');
  const data = compressed ? deflateSync(program).toString('latin1') : program;
  const dictionary = ['/Type /CMap', `/CMapName /${name}`, base ? `/UseCMap ${base} 0 R` : '', compressed ? '/Filter /FlateDecode' : ''];
  return stream(data, dictionary.filter(Boolean).join(' '));
}

function unicodeMap(name, entries, options) {
  return characterMap(name, [
    `${entries.length} beginbfchar`, ...entries.map(([code, value]) => `<${code}> <${value}>`), 'endbfchar',
  ], options);
}

function makeFixture({ compressed = false, cyclic = false, missing = false, variableWidth = false, byteRanges = false }) {
  const show = byteRanges ? '<0001410105400011000200030004>'
    : variableWidth ? '<0001B00005000200030004>' : '<0001000200030004>';
  const encoding = variableWidth || byteRanges ? '10 0 R' : '/Identity-H';
  const codespace = byteRanges ? ['<05> <05>', '<0000> <00FF>', '<4101> <4101>', '<400010> <420020>']
    : variableWidth ? ['<0000> <00FF>', '<B00000> <B000FF>'] : ['<0000> <FFFF>'];
  const baseEntries = [['0004', '0439']];
  if (byteRanges) baseEntries.push(['05', '03A9'], ['400011', '0416']);
  const objects = [
    '<< /Type /Catalog /Pages 2 0 R >>',
    '<< /Type /Pages /Kids [3 0 R] /Count 1 /MediaBox [0 0 200 200] >>',
    '<< /Type /Page /Parent 2 0 R /Resources << /Font << /F0 4 0 R >> >> /Contents 5 0 R >>',
    `<< /Type /Font /Subtype /Type0 /BaseFont /Example /Encoding ${encoding} /DescendantFonts [9 0 R] /ToUnicode 6 0 R >>`,
    stream(`BT /F0 12 Tf 10 50 Td ${show} Tj ET`),
    unicodeMap('ExampleDerived', [['0001', '0429'], ['0003', 'D83EDDEA']], { base: missing ? 10 : 7, baseName: 'ExampleMiddle' }),
    unicodeMap('ExampleMiddle', [['0001', '0430'], ['0002', '0447']], { base: 8, baseName: 'ExampleBase', compressed }),
    unicodeMap('ExampleBase', baseEntries, { ...(cyclic && { base: 7, baseName: 'ExampleMiddle' }), codespace }),
    '<< /Type /Font /Subtype /CIDFontType2 /BaseFont /Example /CIDSystemInfo << /Registry (Adobe) /Ordering (Identity) /Supplement 0 >> >>',
  ];
  if (variableWidth || byteRanges) {
    const cidRanges = byteRanges
      ? ['<05> <05> 5', '<0000> <00FF> 256', '<4101> <4101> 512', '<400010> <400020> 513', '<410010> <410020> 530', '<420010> <420020> 547']
      : ['<0000> <00FF> 0', '<B00000> <B000FF> 256'];
    objects.push(characterMap('ExampleEncoding', [
      `${cidRanges.length} begincidrange`, ...cidRanges, 'endcidrange',
    ], { codespace, kind: 1 }));
  }
  let source = '%PDF-1.7\n';
  const offsets = [];
  const objectNumbers = objects.map((_, index) => index + 1);
  // Place the compressed map before the text operand to exercise both byte coordinate systems.
  for (const number of [...objectNumbers.filter(number => number !== 5), 5]) {
    offsets[number - 1] = Buffer.byteLength(source, 'latin1');
    source += `${number} 0 obj\n${objects[number - 1]}\nendobj\n`;
  }
  const xref = Buffer.byteLength(source, 'latin1');
  source += `xref\n0 ${objects.length + 1}\n0000000000 65535 f \n`;
  for (const offset of offsets) source += `${String(offset).padStart(10, '0')} 00000 n \n`;
  source += `trailer\n<< /Size ${objects.length + 1} /Root 1 0 R >>\nstartxref\n${xref}\n%%EOF\n`;
  const start = source.indexOf(show);
  const partial = 'Щ�🧪�';
  const full = byteRanges ? 'Щ�ΩЖч🧪й' : variableWidth ? 'Щ�ч🧪й' : 'Щч🧪й';
  const text = missing ? partial : full;
  return {
    encoded: Buffer.from(source, 'latin1').toString('hex'), start, end: start + show.length,
    text, undecoded: compressed ? partial : text,
  };
}

const inheritedUnicodeMaps = [
  makeFixture({}),
  makeFixture({ compressed: true }),
  makeFixture({ cyclic: true }),
  makeFixture({ missing: true }),
  makeFixture({ variableWidth: true }),
  makeFixture({ byteRanges: true }),
];

if (process.argv.includes('--write')) {
  const target = new URL('../../parity/fixtures/pdf-document.lino', import.meta.url);
  writeProjectData(target, { ...readProjectData(target), inheritedUnicodeMaps });
} else {
  for (const [index, value] of inheritedUnicodeMaps.entries()) {
    const bytes = Buffer.from(value.encoded, 'hex');
    for (const decoded of [true, false]) {
      const config = ParseConfiguration.default().withDocumentStreamDecoding(decoded);
      const projection = LinkNetwork.parseBytes(bytes, 'PDF', config).visibleText();
      assert.equal(projection.text, decoded ? value.text : value.undecoded, `map ${index}, decode ${decoded}`);
      assert.deepEqual(projection.sourceRange(0, projection.text.length), { start: value.start, end: value.end });
    }
  }
  console.log(`${inheritedUnicodeMaps.length} PDF Unicode map inheritance cases passed with decoding enabled and disabled`);
}
