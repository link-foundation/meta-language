// Compile actual plan construction; never compile or execute fixture sources.
import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/javascript-fixed-integer-cast-plan.js',moduleName:'javascript_fixed_integer_cast_plan',model:'JavaScript constant width modular cast construction 1',programAsset:'javascript-fixed-integer-cast-program',sourceDependencies:true,sourceAliases:true,sourceLints:'preserve',sourceFormatting:'preserve',programEntryPoints:'source module exports 1',nativeEntryPoints:'source module exports 1'});
