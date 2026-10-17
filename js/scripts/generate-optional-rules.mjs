// Translate each complete optional kernel and its exact required-column
// dependency. Styles relocate actual generated library declarations only.
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { existsSync, mkdirSync, readdirSync, readFileSync, unlinkSync, writeFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { DecoratorSet } from '../src/decorators.js';
import { checkProjectData, writeProjectData } from './project-data.mjs';
import { formatProductRuleSource } from './product-rule-style-corrections.mjs';
import { partitionPortableRustLibrary, readPortableSourceLintStyles, readPortableSourceParameterStyles } from './portable-source-library-style.mjs';

const hashOptionalSource = source => createHash('sha256').update(source).digest('hex');
const requiredImport = "import { readRequiredNumberFact, readRequiredTextFact } from './fact-column-rules.js';\n";

export function readOptionalRuleSources() {
  const facts = readFileSync(new URL('../src/translation/fact-column-rules.js', import.meta.url), 'utf8');
  const planning = readFileSync(new URL('../src/translation/optional-inference-rules.js', import.meta.url), 'utf8');
  assert.equal(planning.split(requiredImport).length, 2, 'the optional kernel has one exact required-column import');
  const values = readFileSync(new URL('../src/translation/optional-value-rules.js', import.meta.url), 'utf8');
  return [
    { name: 'optional_inference_rules', source: planning.replace(requiredImport, '') + '\n' + facts,
      inputs: [{ path: 'js/src/translation/optional-inference-rules.js', sourceSha256: hashOptionalSource(planning) },
        { path: 'js/src/translation/fact-column-rules.js', sourceSha256: hashOptionalSource(facts) }], removedImport: requiredImport },
    { name: 'optional_value_rules', source: values,
      inputs: [{ path: 'js/src/translation/optional-value-rules.js', sourceSha256: hashOptionalSource(values) }], removedImport: '' },
  ];
}

export async function generateOptionalRules({ check = false, native = true } = {}) {
  const sources = readOptionalRuleSources();
  const policy = { schemaVersion: 1, model: 'checked optional constraints and value operations 1',
    sources: sources.map(source => ({ ...source, sourceSha256: hashOptionalSource(source.source) })) };
  const policyFile = new URL('../src/translation/data/optional-rules.lino', import.meta.url);
  if (check) checkProjectData(policyFile, policy);
  else writeProjectData(policyFile, policy);
  if (native) {
    const { parseJavaScript } = await import('../src/translation/javascript.js');
    const { checkProgram } = await import('../src/translation/check.js');
    const { emitRust } = await import('../src/translation/emit-rust.js');
    for (const source of sources) {
      const translated = emitRust(checkProgram(parseJavaScript(source.source, { referenceSemantics: false })), { arrayConsumption: true });
      const moduleHeader = translated.text.split('\n').find(line => line.startsWith('#![allow('));
      assert.ok(moduleHeader, 'the checked translator retains its existing generated module header');
      const raw = [moduleHeader, ...translated.preludes, ...translated.definitions].join('\n\n');
      const declarations = new Set([...raw.matchAll(/^pub (?:fn|enum|struct|mod) ([A-Za-z_][A-Za-z_0-9]*)\b/gmu)].map(match => match[1]));
      const actions = [...declarations].filter(name => raw.includes('crate::' + name)).map(name => ({ op: 'replace', field: 'source', from: 'crate::' + name, to: 'self::' + name }));
      const relocated = new DecoratorSet([{ id: source.name + '-relocation', level: 'emitter', order: 0, when: [['format', 'Rust']], actions }]).decorate('emitter', { format: 'Rust', source: raw }).source;
      const publicNames = [...raw.matchAll(/^pub fn ([A-Za-z_][A-Za-z_0-9]*)\b/gmu)].map(match => match[1]);
      const loans = readPortableSourceParameterStyles(relocated, { publicNames });
      actions.push(...loans.actions, ...readPortableSourceLintStyles(loans.source).actions);
      const styles = new DecoratorSet([{ id: source.name + '-library-style', level: 'emitter', order: 0, when: [['format', 'Rust']], actions }]);
      const styleFile = new URL('../../parity/self-translation/' + source.name.replaceAll('_', '-') + '-decorators.lino', import.meta.url);
      if (check) assert.equal(readFileSync(styleFile, 'utf8'), styles.toLino(), 'optional library styles retain actual declaration relocation and mechanical parameter loans');
      else writeFileSync(styleFile, styles.toLino());
      const decorated = DecoratorSet.fromLino(readFileSync(styleFile, 'utf8')).decorate('emitter', { format: 'Rust', source: raw }).source;
      const notice = '// Generated from the complete checked optional kernel and its recorded source dependencies.\n// Do not edit by hand.\n';
      const generated = partitionPortableRustLibrary(formatProductRuleSource(decorated), { notice, directory: source.name });
      const directory = new URL('../../rust/src/translation/' + source.name + '/', import.meta.url);
      const expectedParts = generated.filter(part => part.path !== null).map(part => part.path.split('/').at(-1)).sort();
      const actualParts = existsSync(directory) ? readdirSync(directory).filter(name => /^part_[1-9][0-9]*\.rs$/u.test(name)).sort() : [];
      if (check) assert.deepEqual(actualParts, expectedParts, 'the complete generated optional module has exactly its declared fragments');
      else for (const name of actualParts) if (!expectedParts.includes(name)) unlinkSync(new URL(name, directory));
      for (const part of generated) {
        const output = new URL('../../rust/src/translation/' + (part.path ?? source.name + '.rs'), import.meta.url);
        if (check) assert.equal(readFileSync(output, 'utf8'), part.source, 'complete native optional algorithms are fresh');
        else {
          if (part.path) mkdirSync(new URL('../../rust/src/translation/' + source.name + '/', import.meta.url), { recursive: true });
          writeFileSync(output, part.source);
        }
      }
    }
    const nativePolicy = new URL('../../rust/src/data/optional-rules.lino', import.meta.url);
    if (check) assert.ok(readFileSync(nativePolicy).equals(readFileSync(policyFile)), 'both runtimes retain exact optional source policy bytes');
    else writeFileSync(nativePolicy, readFileSync(policyFile));
  }
  return policy;
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  await generateOptionalRules({ check: process.argv.includes('--check'), native: !process.argv.includes('--javascript-only') });
  console.log('Optional rules: complete checked constraints, absence construction, coercion and four-target rendering');
}
