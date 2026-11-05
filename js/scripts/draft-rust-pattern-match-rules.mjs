import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/rust-pattern-match-rules.js',moduleName:'rust_pattern_match_rules',sourceDependencies:true,model:'ordinary Rust standard pattern macro expansion 1',programAsset:'rust-pattern-match-program'});
