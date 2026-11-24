import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/array-binding-rules.js',moduleName:'array_binding_rules',sourceDependencies:true,
 model:'JavaScript ordered array binding and undefined-only initializer selection 2',programAsset:'array-binding-program'});
