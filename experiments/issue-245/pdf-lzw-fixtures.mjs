import { readProjectData, writeProjectData } from '../../js/scripts/project-data.mjs';

// The code-width boundaries and the repeated entry below follow §3.3.3,
// Table 3.6 and Table 3.7 of the Adobe PDF Reference 1.7.
// https://opensource.adobe.com/dc-acrobat-sdk-docs/pdfstandards/pdfreference1.7old.pdf
function encode(earlyChange, padding) {
  const output = [];
  let buffer = 0;
  let bits = 0;
  function write(code, width) {
    buffer = (buffer << width) | code;
    bits += width;
    while (bits >= 8) {
      bits -= 8;
      output.push((buffer >>> bits) & 255);
    }
  }
  const width = (count) => {
    if (count < 255 - earlyChange) return 9;
    if (count < 767 - earlyChange) return 10;
    if (count < 1791 - earlyChange) return 11;
    return 12;
  };
  write(256, 9);
  for (let count = 0; count < padding; count += 1) write(32, width(count));
  write(256, width(padding));
  for (const character of 'BT 0 20 Td (') write(character.charCodeAt(0), 9);
  write(256, 9);
  for (const code of [45, 258, 258, 65, 259, 66]) write(code, 9);
  for (const character of ') Tj ET') write(character.charCodeAt(0), 9);
  write(257, 9);
  if (bits > 0) output.push((buffer << (8 - bits)) & 255);
  return Buffer.from(output);
}

const file = new URL('../../parity/fixtures/pdf-document.lino', import.meta.url);
const fixture = readProjectData(file);
const padding = 2048;
const text = '-----A---B';
fixture.lzw = {
  padding,
  contents: `BT 0 20 Td (${text}) Tj ET`,
  text,
  streams: [
    { parameters: '/Filter /LZWDecode', compression: 'LZWDecode', data: encode(1, padding).toString('hex') },
    {
      parameters: '/Filter [/ASCIIHexDecode /LZW] /DecodeParms [null << /EarlyChange 0 >>]',
      compression: 'ASCIIHexDecode,LZW',
      data: Buffer.from(`${encode(0, padding).toString('hex')}>`).toString('hex'),
    },
  ],
  invalid: [
    { parameters: '/Filter /LZWDecode', data: '8000' },
    { parameters: '/Filter /LZWDecode /DecodeParms << /EarlyChange 2 >>', data: '804040' },
  ],
};
writeProjectData(file, fixture);
