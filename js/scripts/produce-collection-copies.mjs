// CI-only producer recipe. It records actual producer output only when later
// invoked; this authored file is not a result or an execution observation.
import {once} from 'node:events';
import {collectionCopyTargets as targets,collectionCopyTransports as transports,collectionCopySourceAnchors as sourceAnchors,collectionCopyOriginals as originals,collectionCopyRefusals as refusals} from '../tests/fixtures/collection-copies.mjs';
import {requireCollectionCopyNodeVersion,readCollectionCopyOriginalControl,readCollectionCopyRefusalControl} from '../tests/support/collection-copy-integration.js';
requireCollectionCopyNodeVersion();
async function write(text){if(!process.stdout.write(text))await once(process.stdout,'drain');}
await write(JSON.stringify({model:'standard same-kind collection copy controls 1',nodeVersion:process.versions.node,targets,transports,sourceAnchors}).slice(0,-1)+',"cases":[');let first=true;
for(const row of originals){const record=readCollectionCopyOriginalControl(row);await write((first?'':',')+JSON.stringify(record));first=false;}
await write('],"refusals":[');first=true;
for(const row of refusals){const record=readCollectionCopyRefusalControl(row);await write((first?'':',')+JSON.stringify(record));first=false;}
await write(']}\n');
