// Transport actual provenance; retain reference wrappers in callable signatures.
import {acceptRustSourceSignaturePrimitive} from './rust-source-signature-rules.js';
import {acceptRustCopyClosureParameterSource} from './rust-copy-closure-parameter-rules.js';
export function acceptRustCopyClosureParameterOrigin(parser,origin){
 const owner=origin?.expandedSourceOwner??origin?.sourceOwner,type=origin?.type;
 const primitive=Boolean(owner&&owner.arguments.length===0&&(owner.kind==='unit'&&owner.path.length===0&&type?.kind==='unit'||owner.kind==='primitive'&&acceptRustSourceSignaturePrimitive(type?.kind??'',owner.path.join('::'),String(type?.bits??''),type?.signed===true)));
 return acceptRustCopyClosureParameterSource(origin?.proven===true,origin?parser.origins.immutableProduct({...origin,wrappers:[]}):false,origin?.wrappers.length??-1,origin?.wrappers.every(layer=>layer==='reference')===true,primitive);
}
