import { draftSourceLibrary } from './draft-source-library.mjs';
if (process.argv.slice(2).join(' ') !== '--draft') throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/callable-source-rules.js',moduleName:'callable_source_rules',
  sourceDependencies:true,model:'callable lexical reads writes and scalar expression cells 1',programAsset:'callable-source-program'});
