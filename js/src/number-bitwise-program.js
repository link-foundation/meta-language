// The canonical definition is an ordinary editable typed Links program.
import {LinkId} from './primitives.js';
import {LinkNetwork} from './network.js';
import {decodeTypedProgramLinks} from './typed-program-links.js';
import {recheckLinkedTypedProgram} from './typed-program-checked-projection.js';
import {NUMBER_BITWISE_PROGRAM_ROOT,NUMBER_BITWISE_PROGRAM_LINKS} from './translation/number-bitwise-runtime.js';

/** Return a fresh complete Number definition graph after ordinary validation. */
export function createNumberBitwiseProgramLinks(){
 const network=LinkNetwork.fromLino(NUMBER_BITWISE_PROGRAM_LINKS),root=LinkId.from(NUMBER_BITWISE_PROGRAM_ROOT);
 recheckLinkedTypedProgram(decodeTypedProgramLinks(network,root),{wholeProgram:true});
 return{network,root};
}
