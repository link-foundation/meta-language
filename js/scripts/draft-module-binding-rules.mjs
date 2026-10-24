import { draftSourceLibrary } from './draft-source-library.mjs';
if (process.argv.slice(2).join(' ') !== '--draft') throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/module-binding-rules.js',moduleName:'module_binding_rules',sourceDependencies:true,sourceFormatting:'preserve',
  model:'actual JavaScript module name and lexical binding roles 1',programAsset:'module-binding-program'});
