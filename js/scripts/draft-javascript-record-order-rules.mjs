import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/javascript-record-order-rules.js',moduleName:'javascript_record_order_rules',sourceDependencies:true,model:'canonical layouts and independent JavaScript own-field value orders 1',programAsset:'javascript-record-order-program'});
