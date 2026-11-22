#!/usr/bin/env node
// Generates parity/fixtures/lino-grammar-cases.json: LiNo sources with the
// links the official links-notation parser (the reference implementation of
// links-notation/src/grammar.pegjs) returns for them, or null where it rejects
// the source. Both runtimes check their built-in LiNo grammar CST against it.
//
// The source is read the way the official `Parser` class reads it: comments
// are blanked by `stripComments` first, and links nested deeper than
// DEFAULT_MAX_DEPTH are refused.
//
//   node js/scripts/generate-lino-grammar-cases.mjs          # write
//   node js/scripts/generate-lino-grammar-cases.mjs --check  # verify
//
// A link is {id, values, children} with id null when the link is unnamed; a
// reference is a link with an id and no values or children; a parenthesized
// group is {nested, children} with the links read inside the parentheses.
import { createRequire } from 'node:module';
import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { readExternalGrammarAsset } from './external-grammar-cache.mjs';
import { formatProductRuleSource } from './product-rule-style-corrections.mjs';
import { selfTranslate } from '../src/self-translation.js';

import { DEFAULT_MAX_DEPTH } from 'links-notation/src/Parser.js';
import { stripComments } from 'links-notation/src/comments.js';
import { parse } from 'links-notation/src/parser-generated.js';
import { checkProjectData, readProjectData, writeProjectData } from './project-data.mjs';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const outputPath = join(root, 'parity/fixtures/lino-grammar-cases.json');
const { version } = createRequire(import.meta.url)('links-notation/package.json');

/** `count` lines, each indented one space deeper than the line above it. */
const staircase = (count) => `${Array.from({ length: count }, (_, depth) => `${' '.repeat(depth)}a`).join('\n')}\n`;

const HAND_WRITTEN = [
  '', '\n\n', '1 1 1\n', '(papa (lovesMama: loves mama))\n(son lovesMama)\n', 'a\n\nb\n',
  'greeting:\n  hello world\n  (x y)\na b c\n', '  a\n  b\n', '(a\n b)\n', '"quoted ref" \'x\' `y`\n',
  'a: b c\n', '(broken\n', ')\n', 'a:\n', 'x\n  y\n    z\n  w\n', 'a\r\nb\r\n', '(a: b\n)\n', 'a b \n',
  '\t(a)\n', '(a)(b)\n', 'a:b\n', '"""x"""\n', '(a: (b: c) d)\n', 'a\n b\nc\n', 'a  \n\n  \n', '( )\n',
  '()\n', '"a""b"\n', '""\n', '"unclosed\n', 'x\n  (broken\n  y\n', '(obj_0: list obj_0)\n',
  'Hawaii: (state (of USA))\n', 'π: 3.14 ∞\n', 'a\n  b\n c\n', '(a b\n', 'a (b\n',
  // Comments: a `#` that opens a token starts one; inside a token or a quoted
  // reference it is an ordinary character.
  '# a comment\na b\n', 'a: b # why\n  c\n', 'issue#1047\n', '"# kept" x\n', '(a # inside\n b)\n',
  'a\n  # indented comment\n  b\n', '#\n', '(broken # note\n', 'a #b\n#c\n', '(a)"b #c"\n',
  // Parenthesized groups open a nested context that is read like the root.
  '(a\n  b\n  c)\n', '(\n  a: b\n  c: d\n)\n', '((a b))\n', '(a) b c\n', '(a)\n  child\n',
  '(a: b)\n  c\n', '(\n)\n', '(a\n\n b)\n', '(x: (y\n  z))\n', '(a:\n  b\n  c)\n',
  // Lines of nothing but spaces or tabs separate links like empty lines.
  'a\n  \nb\n', 'a\n\t\n  b\n', 'a:\n  b\n   \n  c\n',
  // Quoted references: runs of delimiters, escapes and the empty reference.
  "''\n", '``\n', '""""\n', '""a""\n', '"a""""b"\n', "'it''s'\n", '"(" ")"\n', '"" x\n', '"""\n',
  // Indented identifiers, deep nesting and long delimiter runs, the regressions
  // tracked by relative-meta-logic.
  'id:\n  a\n  b\n    c\n', ' id:\n   a\n',
  `${'('.repeat(DEFAULT_MAX_DEPTH)}a${')'.repeat(DEFAULT_MAX_DEPTH)}\n`,
  `${'('.repeat(DEFAULT_MAX_DEPTH + 1)}a${')'.repeat(DEFAULT_MAX_DEPTH + 1)}\n`,
  staircase(DEFAULT_MAX_DEPTH + 1), staircase(DEFAULT_MAX_DEPTH + 2),
  `${'"'.repeat(40)}x${'"'.repeat(40)}\n`, `${'"'.repeat(41)}\n`, `${'(a '.repeat(30)}\n`,
  // Version 1 reference literals encode exact UTF-8 and reserve the prefix.
  '~1{61}: ~1{c3a9}\n', '~1{}\n', '~1{00}\n', '~1{efbbbf}\n',
  '~1{gg}\n', '~1{ff}\n', '~2{61}\n', '~1{6}\n', '~1{61\n', '"~1{61}"\n',
  '(a)" #kept" # π discarded\n', '~1{61}" #kept"\n',
];

// A fixed linear congruential generator keeps the generated cases stable.
const ALPHABET = ['a', 'b', 'c', ' ', ' ', ' ', '\t', '\n', '\n', '\r', '(', ')', ':', '"', "'", '`', 'é', '#'];
function generated(count) {
  let seed = 195;
  const random = () => ((seed = (seed * 1103515245 + 12345) % 2147483648) / 2147483648);
  const sources = [];
  for (let index = 0; index < count; index += 1) {
    const length = 1 + Math.floor(random() * 20);
    let source = '';
    for (let offset = 0; offset < length; offset += 1) {
      source += ALPHABET[Math.floor(random() * ALPHABET.length)];
    }
    sources.push(source);
  }
  return sources;
}

const normalize = (link) => (link.nested !== undefined
  ? { nested: link.nested.map(normalize), children: (link.children ?? []).map(normalize) }
  : {
    id: link.id ?? null,
    values: (link.values ?? []).map(normalize),
    children: (link.children ?? []).map(normalize),
  });

function official(source) {
  try {
    return parse(stripComments(source), { maxDepth: DEFAULT_MAX_DEPTH, source }).map(normalize);
  } catch {
    return null;
  }
}

// The conformance sources of the language inventory are always cases.
const inventoryPath = join(root, 'parity/language-grammar-inventory.json');
const inventory = readProjectData(inventoryPath);
const specification = `parity/grammars/links-notation-${version}.pegjs`;
const installedGrammar = createRequire(import.meta.url).resolve('links-notation/src/grammar.pegjs');
if (!readExternalGrammarAsset(specification).equals(readFileSync(installedGrammar))) {
  throw new Error('the authenticated Links Notation grammar differs from the installed package');
}
const declaration = { version, specification, reference: `links-notation@${version} src/grammar.pegjs` };
const { source: inventorySource, recoverySource } = inventory.languages.find(({ name }) => name === 'LiNo');
const sources = [...new Set([inventorySource, recoverySource, ...HAND_WRITTEN, ...generated(600)])];
const expected = {
  schemaVersion: 2,
  oracle: `links-notation@${version} src/parser-generated.js parse(stripComments(source), { maxDepth: ${DEFAULT_MAX_DEPTH}, source })`,
  cases: sources.map((source) => ({ source, links: official(source) })),
  historicalOracles: [{
    dependency: 'links-notation',
    versionRange: '=0.22.0',
    source: '(a)"b #c"\n',
    links: [{
      id: null,
      values: [
        { nested: [{ id: null, values: [{ id: 'a', values: [], children: [] }], children: [] }], children: [] },
        { id: '"b', values: [], children: [] },
      ],
      children: [],
    }],
    upstreamIssue: 'https://github.com/link-foundation/links-notation/issues/377',
  }],
};
const ruleSource = readFileSync(join(root, 'js/src/lino-reference-literal-rules.js'), 'utf8');
const ruleTranslation = selfTranslate(ruleSource, 'JavaScript', 'Rust');
if (ruleTranslation.items.some(({ status }) => status !== 'translated')
  || selfTranslate(ruleTranslation.code, 'Rust', 'JavaScript').code !== ruleSource) {
  throw new Error('the versioned reference boundary must translate to Rust and back losslessly');
}
const rustRulesPath = join(root, 'rust/src/lino_reference_literal_rules.rs');
const formattedRules = formatProductRuleSource(ruleTranslation.code);
// Rebind the single translated function after rustfmt so the checked-in file
// retains the exact restoration witness for its executable Rust source.
const rustFunction = /^pub fn advance_reference_literal\([\s\S]+/mu.exec(formattedRules)?.[0].trimEnd();
if (!rustFunction) throw new Error('the translated reference boundary function is missing');
const rustHash = createHash('sha256').update(rustFunction).digest('hex');
const rustRules = formattedRules.replace(
  /^(\/\/ meta-language:translated JavaScript export_statement items=1 sha256=)[a-f0-9]{64}$/mu,
  `$1${rustHash}`,
);
if (selfTranslate(rustRules, 'Rust', 'JavaScript').code !== ruleSource) {
  throw new Error('the formatted Rust reference boundary must restore its JavaScript source');
}

if (process.argv.includes('--check')) {
  try {
    if (JSON.stringify(inventory.builtinGrammars['links-notation']) !== JSON.stringify(declaration)) {
      throw new Error('the recorded Links Notation grammar declaration differs from the installed package');
    }
    if (readFileSync(rustRulesPath, 'utf8') !== rustRules) throw new Error('the Rust reference literal rules are stale');
    checkProjectData(outputPath, expected);
  } catch {
    console.error(`${outputPath} is stale; run node js/scripts/generate-lino-grammar-cases.mjs`);
    process.exit(1);
  }
  console.log(`lino grammar cases: ${sources.length} cases match the official parser`);
} else {
  inventory.builtinGrammars['links-notation'] = declaration;
  writeProjectData(inventoryPath, inventory);
  writeProjectData(outputPath, expected);
  writeFileSync(rustRulesPath, rustRules);
  console.log(`wrote ${sources.length} cases to ${outputPath}`);
}
