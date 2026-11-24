import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/unsigned-source-document-rules.js',moduleName:'unsigned_source_document_rules',directory:'',sourceDependencies:true,model:'exact unsigned source document coordinate allocation 1',programAsset:'unsigned-source-document-program',programEntryPoints:'source module exports 1',nativeEntryPoints:'source module exports 1'});
