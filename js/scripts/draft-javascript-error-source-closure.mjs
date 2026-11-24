// Deferred semantic sourceRule author. It captures and links complete modules;
// the shared Error source compiler/call machine is self-translated separately.
import {mkdirSync} from 'node:fs';
import {readSourceLibraryClosure} from './read-source-library-closure.mjs';
import {captureSourceLibraryErrorClosure} from './source-library-errors.mjs';
import {writeProjectData} from './project-data.mjs';
const root=new URL('../../',import.meta.url);
export function draftJavaScriptErrorSourceClosure({sourcePath,programAsset,sourceDependencies=false,sourceAliases=false,sourceClasses='',sourceErrorClasses='',sourceReExports=''}={}){
 if(sourceErrorClasses!=='resolved supplied Error source modules 1'||sourceDependencies!==true||sourceAliases!==true||sourceClasses!==''||typeof sourcePath!=='string'||typeof programAsset!=='string'||!/^[a-z][a-z0-9-]*$/u.test(programAsset))throw new TypeError('semantic Error source assets need their actual supplied module/alias profile and exact asset identity');
 const closure=readSourceLibraryClosure(sourcePath,{aliases:true,errorClasses:sourceErrorClasses,reExports:sourceReExports}),archive=captureSourceLibraryErrorClosure(closure);
 for(const runtime of ['js','rust']){mkdirSync(new URL(runtime+'/src/data/',root),{recursive:true});writeProjectData(new URL(runtime+'/src/data/'+programAsset+'.lino',root),archive);}
 return {kind:'semantic Error source closure',archive,programAsset};
}
