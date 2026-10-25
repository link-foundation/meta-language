import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/binding-initialization-rules.js',moduleName:'binding_initialization_rules',sourceDependencies:true,model:'actual source initialization boundaries for nameless binding patterns 1',programAsset:'binding-initialization-program'});
