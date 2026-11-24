import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/unsigned-source-command-rules.js',moduleName:'unsigned_source_command_rules',directory:'',sourceDependencies:true,model:'exact source command interpretation 1',programAsset:'unsigned-source-command-program',programEntryPoints:'source module exports 1',nativeEntryPoints:'source module exports 1'});
