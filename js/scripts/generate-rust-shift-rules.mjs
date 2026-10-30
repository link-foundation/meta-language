import {draftSourceLibrary}from'./draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft for source-only authoring');
draftSourceLibrary({sourcePath:'js/src/translation/rust-shift-rules.js',moduleName:'rust_shift_rules',
 model:'overflow checked Rust primitive shifts 1',programAsset:'rust-shift-program',sourceFormatting:'preserve',sourceLints:'preserve'});
