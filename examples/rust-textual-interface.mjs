// Run after the complete requirements draft and deferred verification phase.
import {createRustSourceOwnerProgramLinks,emitTypedProgramLinks} from '../js/src/index.js';
const packet=createRustSourceOwnerProgramLinks('pub fn size(text:&str)->usize{text.len()}fn main(){}');
const result=emitTypedProgramLinks(packet.network,packet.root,'Rust',{rustSourceHeaderEmissionModel:'retained textual Rust function interface 1'});
console.log(result.code);
