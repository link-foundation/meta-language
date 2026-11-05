import{draftSourceLibrary}from'./draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft for source-only authoring');
draftSourceLibrary({sourcePath:'js/src/translation/javascript-property-call-rules.js',moduleName:'javascript_property_call_rules',
 model:'actual JavaScript property call owner and concatenation 2',programAsset:'javascript-property-call-program'});
