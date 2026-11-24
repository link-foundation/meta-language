import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/source-construct-signal-rules.js',moduleName:'source_construct_signal_rules',model:'literal source construct candidate statistics 1',programAsset:'source-construct-signal-program'});
