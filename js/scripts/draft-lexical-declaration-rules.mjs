import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/lexical-declaration-rules.js',moduleName:'lexical_declaration_rules',model:'actual lexical declaration early errors 1',sourceDependencies:true,programAsset:'lexical-declaration-program'});
