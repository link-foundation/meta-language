// Author metadata only. External implementation bytes remain in the ignored
// authenticated cache; source acquisition does not discharge semantic obligations.
import {isDeepStrictEqual} from 'node:util';
import {fileURLToPath} from 'node:url';
import {resolve} from 'node:path';
import {readProjectData, writeProjectData, checkProjectData} from './project-data.mjs';

export const ANTLR_AUXILIARY_SOURCE_REVISION = '7df52be94698550d219d299d04105c6bafadd9c3';
const repository = 'https://github.com/antlr/grammars-v4';
const registryFile = new URL('../../parity/grammars-v4-sources.lino', import.meta.url);
const manifestFile = new URL('../../parity/grammars/external-assets.lino', import.meta.url);

// Actual GitHub Contents responses at the immutable registered revision were
// checked against their Git blob identities and exact decoded byte lengths.
// These audit observations are proposed acquisition pins, not translated methods
// or behavioral evidence. They become registered only after this author runs
// and the generated metadata changes are reviewed; no current registration is implied.
const pins = [
  {language: 'JavaScript', grammarPath: 'javascript/javascript/JavaScriptLexer.g4',
    grammarBytes: 9365, grammarSha256: 'bddbd846633b6d6ef2b5de205cd3a639a72d3f01ec3254c720e10f9a96099278',
    path: 'javascript/javascript/JavaScript/JavaScriptLexerBase.js', role: 'lexer-base', className: 'JavaScriptLexerBase', sourceLanguage: 'JavaScript',
    bytes: 3522, sha256: '6dde1a2a0f15aab1e5ab52892eb08aca9e67c4aaf6f5ad21dfb823471c8fd5ad', gitBlobSha1: '28067bcb741c73acff5caded8ab334dc4ad2a98b'},
  {language: 'JavaScript', grammarPath: 'javascript/javascript/JavaScriptParser.g4',
    grammarBytes: 16369, grammarSha256: 'c50664be00277d48c8a06d1ceabe4f6d46a04284a5e17eb287628727a95efca4',
    path: 'javascript/javascript/JavaScript/JavaScriptParserBase.js', role: 'parser-base', className: 'JavaScriptParserBase', sourceLanguage: 'JavaScript',
    bytes: 1942, sha256: 'a31c866e7363e4b8d5f05c2a52143dbc2e5328361b763418a249a54e0e578fc6', gitBlobSha1: '154f2e5c8b676a67d58a3cb6edba85a298e6ce30'},
  {language: 'TypeScript', grammarPath: 'javascript/typescript/TypeScriptLexer.g4',
    grammarBytes: 9570, grammarSha256: '479e04f19f314973d4a85d8f75748ff00887f0feb51414cf01fd7b216e8cebbc',
    path: 'javascript/typescript/TypeScript/TypeScriptLexerBase.ts', role: 'lexer-base', className: 'TypeScriptLexerBase', sourceLanguage: 'TypeScript',
    bytes: 5220, sha256: '92812b2b51589a641518a6401c7c9a6c2702dd89ed55394191ed757709589028', gitBlobSha1: 'da5e1b29b4014601aaab84f70a7a246351efb1b5'},
  {language: 'TypeScript', grammarPath: 'javascript/typescript/TypeScriptParser.g4',
    grammarBytes: 25372, grammarSha256: '774f93d81d13ca89fb78fc717c6db3e8a7e35b06962df0010169370403e67645',
    path: 'javascript/typescript/TypeScript/TypeScriptParserBase.ts', role: 'parser-base', className: 'TypeScriptParserBase', sourceLanguage: 'TypeScript',
    bytes: 4144, sha256: '24a2c203d773902b63b7e7a1421c2757bf24e3bdf0f14420793be36ae7ace977', gitBlobSha1: '0d85c525e3fb3989de628dd40ba7fc9475af58c4'},
  {language: 'Rust', grammarPath: 'rust/RustLexer.g4',
    grammarBytes: 7514, grammarSha256: '22a0e797c642a9f8a920fd5e7b6eb86a0176a7968906adc0553b1997a041857c',
    path: 'rust/TypeScript/RustLexerBase.ts', role: 'lexer-base', className: 'RustLexerBase', sourceLanguage: 'TypeScript',
    bytes: 2986, sha256: '42efd3abced9dd4ff85b19980f6bd4960c712d0ec751a79cd0e2898542add6bf', gitBlobSha1: '8ca78fa5393ce4a310c1a9efd3fa78d8610d3f33'},
  {language: 'Rust', grammarPath: 'rust/RustParser.g4',
    grammarBytes: 26661, grammarSha256: '2addda9f60ebf9b8887e962328ee8a12c1dc945a58bce0f1de7034c00be7aae1',
    path: 'rust/TypeScript/RustParserBase.ts', role: 'parser-base', className: 'RustParserBase', sourceLanguage: 'TypeScript',
    bytes: 385, sha256: '13672b05734f048d389d753d8a4c39051566c4562e5ce7aa7181fb4b286c5846', gitBlobSha1: '42b1f77aaa09f719f3f6cbe4b9e394dead00496a'},
];

/** Append exact acquisition metadata without replacing an existing pin or field. */
export function authorAntlrAuxiliarySourceMetadata(registry, manifest) {
  if (registry.repository !== repository || registry.revision !== ANTLR_AUXILIARY_SOURCE_REVISION) {
    throw new Error('ANTLR auxiliary source pins require review after the repository or revision changes');
  }
  if (manifest.schemaVersion !== 1 || !manifest.assets || Array.isArray(manifest.assets)) throw new Error('invalid external asset manifest');
  const auxiliarySources = {...registry.auxiliarySources};
  const assets = {...manifest.assets};
  const languages = new Map();
  for (const {language, ...pin} of pins) {
    if (!registry.languages[language]?.includes(pin.grammarPath)) throw new Error(`ANTLR auxiliary owner is not registered: ${pin.grammarPath}`);
    const asset = `parity/grammars/sources/antlr/${registry.revision}/${pin.path}`;
    const descriptor = {...pin, asset, maxBytes: 65536};
    if (!languages.has(language)) languages.set(language, []);
    languages.get(language).push(descriptor);
    const value = {group: 'sources', bytes: pin.bytes, sha256: pin.sha256, encoding: 'identity',
      downloads: [{url: `https://raw.githubusercontent.com/antlr/grammars-v4/${registry.revision}/${pin.path}`, transform: 'identity'}],
      upstream: {repository, revision: registry.revision, path: pin.path, gitBlobSha1: pin.gitBlobSha1}};
    if (Object.hasOwn(assets, asset) && !isDeepStrictEqual(assets[asset], value)) throw new Error(`existing ANTLR auxiliary asset pin differs: ${asset}`);
    if (!Object.hasOwn(assets, asset)) assets[asset] = value;
  }
  for (const [language, descriptors] of languages) {
    if (Object.hasOwn(auxiliarySources, language) && !isDeepStrictEqual(auxiliarySources[language], descriptors)) {
      throw new Error(`existing ANTLR auxiliary source registration differs: ${language}`);
    }
    if (!Object.hasOwn(auxiliarySources, language)) auxiliarySources[language] = descriptors;
  }
  return {registry: {...registry, auxiliarySources}, manifest: {...manifest, assets}};
}

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  if (process.argv.slice(2).some(value => value !== '--check')) throw new Error('usage: author-antlr-auxiliary-sources.mjs [--check]');
  const result = authorAntlrAuxiliarySourceMetadata(readProjectData(registryFile), readProjectData(manifestFile));
  const write = process.argv.includes('--check') ? checkProjectData : writeProjectData;
  write(registryFile, result.registry);
  write(manifestFile, result.manifest);
}
