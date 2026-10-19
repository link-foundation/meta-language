// Experimental mandatory-facet adapter. The real production package is unchanged.
import {checkOwnedProgramContainers} from '../../js/src/translation/owned-program-containers.js';
import {acceptRustOwnedLocalContainer} from '../../js/src/translation/rust-owned-local-state-rules.js';
import {checkPortableRustTextCertificate,createPortableRustTextCertificate} from '../rust-text-certificate/bridge.mjs';
import {emitRustTextValueProjection} from '../rust-text-value-projection/model.mjs';
import {encodeGeneratedOwnedRust,decodeGeneratedOwnedRust} from '../generated-owned-rust/codec.mjs';
import {RUST_TEXT_VALUE_FACET} from './policy.mjs';
export {decodeRustTextCertificateFacet} from './raw-identity.mjs';
const fail=reason=>{throw new TypeError('Owned Rust text certificate adapter: '+reason);};
export function readOwnedRustTextCertificate(program){
 // The checked Program boundary is a data record even before selection. Do
 // not read inherited/accessor source facts from a function or host instance.
 if(!program||typeof program!=='object'||Array.isArray(program)||![Object.prototype,null].includes(Object.getPrototypeOf(program)))fail('the actual program root must be a plain data record');
 const main=Object.getOwnPropertyDescriptor(program,'main');
 if(main&&(!Object.hasOwn(main,'value')||main.value!=null&&(typeof main.value!=='object'||Array.isArray(main.value)||![Object.prototype,null].includes(Object.getPrototypeOf(main.value)))))fail('the actual main container must be a plain data record');
 const selected=checkOwnedProgramContainers(program,{selectedFields:[RUST_TEXT_VALUE_FACET],acceptContainer:acceptRustOwnedLocalContainer,fail});
 if(!selected)return null;
 if(!Object.hasOwn(program,RUST_TEXT_VALUE_FACET))fail('the owned source facet is misplaced outside the actual program root');
 // Remove exactly the adapter-owned root field. Every other descriptor and
 // every nested Fn/Param/signature/core/main/proof reference remains actual.
 const view=Object.create(Object.getPrototypeOf(program));
 for(const key of Reflect.ownKeys(program))if(key!==RUST_TEXT_VALUE_FACET)Object.defineProperty(view,key,Object.getOwnPropertyDescriptor(program,key));
 const proof=program[RUST_TEXT_VALUE_FACET],checked=checkPortableRustTextCertificate(view,proof);return{program:view,proof,...checked};
}
export function checkOwnedRustTextCertificate(program){return readOwnedRustTextCertificate(program)!==null;}
export function captureOwnedRustTextCertificate(source,options){const value=createPortableRustTextCertificate(source,options);value.program[RUST_TEXT_VALUE_FACET]=value.proof;readOwnedRustTextCertificate(value.program);return value.program;}
export function emitOwnedRustTextCertificate(program,target,options={}){
 const owned=readOwnedRustTextCertificate(program);if(!owned)fail('the explicit owned source facet is absent');
 if(options.sourceHeaderModel!==undefined||options.scopedBindingModel!==undefined)fail('an independent target interface model requires a separate composition proof');
 // Authenticate original owners first, then reuse the actual Links codec as
 // an emission-only data-record representation bridge. Null prototypes have
 // the same linked record meaning; all aliases/order survive this snapshot.
 const packet=encodeGeneratedOwnedRust(owned.program,owned.proof),snapshot=decodeGeneratedOwnedRust(packet.network,packet.root);
 checkPortableRustTextCertificate(snapshot.program,snapshot.proof);
 // Capture and replay remain existing JavaScript adapters, explicitly not native.
 const emitted=emitRustTextValueProjection(snapshot.program,snapshot.proof,target);
 return{...emitted,mappings:emitted.mappings??[...snapshot.program.declarations.values()].map(entry=>({kind:'function',source:entry.fullName,target:entry.name,sourceSpan:entry.span})),encodings:emitted.encodings??[],theorems:emitted.theorems??[]};
}
