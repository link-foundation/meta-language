import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/absence-only-rules.js',moduleName:'absence_only_rules',model:'JavaScript absence-only values and nullish fallback 1',programAsset:'absence-only-program'});
