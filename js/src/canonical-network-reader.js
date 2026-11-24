// Internal Node asset boundary. Stream the ordinary lossless Links serializer
// format using its existing metadata decoder, with budgets before insertion.
// This does not replace the general language/grammar parser or fromLino API.
import {StreamParser} from 'links-notation';
import {LinkNetwork} from './network.js';
import {decodeCanonicalStatements} from './lino-serialization.js';
const fail=reason=>{throw new TypeError('bounded canonical Links: '+reason);};
export function readBoundedCanonicalNetwork(source,{byteLimit=48*1024*1024,nodeLimit=1000000,referenceLimit=4000000}={}){
 for(const [name,value]of Object.entries({byteLimit,nodeLimit,referenceLimit}))if(!Number.isSafeInteger(value)||value<1)fail(name+' must be a positive safe integer');
 if(typeof source!=='string'||Buffer.byteLength(source)>byteLimit)fail('input exceeds its byte budget');
 const network=new LinkNetwork(),seen=new Set();let references=0;
 const parser=new StreamParser({comments:false,collect:false,maxInputSize:byteLimit,maxBufferSize:byteLimit,maxDepth:16});
 parser.on('error',error=>fail('notation parse error: '+error.message));
 parser.on('link',statement=>{
  if(seen.size>=nodeLimit)fail('node budget exceeded before insertion');
  const [record]=decodeCanonicalStatements([statement]);if(seen.has(record.id))fail('duplicate actual link identity');
  if(record.references.length>referenceLimit-references)fail('reference budget exceeded before insertion');
  seen.add(record.id);references+=record.references.length;
  const id=network.insertLinkWithOptionalId(record.id,record.references,record.metadata);
  // Preserve the same registered-term table as LinkNetwork.fromLino; it is
  // serializer ownership, not a second semantic term-normalization rule.
  if(record.registered&&record.metadata.term!==undefined)network._terms.set(record.metadata.term,id);
 });
 for(let offset=0;offset<source.length;offset+=64*1024)parser.write(source.slice(offset,offset+64*1024));parser.end();
 for(const link of network.links())for(const reference of link.references())if(!network.link(reference))fail('dangling actual link reference');return network;
}
