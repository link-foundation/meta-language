import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/rust-numeric-pattern-rules.js',moduleName:'rust_numeric_pattern_rules',model:'ordinary Rust suffixed numeric literal patterns 1',programAsset:'rust-numeric-pattern-program',sourceDependencies:true});
