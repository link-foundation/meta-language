// Actual normative source/type/format construction, never coercion execution.
import './draft-javascript-primitive-conversion-spellings.mjs';
import {draftSourceLibrary} from './draft-source-library.mjs';
import {draftNativeModelCodecs} from './draft-native-model-codecs.mjs';
import {readProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const result=draftSourceLibrary({sourcePath:'js/src/translation/javascript-primitive-conversion.js',moduleName:'javascript_primitive_conversion',
 model:'ordinary JavaScript resumable ToPrimitive 1',programAsset:'javascript-primitive-conversion-program',sourceDependencies:true});
const definition=readProjectData(new URL('../../parity/translation/native-model-wire-rules.lino',import.meta.url));
if(definition.schemaVersion!==1||definition.model!=='JavaScript semantic model exact Number wire fields 1')throw new TypeError('actual qualified Links Number wire rules are required');
draftNativeModelCodecs({...result,roots:['JavaScriptOrdinaryHeap','JavaScriptSemanticValue','JavaScriptPrimitiveConversionState','JavaScriptPrimitiveConversionCompletion','JavaScriptPrimitiveConversionTransition'],wireRules:definition.rules,
 output:'rust/tests/unit/javascript_primitive_conversion_codecs.rs',receipt:'parity/self-translation/javascript-primitive-conversion-model-codecs.lino'});
