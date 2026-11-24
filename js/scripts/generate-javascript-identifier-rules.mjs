import { draftSourceLibrary } from './draft-source-library.mjs';
if (process.argv.slice(2).join(' ') !== '--draft') throw new TypeError('use --draft for source-only authoring');
draftSourceLibrary({ sourcePath: 'js/src/translation/javascript-identifier-rules.js', moduleName: 'javascript_identifier_rules',
  model: 'JavaScript Unicode identifier escapes and strict module roles 1', programAsset: 'javascript-identifier-program' });
