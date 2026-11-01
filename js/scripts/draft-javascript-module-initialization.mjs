// Future automatic source authoring; never run during the source-only phase.
import {draftSourceLibrary} from './draft-source-library.mjs';
import {draftNativeModelCodecs} from './draft-native-model-codecs.mjs';
import {readProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const result=draftSourceLibrary({
 sourcePath:'js/src/translation/javascript-module-initialization.js',
 moduleName:'javascript_module_initialization',
 model:'actual runtime Links policy initialization machine 1',
 programAsset:'javascript-module-initialization-program',
 sourceDependencies:true,sourceAliases:true,sourceLints:'preserve',sourceFormatting:'preserve',
 valueSharing:'exact scalars, types and source spans 1',
 programEntryPoints:'source module exports 1',nativeEntryPoints:'source module exports 1',
 constructorMatchSharing:'closed constructor match sharing 1',
 conditionalConstructorSharing:'closed value constructor selection 1',
 recordShapes:'ordered typed program record fields 1',
});
const definition=readProjectData(new URL('../../parity/translation/native-model-wire-rules.lino',import.meta.url));
if(definition.schemaVersion!==1||definition.model!=='JavaScript semantic model exact Number wire fields 1')throw new TypeError('actual qualified semantic model wire rules are required');
draftNativeModelCodecs({...result,
 roots:['JavaScriptModuleInitializationPlan','JavaScriptModuleInitializationState','JavaScriptModuleInitializationTransition','JavaScriptModuleBindingRead'],
 wireRules:definition.rules,
 output:'rust/tests/unit/javascript_module_initialization_codecs.rs',
 receipt:'parity/self-translation/javascript-module-initialization-model-codecs.lino',
 javascriptOutput:'experiments/javascript-properties/module-initialization-codecs.mjs',
});
