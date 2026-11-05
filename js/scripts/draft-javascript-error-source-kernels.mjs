// Future source generation only; never execute during source-only drafting.
import {draftSourceLibrary} from './draft-source-library.mjs';
import {draftNativeModelCodecs} from './draft-native-model-codecs.mjs';
import {readProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
for(const specification of [
 {sourcePath:'js/src/translation/javascript-error-construction-rules.js',moduleName:'javascript_error_construction_rules',model:'Authoritative JavaScript Error value construction 1',programAsset:'javascript-error-construction-program'},
 {sourcePath:'js/src/translation/javascript-error-class-rules.js',moduleName:'javascript_error_class_rules',model:'Actual intrinsic Error subclass source ownership 1',programAsset:'javascript-error-class-program'},
 {sourcePath:'js/src/translation/javascript-error-class-source.js',moduleName:'javascript_error_class_source',model:'Actual Error subclass plans to registered call code 1',programAsset:'javascript-error-class-source-program'},
 {sourcePath:'js/src/translation/javascript-collection-call-source.js',moduleName:'javascript_collection_call_source',model:'Inspectable JavaScript standard collection call compilation 1',programAsset:'javascript-collection-call-source-program'},
]){
 const result=draftSourceLibrary({...specification,sourceDependencies:true,sourceAliases:true,sourceLints:'preserve',sourceFormatting:'preserve',valueSharing:'exact scalars, types and source spans 1',programEntryPoints:'source module exports 1',nativeEntryPoints:'source module exports 1',constructorMatchSharing:'closed constructor match sharing 1',conditionalConstructorSharing:'closed value constructor selection 1',recordShapes:'ordered typed program record fields 1'});
 if(specification.moduleName==='javascript_error_class_source'){
  const definition=readProjectData(new URL('../../parity/translation/native-model-wire-rules.lino',import.meta.url));
  if(definition.schemaVersion!==1||definition.model!=='JavaScript semantic model exact Number wire fields 1')throw new TypeError('actual qualified semantic model wire rules are required');
  draftNativeModelCodecs({...result,roots:['JavaScriptErrorModulePlan','JavaScriptErrorModuleRegistry','JavaScriptErrorModuleCompilation','JavaScriptErrorModuleBinding','JavaScriptSourceCapabilityOwner'],wireRules:definition.rules,output:'rust/tests/unit/javascript_error_class_source_codecs.rs',receipt:'parity/self-translation/javascript-error-class-source-model-codecs.lino',javascriptOutput:'experiments/javascript-properties/error-class-source-codecs.mjs'});
 }
 if(specification.moduleName==='javascript_collection_call_source'){
  const definition=readProjectData(new URL('../../parity/translation/native-model-wire-rules.lino',import.meta.url));
  if(definition.schemaVersion!==1||definition.model!=='JavaScript semantic model exact Number wire fields 1')throw new TypeError('actual qualified semantic model wire rules are required');
  draftNativeModelCodecs({...result,roots:['JavaScriptCollectionCallableDirectory','JavaScriptCollectionCallCompilation'],wireRules:definition.rules,output:'rust/tests/unit/javascript_collection_call_source_codecs.rs',receipt:'parity/self-translation/javascript-collection-call-source-model-codecs.lino',javascriptOutput:'experiments/javascript-properties/collection-call-source-codecs.mjs'});
 }
 globalThis.gc?.();
}
