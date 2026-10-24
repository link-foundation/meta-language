// Future CI product. Never run during the complete-code drafting phase.
import {readProjectData} from '../../js/scripts/project-data.mjs';
import {UnsignedLinkNetwork,encodeUnsignedLinkNetworkBinary} from '../../js/src/unsigned-link-network.js';
import {structuredDataToLino} from '../../js/src/structured-data-links.js';
const fixture=readProjectData(new URL('../../parity/fixtures/unsigned-link-network.lino',import.meta.url));
const cases=fixture.cases.map(row=>{
 const graph=UnsignedLinkNetwork.fromLino(row.source);
 return {...row,text:graph.toLino(),bytes:Array.from(encodeUnsignedLinkNetworkBinary(graph))};
});
process.stdout.write(structuredDataToLino({schemaVersion:1,cases}));
