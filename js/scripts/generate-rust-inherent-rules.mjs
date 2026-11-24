import{draftSourceLibrary}from'./draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft for source-only authoring');
draftSourceLibrary({sourcePath:'js/src/translation/rust-inherent-rules.js',moduleName:'rust_inherent_rules',
 model:'pure owned Rust inherent function and immutable receiver 1',programAsset:'rust-inherent-program'});
