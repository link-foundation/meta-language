// Future publication of source products only. No represented program executes.
import {draftRustPackageSourceAssets} from './rust-package-source-assets.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
process.argv.splice(2,process.argv.length-2,'--draft','--reuse-roots');
try{
 await import('./draft-source-definition-catalog.mjs');globalThis.gc?.();
 draftRustPackageSourceAssets();
}finally{process.argv.splice(2,process.argv.length-2,'--draft');}
