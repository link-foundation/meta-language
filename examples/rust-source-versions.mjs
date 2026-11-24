// Inspect source-version-dependent translations without executing them.
import {createRustSourceOwnerProgramLinks,emitTypedProgramLinks} from '../js/src/index.js';
const source='pub fn result()->u8{u8::try_from(256u16).unwrap()}fn main(){}';
for(const sourceVersion of ['1.96.0','1.99.0']){
 const packet=createRustSourceOwnerProgramLinks(source,{sourceVersion});
 console.log('Rust source std '+sourceVersion);
 console.log(emitTypedProgramLinks(packet.network,packet.root,'JavaScript').code);
}
