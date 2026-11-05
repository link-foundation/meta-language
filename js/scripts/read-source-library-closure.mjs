// Filesystem author adapter. Supplied source/CST composition itself is shipped
// in src and is the same implementation used to authenticate runtime reloads.
import {readFileSync,realpathSync} from 'node:fs';
import {relative,sep} from 'node:path';
import {fileURLToPath} from 'node:url';
import {captureSourceLibraryClosure,sourceLibraryLogicalModuleURL,sourceLibraryLogicalModulePath} from '../src/translation/source-library-closure.js';
export {readSourceLibraryModuleFacts,readSuppliedSourceLibraryClosure,readSourceLibraryDeclarationOwnership} from '../src/translation/source-library-closure.js';
const root=new URL('../../',import.meta.url),rootPath=realpathSync(fileURLToPath(root));
export function readSourceLibraryClosure(sourcePath,options={}){
 return captureSourceLibraryClosure(sourcePath,options,url=>{
  const logical=sourceLibraryLogicalModulePath(url),file=new URL(logical.split('/').map(encodeURIComponent).join('/'),root),absolute=realpathSync(fileURLToPath(file)),path=relative(rootPath,absolute).split(sep).join('/');
  if(path==='..'||path.startsWith('../'))throw new TypeError('source library dependencies must remain in the actual project');
  return {identity:absolute,url:sourceLibraryLogicalModuleURL(path),path,source:readFileSync(absolute,'utf8')};
 });
}
