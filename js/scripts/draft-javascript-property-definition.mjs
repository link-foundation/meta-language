// Complete actual composed source. Preserve raw target style for this draft;
// formatting/lint obligations and every CI gate remain deferred and unchanged.
import './draft-javascript-primitive-conversion-spellings.mjs';
import './draft-javascript-descriptor-field-rules.mjs';
import {draftSourceLibrary} from './draft-source-library.mjs';
import {draftNativeModelCodecs} from './draft-native-model-codecs.mjs';
import {readProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const result=draftSourceLibrary({sourcePath:'js/src/translation/javascript-property-definition.js',moduleName:'javascript_property_definition',
 model:'ordinary JavaScript resumable Object.defineProperty 1',programAsset:'javascript-property-definition-program',sourceDependencies:true,sourceLints:'preserve',sourceFormatting:'preserve'});
const definition=readProjectData(new URL('../../parity/translation/native-model-wire-rules.lino',import.meta.url));
if(definition.schemaVersion!==1||definition.model!=='JavaScript semantic model exact Number wire fields 1')throw new TypeError('actual qualified Links Number rules are required');
draftNativeModelCodecs({...result,roots:['JavaScriptOrdinaryHeap','JavaScriptSemanticValue','JavaScriptPropertyDefinitionState','JavaScriptSemanticCompletion','JavaScriptPropertyDefinitionTransition'],wireRules:definition.rules,
 output:'rust/tests/unit/javascript_property_definition_codecs.rs',receipt:'parity/self-translation/javascript-property-definition-model-codecs.lino'});
