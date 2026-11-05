import {draftSourceRuntime} from './draft-source-runtime.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceRuntime({sourcePath:'js/src/translation/javascript-record-order-rules.js',moduleName:'javascript_record_order_rules',sourceDependencies:true,
 model:'canonical layouts and independent JavaScript own-field value orders 1',programAsset:'javascript-record-order-program',
 runtimeStem:'record-property-order',constantPrefix:'RECORD_PROPERTY_ORDER',formalNamespace:'MLRecordPropertyOrder',
 entries:[['create','createJavaScriptRecordPropertyOrder'],['merge','mergeJavaScriptRecordPropertyOrder'],['rebase','rebaseJavaScriptRecordPropertyOrder'],['keys','readJavaScriptRecordPropertyKeys'],['valid','acceptJavaScriptRecordPropertyOrder']]});
