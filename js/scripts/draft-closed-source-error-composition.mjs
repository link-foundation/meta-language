// Complete real diagnostics dependency, semantic importer and source receipts.
// The model is interpreted by automatically self-translated shared kernels;
// no original source body or handwritten native Error definition is emitted.
import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'examples/closed-source-error-composition/entry.js',programAsset:'closed-source-error-composition-program',sourceDependencies:true,sourceAliases:true,sourceErrorClasses:'resolved supplied Error source modules 1'});
