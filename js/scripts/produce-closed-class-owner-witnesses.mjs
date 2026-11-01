// Future CI producer only. Authoring this file does not execute its controls.
import {closedClassOwnerOriginals as originals,closedClassOwnerArchiveEdits as edits,closedClassOwnerGraphEdits as graphEdits} from '../tests/fixtures/closed-class-owner-witnesses.mjs';
import {produceCollectionControlFixture} from './collection-control-producer.mjs';
import {readClosedClassOwnerArchiveControl,readClosedClassOwnerGraphControl} from '../tests/support/closed-class-owner-integration.js';
const archiveEditsOnly=process.argv.slice(2).includes('--archive-edits-only');
const definition=name=>({name,source:originals.at(-1).source});
await produceCollectionControlFixture({model:'persistent closed class owner controls 1',originals:archiveEditsOnly?[]:originals,refusals:[],additionalGroups:[
 {key:'archiveEdits',rows:edits.map(definition),read:row=>readClosedClassOwnerArchiveControl(row.name)},
 {key:'graphEdits',rows:graphEdits.map(definition),read:row=>readClosedClassOwnerGraphControl(row.name)},
]});
