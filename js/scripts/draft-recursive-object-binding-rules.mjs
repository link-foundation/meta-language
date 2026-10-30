import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/recursive-object-binding-rules.js',moduleName:'recursive_object_binding_rules',sourceDependencies:true,model:'actual recursive object pattern ownership and initialization 1',programAsset:'recursive-object-binding-program'});
