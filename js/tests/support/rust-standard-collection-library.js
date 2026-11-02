// Future controls use the same public automatic binding derivation as consumers.
// No source compilation/projection occurs until a test explicitly calls this.
import {createHash} from 'node:crypto';
import {readSourceLibraryClosure,readSourceLibraryDeclarationOwnership} from '../../scripts/read-source-library-closure.mjs';
import {parseJavaScript} from '../../src/translation/javascript.js';
import {checkProgram} from '../../src/translation/check.js';
import {encodeTypedProgramLinks} from '../../src/typed-program-links.js';
import {createRustStandardCollectionSourceLibrary} from '../../src/translation/rust-standard-collection-library.js';
let currentAsset,currentLibrary;
export function readRustStandardCollectionTestAsset(){
 if(!currentAsset){
  const entryPath='js/src/translation/rust-standard-collection-value-packet.js',closure=readSourceLibraryClosure(entryPath,{aliases:true}),program=checkProgram(parseJavaScript(closure.source)),linked=encodeTypedProgramLinks(program),text=linked.network.toLino();
  currentAsset={text,receipt:{model:'Complete source library asset receipt 1',schemaVersion:1,entryPath,referenceSemantics:false,sourceClosure:closure,sourceDeclarationOwners:readSourceLibraryDeclarationOwnership(closure,program),programAsset:{path:'js/src/data/rust-standard-collection-value-packet-program.lino',bytes:Buffer.byteLength(text),sha256:createHash('sha256').update(text).digest('hex')}}};
 }
 return structuredClone(currentAsset);
}
export function readRustStandardCollectionSourceTestLibrary(){
 if(!currentLibrary){const {text,receipt}=readRustStandardCollectionTestAsset();currentLibrary=createRustStandardCollectionSourceLibrary(text,receipt,{invalid:()=>({k:'abort',message:'invalid actual collection source packet'}),resource:()=>({k:'abort',message:'actual collection source resource limit'})});}
 return currentLibrary;
}
