import { draftSourceLibrary } from './draft-source-library.mjs';
if (process.argv.slice(2).join(' ') !== '--draft') throw new TypeError('use --draft for source generation without behavioral execution');
draftSourceLibrary({ sourcePath: 'js/src/translation/fixed-integer-cast-rules.js',
  moduleName: 'fixed_integer_cast_rules', model: 'fixed integer modular cast meaning 1',
  programAsset: 'fixed-integer-cast-program' });
