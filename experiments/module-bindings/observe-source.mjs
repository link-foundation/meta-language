// CI-only observer of the exact artifact supplied by the native producer.
import { readFileSync } from 'node:fs';
import { readProjectData } from '../../js/scripts/project-data.mjs';
import { observeJavaScriptModule } from './node-source.mjs';
const fixture=readProjectData(new URL('../../parity/fixtures/module-bindings.lino',import.meta.url));
const index=Number(process.argv[3]);
if(!Number.isSafeInteger(index)||index<0||index>=fixture.cases.length)throw new TypeError('an actual fixture row is required');
console.log(JSON.stringify(observeJavaScriptModule(readFileSync(process.argv[2],'utf8'),fixture.provider,fixture.cases[index].observations)));
