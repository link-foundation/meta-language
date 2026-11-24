import { draftSourceLibrary } from './draft-source-library.mjs';
if (process.argv.slice(2).join(' ') !== '--draft') throw new TypeError('use --draft for source generation without behavioral execution');
draftSourceLibrary({ sourcePath: 'js/src/translation/reference-method-rules.js',
  moduleName: 'reference_method_rules', model: 'shared reference collection method construction 1',
  programAsset: 'reference-method-program' });
