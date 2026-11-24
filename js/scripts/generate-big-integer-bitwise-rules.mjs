import { writeFileSync } from 'node:fs';
import { draftSourceLibrary } from './draft-source-library.mjs';
import { readPortableLibraryRelocation } from './record-rule-library.mjs';
import { writeProjectData } from './project-data.mjs';
if (process.argv.slice(2).join(' ') !== '--draft') throw new TypeError('use --draft for source-only authoring');
const root = new URL('../../', import.meta.url);
const { emitted } = draftSourceLibrary({ sourcePath: 'js/src/translation/big-integer-bitwise-rules.js',
  moduleName: 'big_integer_bitwise_rules', model: 'exact infinite two complement BigInt operations 1',
  programAsset: 'big-integer-bitwise-program', sourceFormatting: 'preserve' });
const header = emitted.text.split('\n').find(line => line.startsWith('#![allow('));
if (!header) throw new TypeError('the complete emitted library requires its actual attributes');
// Root source functions become siblings in the generated runtime module. The
// actual integer representation remains the one already emitted by its owner.
const dependencies = emitted.preludes.filter(source => !/^pub mod ml \{/mu.test(source));
const original = [header, 'use crate::ml;', ...dependencies, ...emitted.definitions].join('\n\n');
const relocation = readPortableLibraryRelocation(original);
const runtime = 'pub mod ml_big_integer_bitwise {\n' + relocation.source + '\n}\n';
writeFileSync(new URL('rust/src/translation/runtime/big-integer-bitwise.rs', root), runtime);
writeFileSync(new URL('js/src/translation/big-integer-bitwise-runtime.js', root),
  '// Generated from complete checked BigInt definitions. Do not edit.\nexport const BIG_INTEGER_BITWISE_PRELUDE = ' + JSON.stringify(runtime) + ';\n');
writeProjectData(new URL('parity/self-translation/big-integer-bitwise-runtime.lino', root), {
  schemaVersion: 1, model: 'complete BigInt runtime lexical relocation 1', original,
  actions: relocation.actions, edits: relocation.edits, runtime,
  integerOwner: 'the ordinary emitted crate::ml module; no replacement arithmetic or duplicated integer type',
  boundary: 'Complete source construction only. No represented invocation, fixture, target execution, test or native compilation.',
});
