// Actual source compilation only; no represented fixture is parsed or executed.
import './draft-boolean-constructor-match-rules.mjs';
import {draftSourceLibrary} from './draft-source-library.mjs';
import {draftNativeModelCodecs} from './draft-native-model-codecs.mjs';
import {readProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const result=draftSourceLibrary({sourcePath:'js/src/translation/javascript-call-natural-source.js',moduleName:'javascript_call_natural_source',model:'checked JavaScript natural source construction 1',programAsset:'javascript-call-natural-source-program',sourceDependencies:true,sourceAliases:true,sourceLints:'preserve',sourceFormatting:'preserve',valueSharing:'exact scalars, types and source spans 1',programEntryPoints:'source module exports 1',nativeEntryPoints:'source module exports 1',constructorMatchSharing:'closed constructor match sharing 1',recordShapes:'ordered typed program record fields 1'});
const definition=readProjectData(new URL('../../parity/translation/native-model-wire-rules.lino',import.meta.url));
if(definition.schemaVersion!==1||definition.model!=='JavaScript semantic model exact Number wire fields 1')throw new TypeError('actual semantic wire rules required');
draftNativeModelCodecs({...result,roots:['JavaScriptCallSource','JavaScriptCallSourceCompilation'],wireRules:definition.rules,output:'rust/tests/unit/javascript_call_natural_source_codecs.rs',receipt:'parity/self-translation/javascript-call-natural-source-model-codecs.lino',javascriptOutput:'experiments/javascript-properties/call-natural-source-codecs.mjs'});
