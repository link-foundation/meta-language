// Filesystem author adapter. Supplied source/CST composition itself is shipped
// in src and is the same implementation used to authenticate runtime reloads.
import {readFileSync,realpathSync} from 'node:fs';
import {dirname,posix,relative,sep} from 'node:path';
import {fileURLToPath} from 'node:url';
import {captureSourceLibraryClosure,sourceLibraryLogicalModuleURL,sourceLibraryLogicalModulePath} from '../src/translation/source-library-closure.js';
import {readProjectData} from './project-data.mjs';
export {readSourceLibraryModuleFacts,readSuppliedSourceLibraryClosure,readSourceLibraryDeclarationOwnership} from '../src/translation/source-library-closure.js';
const root=new URL('../../',import.meta.url),rootPath=realpathSync(fileURLToPath(root));
let portablePackageImports;
function readPortablePackageImports(){
 const registration=readProjectData(new URL('parity/self-translation/source-package-imports.lino',root));
 if(registration.schemaVersion!==1||registration.model!=='Portable source package import resolution 1'||registration.condition!=='meta-language-portable-source'||!Array.isArray(registration.packages)||registration.packages.length>16)
  throw new TypeError('portable source package imports require their registered Links model and condition');
 return registration.packages.map(rule=>{
  const {manifest,importer,specifier,binding,provider,defaultProvider}=rule;
  sourceLibraryLogicalModuleURL(manifest);
  const target=JSON.parse(readFileSync(new URL(manifest,root),'utf8')).imports?.[specifier];
  const scope=dirname(manifest).split(sep).join('/')+'/';
  const isTarget=(path,mapping)=>path.startsWith(scope)&&mapping==='./'+posix.relative(scope,path);
  const keys=Object.keys(target??{});
  if(keys.length!==2||keys[0]!==registration.condition||keys[1]!=='default'||
    !importer.startsWith(scope)||!isTarget(provider,target[registration.condition])||!isTarget(defaultProvider,target.default))
   throw new TypeError('the portable package import differs from its manifest and exact runtime providers: '+specifier);
  return {specifier,importer,binding,provider,condition:registration.condition,manifest,defaultProvider};
 });
}
export function readSourceLibraryClosure(sourcePath,options={}){
 portablePackageImports??=readPortablePackageImports();
 return captureSourceLibraryClosure(sourcePath,{...options,packageImports:portablePackageImports},url=>{
  const logical=sourceLibraryLogicalModulePath(url),file=new URL(logical.split('/').map(encodeURIComponent).join('/'),root),absolute=realpathSync(fileURLToPath(file)),path=relative(rootPath,absolute).split(sep).join('/');
  if(path==='..'||path.startsWith('../'))throw new TypeError('source library dependencies must remain in the actual project');
  return {identity:absolute,url:sourceLibraryLogicalModuleURL(path),path,source:readFileSync(absolute,'utf8')};
 });
}
