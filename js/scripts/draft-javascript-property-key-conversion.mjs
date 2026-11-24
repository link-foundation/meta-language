// Translate actual normative source, then derive model I/O; never run coercion.
import './draft-javascript-primitive-conversion-spellings.mjs';
import {draftSourceLibrary} from './draft-source-library.mjs';
import {draftNativeModelCodecs} from './draft-native-model-codecs.mjs';
import {readProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const result=draftSourceLibrary({sourcePath:'js/src/translation/javascript-property-key-conversion.js',moduleName:'javascript_property_key_conversion',
 model:'ordinary JavaScript resumable ToPropertyKey 1',programAsset:'javascript-property-key-conversion-program',sourceDependencies:true});
const definition=readProjectData(new URL('../../parity/translation/native-model-wire-rules.lino',import.meta.url));
if(definition.schemaVersion!==1||definition.model!=='JavaScript semantic model exact Number wire fields 1')throw new TypeError('actual qualified Links Number rules are required');
draftNativeModelCodecs({...result,roots:['JavaScriptOrdinaryHeap','JavaScriptSemanticValue','JavaScriptPrimitiveConversionState','JavaScriptPrimitiveConversionCompletion','JavaScriptPropertyKeyConversionTransition'],wireRules:definition.rules,
 output:'rust/tests/unit/javascript_property_key_conversion_codecs.rs',receipt:'parity/self-translation/javascript-property-key-conversion-model-codecs.lino'});
