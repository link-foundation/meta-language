// Deferred actual process results. Never execute during the full draft phase.
import {readProjectData} from '../../js/scripts/project-data.mjs';
import {runUnsignedSourceCommand} from '../../js/src/unsigned-source-cli.js';
import {structuredDataToLino} from '../../js/src/structured-data-links.js';
const fixture=readProjectData(new URL('../../parity/fixtures/unsigned-source-command.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);if(!(major===22&&minor>=15||major===24))throw new TypeError('uncovered Node '+process.versions.node+'; '+fixture.versions.JavaScript);
const io=fixture.io.map(row=>{let reads=0;const result=runUnsignedSourceCommand(row.args,{readFile:path=>{reads+=1;if(path!=='input.source')throw new Error('unknown fixture path');return row.source;}});return{...row,reads,exitCode:result.exitCode,stdout:typeof result.stdout==='string'?result.stdout:'',bytes:typeof result.stdout==='string'?[]:Array.from(result.stdout),stderr:result.stderr};});
process.stdout.write(structuredDataToLino({schemaVersion:1,io}));
