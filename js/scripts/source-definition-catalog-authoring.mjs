// Writing-only cache over an authored Links snapshot. A cache hit is not
// graph-validation evidence. The complete check always ignores these entries.
import {existsSync,lstatSync,readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {readBootstrapStructuredDataLino} from '../../scripts/lib/structured-data-bootstrap.mjs';
import {readAuthoredSourceDefinitionRoot} from './source-definition-catalog-roots.mjs';
const model='complete qualified typed source programs 1';
export function readSourceDefinitionCatalogAuthorCache(file,enabled=false){
 if(!enabled||!existsSync(file))return new Map();
 const stat=lstatSync(file);
 if(!stat.isFile()||stat.isSymbolicLink())throw new TypeError('an authored source catalog must be a regular file');
 // Revert to complete source authoring when the optional snapshot is too big;
 // never enlarge the metadata reader's heap to materialize it.
 if(stat.size>4*1024*1024)return new Map();
 const snapshot=readBootstrapStructuredDataLino(readFileSync(file,'utf8'),{byteLimit:4*1024*1024});
 if(snapshot.schemaVersion!==1||snapshot.model!==model||!Array.isArray(snapshot.entries))throw new TypeError('an authored source catalog needs its original schema');
 const entries=new Map();
 for(const entry of snapshot.entries){
  if(!entry||typeof entry.asset!=='string'||!/^[a-z][a-z0-9-]*-program\.lino$/u.test(entry.asset)||entries.has(entry.asset))throw new TypeError('an authored source catalog needs distinct actual assets');
  entries.set(entry.asset,entry);
 }
 return entries;
}
export function readSourceDefinitionCatalogAuthorEntry(asset,bytes,{previous,complete=false}={}){
 if(typeof asset!=='string'||!/^[a-z][a-z0-9-]*-program\.lino$/u.test(asset))throw new TypeError('a source program needs its actual asset name');
 if(!Buffer.isBuffer(bytes)||bytes.length>16*1024*1024)throw new TypeError('a source program exceeds the author bound');
 const sha256=createHash('sha256').update(bytes).digest('hex'),identifier='meta-language/source-program/'+asset.slice(0,-5);
 const reusable=!complete&&previous?.asset===asset&&previous.identifier===identifier&&previous.bytes===bytes.length&&previous.sha256===sha256&&typeof previous.root==='string'&&/^[1-9][0-9]*$/u.test(previous.root)&&Number.isSafeInteger(Number(previous.root));
 let root;
 try{root=reusable?previous.root:readAuthoredSourceDefinitionRoot(bytes.toString('utf8'),{complete});}
 catch(error){error.message=asset+': '+error.message;throw error;}
 return {identifier,asset,bytes:bytes.length,sha256,root};
}
