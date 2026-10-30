import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/conditional-constructor-sharing-rules.js',moduleName:'conditional_constructor_sharing_rules',
 model:'complete finite value constructor choices 1',programAsset:'conditional-constructor-sharing-program'});
