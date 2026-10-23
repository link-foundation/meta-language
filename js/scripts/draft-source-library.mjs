// Shared authoring pipeline. It writes complete translated declarations and
// source records without fixtures, behavioral replays or native compilation.
import { readFileSync, writeFileSync, mkdirSync } from 'node:fs';
import { createHash } from 'node:crypto';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { DecoratorSet } from '../src/decorators.js';
import { readPortableLibraryRelocation } from './record-rule-library.mjs';
import { readPortableSourceParameterStyles, partitionPortableRustLibrary } from './portable-source-library-style.mjs';
import { readPortableSourceLintStyles } from './portable-source-lint-style.mjs';
import { formatProductRuleSource } from './product-rule-style-corrections.mjs';
import { writeProjectData } from './project-data.mjs';

const root = new URL('../../', import.meta.url);
export function draftSourceLibrary({ sourcePath, moduleName, directory = 'translation', model }) {
  if (!/^[a-z][a-z_0-9]*$/u.test(moduleName) || !['translation', ''].includes(directory)) throw new TypeError('invalid generated library location');
  const source = readFileSync(new URL(sourcePath, root), 'utf8');
  const program = checkProgram(parseJavaScript(source, { referenceSemantics: false }));
  const emitted = emitRust(program, { arrayConsumption: true });
  const header = emitted.text.split('\n').find(line => line.startsWith('#![allow('));
  if (!header) throw new TypeError('the source translator omitted its library attributes');
  const relocation = readPortableLibraryRelocation([header, ...emitted.preludes, ...emitted.definitions].join('\n\n'));
  const names = [...relocation.source.matchAll(/^pub fn (\w+)\(/gmu)].map(match => match[1]);
  const loans = readPortableSourceParameterStyles(relocation.source, { publicNames: names });
  const styles = readPortableSourceLintStyles(loans.source);
  const output = formatProductRuleSource(styles.source);
  const files = partitionPortableRustLibrary(output, { directory: moduleName,
    notice: '// Generated from complete ' + sourcePath + '. Do not edit by hand.\n' });
  const base = new URL('rust/src/' + (directory ? directory + '/' : ''), root);
  for (const file of files) {
    const path = new URL(file.path ?? moduleName + '.rs', base);
    mkdirSync(new URL('.', path), { recursive: true });
    writeFileSync(path, file.source);
  }
  const decorators = new DecoratorSet([{ id: moduleName.replaceAll('_', '-') + '-source-styles',
    level: 'emitter', order: 0, when: [['format', 'Rust']],
    actions: [...relocation.actions, ...loans.actions, ...styles.actions] }]);
  const stem = 'parity/self-translation/' + moduleName.replaceAll('_', '-');
  writeFileSync(new URL(stem + '-decorators.lino', root), decorators.toLino());
  const sourceSha256 = createHash('sha256').update(source).digest('hex');
  writeProjectData(new URL(stem + '-native.lino', root), {
    schemaVersion: 1, model, sourceClosure: { source, bytes: Buffer.byteLength(source), sourceSha256,
      inputs: [{ path: sourcePath, source, bytes: Buffer.byteLength(source), sourceSha256, removedImports: [] }] },
    declarations: [...program.declarations].map(([name, declaration]) => ({ name, kind: declaration.k })),
    emitted: { text: emitted.text, definitions: emitted.definitions, preludes: emitted.preludes, mappings: emitted.mappings },
    files: files.map(file => ({ path: file.path ?? moduleName + '.rs', source: file.source })),
    boundary: 'Complete automatic source generation only. No fixture, behavior comparison, archive replay, product execution or native compilation.',
  });
  return { files, program, emitted };
}
