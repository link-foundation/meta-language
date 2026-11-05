// Draft demonstration: node examples/rust-vector-copies.mjs [Rust|JavaScript|Lean|Rocq|Links]
import {createRustSourceOwnerProgramLinks,emitTypedProgramLinks} from '../js/src/index.js';
const packet=createRustSourceOwnerProgramLinks('pub fn copied(values:Vec<f64>)->Vec<f64>{values.clone()}fn main(){}');
const target=process.argv[2]??'Rust';
process.stdout.write(target==='Links'?packet.network.toLino():emitTypedProgramLinks(packet.network,packet.root,target).code);
