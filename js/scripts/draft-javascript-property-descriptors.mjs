// Source generation only. Do not invoke semantic operations or future controls.
import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/javascript-property-descriptors.js',moduleName:'javascript_property_descriptors',
 model:'normalized JavaScript semantic values and ordinary property descriptor transitions 1',
 programAsset:'javascript-property-descriptor-program',sourceDependencies:true});
