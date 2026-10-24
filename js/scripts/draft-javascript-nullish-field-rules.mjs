import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/javascript-nullish-field-rules.js',moduleName:'javascript_nullish_field_rules',sourceDependencies:true,model:'ordinary JavaScript property reads, method receivers and reference slice construction 2',programAsset:'javascript-nullish-field-program'});
