// Run after the full requirements draft and its deferred verification phase.
import {createRustSourceOwnerProgramLinks,decodeTypedProgramLinks} from '../js/src/index.js';
const packet=createRustSourceOwnerProgramLinks('struct Position{value:usize}fn main(){}');
const program=decodeTypedProgramLinks(packet.network,packet.root);
console.log(program.declarations.get('Position').sourceFieldSignatures);
