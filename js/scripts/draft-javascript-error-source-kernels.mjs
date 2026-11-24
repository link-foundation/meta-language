// Future source generation only; never execute during source-only drafting.
import {draftSourceLibrary} from './draft-source-library.mjs';
import {draftNativeModelCodecs} from './draft-native-model-codecs.mjs';
import {readProjectData} from './project-data.mjs';
const [mode, moduleName, ...extra] = process.argv.slice(2);
const specifications = [
 {sourcePath:'js/src/translation/javascript-error-construction-rules.js',moduleName:'javascript_error_construction_rules',model:'Authoritative JavaScript Error value construction 1',programAsset:'javascript-error-construction-program'},
 {sourcePath:'js/src/translation/javascript-error-class-rules.js',moduleName:'javascript_error_class_rules',model:'Actual intrinsic Error subclass source ownership 1',programAsset:'javascript-error-class-program'},
 {sourcePath:'js/src/translation/javascript-error-class-source.js',moduleName:'javascript_error_class_source',model:'Actual Error subclass plans to registered call code 1',programAsset:'javascript-error-class-source-program'},
 {sourcePath:'js/src/translation/javascript-collection-call-source.js',moduleName:'javascript_collection_call_source',model:'Inspectable JavaScript standard collection call compilation 1',programAsset:'javascript-collection-call-source-program',referenceSemantics:true},
];
if(mode!=='--draft'||extra.length||moduleName!==undefined&&!specifications.some(specification=>specification.moduleName===moduleName))throw new TypeError('use --draft with an optional registered module name');
for(const specification of specifications){
 if(moduleName!==undefined&&specification.moduleName!==moduleName)continue;
 const conditionalConstructorSharing=specification.moduleName.endsWith('_rules')?'closed value constructor selection 1':'';
 const constructorMatchSharing=specification.moduleName==='javascript_error_class_source'?'closed constructor match with direct calls sharing 1':'closed constructor match sharing 1';
 const result=draftSourceLibrary({...specification,sourceDependencies:true,sourceAliases:true,sourceLints:'preserve',sourceFormatting:'preserve',valueSharing:'exact scalars, types and source spans 1',programEntryPoints:'source module exports 1',nativeEntryPoints:'source module exports 1',constructorMatchSharing,conditionalConstructorSharing,recordShapes:'ordered typed program record fields 1'});
 if(specification.moduleName==='javascript_error_class_source'){
  const definition=readProjectData(new URL('../../parity/translation/native-model-wire-rules.lino',import.meta.url));
  if(definition.schemaVersion!==1||definition.model!=='JavaScript semantic model exact Number wire fields 1')throw new TypeError('actual qualified semantic model wire rules are required');
  draftNativeModelCodecs({...result,roots:['JavaScriptErrorModulePlan','JavaScriptErrorModuleRegistry','JavaScriptErrorModuleCompilation','JavaScriptErrorModuleBinding','JavaScriptSourceCapabilityOwner'],wireRules:definition.rules,referenceWire:'source value tree 1',output:'rust/tests/unit/javascript_error_class_source_codecs.rs',receipt:'parity/self-translation/javascript-error-class-source-model-codecs.lino',javascriptOutput:'experiments/javascript-properties/error-class-source-codecs.mjs'});
 }
 if(specification.moduleName==='javascript_collection_call_source'){
  const definition=readProjectData(new URL('../../parity/translation/native-model-wire-rules.lino',import.meta.url));
  if(definition.schemaVersion!==1||definition.model!=='JavaScript semantic model exact Number wire fields 1')throw new TypeError('actual qualified semantic model wire rules are required');
  draftNativeModelCodecs({...result,roots:['JavaScriptCollectionCallableDirectory','JavaScriptCollectionCallCompilation'],wireRules:definition.rules,referenceWire:'source value tree 1',output:'rust/tests/unit/javascript_collection_call_source_codecs.rs',receipt:'parity/self-translation/javascript-collection-call-source-model-codecs.lino',javascriptOutput:'experiments/javascript-properties/collection-call-source-codecs.mjs'});
 }
 globalThis.gc?.();
}
