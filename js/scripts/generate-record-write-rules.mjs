import { draftSourceLibrary } from './draft-source-library.mjs';
if (process.argv.slice(2).join(' ') !== '--draft') throw new TypeError('use --draft for source-only authoring');
draftSourceLibrary({ sourcePath: 'js/src/translation/record-write-rules.js',
  moduleName: 'record_write_rules', model: 'closed shared own record field assignment 1',
  programAsset: 'record-write-program' });
