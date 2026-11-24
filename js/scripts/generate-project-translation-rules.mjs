import { draftSourceLibrary } from './draft-source-library.mjs';
if (process.argv.slice(2).join(' ') !== '--draft') throw new TypeError('use --draft for source generation without behavioral execution');
draftSourceLibrary({ sourcePath: 'js/src/project-translation-rules.js',
  moduleName: 'project_translation_rules', directory: '',
  model: 'source-qualified project translation worklist 1' });
