// Load automatically authored Links assets; no collection result is evaluated
// while constructing source interfaces. The complete source/body checker remains
// the authority, independently of receipt hashes and declaration spellings.
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {LinkNetwork} from '../network.js';
import {readProjectData} from '../project-data-files.js';
import {decodeTypedProgramLinks,TYPED_PROGRAM_LINKS_SCHEMA} from '../typed-program-links.js';
import {parseJavaScript} from './javascript.js';
import {readSuppliedSourceLibraryClosure,readSourceLibraryDeclarationOwnership} from './source-library-closure.js';
import {readRustStandardCollectionProjectedLibraryError,testRustStandardCollectionProjectedGraphEquality} from './rust-standard-collection-call-adapter.js';
const fail=reason=>{throw new TypeError('Rust standard collection library asset: '+reason);};
const entryPath='js/src/translation/rust-standard-collection-value-packet.js';
const sha256=text=>createHash('sha256').update(text).digest('hex');
function budget(value,name){if(!Number.isSafeInteger(value)||value<1)fail(name+' must be a positive safe integer');return value;}
/** Construct actual source interfaces from the complete generated asset. The
 * caller supplies the original asset text so byte/hash checks bind the graph
 * before decoding; no hand-authored function or constructor binding is accepted.
 * sourceHandlers must build typed Invalid/Resource continuations, never ignore
 * protocol errors or turn resource exhaustion into a successful value.
 */
export function createRustStandardCollectionSourceLibrary(programText,receipt,sourceHandlers,{byteLimit=48*1024*1024,nodeLimit=1000000,referenceLimit=4000000,depthLimit=256}={}){
 for(const [name,value]of Object.entries({byteLimit,nodeLimit,referenceLimit,depthLimit}))budget(value,name);
 if(typeof programText!=='string'||Buffer.byteLength(programText)>byteLimit)fail('complete program text exceeds its byte budget');
 if(receipt?.model!=='Complete source library asset receipt 1'||receipt.schemaVersion!==1||receipt.entryPath!==entryPath||receipt.referenceSemantics!==false)fail('a complete ordinary source receipt with the genuine entry owner is required');
 const asset=receipt.programAsset;if(asset?.path!=='js/src/data/rust-standard-collection-value-packet-program.lino'||asset.bytes!==Buffer.byteLength(programText)||asset.sha256!==sha256(programText))fail('program bytes differ from their source receipt');
 if(typeof sourceHandlers?.invalid!=='function'||typeof sourceHandlers?.resource!=='function')fail('distinct typed invalid and resource continuations are required');
 const supplied=receipt.sourceClosure;if(!Array.isArray(supplied?.inputs)||supplied.inputs.length>1024)fail('complete bounded source inputs are required');
 let bytes=0;for(const input of supplied.inputs){if(typeof input?.source!=='string'||typeof input.path!=='string')fail('every source input retains its actual text and logical path');bytes+=Buffer.byteLength(input.source);if(bytes>byteLimit)fail('source closure exceeds its byte budget');}
 const closure=readSuppliedSourceLibraryClosure(entryPath,supplied.inputs.map(({path,source})=>({path,source})),{aliases:true});
 if(closure.source!==supplied.source||closure.sourceSha256!==supplied.sourceSha256||closure.bytes!==supplied.bytes)fail('source closure differs from actual recomposition');
 const network=LinkNetwork.fromLino(programText),links=network.links();if(links.length>nodeLimit||links.reduce((sum,link)=>sum+link.references().length,0)>referenceLimit)fail('complete program graph exceeds its resource budget');
 const roots=links.filter(link=>link.metadata().term===TYPED_PROGRAM_LINKS_SCHEMA.root&&link.metadata().definition===String(TYPED_PROGRAM_LINKS_SCHEMA.schemaVersion));if(roots.length!==1)fail('one actual typed program root is required');
 const program=decodeTypedProgramLinks(network,roots[0].id(),{nodeLimit,depthLimit}),sourceProgram=parseJavaScript(closure.source),owners=readSourceLibraryDeclarationOwnership(closure,program);
 if(!testRustStandardCollectionProjectedGraphEquality(owners,receipt.sourceDeclarationOwners))fail('source declaration ownership differs from the complete actual program');
 function callable(sourceName,suffix,role){const matching=owners.filter(row=>(row.sourceName??row.name)===sourceName&&row.path?.endsWith(suffix));if(matching.length!==1)fail('one actual source callable owner is required for '+role);const name=matching[0].name,declaration=program.declarations.get(name);if(declaration?.k!=='fn'||!declaration.body)fail('complete '+role+' callable body is required');return {role,checkedName:name,sourcePath:['crate',...name.split('.')]};}
 const functions=[callable('executeRustStandardCollectionValuePacket','/rust-standard-collection-value-packet.js','packet'),callable('executeRustStandardCollectionOperation','/rust-standard-collection-state.js','kernel')],packet=program.declarations.get(functions[0].checkedName),type=packet.ret;
 if(type?.kind!=='data')fail('actual packet callable must return its concrete transition datatype');const declaration=program.declarations.get(type.name);if(declaration?.k!=='data'||declaration.ctors.length!==3)fail('complete packet transition datatype is required');
 const constructors={};for(const [role,fields]of [['normal',['values','writeback']],['invalid',['reason']],['resource',['state','reason']]]){const rows=declaration.ctors.filter(row=>row.fields.length===fields.length&&row.fields.every((field,index)=>field.name===fields[index]));if(rows.length!==1)fail('actual '+role+' transition fields are required');constructors[role]={checkedName:rows[0].name,sourceName:rows[0].name};}
 const library={entryPath,closure,program,sourceProgram,sourceHandlers,functions,packetType:{checkedName:type.name,sourcePath:['crate',...type.name.split('.')],constructors}};
 const error=readRustStandardCollectionProjectedLibraryError(library);if(error)fail(error);return library;
}
/** Default generated assets are intentionally unavailable until source
 * authoring has produced them. Missing/stale files refuse rather than replacing
 * their semantics with a host Map/Set or a declaration-only placeholder. */
export function loadRustStandardCollectionSourceLibrary(sourceHandlers,options={}){
 const programURL=new URL('../data/rust-standard-collection-value-packet-program.lino',import.meta.url),receiptURL=new URL('../data/rust-standard-collection-value-packet-program-source.lino',import.meta.url);
 return createRustStandardCollectionSourceLibrary(readFileSync(programURL,'utf8'),readProjectData(receiptURL,{limits:options}),sourceHandlers,options);
}
