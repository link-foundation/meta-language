import { draftSourceLibrary } from './draft-source-library.mjs';
if (process.argv.slice(2).join(' ') !== '--draft') throw new TypeError('use --draft for source-only authoring');
draftSourceLibrary({ sourcePath: 'js/src/translation/source-identifier-rules.js', moduleName: 'source_identifier_rules',
  model: 'capture free source generated and target identifier namespaces 1', programAsset: 'source-identifier-program' });
