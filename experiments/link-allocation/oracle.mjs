// Future definition product; never execute during the whole requirements draft.
import{createLinkAllocationProgramLinks}from'../../js/src/link-allocation-program.js';
import{emitTypedProgramLinks}from'../../js/src/typed-program-links.js';
import{structuredDataToLino}from'../../js/src/structured-data-links.js';
const[major,minor]=process.versions.node.split('.').map(Number);
if(!(major===22&&minor>=15||major===24))throw new RangeError('uncovered Node '+process.versions.node);
const packet=createLinkAllocationProgramLinks();
process.stdout.write(structuredDataToLino({root:packet.root.asU64(),links:packet.network.toLino(),emissions:Object.fromEntries(['JavaScript','Rust','Lean','Rocq'].map(target=>[target,emitTypedProgramLinks(packet.network,packet.root,target)]))}));
