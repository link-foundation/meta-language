// Actual normative source generation only; never invoke heap operations/tests.
import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/javascript-ordinary-heap.js',moduleName:'javascript_ordinary_heap',
 model:'ordinary JavaScript global identity heap and prototype property transitions 1',programAsset:'javascript-ordinary-heap-program',sourceDependencies:true});
