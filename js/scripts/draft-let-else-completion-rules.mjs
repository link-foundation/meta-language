import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/let-else-completion-rules.js',moduleName:'let_else_completion_rules',sourceDependencies:true,model:'Rust let-else ordinary completed fallback and lexical continuation 1',programAsset:'let-else-completion-program'});
