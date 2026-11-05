import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/object-rest-rules.js',moduleName:'object_rest_rules',model:'actual closed own-field object rest selection 1',sourceDependencies:true,programAsset:'object-rest-program'});
