import { readProjectData, writeProjectData } from '../../js/scripts/project-data.mjs';

const stream = (content, entries = '') => `<< /Length ${Buffer.byteLength(content)} ${entries} >>\nstream\n${content}\nendstream`;
const resources = '<< /Font << /F1 4 0 R >> /XObject << /Outer 7 0 R >> >>';
const objects = [
  '<< /Type /Catalog /Pages 2 0 R >>',
  '<< /Type /Pages /Kids [3 0 R 9 0 R] /Count 2 /MediaBox [0 0 500 500] >>',
  `<< /Type /Page /Parent 2 0 R /Resources ${resources} /Contents [5 0 R 6 0 R] >>`,
  '<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>',
  stream('q 1 0 0 1 0 240 cm /Outer Do Q\nBT /F1 12 Tf 20 40 Td (Bottom) Tj ET\nq .5 0 0 .5 15 30 cm'),
  stream('BT /F1 12 Tf 1 0 0 1 40 220 Tm (Middle) Tj ET Q\nBT /F1 12 Tf 25 303 Td (Top) Tj ET'),
  stream('/Inner Do', '/Type /XObject /Subtype /Form /BBox [-200 -200 200 200] /Matrix [0 2 -2 0 40 5] /Resources << /XObject << /Inner 8 0 R >> >>'),
  stream('BT /F1 12 Tf 12 TL 1 0 0 1 10 5 Tm (Form) Tj T* (Continued) Tj ET',
    '/Type /XObject /Subtype /Form /BBox [0 -100 200 200] /Matrix [0 -1 1 0 24 0] /Resources << /Font << /F1 4 0 R >> >>'),
  '<< /Type /Pages /Parent 2 0 R /Kids [10 0 R] /Count 1 /Rotate -270 >>',
  '<< /Type /Page /Parent 9 0 R /Resources << /Font << /F1 4 0 R >> >> /Contents 11 0 R >>',
  stream('BT /F1 12 Tf 0 1 -1 0 120 50 Tm (Rotated lower) Tj ET\nBT /F1 12 Tf 0 1 -1 0 30 50 Tm (Rotated upper) Tj ET'),
];

let page = '%PDF-1.7\n';
const offsets = [];
for (const [index, object] of objects.entries()) {
  offsets.push(Buffer.byteLength(page));
  page += `${index + 1} 0 obj\n${object}\nendobj\n`;
}
const xref = Buffer.byteLength(page);
page += `xref\n0 ${objects.length + 1}\n0000000000 65535 f \n`;
for (const offset of offsets) page += `${String(offset).padStart(10, '0')} 00000 n \n`;
page += `trailer\n<< /Size ${objects.length + 1} /Root 1 0 R >>\nstartxref\n${xref}\n%%EOF\n`;

const lines = ['Top', 'Form', 'Continued', 'Middle', 'Bottom', 'Rotated upper', 'Rotated lower'];
const transformedReadingOrder = { page, text: lines.join('\n'), ranges: lines.map(line => [line, [`(${line})`]]) };

if (process.argv.includes('--write')) {
  const target = new URL('../../parity/fixtures/pdf-document.lino', import.meta.url);
  writeProjectData(target, { ...readProjectData(target), transformedReadingOrder });
} else process.stdout.write(page);
