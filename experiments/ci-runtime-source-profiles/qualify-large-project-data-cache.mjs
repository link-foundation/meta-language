import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { readBootstrapStructuredDataLino } from '../../scripts/lib/structured-data-bootstrap.mjs';
import { captureGitArtifactCertificate } from '../../js/scripts/capture-git-artifact-certificate.mjs';
import { readProfiledNamingSourceBindingDocument } from '../../js/scripts/naming-source-binding-codec-profile.mjs';
import { readOwnCodebaseHistoricalNamingPolicy } from '../../js/scripts/own-codebase-baseline-naming-history.mjs';
import { writeProjectData } from '../../js/scripts/project-data.mjs';

const root = new URL('../../', import.meta.url);
const revision = 'ec9f4efb0e9ef936114e09823981352f6db13700';
const sources = [
  {
    file: 'js/src/project-data-files.js',
    previous: 'large structured data snapshot with exact current text',
    phrase: 'large structured data register snapshot with exact current text',
    spelling: 'largeDataSnapshot',
    definition: 'The single private decoded snapshot of a large Links Notation data file. Reuse requires identical complete current text and each read returns an independent value.',
    sha256: '2667a331432dff246b217aaf847ce8934d531aa09f052f63a0c0500bdbbc4ad5',
  },
  {
    file: 'js/tests/project-data-large-read-cache.test.js',
    previous: 'check repeated large data reads and complete current source bytes',
    phrase: 'check cache reuse for repeated large data reads and complete current source bytes',
    spelling: 'repeated large Links reads retain exact independent values and fresh owners while checking all current bytes',
    definition: 'Check that repeated reads reuse the decoded snapshot of the complete naming register, return independent values and check every byte of the current file.',
    sha256: 'ae166e635b765554b000d0df2aecb4c719130739a1becdb00e49e94889a55a5d',
  },
];
const pin = value => ({ bytes: Buffer.byteLength(value), sha256: createHash('sha256').update(value).digest('hex') });
const conceptId = phrase => `repository.${phrase.replaceAll(' ', '-')}`;
const [mode] = process.argv.slice(2);
assert.ok(['--bindings', '--concepts', '--history'].includes(mode));

const current = sources.map(source => {
  const sourceContext = readFileSync(new URL(source.file, root), 'utf8');
  const { sha256 } = pin(sourceContext);
  assert.notEqual(sha256, source.sha256);
  return { ...source, sourceContext, currentDigest: sha256, name: `${source.spelling}::source-sha256:${sha256}` };
});

if (mode === '--bindings') {
  const bindings = readProfiledNamingSourceBindingDocument();
  for (const source of current) {
    const previous = bindings.find(row => row.id === conceptId(source.previous));
    assert.equal(previous.source, source.file);
    assert.equal(previous.sourceSha256, source.sha256);
    assert.equal(pin(previous.sourceContext).sha256, source.sha256);
    assert.equal(previous.actualSourceName, source.spelling);
    const { start, end } = previous.sourceRange;
    const prefix = previous.sourceContext.slice(0, start);
    const suffix = previous.sourceContext.slice(end);
    assert.equal(previous.sourceContext.slice(start, end), previous.sourceText);
    assert.ok(source.sourceContext.startsWith(prefix));
    let currentEnd = end;
    if (source.sourceContext.endsWith(suffix)) {
      currentEnd = source.sourceContext.length - suffix.length;
    } else {
      assert.equal(source.sourceContext.slice(start, end), previous.sourceText);
    }
    const id = conceptId(source.phrase);
    assert.ok(bindings.every(row => row.id !== id));
    bindings.push({
      ...previous,
      id,
      name: source.name,
      inventoryName: source.phrase,
      sourceText: source.sourceContext.slice(start, currentEnd),
      sourceContext: source.sourceContext,
      sourceRange: { start, end: currentEnd },
      sourceSha256: source.currentDigest,
    });
  }
  writeProjectData(new URL('parity/naming/source-bindings.lino', root), bindings);
} else if (mode === '--concepts') {
  const file = new URL('parity/naming/canonical-concepts.lino', root);
  const registry = readBootstrapStructuredDataLino(readFileSync(file, 'utf8'));
  for (const source of current) {
    const previous = registry.concepts.find(record => record.id === conceptId(source.previous));
    const id = conceptId(source.phrase);
    assert.ok(previous && registry.concepts.every(record => record.id !== id));
    registry.concepts.push({
      ...previous,
      id,
      phrase: source.phrase,
      definition: source.definition,
      sourceAliases: [{ source: source.file, name: source.name }],
    });
  }
  writeProjectData(file, registry);
} else {
  const history = readOwnCodebaseHistoricalNamingPolicy().additionalSourceViews;
  for (const source of current) {
    assert.ok(history.sources.every(row => row.file !== source.file));
    const { blob, certificate } = captureGitArtifactCertificate(revision, source.file);
    assert.equal(pin(blob).sha256, source.sha256);
    history.sources.push({
      file: source.file,
      predecessor: { revision, source: blob.toString('utf8'), ...pin(blob), certificate },
      current: pin(source.sourceContext),
    });
    history.views.push({
      inventory: `B70 physical field source view names for ${source.file}`,
      file: source.file,
      revision,
      names: [{ name: source.previous, record: conceptId(source.previous) }],
    });
  }
  writeProjectData(new URL('parity/naming/array-identity-historical-inventories.lino', root), history);
}
