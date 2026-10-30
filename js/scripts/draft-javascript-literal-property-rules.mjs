import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/javascript-literal-property-rules.js',moduleName:'javascript_literal_property_rules',sourceDependencies:true,model:'literal property key classification and UTF16 indexed string reads 1',programAsset:'javascript-literal-property-program'});
