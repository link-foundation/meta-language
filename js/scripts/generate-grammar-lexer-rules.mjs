// Capture the complete lexical kernels and schema as inspectable shared Links
// data, then translate the actual source when --native is explicitly requested.
import { createHash } from 'node:crypto';
import { readFileSync, mkdirSync } from 'node:fs';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import { isDeepStrictEqual } from 'node:util';
import { DecoratorSet } from '../src/decorators.js';
import { GRAMMAR_LEXER_FORMS } from '../src/grammar-lexer-forms.js';
import { readProjectData, writeProjectData } from './project-data.mjs';
import { readPortableSourceParameterStyles, readPortableSourceLintStyles, partitionPortableRustLibrary } from './portable-source-library-style.mjs';
import { formatProductRuleSource } from './product-rule-style-corrections.mjs';

const sourceFile = new URL('../src/translation/grammar-lexer-rules.js', import.meta.url);
const formFile = new URL('../src/grammar-lexer-forms.js', import.meta.url);
const policyFiles = [new URL('../src/translation/data/grammar-lexer-rules.lino', import.meta.url), new URL('../../rust/src/data/grammar-lexer-rules.lino', import.meta.url)];
const nativeFile = new URL('../../rust/src/translation/grammar_lexer_rules.rs', import.meta.url);
const formOutput = new URL('../../rust/src/grammar/lexical_forms.rs', import.meta.url);
const nativeArtifact = new URL('../../parity/self-translation/grammar-lexer-rules-native.lino', import.meta.url);
const nativeStyles = new URL('../../parity/self-translation/grammar-lexer-rules-decorators.lino', import.meta.url);
const hash = (source) => createHash('sha256').update(source).digest('hex');
const factFile = new URL('../src/translation/fact-column-rules.js', import.meta.url);
const factImport = "import { readRequiredNumberFact, readRequiredTextFact, readRequiredBooleanFact } from './fact-column-rules.js';";

/** Close the one actual source import, retaining every accessor and abort. */
export function readGrammarLexerSourceModule(source = readFileSync(sourceFile, 'utf8')) {
  if (source.split(factImport).length !== 2) throw new Error('the lexical module needs exactly its recorded fact-column source import');
  const body = source.replace(factImport, '');
  if (/^\s*import\b/mu.test(body)) throw new Error('the lexical module has an unrecorded source import');
  return readFileSync(factFile, 'utf8') + '\n' + body;
}

export function grammarLexerRulesPolicy() {
  const source = readFileSync(sourceFile, 'utf8');
  const formSource = readFileSync(formFile, 'utf8');
  const closedSource = readGrammarLexerSourceModule(source);
  return { schemaVersion: 1, model: 'source scoped maximal lexical election 1', source, sourceSha256: hash(source),
    closedSource, closedSourceSha256: hash(closedSource),
    sourceClosure: { removedImport: factImport, dependencies: [{ path: 'js/src/translation/fact-column-rules.js', sourceSha256: hash(readFileSync(factFile)) }] },
    formSource, formSourceSha256: hash(formSource), forms: GRAMMAR_LEXER_FORMS,
    primitiveBasis: {
      order: 'original source declaration order, implicit combined-grammar literal tokens first',
      eligibility: 'full source-local lexical match in the active source-family mode',
      selection: 'longest completed token, first declaration for equal completed length',
      channels: 'apply only the elected token channel; a custom channel name does not imply an error',
      disposition: 'skip replaces the token type with SKIP independently of channel; a later type command restores emission; warning-bearing command combinations keep their ordered runtime effects',
      actions: 'commit only the winning token command state; referenced lexical-rule actions do not execute',
      opacity: 'unresolved potentially winning predicates, actions, imports, commands and non-greedy decisions refuse execution',
      limits: 'the caller owns one unchanged parse step, memory, nesting and recovery budget',
    },
    upstreamSemantics: {
      version: 'ANTLR 4.13.2',
      runtime: 'https://raw.githubusercontent.com/antlr/antlr4/4.13.2/runtime/Java/src/org/antlr/v4/runtime/Lexer.java',
      actions: 'https://raw.githubusercontent.com/antlr/antlr4/4.13.2/tool/src/org/antlr/v4/automata/LexerATNFactory.java',
      diagnostics: 'https://raw.githubusercontent.com/antlr/antlr4/4.13.2/tool/src/org/antlr/v4/tool/ErrorType.java',
    },
    columns: { declaration: ['kind', 'source name', 'item kind', 'literal value', 'mode', 'channel', 'disposition'],
      action: ['declaration position', 'operation', 'mode'], edge: ['source node position', 'target node position'],
      lazyNormalization: ['original capture position', 'original delimiter position or refusal -1'],
      declarationIssue: ['issue code', 'original declaration position', 'original action position'] },
    boundary: 'Host adapters project exact inspected graph fields, run existing bounded matchers/operation interpreters and materialize original positions. Native execution and context-dependent concept equivalence require independent evidence.',
  };
}

export function renderGrammarLexerNativeForms(forms) {
  const fieldType = (type) => {
    if (type === 'name') return 'FieldType::Name';
    if (type === 'text') return 'FieldType::Text';
    if (type === 'expression') return 'FieldType::Expression';
    if (type === 'block:action') return 'FieldType::Block("action", false)';
    throw new Error(`unsupported lexical schema field ${type}`);
  };
  return '// Generated from the shared grammar-lexer-rules.lino policy. Do not edit.\n'
    + 'use super::feature::{FieldType, FormFields};\n\n'
    + 'pub const LEXICAL_EXPRESSION_FORMS: &[(&str, FormFields)] = &[\n'
    + Object.entries(forms).map(([head, fields]) => `    (${JSON.stringify(head)}, &[${fields.map(([key, type]) => `(${JSON.stringify(key)}, ${fieldType(type)})`).join(', ')}]),`).join('\n')
    + '\n];\n';
}

export async function generateGrammarLexerRules({ check = false, native = false } = {}) {
  const policy = grammarLexerRulesPolicy();
  for (const file of policyFiles) {
    if (check) {
      if (!isDeepStrictEqual(readProjectData(file), policy)) throw new Error('shared lexical-rule source/schema policy is stale');
    } else writeProjectData(file, policy);
  }
  const forms = renderGrammarLexerNativeForms(policy.forms);
  const formattedForms = execFileSync('rustfmt', ['--edition', '2024', '--emit', 'stdout'], { input: forms, encoding: 'utf8', maxBuffer: 1024 * 1024 });
  if (check) {
    if (readFileSync(formOutput, 'utf8') !== formattedForms) throw new Error('native lexical schema table is stale');
  } else {
    const { writeFileSync } = await import('node:fs');
    writeFileSync(formOutput, formattedForms);
  }
  if (native) {
    const { parseJavaScript } = await import('../src/translation/javascript.js');
    const { checkProgram } = await import('../src/translation/check.js');
    const { emitRust } = await import('../src/translation/emit-rust.js');
    const emitted = emitRust(checkProgram(parseJavaScript(policy.closedSource, { referenceSemantics: false })), { arrayConsumption: true });
    const moduleHeader = emitted.text.split('\n').find((line) => line.startsWith('#![allow('));
    if (!moduleHeader) throw new Error('the checked lexical translation has no existing generated module header');
    const raw = [moduleHeader, ...emitted.preludes, ...emitted.definitions].join('\n\n');
    const declarations = new Set([...raw.matchAll(/^pub (?:fn|enum|struct|mod) ([A-Za-z_][A-Za-z_0-9]*)\b/gmu)].map((match) => match[1]));
    const actions = [...declarations].filter((name) => raw.includes('crate::' + name))
      .map((name) => ({ op: 'replace', field: 'source', from: 'crate::' + name, to: 'self::' + name }));
    const relocated = new DecoratorSet([{ id: 'grammar-lexer-rules-library-owner', level: 'emitter', order: 0, when: [['format', 'Rust']], actions }])
      .decorate('emitter', { format: 'Rust', source: raw }).source;
    const publicNames = [...relocated.matchAll(/^pub fn (\w+)\(/gmu)].map((match) => match[1]);
    const loans = readPortableSourceParameterStyles(relocated, { publicNames });
    const reviewed = readPortableSourceLintStyles(loans.source);
    actions.push(...loans.actions, ...reviewed.actions);
    const styles = new DecoratorSet([{ id: 'grammar-lexer-rules-library-style', level: 'emitter', order: 0, when: [['format', 'Rust']], actions }]);
    if (check) {
      if (readFileSync(nativeStyles, 'utf8') !== styles.toLino()) throw new Error('native lexical styles must retain actual generated declarations, immutable loans and reviewed source corrections');
    } else {
      const { writeFileSync } = await import('node:fs');
      writeFileSync(nativeStyles, styles.toLino());
    }
    const decorated = DecoratorSet.fromLino(readFileSync(nativeStyles, 'utf8')).decorate('emitter', { format: 'Rust', source: raw }).source;
    if (decorated !== reviewed.source) throw new Error('recorded lexical library styles do not reproduce their reviewed source');
    if (/\bcrate::/u.test(decorated)) throw new Error('the native lexical library has an unresolved crate-root reference');
    const notice = '// Generated from the complete shared grammar-lexer-rules.js and recorded source closure.\n// Do not edit by hand.\n';
    const rustSource = formatProductRuleSource(decorated);
    const nativeFiles = partitionPortableRustLibrary(rustSource, { notice, directory: 'grammar_lexer_rules' });
    const artifact = { schemaVersion: 1, model: policy.model, sourceSha256: policy.sourceSha256, closedSourceSha256: policy.closedSourceSha256,
      sourceClosure: policy.sourceClosure, formSourceSha256: policy.formSourceSha256, styleSha256: hash(readFileSync(nativeStyles)),
      definitions: emitted.definitions, assumptions: emitted.assumptions, encodings: emitted.encodings, rustSource, rustSha256: hash(rustSource),
      nativeFiles: nativeFiles.map((file) => ({ path: file.path ?? 'grammar_lexer_rules.rs', source: file.source, sha256: hash(file.source) })) };
    if (check) {
      if (!isDeepStrictEqual(readProjectData(nativeArtifact), artifact)) throw new Error('complete native lexical-rule translation is stale');
    } else {
      writeProjectData(nativeArtifact, artifact);
    }
    const { writeFileSync } = await import('node:fs');
    for (const file of nativeFiles) {
      const output = file.path === null ? nativeFile : new URL('../../rust/src/translation/' + file.path, import.meta.url);
      if (check) {
        if (readFileSync(output, 'utf8') !== file.source) throw new Error('a complete native lexical library partition is stale');
      } else {
        mkdirSync(new URL('.', output), { recursive: true });
        writeFileSync(output, file.source);
      }
    }
  }
  return policy;
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  await generateGrammarLexerRules({ check: process.argv.includes('--check'), native: process.argv.includes('--native') });
  console.log('lexical rule source, schema and requested native generation match the shared policy');
}
