import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/absence-conversion-rules.js',moduleName:'absence_conversion_rules',sourceDependencies:true,model:'tag-preserving JavaScript absence-only optional conversion 1',programAsset:'absence-conversion-program'});
