import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/javascript-nullish-field-rules.js',moduleName:'javascript_nullish_field_rules',sourceDependencies:true,model:'ordinary JavaScript properties, computed-key errors and nullable receivers 3',programAsset:'javascript-nullish-field-program'});
