// Source generation only. The --draft phase runs no fixture, behavioral replay,
// native compiler or test. Verification belongs after the complete scope draft.
import { readFileSync, writeFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { readPortableLibraryRelocation } from './record-rule-library.mjs';
import { readPortableSourceParameterStyles } from './portable-source-library-style.mjs';
import { readPortableSourceLintStyles } from './portable-source-lint-style.mjs';
import { formatProductRuleSource } from './product-rule-style-corrections.mjs';
import { createHash } from 'node:crypto';
import { DecoratorSet } from '../src/decorators.js';
import { writeProjectData } from './project-data.mjs';

export function draftSemanticInputRules() {
  const source = readFileSync(new URL('../src/translation/semantic-input-rules.js', import.meta.url), 'utf8');
  const program = checkProgram(parseJavaScript(source, { referenceSemantics: false }));
  const emitted = emitRust(program, { arrayConsumption: true });
  const header = emitted.text.split('\n').find(line => line.startsWith('#![allow('));
  if (!header) throw new TypeError('the source translator omitted its library attributes');
  const raw = [header, ...emitted.preludes, ...emitted.definitions].join('\n\n');
  const relocation = readPortableLibraryRelocation(raw);
  const names = [...relocation.source.matchAll(/^pub fn (\w+)\(/gmu)].map(match => match[1]);
  const borrowed = readPortableSourceParameterStyles(relocation.source, { publicNames: names });
  const styled = readPortableSourceLintStyles(borrowed.source);
  const output = formatProductRuleSource(styled.source);
  writeFileSync(new URL('../../rust/src/translation/semantic_input_rules.rs', import.meta.url),
    '// Generated from complete semantic-input-rules.js. Do not edit by hand.\n' + output);
  const styles = new DecoratorSet([{ id: 'semantic-input-library-source-styles', level: 'emitter', order: 0,
    when: [['format', 'Rust']], actions: [...relocation.actions, ...borrowed.actions, ...styled.actions] }]);
  writeFileSync(new URL('../../parity/self-translation/semantic-input-rules-decorators.lino', import.meta.url), styles.toLino());
  writeProjectData(new URL('../../parity/self-translation/semantic-input-rules-native.lino', import.meta.url), {
    schemaVersion: 1, model: 'independent semantic source grouping 1',
    sourceClosure: { source, sourceSha256: createHash('sha256').update(source).digest('hex'),
      inputs: [{ path: 'js/src/translation/semantic-input-rules.js', source, removedImports: [] }] },
    emitted: { text: emitted.text, preludes: emitted.preludes, definitions: emitted.definitions, mappings: emitted.mappings },
    nativeSource: output,
    boundary: 'Automatic complete-source authoring only. No fixture, behavioral comparison, archive replay, product execution or native compilation has run.',
  });
  return { source, program, emitted };
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  if (process.argv.length !== 3 || process.argv[2] !== '--draft') throw new TypeError('usage: generate-semantic-input-rules.mjs --draft');
  draftSemanticInputRules();
}
