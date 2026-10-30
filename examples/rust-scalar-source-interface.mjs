// Draft demonstration: node examples/rust-scalar-source-interface.mjs [--links]
import {createRustSourceOwnerProgramLinks,emitTypedProgramLinks} from '../js/src/index.js';
const source='pub fn offset(value:&usize)->usize{*value+1usize}pub fn next(value:usize)->usize{offset(&value)}fn main(){}';
const packet=createRustSourceOwnerProgramLinks(source);
if(process.argv.includes('--links'))process.stdout.write(packet.network.toLino());
else process.stdout.write(emitTypedProgramLinks(packet.network,packet.root,'Rust',{
 rustSourceHeaderEmissionModel:'retained scalar Rust function interface 1',
}).code);
