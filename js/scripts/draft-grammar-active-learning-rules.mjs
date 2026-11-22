import { draftSourceLibrary } from './draft-source-library.mjs';

if (process.argv.slice(2).join(' ') !== '--draft') throw new TypeError('use --draft');
draftSourceLibrary({
  sourcePath: 'js/src/grammar-active-learning-rules.js',
  moduleName: 'grammar_active_learning_rules',
  directory: '',
  model: 'ordered regular grammar observation table 1',
  programAsset: 'grammar-active-learning-program',
});
