import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/javascript-own-property-rules.js',moduleName:'javascript_own_property_rules',model:'closed JavaScript own property membership and object coercion 1',sourceDependencies:true,programAsset:'javascript-own-property-program'});
