import assert from 'node:assert/strict';
import { deflateSync } from 'node:zlib';
import { LinkNetwork } from '../../js/src/index.js';
import { readProjectData, writeProjectData } from '../../js/scripts/project-data.mjs';

function document(objects) {
  let source = '%PDF-1.7\n';
  const streams = [];
  for (const [index, object] of objects.entries()) {
    source += `${index + 1} 0 obj\n`;
    if (typeof object === 'string') {
      source += object + '\n';
    } else {
      const { content, dictionary = '', exposed = true } = object;
      const encoded = deflateSync(content);
      source += `<< /Length ${encoded.length} /Filter /FlateDecode ${dictionary}>>\nstream\n`;
      if (exposed) streams.push({ content, start: source.length, end: source.length + encoded.length });
      source += encoded.toString('latin1') + '\nendstream\n';
    }
    source += 'endobj\n';
  }
  source += '%%EOF\n';
  return { encoded: Buffer.from(source, 'latin1').toString('hex'), streams };
}

const cases = [
  document([
    '<< /Type /Catalog /Pages 2 0 R >>',
    '<< /Type /Pages /Kids [3 0 R] /Count 1 >>',
    '<< /Type /Page /Parent 2 0 R /MediaBox [0 0 100 100] /Contents [4 0 R 5 0 R 6 0 R] >>',
    { content: '% Intentionally blank page content\n' },
    { content: '' },
    { content: 'BI /W 1 /H 1 /BPC 8 /CS /G ID x EI\n' },
    { content: '% Unreferenced stream\n', exposed: false },
  ]),
  document([
    { content: '% Intentionally blank form\n', dictionary: '/Type /XObject /Subtype /Form /BBox [0 0 100 100] ' },
    { content: '% Unclassified stream\n', exposed: false },
  ]),
];

if (process.argv.includes('--write')) {
  const fixtureFile = new URL('../../parity/fixtures/pdf-document.lino', import.meta.url);
  const fixture = readProjectData(fixtureFile);
  fixture.declaredContent = cases;
  writeProjectData(fixtureFile, fixture);
} else {
  for (const { encoded, streams } of cases) {
    const bytes = Buffer.from(encoded, 'hex');
    const network = LinkNetwork.parseBytes(bytes, 'PDF');
    assert.equal(network.verifyFullMatch().isClean(), true);
    assert.deepEqual(network.derivedDocuments().map(stream => ({
      content: stream.network.reconstructText(), start: stream.sourceRange.start, end: stream.sourceRange.end,
    })), streams);
  }
}
