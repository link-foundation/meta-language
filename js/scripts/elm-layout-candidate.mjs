// Author one native Elm candidate from immutable source bytes and the shared
// section-layout family. Promotion is a separate catalog change after corpus CI.
import { createHash } from 'node:crypto';
import { gunzipSync } from 'node:zlib';
import { nativeName } from './import-native-grammars.mjs';
import { sectionLayoutScanner } from './section-layout-scanner.mjs';
import { importTreeSitterNative, renderTreeSitterNative } from '../src/grammar-importers/tree-sitter-native.js';
import { parseGrammarLinks } from '../src/grammar-links.js';
import { readExternalGrammarAsset } from './external-grammar-cache.mjs';
import { readProjectData } from './project-data.mjs';

const root = new URL('../../', import.meta.url);
const SOURCE = 'parity/grammars/candidates/elm/source.json';

/** Read one pinned candidate source, including its exact byte-length contract. */
export function elmCandidateSource(path) {
  const register = readProjectData(new URL(SOURCE, root));
  const entry = register.files.find((file) => file.path === path);
  if (!entry) throw new Error(`Elm candidate does not pin ${path}`);
  const bytes = gunzipSync(readExternalGrammarAsset(entry.file));
  const digest = createHash('sha256').update(bytes).digest('hex');
  if (digest !== entry.sha256 || bytes.length !== entry.bytes) throw new Error(`Elm source ${path} differs from its immutable record`);
  return bytes.toString('utf8');
}

/** The full candidate grammar and its scanner, produced by authoring tools. */
export function elmLayoutCandidate() {
  const source = readProjectData(new URL(SOURCE, root));
  const grammarSource = JSON.parse(elmCandidateSource('src/grammar.json'));
  const scannerSource = elmCandidateSource('src/scanner.c');
  const naming = readProjectData(new URL('parity/naming/grammar-name-expansions.json', root));
  const expansions = new Map(naming.words.map(({ word, replacement }) => [word, replacement]));
  const nameOf = (name) => nativeName(name, expansions, naming.grammars.elm ?? {});
  const externals = grammarSource.externals.map(({ name }) => name);
  const expected = ['_virtual_end_decl', '_virtual_open_section', '_virtual_end_section',
    'minus_without_trailing_whitespace', 'glsl_content', '_block_comment_content', '_string_content_multiline'];
  if (JSON.stringify(externals) !== JSON.stringify(expected)) throw new Error('Elm scanner token inventory changed; review the complete profile');
  const maximumDepth = Number(/#define MAX_INDENT_DEPTH (\d+)/u.exec(scannerSource)?.[1]);
  if (!/uint8_t \*data;/u.test(scannerSource)) throw new Error('Elm indentation integer representation changed');
  const descriptor = {
    name: 'elm_sections',
    endDeclarationToken: nameOf(expected[0]), openSectionToken: nameOf(expected[1]),
    endSectionToken: nameOf(expected[2]), minusToken: nameOf(expected[3]),
    shaderContentToken: nameOf(expected[4]), commentContentToken: nameOf(expected[5]),
    stringContentToken: nameOf(expected[6]), maximumDepth, indentationModulo: 256,
    sectionEndWord: 'in', sectionClosers: [')', ',', '}'],
    commentOpening: '{-', commentClosing: '-}', lineCommentOpening: '--',
    multilineClosing: '"""', stringEscape: '\\', shaderClosing: '|]',
    minusFollowing: '[A-Za-z(\\u0080-\\u{10ffff}]',
  };
  const scanner = sectionLayoutScanner(descriptor);
  const imported = importTreeSitterNative(grammarSource, {
    nameOf, scanners: scanner,
    immediate: [expected[0], expected[1], expected[2], expected[5], expected[6]],
  });
  const text = renderTreeSitterNative(imported);
  const grammar = parseGrammarLinks(text);
  const oracleKinds = Object.fromEntries([...imported.rules, ...imported.kinds]
    .filter(({ sourceName }) => sourceName !== null && sourceName !== undefined)
    .map(({ name, sourceName }) => [name, sourceName]));
  const projection = {
    hidden: expected.slice(0, 3).map(nameOf), anonymous: ['unnamed_token'],
    extras: grammarSource.extras.filter(({ type }) => type === 'SYMBOL').map(({ name }) => nameOf(name)),
    oracleKinds,
  };
  return { source, descriptor, scanner, imported, text, grammar, projection };
}
