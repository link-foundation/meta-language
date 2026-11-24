import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/pattern-alias-rules.js',moduleName:'pattern_alias_rules',sourceDependencies:true,model:'actual matched column pattern aliases 1',programAsset:'pattern-alias-program'});
