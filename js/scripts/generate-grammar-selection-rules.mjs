// Capture the inspectable selection schema and translate its portable decisions.
// Host executor integration and native execution require independent CI checks.
import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync } from 'node:fs';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import { isDeepStrictEqual } from 'node:util';
import { GRAMMAR_SELECTION_FORMS } from '../src/grammar-selection-forms.js';
import { readGrammarSelectionEqualityFields, readGrammarTreeEqualityFields,
  readGrammarTreeRecursionPlan } from '../src/translation/grammar-selection-rules.js';
import { readProjectData, writeProjectData } from './project-data.mjs';

const sourceFile = new URL('../src/translation/grammar-selection-rules.js', import.meta.url);
const formFile = new URL('../src/grammar-selection-forms.js', import.meta.url);
const policyFiles = [new URL('../src/translation/data/grammar-selection-rules.lino', import.meta.url), new URL('../../rust/src/data/grammar-selection-rules.lino', import.meta.url)];
const nativeFile = new URL('../../rust/src/translation/grammar_selection_rules.rs', import.meta.url);
const formOutput = new URL('../../rust/src/grammar/selection_forms.rs', import.meta.url);
const nativeArtifact = new URL('../../parity/self-translation/grammar-selection-rules-native.lino', import.meta.url);
const digest = source => createHash('sha256').update(source).digest('hex');

export function grammarSelectionRulesPolicy() {
  const source = readFileSync(sourceFile, 'utf8'), formSource = readFileSync(formFile, 'utf8');
  return { schemaVersion: 1, model: 'explicit same-region grammar selection 1',
    source, sourceSha256: digest(source), formSource, formSourceSha256: digest(formSource), forms: GRAMMAR_SELECTION_FORMS,
    primitiveBasis: {
      any: 'try every explicitly named grammar, retain every accepted concrete tree, and report multiple accepted witnesses as a language ambiguity',
      all: 'every explicitly named grammar must accept the same physical byte region; alternate trees are parallel witnesses and never duplicate physical source ownership',
      candidates: 'source-qualified grammar names, never host language-pair dispatch or inference from samples',
      boundaries: 'a grammar expression selects the complete source region; enclosing tokens retain delimiters, escapes and trivia',
      mapping: 'identity mapping only; other named mappings retain an explicit unsupportedEmbeddingMapping diagnostic',
      recursion: 'each child uses the same executor algorithms and step, memory and remaining nesting budgets; repeated active program and byte bounds diagnose a non-consuming cycle',
      provenance: 'stored global origin membership requires the actual full source definition and declaration context to match; unproved origins remain unknown',
      equality: 'embedded roots and every accepted witness use full concrete-tree equality, preserving source metadata, explicit presence and linked object order; private program pointers do not replace tree or provenance equality',
    },
    equality: {
      trees: ['node', 'token', 'error', 'missing', 'embed'].flatMap(kind => [false, true].map(output => ({ kind, output,
        fields: readGrammarTreeEqualityFields(kind, output).split('|'), recursion: readGrammarTreeRecursionPlan(kind, output) }))),
      selectionFields: readGrammarSelectionEqualityFields(false).split('|'),
      alternativeFields: readGrammarSelectionEqualityFields(true).split('|'),
      metadata: 'ordered own object keys, ordered arrays with exact length and presence; ordinary action attributes retain their existing order-independent equality',
    },
    boundary: 'This policy and its generated decisions are not evidence that every catalog grammar, escape transform, AST projection or native host adapter is implemented.',
  };
}

export function renderGrammarSelectionNativeForms(forms) {
  const fields = { name: 'FieldType::Name', expression: 'FieldType::Expression', expressions: 'FieldType::Expressions' };
  return '// Generated from the shared grammar selection schema. Do not edit.\n'
    + 'use super::feature::{FieldType, FormFields};\n\n'
    + 'pub const SELECTION_EXPRESSION_FORMS: &[(&str, FormFields)] = &[\n'
    + Object.entries(forms).map(([head, columns]) => `    (${JSON.stringify(head)}, &[${columns.map(([name, type]) => {
      if (!fields[type]) throw new TypeError(`unsupported grammar selection field ${type}`);
      return `(${JSON.stringify(name)}, ${fields[type]})`;
    }).join(', ')}]),`).join('\n') + '\n];\n';
}

export async function generateGrammarSelectionRules({ check = false, native = false } = {}) {
  const policy = grammarSelectionRulesPolicy();
  for (const file of policyFiles) {
    if (check) {
      if (!isDeepStrictEqual(readProjectData(file), policy)) throw new Error('shared grammar selection source/schema policy is stale');
    } else writeProjectData(file, policy);
  }
  const format = source => execFileSync('rustfmt', ['--edition', '2024', '--emit', 'stdout'], { input: source, encoding: 'utf8', maxBuffer: 4 * 1024 * 1024 });
  const forms = format(renderGrammarSelectionNativeForms(policy.forms));
  if (check) {
    if (readFileSync(formOutput, 'utf8') !== forms) throw new Error('native grammar selection schema is stale');
  } else writeFileSync(formOutput, forms);
  if (native) {
    const { parseJavaScript } = await import('../src/translation/javascript.js');
    const { checkProgram } = await import('../src/translation/check.js');
    const { emitRust } = await import('../src/translation/emit-rust.js');
    const emitted = emitRust(checkProgram(parseJavaScript(policy.source, { referenceSemantics: false })), { arrayConsumption: true });
    const rustSource = format('// Generated from the complete shared grammar-selection-rules.js. Do not edit.\n' + emitted.text);
    const artifact = { schemaVersion: 1, model: policy.model, sourceSha256: policy.sourceSha256, formSourceSha256: policy.formSourceSha256,
      definitions: emitted.definitions, assumptions: emitted.assumptions, encodings: emitted.encodings, rustSource, rustSha256: digest(rustSource) };
    if (check) {
      if (readFileSync(nativeFile, 'utf8') !== rustSource || !isDeepStrictEqual(readProjectData(nativeArtifact), artifact)) throw new Error('native grammar selection translation is stale');
    } else {
      writeFileSync(nativeFile, rustSource);
      writeProjectData(nativeArtifact, artifact);
    }
  }
  return policy;
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  await generateGrammarSelectionRules({ check: process.argv.includes('--check'), native: process.argv.includes('--native') });
  console.log('grammar selection policy, schema and requested native translation match');
}
