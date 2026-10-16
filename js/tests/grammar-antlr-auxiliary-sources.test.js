import assert from 'node:assert/strict';
import test from 'node:test';
import {createHash} from 'node:crypto';
import {existsSync, mkdtempSync, readFileSync, rmSync, writeFileSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join} from 'node:path';
import {readProjectData} from '../scripts/project-data.mjs';
import {authorAntlrAuxiliarySourceMetadata, ANTLR_AUXILIARY_SOURCE_REVISION} from '../scripts/author-antlr-auxiliary-sources.mjs';
import {acquireAntlrAuxiliarySources, captureAntlrAuxiliarySources} from '../scripts/antlr-auxiliary-sources.mjs';
import {importAntlr} from '../src/grammar-importers/antlr.js';
import {tokenizeAntlr} from '../src/grammar-importers/antlr-lexer.js';
import {compileGrammar} from '../src/grammar.js';
import {parseGrammarLinks, renderGrammarLinks} from '../src/grammar-links.js';
import {readStructuredDataLino, structuredDataToLino} from '../src/structured-data-links.js';
import {bulkLanguageRow} from '../scripts/run-grammar-bulk-pipeline.mjs';

const hash = bytes => createHash('sha256').update(bytes).digest('hex');
function fixture() {
  const files = ['fixture/Parser.g4', 'fixture/Lexer.g4'];
  const texts = ["/* π */ parser grammar Parser; options { tokenVocab=Lexer; superClass=ParserBase; } start: A {guard()}? EOF;",
    "lexer grammar Lexer; options { superClass=LexerBase; } A: 'a' {effect();};"];
  const assets = new Map();
  const records = files.map((grammarPath, index) => {
    const path = `fixture/${index === 0 ? 'Parser' : 'Lexer'}Base.js`;
    const bytes = Buffer.from(`// authored fixture base ${index}\n`);
    const asset = `parity/grammars/sources/fixture/${path}`;
    assets.set(asset, bytes);
    return {path, asset, grammarPath, grammarBytes: Buffer.byteLength(texts[index]), grammarSha256: hash(texts[index]),
      role: index === 0 ? 'parser-base' : 'lexer-base', className: index === 0 ? 'ParserBase' : 'LexerBase', sourceLanguage: 'JavaScript',
      bytes: bytes.length, sha256: hash(bytes), gitBlobSha1: createHash('sha1').update(`blob ${bytes.length}\0`).update(bytes).digest('hex'), maxBytes: 65536};
  });
  const registry = {repository: 'https://github.com/example/grammars', revision: '1'.repeat(40), languages: {Example: files}, auxiliarySources: {Example: records}};
  const readAsset = path => { if (!assets.has(path)) throw new Error('missing asset'); return assets.get(path); };
  return {files, texts, assets, registry, readAsset};
}

test('ANTLR auxiliary metadata author preserves every existing registry and manifest entry', () => {
  const registry = readProjectData(new URL('../../parity/grammars-v4-sources.lino', import.meta.url));
  const manifest = {schemaVersion: 1, assets: {old: {bytes: 1, sha256: 'a'.repeat(64)}}, oldField: ['preserved']};
  const before = structuredClone(registry);
  const result = authorAntlrAuxiliarySourceMetadata(registry, manifest);
  assert.deepEqual(registry, before);
  for (const key of Object.keys(registry)) if (key !== 'auxiliarySources') assert.deepEqual(result.registry[key], registry[key]);
  assert.deepEqual(result.manifest.assets.old, manifest.assets.old);
  assert.equal(result.manifest.assets.old, manifest.assets.old, 'all original asset fields and their identity are retained');
  assert.deepEqual(result.manifest.oldField, manifest.oldField);
  assert.equal(Object.keys(result.manifest.assets).length, 7);
  const repeated = authorAntlrAuxiliarySourceMetadata(result.registry, result.manifest);
  assert.deepEqual(repeated, result);
  for (const [path, entry] of Object.entries(result.manifest.assets)) assert.equal(repeated.manifest.assets[path], entry);
  for (const language of Object.keys(result.registry.auxiliarySources)) assert.equal(repeated.registry.auxiliarySources[language], result.registry.auxiliarySources[language]);
  assert.equal(result.registry.revision, ANTLR_AUXILIARY_SOURCE_REVISION);
  for (const language of ['JavaScript', 'TypeScript', 'Rust']) {
    assert.equal(result.registry.auxiliarySources[language].length, 2);
    for (const source of result.registry.auxiliarySources[language]) {
      assert.ok(registry.languages[language].includes(source.grammarPath));
      assert.equal(result.manifest.assets[source.asset].sha256, source.sha256);
      assert.ok(result.manifest.assets[source.asset].downloads[0].url.includes(`/${registry.revision}/`));
      assert.ok(source.bytes <= source.maxBytes);
    }
  }
  assert.throws(() => authorAntlrAuxiliarySourceMetadata({...registry, revision: '2'.repeat(40)}, manifest), /require review/u);
  const changed = structuredClone(result.manifest);
  changed.assets[result.registry.auxiliarySources.Rust[0].asset].sha256 = '0'.repeat(64);
  const beforeChanged = structuredClone(changed);
  assert.throws(() => authorAntlrAuxiliarySourceMetadata(registry, changed), /existing.*pin differs/u);
  assert.deepEqual(changed, beforeChanged, 'a conflicting descriptor is never overwritten');
  const annotated = structuredClone(result.manifest);
  annotated.assets[result.registry.auxiliarySources.Rust[0].asset].auditAnnotation = {retained: ['all original fields']};
  const beforeAnnotated = structuredClone(annotated);
  assert.throws(() => authorAntlrAuxiliarySourceMetadata(registry, annotated), /existing.*pin differs/u);
  assert.deepEqual(annotated, beforeAnnotated, 'extra existing descriptor fields cause review, never silent replacement');
  const conflictingOwners = structuredClone(result.registry);
  conflictingOwners.auxiliarySources.Rust[0].className = 'DifferentBase';
  const beforeOwners = structuredClone(conflictingOwners);
  assert.throws(() => authorAntlrAuxiliarySourceMetadata(conflictingOwners, result.manifest), /existing.*registration differs/u);
  assert.deepEqual(conflictingOwners, beforeOwners);
});

test('authenticated auxiliary capture retains byte positions, source-qualified owners and all semantic refusals', async () => {
  const {files, texts, registry, readAsset} = fixture();
  const original = texts.join('\n');
  const tokens = tokenizeAntlr(original);
  const grammar = importAntlr(original);
  const links = renderGrammarLinks(grammar);
  const capture = captureAntlrAuxiliarySources('Example', files, texts, registry, readAsset);
  assert.equal(capture.behavioralEvidence, false);
  assert.equal(capture.grammarInputs[0].start, 0);
  assert.equal(capture.grammarInputs[0].end, Buffer.byteLength(texts[0]));
  assert.equal(capture.grammarInputs[1].start, Buffer.byteLength(texts[0]) + 1);
  for (let index = 0; index < files.length; index++) {
    const input = capture.grammarInputs[index];
    assert.equal(Buffer.from(original).subarray(input.start, input.end).toString('utf8'), texts[index]);
    assert.equal(capture.sources[index].grammarSourceId, input.sourceId);
    assert.ok(input.sourceId.includes(`${registry.revision}/${files[index]}`));
  }
  assert.equal(new Set(capture.sources.map(value => value.sourceId)).size, 2);
  assert.deepEqual(readStructuredDataLino(structuredDataToLino(capture)), capture);
  assert.deepEqual(tokenizeAntlr(texts.join('\n')), tokens);
  assert.equal(renderGrammarLinks(grammar), links);
  for (const candidate of [grammar, parseGrammarLinks(links)]) {
    assert.throws(() => compileGrammar(candidate), /unresolved option:.*superClass=LexerBase/u);
    const obligations = candidate.declarations.lexers[0].expression.items.filter(value => value.kind === 'lexicalObligation');
    assert.ok(obligations.some(value => value.category === 'action' && value.source === 'effect();'));
    assert.ok(obligations.some(value => value.category === 'predicate' && value.source === 'guard()'));
  }
  const row = await bulkLanguageRow({name: 'Example', grammars: [], source: 'a'}, () => {}, {
    loadGrammarsV4: async () => ({files, text: original, entryPoint: null, capturedAuxiliarySources: capture}), loadAdditionalSources: () => [],
  });
  assert.deepEqual(row.grammarsV4.capturedAuxiliarySources, capture);
  assert.match(row.grammarsV4.error, /unresolved option/u);
  assert.equal(Object.hasOwn(row.grammarsV4, 'compiled'), false);
  assert.equal(Object.hasOwn(row.grammarsV4, 'sample'), false);
});

test('auxiliary capture refuses wrong sources, hash or blob pins, missing files, changed owners and excessive bounds', () => {
  const {files, texts, registry, readAsset} = fixture();
  const capture = (value = registry, read = readAsset, input = texts, order = files) => captureAntlrAuxiliarySources('Example', order, input, value, read);
  assert.throws(() => capture(registry, () => { throw new Error('not present'); }), /auxiliary source unavailable:.*@111/u);
  assert.throws(() => capture(registry, () => Buffer.from('other source')), /pinned source bytes differ/u);
  assert.throws(() => capture(registry, () => Buffer.alloc(65537)), /pinned source bytes differ/u);
  assert.throws(() => capture(registry, readAsset, [texts[0] + ' ', texts[1]]), /pinned source bytes differ/u);
  assert.throws(() => capture(registry, readAsset, [...texts].reverse(), [...files].reverse()), /file order differs/u);
  for (const [field, value] of [['sha256', '0'.repeat(64)], ['gitBlobSha1', '0'.repeat(40)], ['bytes', 65537], ['maxBytes', 65537], ['grammarPath', 'other/Parser.g4'],
    ['sourceLanguage', undefined], ['sourceLanguage', ''], ['sourceLanguage', ' \t'], ['grammarPath', '../Parser.g4']]) {
    const changed = structuredClone(registry);
    if (value === undefined) delete changed.auxiliarySources.Example[0][field];
    else changed.auxiliarySources.Example[0][field] = value;
    if (value === '../Parser.g4') changed.languages.Example[0] = value;
    assert.throws(() => capture(changed, readAsset, texts, changed.languages.Example), /pin|bytes differ|Git blob differs/u, field);
  }
  const other = {...registry, repository: 'https://github.com/other/grammars'};
  assert.notEqual(capture(other).sources[0].sourceId, capture().sources[0].sourceId);
  assert.equal(captureAntlrAuxiliarySources('Unregistered', [], [], registry, () => { throw new Error('must not read'); }), null);
});

test('captured auxiliary imports retain unresolved dependency and base behavior obligations', () => {
  const {files, texts, registry, assets} = fixture();
  const record = registry.auxiliarySources.Example[1];
  const bytes = Buffer.from("import antlr4 from 'antlr4';\nimport Lexer from './GeneratedLexer.js';\nexport default class LexerBase extends antlr4.Lexer {}\n");
  Object.assign(record, {bytes: bytes.length, sha256: hash(bytes),
    gitBlobSha1: createHash('sha1').update(`blob ${bytes.length}\0`).update(bytes).digest('hex')});
  assets.set(record.asset, bytes);
  const reads = [];
  const capture = captureAntlrAuxiliarySources('Example', files, texts, registry, asset => {
    reads.push(asset);
    if (!assets.has(asset)) throw new Error('unregistered transitive dependency');
    return assets.get(asset);
  });
  assert.deepEqual(reads, registry.auxiliarySources.Example.map(value => value.asset));
  const base = capture.sources.find(value => value.path === record.path);
  assert.deepEqual(capture.unresolvedObligations.filter(value => value.sourceId === base.sourceId).map(value => value.category),
    ['transitive-dependencies', 'base-behavior']);
  const replay = readStructuredDataLino(structuredDataToLino(capture));
  assert.deepEqual(replay.unresolvedObligations, capture.unresolvedObligations);
  assert.equal(replay.behavioralEvidence, false);
  const grammar = importAntlr(texts.join('\n'));
  const before = renderGrammarLinks(grammar);
  assert.throws(() => compileGrammar(grammar), /unresolved option:.*superClass=LexerBase/u);
  assert.equal(renderGrammarLinks(grammar), before);
});

test('auxiliary acquisition selects two assets serially and authenticates cached and downloaded bytes', async () => {
  const {files, texts, registry, assets} = fixture();
  const root = mkdtempSync(join(tmpdir(), 'meta-language-antlr-auxiliary-'));
  try {
    const records = registry.auxiliarySources.Example;
    const unrelated = 'parity/grammars/sources/fixture/unrelated.wasm';
    const manifestAssets = {[unrelated]: {group: 'wasm', bytes: 1, sha256: hash(Buffer.from('x')),
      encoding: 'identity', downloads: [{url: 'https://assets.example.test/unrelated', transform: 'identity'}]}};
    const urls = new Map();
    for (const record of records) {
      const url = `https://assets.example.test/${record.path}`;
      urls.set(url, record.asset);
      manifestAssets[record.asset] = {group: 'sources', bytes: record.bytes, sha256: record.sha256,
        encoding: 'identity', downloads: [{url, transform: 'identity'}]};
    }
    const options = {root, cacheRoot: join(root, 'cache'), manifest: {schemaVersion: 1, assets: manifestAssets},
      assets: [unrelated], groups: ['wasm', 'sources'], all: true, jobs: 16, materialize: true};
    const requests = [];
    let active = 0, maximumActive = 0;
    const capture = await acquireAntlrAuxiliarySources('Example', files, texts, registry, {...options,
      fetchImpl: async url => {
        assert.ok(urls.has(url), `only selected auxiliary sources are fetched: ${url}`);
        if (requests.length > 0) assert.deepEqual(readFileSync(join(options.cacheRoot, records[0].asset)), assets.get(records[0].asset), 'the first asset is stored before the second starts');
        requests.push(url);
        active++;
        maximumActive = Math.max(maximumActive, active);
        try { await Promise.resolve(); return new Response(assets.get(urls.get(url))); }
        finally { active--; }
      },
    });
    assert.deepEqual(requests, [...urls.keys()]);
    assert.equal(maximumActive, 1);
    for (const record of records) {
      assert.deepEqual(readFileSync(join(options.cacheRoot, record.asset)), assets.get(record.asset));
      assert.equal(existsSync(join(root, record.asset)), false, 'caller materialization is not inherited');
    }
    assert.equal(existsSync(join(options.cacheRoot, unrelated)), false);
    assert.deepEqual(readStructuredDataLino(structuredDataToLino(capture)), capture);
    let unexpectedRequests = 0;
    const offline = {...options, fetchImpl: () => { unexpectedRequests++; throw new Error('no download may occur'); }};
    assert.deepEqual(await acquireAntlrAuxiliarySources('Example', files, texts, registry, offline), capture);
    assert.equal(unexpectedRequests, 0);
    const counterfeit = Buffer.alloc(records[0].bytes, 120);
    writeFileSync(join(options.cacheRoot, records[0].asset), counterfeit);
    await assert.rejects(acquireAntlrAuxiliarySources('Example', files, texts, registry, offline), /acquisition failed:.*checksum mismatch/u);
    assert.equal(unexpectedRequests, 0);
    assert.deepEqual(readFileSync(join(options.cacheRoot, records[0].asset)), counterfeit, 'invalid cached source is not overwritten');
    for (const outcome of ['wrong', 'missing']) {
      const cacheRoot = join(root, outcome);
      await assert.rejects(acquireAntlrAuxiliarySources('Example', files, texts, registry, {...options, cacheRoot,
        fetchImpl: async url => {
          assert.ok(urls.has(url));
          return outcome === 'missing' ? new Response('', {status: 404})
            : new Response(Buffer.alloc(assets.get(urls.get(url)).length, 120));
        },
      }), outcome === 'missing' ? /acquisition failed:.*HTTP 404/u : /acquisition failed:.*checksum mismatch/u);
      for (const record of records) assert.equal(existsSync(join(cacheRoot, record.asset)), false);
    }
    await assert.rejects(acquireAntlrAuxiliarySources('Example', files, [texts[0] + ' ', texts[1]], registry, {...offline,
      cacheRoot: join(root, 'invalid-primary'),
    }), /pinned source bytes differ: fixture\/Parser.g4/u);
    assert.equal(unexpectedRequests, 0, 'wrong primary source is refused before any download');
    assert.equal(existsSync(join(root, 'invalid-primary')), false);
    await assert.rejects(acquireAntlrAuxiliarySources('Example', files, ['x'.repeat(65537), texts[1]], registry, {...offline,
      cacheRoot: join(root, 'oversized-primary'),
    }), /pinned source bytes differ: fixture\/Parser.g4/u);
    assert.equal(unexpectedRequests, 0, 'oversized primary source is refused before any download');
    assert.equal(existsSync(join(root, 'oversized-primary')), false);
  } finally {
    rmSync(root, {recursive: true, force: true});
  }
});
