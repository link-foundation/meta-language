import {readFileSync} from 'node:fs';
import {LinkNetwork} from './network.js';
import {decodeTypedProgramLinks} from './typed-program-links.js';
import {recheckLinkedTypedProgram} from './typed-program-checked-projection.js';
/** Complete ordinary callable contexts and actual original pattern definitions. */
export function createJavaScriptRecordCallableParameterProgramLinks(){
 const network=LinkNetwork.fromLino(readFileSync(new URL('./data/record-callable-parameter-program.lino',import.meta.url),'utf8'));
 const roots=network.links().filter(link=>link.metadata().language==='meta-language typed program'&&link.metadata().term==='typed program'&&link.metadata().definition==='1');
 if(roots.length!==1)throw new TypeError('the record callable parameter program needs one actual schema root');
 const root=roots[0].id();recheckLinkedTypedProgram(decodeTypedProgramLinks(network,root),{wholeProgram:true});return {network,root};
}
