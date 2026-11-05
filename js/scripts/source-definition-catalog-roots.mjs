// Source inventory only. Stream the actual canonical decoder's records without
// retaining a complete second object graph. No represented program is decoded.
import {StreamParser} from 'links-notation';
import {decodeCanonicalStatements} from '../src/lino-serialization.js';
import {LinkNetwork} from '../src/network.js';

function acceptSourceDefinitionRootMetadata(metadata){return metadata.language==='meta-language typed program'&&metadata.term==='typed program'&&metadata.definition==='1';}
/** The later gate retains its original complete-network reader. Authoring
 * streams every original statement through the same metadata decoder, and
 * checks identities/references rather than trusting a cached root or digest. */
export function readAuthoredSourceDefinitionRoot(source,{complete=false}={}){
 if(typeof source!=='string'||Buffer.byteLength(source)>16*1024*1024)throw new TypeError('a source program exceeds its authoring byte bound');
 if(complete){const roots=LinkNetwork.fromLino(source).links().filter(link=>acceptSourceDefinitionRootMetadata(link.metadata()));if(roots.length!==1)throw new TypeError('a source program needs one actual existing schema root');return String(roots[0].id().asU64());}
 const roots=[],identities=new Set(),references=new Set();let referenceCount=0;
 const parser=new StreamParser({comments:false,collect:false,maxInputSize:16*1024*1024,maxBufferSize:16*1024*1024,maxDepth:16});
 parser.on('error',error=>{throw new TypeError('source definition inventory: '+error.message);});
 parser.on('link',statement=>{
  const [record]=decodeCanonicalStatements([statement]);
  if(identities.size>=100000||identities.has(record.id))throw new TypeError('source definition inventory: duplicate identity or node bound');
  identities.add(record.id);referenceCount+=record.references.length;
  if(referenceCount>1000000)throw new TypeError('source definition inventory: reference bound');
  for(const reference of record.references)references.add(reference);
  if(acceptSourceDefinitionRootMetadata(record.metadata))roots.push(String(record.id));
 });
 for(let offset=0;offset<source.length;offset+=64*1024)parser.write(source.slice(offset,offset+64*1024));
 parser.end();
 if(roots.length!==1)throw new TypeError('a source program needs one actual existing schema root');
 for(const reference of references)if(!identities.has(reference))throw new TypeError('source definition inventory: dangling reference');
 return roots[0];
}
