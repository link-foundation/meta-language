import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/javascript-data-field-rules.js',moduleName:'javascript_data_field_rules',
 model:'JavaScript common datatype fields and own absence source projection 1',programAsset:'javascript-data-field-program'});
