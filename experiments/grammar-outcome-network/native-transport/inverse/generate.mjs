// Shared inverse plans are generated only from authenticated actual source CSTs.
import assert from 'node:assert/strict';
import {fileURLToPath} from 'node:url';
import {readProjectData} from '../../../../js/scripts/project-data.mjs';
import {captureOrdinaryInversePlan,authenticateOrdinaryInversePlan,writeOrdinaryInversePlan,extendRecursiveInversePlan} from './plan.mjs';

const output=fileURLToPath(new URL('./ordinary-inverse-plan.lino',import.meta.url));
const args=process.argv.slice(2);
assert.ok(args.length===1&&['--write','--check'].includes(args[0]),'choose --write or --check explicitly');
const authority=captureOrdinaryInversePlan();
if(args[0]==='--write')writeOrdinaryInversePlan(authority,output);
authenticateOrdinaryInversePlan(authority,readProjectData(output));
const recursive=extendRecursiveInversePlan(authority),recursiveOutput=fileURLToPath(new URL('./recursive-inverse-plan.lino',import.meta.url));
if(args[0]==='--write')writeOrdinaryInversePlan(recursive,recursiveOutput);
authenticateOrdinaryInversePlan(recursive,readProjectData(recursiveOutput));
console.log('Complete ordinary inverse plan matches its authenticated source graph: '+authority.sha256);
