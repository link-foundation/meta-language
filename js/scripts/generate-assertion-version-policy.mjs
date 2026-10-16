import { fileURLToPath } from 'node:url';
import { checkProjectData, writeProjectData } from './project-data.mjs';

export function assertionVersionPolicy() {
  return {
    schemaVersion: 1,
    assertions: [{
      id: 'unicode-identifier-property-generation',
      subject: 'Unicode',
      comparison: { kind: 'release', precision: 2 },
      expectations: [{
        id: 'unicode-17-identifier-properties',
        minimum: '17.0', maximum: '17.1', includeMinimum: true, includeMaximum: false,
        expected: '17.0',
        evidence: 'bbb13433fbdd3db3df8a5831eed8b9a6e5b83be1:js/scripts/build-unicode-identifier-properties.mjs requires process.versions.unicode === 17.0; original source sha256 58bb5fe590aed45a4b6d5e68e34e1120d8d7b7fcb7b74907e09f9d3ebde56d78; existing js/src/data/unicode-identifier-properties.lino records Unicode 17.0',
      }],
    }],
  };
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  const file = new URL('../../parity/assertion-version-ranges.lino', import.meta.url);
  if (process.argv.includes('--check')) checkProjectData(file, assertionVersionPolicy());
  else writeProjectData(file, assertionVersionPolicy());
  console.log('Assertion version ranges: explicit active-version expectations recorded as Links');
}
