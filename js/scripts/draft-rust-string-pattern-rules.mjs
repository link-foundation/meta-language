import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/rust-string-pattern-rules.js',moduleName:'rust_string_pattern_rules',model:'ordinary Rust borrowed string literal patterns 1',programAsset:'rust-string-pattern-program'});
