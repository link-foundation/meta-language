import { draftSourceLibrary } from './draft-source-library.mjs';

if (process.argv.slice(2).join(' ') !== '--draft') throw new TypeError('use --draft');
draftSourceLibrary({
  sourcePath: 'js/src/translation/javascript-constructor-field-type-rules.js',
  moduleName: 'javascript_constructor_field_type_rules',
  model: 'JavaScript tagged constructor field type boundaries 1',
  sourceDependencies: true,
  nativeEntryPoints: 'source module exports 1',
});
