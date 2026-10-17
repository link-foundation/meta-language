// Archive the actual public JavaScript producer for native CI interoperability.
import {fileURLToPath} from 'node:url';
import {readProjectData,writeProjectData,checkProjectData} from '../js/scripts/project-data.mjs';
import {createTypedProgramLinks,decodeTypedProgramLinks} from '../js/src/index.js';
import {emitRust} from '../js/src/translation/emit-rust.js';
export function ownedTupleCallNativeFixture(){
 const fixture=readProjectData(new URL('./owned-tuple-call-fixture.lino',import.meta.url));
 return {version:1,model:fixture.model,cases:fixture.cases.map(entry=>{
  const packet=createTypedProgramLinks(entry.source,'Rust',{rustOwnedLocalStateModel:fixture.model});
  const program=decodeTypedProgramLinks(packet.network,packet.root);
  return {name:entry.name,links:packet.network.toLino(),root:Number(packet.root),
   proofs:[...program.declarations.values()].map(declaration=>({name:declaration.name,proof:declaration.sourceLocalState})),
   rustDefinitions:emitRust(program).definitions.join('\n\n')};
 })};
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
 const file=new URL('./owned-tuple-call-native-fixture.lino',import.meta.url),data=ownedTupleCallNativeFixture();
 if(process.argv.includes('--check'))checkProjectData(file,data);else writeProjectData(file,data);
 console.log(`Owned tuple native archives: ${data.cases.length} complete actual public producer programs`);
}
