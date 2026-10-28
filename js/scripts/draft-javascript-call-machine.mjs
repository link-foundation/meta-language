// Parse/type/emit the actual call machine, without executing any program.
import './draft-javascript-primitive-conversion-spellings.mjs';
import './draft-javascript-descriptor-field-rules.mjs';
import {draftSourceLibrary} from './draft-source-library.mjs';
import {draftNativeModelCodecs} from './draft-native-model-codecs.mjs';
import {readProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const result=draftSourceLibrary({sourcePath:'js/src/translation/javascript-call-machine.js',moduleName:'javascript_call_machine',
 model:'ordinary JavaScript registered strict call execution 1',programAsset:'javascript-call-machine-program',sourceDependencies:true,sourceAliases:true,sourceLints:'preserve',sourceFormatting:'preserve',valueSharing:'exact scalars, types and source spans 1',programEntryPoints:'source module exports 1',recordShapes:'ordered typed program record fields 1'});
const definition=readProjectData(new URL('../../parity/translation/native-model-wire-rules.lino',import.meta.url));
if(definition.schemaVersion!==1||definition.model!=='JavaScript semantic model exact Number wire fields 1')throw new TypeError('actual qualified Links Number rules are required');
draftNativeModelCodecs({...result,roots:['JavaScriptOrdinaryHeap','JavaScriptSemanticValue','JavaScriptCallProgram','JavaScriptCallMachine','JavaScriptCallMachineTransition'],wireRules:definition.rules,
 output:'rust/tests/unit/javascript_call_machine_codecs.rs',receipt:'parity/self-translation/javascript-call-machine-model-codecs.lino',javascriptOutput:'experiments/javascript-properties/call-machine-codecs.mjs'});
