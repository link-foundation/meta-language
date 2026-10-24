import { draftSourceLibrary } from './draft-source-library.mjs';
if (process.argv.slice(2).join(' ') !== '--draft') throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/scalar-assignment-rules.js',moduleName:'scalar_assignment_rules',
  sourceDependencies:true,model:'ordered scalar assignment expression cells 1',programAsset:'scalar-assignment-program'});
