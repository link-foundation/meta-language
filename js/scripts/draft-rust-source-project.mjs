// CI source drafting entry. Reuse the complete tracked-file inventory; no
// compilation, program execution, test oracle, download or clone is performed.
import {writeFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {materializeProjectTranslationInputs} from './draft-project-translation.mjs';
import {createRustSourceProjectProgramLinks} from '../src/rust-source-project.js';
import {emitTypedProgramLinks} from '../src/typed-program-links.js';
export function draftTrackedRustSourceProject(project,entry,options={}){
 const inventory=materializeProjectTranslationInputs(project,options),inputs=inventory.inputs.filter(row=>row.language==='Rust').map(row=>({path:row.identifier,source:row.source}));
 return createRustSourceProjectProgramLinks(inputs,entry,{...options,provenance:{projectRevision:inventory.revision,files:inventory.files,totalBytes:inventory.totalBytes}});
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
 const values={},args=process.argv.slice(2);let library=false;
 for(let index=0;index<args.length;index+=1){if(args[index]==='--library'){if(library)throw new TypeError('duplicate --library');library=true;continue;}
  if(!['--project','--entry','--out','--to','--code'].includes(args[index])||index+1>=args.length||Object.hasOwn(values,args[index].slice(2)))throw new TypeError('usage: --project <directory> --entry <Rust path> --out <Links file> [--library] [--to <language> --code <source file>]');values[args[index].slice(2)]=args[++index];}
 if(!values.project||!values.entry||!values.out||Boolean(values.to)!==Boolean(values.code)||values.out===values.code)throw new TypeError('explicit project, entry and distinct output paths are required');
 const packet=draftTrackedRustSourceProject(values.project,values.entry,{library});
 const emission=values.to?emitTypedProgramLinks(packet.network,packet.root,values.to):null;
 writeFileSync(values.out,packet.network.toLino());if(emission)writeFileSync(values.code,emission.code);
 process.stdout.write('root '+packet.root.asU64()+'\n');
}
