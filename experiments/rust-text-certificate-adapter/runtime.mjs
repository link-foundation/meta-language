import {readProjectData} from '../../js/scripts/project-data.mjs';
import {createCheckedModuleOverlay} from './overlay.mjs';
export async function createRustTextCertificatePublicOverlay(){
 const sourceRoot=new URL('../../js/src/',import.meta.url),virtual=new URL('rust-text-certificate-adapter.js',sourceRoot).href;
 const overlay=await createCheckedModuleOverlay({records:readProjectData(new URL('./data/overlays-current.lino',import.meta.url)),sourceRoot,virtualModules:new Map([[virtual,new URL('./model.mjs',import.meta.url).href]])});
 try{return{...overlay,api:await overlay.importModule('typed-program-links.js'),javascript:await overlay.importModule('translation/emit-javascript.js'),rust:await overlay.importModule('translation/emit-rust.js'),validation:await overlay.importModule('typed-program-links-validation.js'),projection:await overlay.importModule('typed-program-checked-projection.js')};}catch(error){overlay.dispose();throw error;}
}
