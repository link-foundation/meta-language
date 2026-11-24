// Future paired CST/source/native controls; do not run until all code is drafted.
import { readProjectData } from '../../js/scripts/project-data.mjs';
import { analyzeProgram } from '../../js/src/program-representation.js';
import { createModuleBindingProgramLinks } from '../../js/src/module-binding-program.js';
import { observeJavaScriptModule } from './node-source.mjs';
const fixture=readProjectData(new URL('../../parity/fixtures/module-bindings.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);
if(!(major===22&&minor>=15||major===24))throw new Error('uncovered Node '+process.versions.node);
const cases=fixture.cases.map(row=>{
  const program=analyzeProgram(row.source,'JavaScript');
  const binding=program.bindings.find(binding=>binding.scope==='scope:0'&&binding.name===row.binding);
  if(!binding)throw new Error('the original source has no declared module binding '+row.binding);
  const renamed=program.renameBinding(binding.id,row.replacement);
  return {...row,javascript:renamed.source,original:observeJavaScriptModule(row.source,fixture.provider,row.observations),renamed:observeJavaScriptModule(renamed.source,fixture.provider,row.observations)};
});
const definition=createModuleBindingProgramLinks();
console.log(JSON.stringify({fixture,cases,definition:{network:definition.network.toLino(),root:Number(definition.root)}}));
