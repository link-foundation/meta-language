// CI-only future producer. This source contains no execution observations.
import {once} from 'node:events';
import {collectionCopyTargets as targets,collectionCopyTransports as transports} from '../tests/fixtures/collection-copies.mjs';
import {disjointCollectionIterationOriginals as originals,disjointCollectionIterationRefusals as refusals,disjointCollectionIterationSourceAnchors as sourceAnchors} from '../tests/fixtures/disjoint-collection-iterations.mjs';
import {requireCollectionCopyNodeVersion,readCollectionCopyOriginalControl,readCollectionCopyRefusalControl} from '../tests/support/collection-copy-integration.js';
requireCollectionCopyNodeVersion();
async function write(text){if(!process.stdout.write(text))await once(process.stdout,'drain');}
await write(JSON.stringify({model:'closed disjoint collection iteration controls 1',nodeVersion:process.versions.node,targets,transports,sourceAnchors}).slice(0,-1)+',"cases":[');let first=true;
for(const row of originals){await write((first?'':',')+JSON.stringify(readCollectionCopyOriginalControl(row)));first=false;}
await write('],"refusals":[');first=true;
for(const row of refusals){await write((first?'':',')+JSON.stringify(readCollectionCopyRefusalControl(row)));first=false;}
await write(']}\n');
