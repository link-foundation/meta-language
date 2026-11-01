// Deferred author only. Capturing source never imports its represented bodies.
import {readFileSync,mkdirSync} from 'node:fs';
import {captureJavaScriptErrorClassModule} from '../src/translation/javascript-error-classes.js';
import {writeProjectData} from './project-data.mjs';
const root=new URL('../../',import.meta.url);
export function draftJavaScriptErrorClassSource({sourcePath='js/src/translation/diagnostics.js',asset='javascript-translation-error-source'}={}){
 if(!/^[a-z][a-z0-9-]*$/u.test(asset)||typeof sourcePath!=='string'||!sourcePath.startsWith('js/src/')||sourcePath.includes('\\')||sourcePath.split('/').some(part=>!part||part==='.'||part==='..'))throw new TypeError('a bounded actual source asset is required');
 const source=readFileSync(new URL(sourcePath,root),'utf8');
 const capture=captureJavaScriptErrorClassModule(source,{profile:'actual intrinsic Error subclasses 1',identifier:sourcePath});
 // Source/plan are declarative input to prepareJavaScriptErrorModule. Real
 // engine constructor identities are supplied at execution, never invented in
 // this asset or confused with host IO observations.
 for(const runtime of ['js','rust']){mkdirSync(new URL(runtime+'/src/data/',root),{recursive:true});writeProjectData(new URL(runtime+'/src/data/'+asset+'.lino',root),capture);}
 return capture;
}
if(process.argv[1]===new URL(import.meta.url).pathname){if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');draftJavaScriptErrorClassSource();}
