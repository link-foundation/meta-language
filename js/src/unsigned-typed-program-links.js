// Exact-ID transport around the existing checked semantic Links program.
import {UnsignedLinkId,rebaseUnsignedLinkNetwork,createDenseUnsignedLinkNetwork} from './unsigned-link-network.js';
import {createTypedProgramLinks,emitTypedProgramLinks} from './typed-program-links.js';
import {createRustSourceOwnerProgramLinks} from './rust-source-owner-program.js';
/** Preserve an explicit typed root together with every original graph identity. */
export function rebaseUnsignedTypedProgramLinks(packet,firstIdentity=1,options={}){
 const root=UnsignedLinkId.from(packet.root),rebased=rebaseUnsignedLinkNetwork(packet.network,firstIdentity,options),index=rebased.sourceIdentities.findIndex(id=>id.equals(root));
 if(index<0||!packet.network.link(packet.root))throw new TypeError('an unsigned typed program needs its actual stored source root');
 return {...rebased,root:rebased.identities[index]};
}
/** Ordinary parser/checker semantics and source options are passed unchanged. */
export function createUnsignedTypedProgramLinks(source,language,sourceOptions={},identityOptions={}){
 return rebaseUnsignedTypedProgramLinks(createTypedProgramLinks(source,language,sourceOptions),identityOptions.firstIdentity??1,identityOptions);
}
/** Actual Rust source type owners and selected source version stay in Links. */
export function createUnsignedRustSourceOwnerProgramLinks(source,sourceOptions={},identityOptions={}){
 return rebaseUnsignedTypedProgramLinks(createRustSourceOwnerProgramLinks(source,sourceOptions),identityOptions.firstIdentity??1,identityOptions);
}
/** Emit only from current linked semantics. Exact-to-local coordinates are
 * explicit, and the existing emitter rechecks the complete current graph. */
export function emitUnsignedTypedProgramLinks(network,root,target,emissionOptions={},identityOptions={}){
 const actual=UnsignedLinkId.from(root),view=createDenseUnsignedLinkNetwork(network,identityOptions),index=view.directory.findIndex(id=>id.equals(actual));
 if(index<0||!network.link(actual))throw new TypeError('an unsigned typed program needs its actual stored source root');
 return emitTypedProgramLinks(view.network,index+1,target,emissionOptions);
}
