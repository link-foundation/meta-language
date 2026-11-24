import { deflateSync } from 'node:zlib';

const contents = 'BT /F1 12 Tf 72 712 Td [<000100020003> -300 <00040006>] TJ ET';
const cmap = `/CIDInit /ProcSet findresource begin
12 dict begin begincmap
1 begincodespacerange <0000> <FFFF> endcodespacerange
2 beginbfchar <0001> <0447> <0004> <D83EDDEA> endbfchar
2 beginbfrange <0002> <0003> [<0430> <0439>] <0005> <0006> <0041> endbfrange
endcmap end end`;
const objects = [];

function plain(number, body) {
  objects.push(`${number} 0 obj\n${body}\nendobj\n`);
}

function stream(number, contents, compressed = false) {
  const data = compressed ? deflateSync(Buffer.from(contents)).toString('latin1') : contents;
  const filter = compressed ? ' /Filter /FlateDecode' : '';
  objects.push(`${number} 0 obj\n<< /Length ${Buffer.byteLength(data, 'latin1')}${filter} >>\nstream\n${data}\nendstream\nendobj\n`);
  return data;
}

plain(1, '<< /Type /Catalog /Pages 2 0 R >>');
plain(2, '<< /Type /Pages /Kids [5 0 R 3 0 R] /Count 2 /Resources << /Font << /F1 6 0 R >> >> >>');
plain(3, '<< /Type /Page /Parent 2 0 R /Resources << /Font << /F1 8 0 R >> >> /Contents 4 0 R >>');
stream(4, String.raw`BT /F1 12 Tf (A\200\201) Tj ET`);
plain(5, '<< /Type /Page /Parent 2 0 R /Contents [7 0 R] >>');
plain(6, '<< /Type /Font /Subtype /Type0 /BaseFont /Example /Encoding /Identity-H /ToUnicode 9 0 R >>');
const encodedStream = stream(7, contents, true);
plain(8, '<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica /Encoding << /BaseEncoding /WinAnsiEncoding /Differences [129 /eacute] >> >>');
stream(9, cmap, true);

const source = `%PDF-1.7\n%\xc0\xc1\xc2\xc3\n${objects.join('')}%%EOF\n`;
const start = source.indexOf(encodedStream);
const end = start + encodedStream.length;
const corrupt = `${source.slice(0, start)}x${String.fromCharCode(255)}${source.slice(start + 2)}`;
console.log(JSON.stringify({ contents, encoded: Buffer.from(source, 'latin1').toString('hex'),
  corrupt: Buffer.from(corrupt, 'latin1').toString('hex'), start, end, text: 'чай 🧪B\nA€é' }));
