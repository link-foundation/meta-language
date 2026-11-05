// Deferred shared-policy authoring recipe; never run during source drafting.
import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/source-library-class-rules.js',moduleName:'source_library_class_rules',model:'resolved supplied JavaScript class module ownership 1',programAsset:'source-library-class-program',sourceDependencies:true,sourceAliases:true,sourceLints:'preserve',sourceFormatting:'preserve',valueSharing:'exact scalars, types and source spans 1',programEntryPoints:'source module exports 1',nativeEntryPoints:'source module exports 1'});
