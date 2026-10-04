// Writes parity/fixtures/native-recovery.json: every conformance and generative case of a
// language that parses with its native grammar whose public tree differs from the
// tree-sitter CLI oracle because the native executor recovers from malformed input
// differently, with the digests and repair sites of both trees and a justified category
// (tests/support/native-recovery.js). A differing case whose oracle or native tree is clean
// is no recovery discrepancy: it is reported, not recorded, and the script fails.
//   node scripts/generate-native-recovery.mjs [--check] [LANGUAGE...]
import { readFileSync, writeFileSync } from 'node:fs';

import { LinkNetwork } from '../src/index.js';
import { LANGUAGE_CATALOG } from '../src/language-catalog.js';
import { isNativeGrammar } from '../src/native-grammar-parser.js';
import {
  documentGrammarRoots,
  regionGrammarRoots,
  renderCstLines,
} from '../tests/support/cst-lines.js';
import { parseCorpus } from '../tests/support/cst-sexpression.js';
import { applyEdit } from '../tests/support/generative.js';
import { NATIVE_RECOVERY_FILE, isMalformed, readNativeRecovery, recoveryRecord } from '../tests/support/native-recovery.js';

const CATEGORIES = {
  'same-repair-sites':
    'Both trees skip the same spans as ERROR and insert MISSING leaves at the same points, but differ in what they hold: an ERROR node of tree-sitter holds the tokens its error-mode lexer read in the skipped span, the native ERROR is one leaf over the skipped bytes; tree-sitter inserts the missing token with the lowest symbol id its parse state accepts, the native executor the element its grammar expects first at the repair point, and the nodes around a repair follow from that choice.',
  'oracle-skips-more':
    "tree-sitter covers more of the input with ERROR. Its recovery works on the LR stack: where no state accepts the next token it skips tokens or, at the end of the input above all, reduces what the stack holds into an ERROR node. The native executor repairs only where an element failed farthest, with the cheaper of MISSING leaves (cost 2 each) and skipped bytes (cost 1 each), so it often completes a construct with MISSING leaves, the elements its grammar expects there, where tree-sitter skips the tokens, and the input it keeps stays in the grammar's nodes.",
  'native-skips-more':
    "The native executor covers more of the input with ERROR. It repairs where an element failed farthest, one repair point per round, with the cheaper of a MISSING leaf (cost 2) and a skip to the first later offset where the failing element matches (cost 1 per byte). So it may skip a short stray token that tree-sitter, whose costs (110 per missing tree against 100 per skipped tree, 1 per byte and 30 per line) and parse states differ, keeps in a node with MISSING leaves, or skip up to where the element matches again a region in which tree-sitter, which can also pop its LR stack, makes several smaller repairs.",
  'same-skipped-bytes':
    'Both trees cover as many bytes with ERROR, but place or nest the ERROR nodes or the MISSING leaves differently: tree-sitter hoists skipped tokens into an ERROR node at the level of its LR stack, often as a sibling of the next node, and may wrap an ERROR node in another over the same span, while the native executor keeps one ERROR leaf in the rule whose element skipped it.',
};

const args = process.argv.slice(2);
const check = args.includes('--check');
const root = new URL('../../parity/fixtures/', import.meta.url);
const json = (path) => JSON.parse(readFileSync(new URL(path, root), 'utf8'));
const conformance = json('issue-195-conformance/manifest.json');
const generative = json('issue-195-generative/manifest.json');
const native = (language) => isNativeGrammar(LANGUAGE_CATALOG.languages.find((entry) => entry.name === language).grammars[0].id);
const requested = args.filter((arg) => !arg.startsWith('--'));
const languages = requested.length ? requested : Object.keys(conformance.languages).filter(native);

/** Every conformance and generative case of `language`: its id, source, oracle and native tree. */
function* cases(language) {
  const details = conformance.languages[language];
  const corpus = new Map();
  for (const file of Object.keys(details.corpus.files)) {
    parseCorpus(readFileSync(new URL(`issue-195-conformance/${details.corpus.directory}/${file}`, root), 'utf8'))
      .forEach((entry, index) => corpus.set(`corpus/${file}/${index}`, entry.source));
  }
  for (const project of details.projects) {
    corpus.set(`project/${project.file.split('/').pop()}`, readFileSync(new URL(`issue-195-conformance/${project.file}`, root), 'utf8'));
  }
  const hosts = json('issue-195-conformance/cases.json').mixed;
  const document = (source) => renderCstLines(documentGrammarRoots(LinkNetwork.parse(source, language), language), language).text;
  for (const entry of json(`issue-195-conformance/${details.oracle}`).cases) {
    if (entry.kind === 'mixed') {
      const host = hosts.find((candidate) => candidate.id === entry.hostCase);
      const region = regionGrammarRoots(LinkNetwork.parse(host.source, host.host)).find((candidate) => candidate.language === language &&
        candidate.span.byteRange.start === entry.startByte && candidate.span.byteRange.end === entry.endByte);
      yield { suite: 'conformance', id: entry.id, source: host.source, oracle: entry.cst, native: region ? renderCstLines(region, language).text : '' };
    } else {
      const source = entry.source ?? corpus.get(entry.id);
      yield { suite: 'conformance', id: entry.id, source, oracle: entry.cst, native: document(source) };
    }
  }
  for (const entry of json(`issue-195-generative/${generative.languages[language].file}`).cases) {
    yield { suite: 'generative', id: entry.id, source: entry.source, oracle: entry.cst, native: document(entry.source) };
    if (entry.kind === 'metamorphic') {
      yield { suite: 'generative', id: `${entry.id} (variant)`, source: entry.variant, oracle: entry.variantCst, native: document(entry.variant) };
    }
    let source = entry.source;
    for (const [position, step] of (entry.steps ?? []).entries()) {
      source = applyEdit(source, step);
      yield { suite: 'generative', id: `${entry.id} step ${position}`, source, oracle: step.cst, native: document(source) };
    }
  }
}

const previous = readNativeRecovery();
const records = previous.cases.filter((record) => !languages.includes(record.language));
const refused = [];
for (const language of languages) {
  for (const entry of cases(language)) {
    if (entry.native === entry.oracle) continue;
    if (!isMalformed(entry.oracle) || !isMalformed(entry.native)) {
      refused.push(`${language} ${entry.suite} ${entry.id}: the ${isMalformed(entry.oracle) ? 'native' : 'oracle'} tree is clean`);
      continue;
    }
    records.push(recoveryRecord({ language, ...entry }));
  }
}
const order = (record) => `${record.language}\u0000${record.suite}`;
records.sort((left, right) => order(left).localeCompare(order(right)));
const output = `${JSON.stringify({
  description: 'Conformance and generative cases whose native recovery differs from the tree-sitter oracle, each with a justified category; written by js/scripts/generate-native-recovery.mjs and checked by js/tests/support/native-recovery.js and rust/tests/unit/native_recovery_records.rs.',
  categories: CATEGORIES,
  cases: records,
}, null, 2)}\n`;
const target = new URL(`../../${NATIVE_RECOVERY_FILE}`, import.meta.url);
if (check) {
  if (readFileSync(target, 'utf8') !== output) {
    console.error(`${NATIVE_RECOVERY_FILE} is stale; run node scripts/generate-native-recovery.mjs`);
    process.exitCode = 1;
  }
} else {
  writeFileSync(target, output);
  console.log(`wrote ${NATIVE_RECOVERY_FILE}: ${records.length} cases`);
}
for (const problem of refused) console.error(problem);
if (refused.length) process.exitCode = 1;
