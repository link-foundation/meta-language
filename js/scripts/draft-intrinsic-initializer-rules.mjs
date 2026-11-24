import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/intrinsic-initializer-rules.js',moduleName:'intrinsic_initializer_rules',
 model:'complete intrinsic allocation and deferred body module scope interpretation 1',sourceDependencies:true});
