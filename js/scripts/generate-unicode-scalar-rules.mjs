import { draftSourceLibrary } from './draft-source-library.mjs';
if (process.argv.slice(2).join(' ') !== '--draft') throw new TypeError('use --draft for source-only authoring');
draftSourceLibrary({ sourcePath: 'js/src/translation/unicode-scalar-rules.js', moduleName: 'unicode_scalar_rules',
  model: 'Unicode 17 scalar source identifier and canonical normalization 1', referenceSemantics: true, programAsset: 'unicode-scalar-program' });
