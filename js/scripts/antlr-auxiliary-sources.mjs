// Authoring-only source capture. No external base is executed or treated as a
// grammar rule. This captures only listed files, not their transitive dependency
// closure. The importer and all unresolved semantic obligations are unchanged.
import {createHash} from 'node:crypto';
import {isDeepStrictEqual} from 'node:util';
import {downloadExternalGrammarAssets, readExternalGrammarAsset} from './external-grammar-cache.mjs';

const maximumSourceBytes = 65536;
const digest = bytes => createHash('sha256').update(bytes).digest('hex');
const safePath = value => typeof value === 'string' && value.length > 0 && !value.includes('\\')
  && !value.includes('\0') && value.split('/').every(part => part !== '' && part !== '.' && part !== '..');

function sourceRecords(language, files, registry) {
  const records = registry.auxiliarySources?.[language];
  if (records === undefined) return [];
  if (!/^https:\/\/github\.com\/[A-Za-z0-9_.-]+\/[A-Za-z0-9_.-]+$/u.test(registry.repository)
    || !/^[a-f0-9]{40}$/u.test(registry.revision)) throw new Error('invalid ANTLR auxiliary repository pin');
  if (!isDeepStrictEqual(files, registry.languages?.[language])) throw new Error(`${language}: ANTLR source file order differs from its registered pin`);
  if (!Array.isArray(records) || records.length === 0) throw new Error(`${language}: empty ANTLR auxiliary source registration`);
  const owners = new Set();
  const paths = new Set();
  for (const record of records) {
    if (!safePath(record.path) || !safePath(record.asset) || !safePath(record.grammarPath) || !files.includes(record.grammarPath)
      || owners.has(record.grammarPath) || paths.has(record.path)
      || !['lexer-base', 'parser-base'].includes(record.role) || !/^[A-Za-z_][A-Za-z_0-9]*$/u.test(record.className)
      || typeof record.sourceLanguage !== 'string' || record.sourceLanguage.trim().length === 0
      || !Number.isSafeInteger(record.maxBytes) || record.maxBytes <= 0 || record.maxBytes > maximumSourceBytes
      || !Number.isSafeInteger(record.bytes) || record.bytes <= 0 || record.bytes > record.maxBytes
      || !Number.isSafeInteger(record.grammarBytes) || record.grammarBytes <= 0 || record.grammarBytes > maximumSourceBytes
      || !/^[a-f0-9]{64}$/u.test(record.sha256) || !/^[a-f0-9]{64}$/u.test(record.grammarSha256)
      || !/^[a-f0-9]{40}$/u.test(record.gitBlobSha1)) throw new Error(`${language}: invalid ANTLR auxiliary source pin`);
    owners.add(record.grammarPath);
    paths.add(record.path);
  }
  if (owners.size !== files.length) throw new Error(`${language}: incomplete ANTLR auxiliary grammar owner inventory`);
  return records;
}

function verifySource(bytes, record, path) {
  if (!Buffer.isBuffer(bytes) || bytes.length > maximumSourceBytes || bytes.length !== record.bytes || digest(bytes) !== record.sha256) {
    throw new Error(`ANTLR pinned source bytes differ: ${path}`);
  }
  if (record.gitBlobSha1) {
    const blob = createHash('sha1').update(`blob ${bytes.length}\0`).update(bytes).digest('hex');
    if (blob !== record.gitBlobSha1) throw new Error(`ANTLR auxiliary Git blob differs: ${path}`);
  }
}

function readAntlrGrammarSourceInputs(language, files, texts, registry, records) {
  if (!Array.isArray(texts) || texts.length !== files.length || texts.some(text => typeof text !== 'string')) throw new Error(`${language}: ANTLR source text inventory differs`);
  const sourceId = path => `${registry.repository}@${registry.revision}/${path}`;
  let start = 0;
  return files.map((path, index) => {
    const record = records.find(value => value.grammarPath === path);
    if (Buffer.byteLength(texts[index], 'utf8') !== record.grammarBytes) throw new Error(`ANTLR pinned source bytes differ: ${path}`);
    const bytes = Buffer.from(texts[index], 'utf8');
    verifySource(bytes, {bytes: record.grammarBytes, sha256: record.grammarSha256}, path);
    const input = {sourceId: sourceId(path), path, bytes: bytes.length, sha256: digest(bytes), start, end: start + bytes.length};
    start = input.end + 1; // Exactly the existing texts.join('\n') byte layout.
    return input;
  });
}

/** Capture exact file identities and concatenation ranges; never rewrite G4 text. */
export function captureAntlrAuxiliarySources(language, files, texts, registry, readAsset = readExternalGrammarAsset) {
  const records = sourceRecords(language, files, registry);
  if (records.length === 0) return null;
  const grammarInputs = readAntlrGrammarSourceInputs(language, files, texts, registry, records);
  const sourceId = path => `${registry.repository}@${registry.revision}/${path}`;
  const sources = records.map(record => {
    let bytes;
    try { bytes = readAsset(record.asset); }
    catch (cause) { throw new Error(`ANTLR auxiliary source unavailable: ${sourceId(record.path)}`, {cause}); }
    verifySource(bytes, record, record.path);
    return {sourceId: sourceId(record.path), grammarSourceId: sourceId(record.grammarPath),
      path: record.path, asset: record.asset, role: record.role, className: record.className,
      sourceLanguage: record.sourceLanguage, bytes: bytes.length, sha256: digest(bytes), gitBlobSha1: record.gitBlobSha1};
  });
  return {schemaVersion: 1, kind: 'authenticated-antlr-auxiliary-source-capture', behavioralEvidence: false,
    repository: registry.repository, revision: registry.revision, language, spanUnit: 'utf8-bytes', grammarInputs, sources,
    unresolvedObligations: sources.flatMap(({sourceId}) => [
      {sourceId, category: 'transitive-dependencies', reason: 'Imports and reachable helper dependencies have not been analyzed or authenticated by this capture.'},
      {sourceId, category: 'base-behavior', reason: 'No lexer or parser base method, predicate, action or hook has been translated by acquisition.'},
    ]),
    boundary: 'Listed-file acquisition only; not a complete source closure. Superclasses and all executable grammar obligations remain unresolved until separately translated and checked.'};
}

/** Reuse the existing size-bounded, checksummed, ignored external asset cache. */
export async function acquireAntlrAuxiliarySources(language, files, texts, registry, options = {}) {
  const records = sourceRecords(language, files, registry);
  if (records.length === 0) return null;
  readAntlrGrammarSourceInputs(language, files, texts, registry, records);
  try {
    await downloadExternalGrammarAssets({...options, assets: records.map(record => record.asset), groups: [], all: false, jobs: 1, materialize: false});
  } catch (cause) {
    throw new Error(`${language}: authenticated ANTLR auxiliary acquisition failed: ${cause.message}`, {cause});
  }
  return captureAntlrAuxiliarySources(language, files, texts, registry, path => readExternalGrammarAsset(path, options));
}
