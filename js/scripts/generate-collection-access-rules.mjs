import { draftSourceLibrary } from './draft-source-library.mjs';
if (process.argv.slice(2).join(' ') !== '--draft') throw new TypeError('use --draft for source generation without behavioral execution');
draftSourceLibrary({ sourcePath: 'js/src/translation/collection-access-rules.js',
  moduleName: 'collection_access_rules', model: 'insertion ordered collection access construction 1',
  programAsset: 'collection-access-program' });
