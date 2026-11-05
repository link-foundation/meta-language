// Future actual process products; never execute during complete-code drafting.
import {readProjectData} from '../../js/scripts/project-data.mjs';
import {createUnsignedGraphFixtureInput} from './fixture-input.mjs';
import {runUnsignedGraphCommand} from '../../js/src/unsigned-graph-cli.js';
import {structuredDataToLino} from '../../js/src/structured-data-links.js';
const fixture=readProjectData(new URL('../../parity/fixtures/unsigned-graph-command.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);if(!(major===22&&minor>=15||major===24))throw new TypeError('uncovered Node '+process.versions.node+'; '+fixture.versions.JavaScript);
const io=fixture.io.map(row=>{const input=createUnsignedGraphFixtureInput(row,fixture);let reads=0,byteReads=0;const output=runUnsignedGraphCommand(input.args,{readFile:path=>{reads+=1;if(path!=='saved.graph')throw new Error('unknown fixture path');return input.text;},readBytes:path=>{byteReads+=1;if(path!=='saved.graph')throw new Error('unknown fixture path');return input.bytes;}});return{name:row.name,args:input.args,inputText:input.text,inputBytes:Array.from(input.bytes),reads,byteReads,exitCode:output.exitCode,stdout:typeof output.stdout==='string'?output.stdout:'',bytes:typeof output.stdout==='string'?[]:Array.from(output.stdout),stderr:output.stderr};});
process.stdout.write(structuredDataToLino({schemaVersion:1,io}));
