import { draftSourceLibrary } from './draft-source-library.mjs';
if (process.argv.slice(2).join(' ') !== '--draft') throw new TypeError('use --draft for source-only authoring');
draftSourceLibrary({ sourcePath: 'js/src/translation/reference-array-projection-rules.js', moduleName: 'reference_array_projection_rules',
  model: 'shared array scalar projection without payload copies 1', programAsset: 'reference-array-projection-program' });
