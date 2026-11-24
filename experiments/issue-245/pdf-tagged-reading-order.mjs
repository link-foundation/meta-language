import { readProjectData, writeProjectData } from '../../js/scripts/project-data.mjs';

const stream = (content, entries = '') =>
  '<< /Length ' + Buffer.byteLength(content) + ' ' + entries + ' >>\nstream\n' + content + '\nendstream';
const font = '/Font << /F1 4 0 R >>';
const objects = [
  '<< /Type /Catalog /Pages 2 0 R /MarkInfo << /Marked true >> /StructTreeRoot 9 0 R >>',
  '<< /Type /Pages /Kids [3 0 R 10 0 R] /Count 2 /MediaBox [0 0 500 500] >>',
  '<< /Type /Page /Parent 2 0 R /StructParents 0 /Contents [5 0 R 7 0 R] /Resources << ' +
    font + ' /XObject << /Form 6 0 R >> /Properties << /First 8 0 R >> >> >>',
  '<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>',
  stream('/P << /MCID 1 >> BDC BT /F1 12 Tf 20 400 Td (Later) Tj ET EMC\n/Form Do\n' +
    '/Artifact BMC BT /F1 12 Tf 20 40 Td (Footer) Tj ET EMC'),
  stream('/P << /MCID 0 >> BDC BT /F1 12 Tf 20 300 Td (Form) Tj ET EMC',
    '/Type /XObject /Subtype /Form /BBox [0 0 500 500] /StructParents 1 /Resources << ' + font + ' >>'),
  stream('/P /First BDC /Span BMC BT /F1 12 Tf 20 200 Td (First) Tj ( paragraph) Tj ET EMC EMC'),
  '<< /MCID 0 >>',
  '<< /Type /StructTreeRoot /K 11 0 R /ParentTree 14 0 R /ParentTreeNextKey 3 >>',
  '<< /Type /Page /Parent 2 0 R /StructParents 2 /Contents 16 0 R /Resources << ' + font + ' >> >>',
  '<< /Type /StructElem /S /Document /P 9 0 R /K [12 0 R 13 0 R 15 0 R] >>',
  '<< /Type /StructElem /S /P /P 11 0 R /Pg 3 0 R /K 0 >>',
  '<< /Type /StructElem /S /P /P 11 0 R /Pg 3 0 R /K [<< /Type /MCR /Stm 6 0 R /MCID 0 >> << /Type /MCR /MCID 1 >>] >>',
  '<< /Nums [0 [12 0 R 13 0 R] 1 [13 0 R] 2 [15 0 R]] >>',
  '<< /Type /StructElem /S /P /P 11 0 R /Pg 10 0 R /K 0 >>',
  stream('/P << /MCID 0 >> BDC BT /F1 12 Tf 20 400 Td (Second page) Tj ET EMC'),
];

let page = '%PDF-1.7\n';
const offsets = [];
for (const [index, object] of objects.entries()) {
  offsets.push(Buffer.byteLength(page));
  page += (index + 1) + ' 0 obj\n' + object + '\nendobj\n';
}
const xref = Buffer.byteLength(page);
page += 'xref\n0 ' + (objects.length + 1) + '\n0000000000 65535 f \n';
for (const offset of offsets) page += String(offset).padStart(10, '0') + ' 00000 n \n';
page += 'trailer\n<< /Size ' + (objects.length + 1) + ' /Root 1 0 R >>\nstartxref\n' + xref + '\n%%EOF\n';

const ranges = [
  ['First paragraph', ['(First)', '( paragraph)']],
  ['Form', ['(Form)']],
  ['Later', ['(Later)']],
  ['Footer', ['(Footer)']],
  ['Second page', ['(Second page)']],
];
const taggedReadingOrder = { page, text: ranges.map(([line]) => line).join('\n'), ranges };

if (process.argv.includes('--write')) {
  const target = new URL('../../parity/fixtures/pdf-document.lino', import.meta.url);
  const fixture = readProjectData(target);
  if (!fixture.taggedReadingOrder) writeProjectData(target, { ...fixture, taggedReadingOrder });
} else process.stdout.write(page);
