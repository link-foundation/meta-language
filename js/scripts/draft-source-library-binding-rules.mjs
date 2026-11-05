// Compile actual source-authoring policies; no fixture or represented execution.
import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
for(const [sourcePath,moduleName,programAsset,model]of [
 ['js/src/translation/source-library-binding-rules.js','source_library_binding_rules','source-library-binding-program','actual source module function bindings 1'],
 ['js/src/translation/source-library-export-rules.js','source_library_export_rules','source-library-export-program','actual source module function exports 1']
]){
 draftSourceLibrary({sourcePath,moduleName,programAsset,model,sourceDependencies:true,sourceAliases:true,
  programEntryPoints:'source module exports 1',nativeEntryPoints:'source module exports 1'});
 globalThis.gc?.();
}
