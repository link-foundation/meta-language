import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/unsigned-graph-command-rules.js',moduleName:'unsigned_graph_command_rules',directory:'',sourceDependencies:true,model:'exact saved graph command and actual typed root selection 1',programAsset:'unsigned-graph-command-program',programEntryPoints:'source module exports 1',nativeEntryPoints:'source module exports 1'});
