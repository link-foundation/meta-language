import{draftSourceLibrary}from'./draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft for source-only authoring');
draftSourceLibrary({sourcePath:'js/src/link-allocation-rules.js',moduleName:'link_allocation_rules',directory:'',
 model:'actual unsigned 64-bit link allocation capacity 1',programAsset:'link-allocation-program'});
