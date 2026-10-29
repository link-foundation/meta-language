import { draftSourceLibrary } from './draft-source-library.mjs';
import { readFileSync } from 'node:fs';
import { writeProjectData } from './project-data.mjs';
import { draftNativeSourceInterfaces } from './draft-native-source-interfaces.mjs';
if (process.argv.slice(2).join(' ') !== '--draft') throw new TypeError('use --draft for source-only authoring');
const { program, emitted } = draftSourceLibrary({ sourcePath: 'js/src/translation/record-write-rules.js',
  moduleName: 'record_write_rules', model: 'closed shared own record field assignment 1',
  programAsset: 'record-write-program' });

// Source/property identifiers are Unicode scalar schema text; runtime field
// values continue to use their lossless UTF16 model. No semantics are replaced.
const definition = { schemaVersion: 1, model: 'native Rust source Unicode scalar interface 1',
  module: 'record_write_rules', functions: [...readFileSync(new URL('../src/translation/record-write-rules.js', import.meta.url), 'utf8')
    .matchAll(/^export function ([A-Za-z][A-Za-z0-9]*)\(/gmu)].map(match => ({ source: match[1] })) };
writeProjectData(new URL('../../parity/translation/record-write-source-interface.lino', import.meta.url), definition);
draftNativeSourceInterfaces({ program, emitted, definition, module: 'record_write_rules',
  output: 'rust/src/translation/record_write_source_interfaces.rs',
  receipt: 'parity/self-translation/record-write-source-interfaces.lino' });
