// Actual source generation only, never descriptor conversions or call controls.
import './draft-javascript-descriptor-field-rules.mjs';
import {draftSourceLibrary} from './draft-source-library.mjs';
import {draftNativeModelCodecs} from './draft-native-model-codecs.mjs';
import {readProjectData,writeProjectData} from './project-data.mjs';
import {existsSync} from 'node:fs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const result=draftSourceLibrary({sourcePath:'js/src/translation/javascript-descriptor-conversion.js',moduleName:'javascript_descriptor_conversion',
 model:'ordinary JavaScript resumable ToPropertyDescriptor 1',programAsset:'javascript-descriptor-conversion-program',sourceDependencies:true});
const file=new URL('../../parity/translation/native-model-wire-rules.lino',import.meta.url);
if(!existsSync(file))writeProjectData(file,{schemaVersion:1,model:'JavaScript semantic model exact Number wire fields 1',rules:[{owner:'JavaScriptSemanticValue',constructor:'number',field:'value',wire:'bits',codec:'ieee754-binary64-hex'}]});
const definition=readProjectData(file);if(definition.schemaVersion!==1||definition.model!=='JavaScript semantic model exact Number wire fields 1'||!Array.isArray(definition.rules))throw new TypeError('native model transport needs its actual qualified Links rules');
draftNativeModelCodecs({...result,roots:['JavaScriptOrdinaryHeap','JavaScriptSemanticValue','JavaScriptDescriptorConversionState','JavaScriptSemanticCompletion','JavaScriptDescriptorConversionTransition'],wireRules:definition.rules,
 output:'rust/tests/unit/javascript_descriptor_conversion_codecs.rs',receipt:'parity/self-translation/javascript-descriptor-conversion-model-codecs.lino'});
