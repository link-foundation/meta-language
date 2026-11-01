// Future CI producer only. Authoring this file does not execute its controls.
import {once} from 'node:events';
import {collectionCopyTargets as targets,collectionCopyTransports as transports} from '../tests/fixtures/collection-copies.mjs';
import {closedClassOwnerOriginals as originals,closedClassOwnerArchiveEdits as edits,closedClassOwnerGraphEdits as graphEdits} from '../tests/fixtures/closed-class-owner-witnesses.mjs';
import {requireCollectionCopyNodeVersion,readCollectionCopyOriginalControl} from '../tests/support/collection-copy-integration.js';
import {readClosedClassOwnerArchiveControl,readClosedClassOwnerGraphControl} from '../tests/support/closed-class-owner-integration.js';
requireCollectionCopyNodeVersion();
const archiveEditsOnly=process.argv.slice(2).includes('--archive-edits-only');
async function write(text){if(!process.stdout.write(text))await once(process.stdout,'drain');}
await write(JSON.stringify({model:'persistent closed class owner controls 1',nodeVersion:process.versions.node,targets,transports,sourceAnchors:[]}).slice(0,-1)+',"cases":[');let first=true;
if(!archiveEditsOnly)for(const row of originals){await write((first?'':',')+JSON.stringify(readCollectionCopyOriginalControl(row)));first=false;}
await write('],"refusals":[],"archiveEdits":[');first=true;
for(const name of edits){await write((first?'':',')+JSON.stringify(readClosedClassOwnerArchiveControl(name)));first=false;}
await write('],"graphEdits":[');first=true;
for(const name of graphEdits){await write((first?'':',')+JSON.stringify(readClosedClassOwnerGraphControl(name)));first=false;}
await write(']}\n');
