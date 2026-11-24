// Deferred physical JavaScript CST controls, independent of typed translation.
import assert from 'node:assert/strict';
import {LinkNetwork} from '../../src/network.js';
import {LinkType,NetworkProjection} from '../../src/primitives.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../../src/link-network-binary.js';
export function readCollectionCopyConcreteControl(source){
 const bytes=Buffer.from(source,'utf8');assert.ok(bytes.length<=65536);
 const network=LinkNetwork.parse(source,'JavaScript');assert.equal(network.verifyFullMatch().isClean(),true);
 const text=network.toLino(),binary=encodeLinkNetworkBinary(network),syntax=network.links().filter(link=>link.metadata().linkType===LinkType.Syntax&&link.metadata().language==='JavaScript');
 const referenced=new Set(syntax.flatMap(link=>link.references().map(id=>id.asU64()))),roots=syntax.filter(link=>!referenced.has(link.id().asU64()));assert.equal(roots.length,1);
 const root=roots[0],fields=new Map(network.fieldRelations().map(row=>[`${row.parent.asU64()}:${row.child.asU64()}`,row.label]));
 const rows=[],pending=[{link:root,depth:0,field:null}];
 while(pending.length){
  assert.ok(rows.length<100000);const{link,depth,field}=pending.pop(),{term,named,span,flags}=link.metadata();assert.ok(span);
  rows.push([depth,field,term,named?1:0,span.byteRange.start,span.byteRange.end,flags.isError,flags.isMissing,flags.isExtra,flags.hasError]);
  const children=link.references().map(reference=>network.link(reference)).filter(child=>child?.metadata().linkType===LinkType.Syntax&&child.metadata().language==='JavaScript');
  for(let at=children.length-1;at>=0;at-=1){const child=children[at];pending.push({link:child,depth:depth+1,field:fields.get(`${link.id().asU64()}:${child.id().asU64()}`)??null});}
 }
 assert.equal(rows.length,syntax.length);assert.ok(rows.some(row=>row[0]>=3));
 const ids=network.projectedLinks().map(link=>link.id().asU64());
 for(const restored of [network,LinkNetwork.fromLino(text),decodeLinkNetworkBinary(binary)]){
  assert.equal(restored.toLino(),text);assert.deepEqual(Buffer.from(restored.reconstructText(),'utf8'),bytes);assert.deepEqual(restored.projectedLinks().map(link=>link.id().asU64()),ids);
  for(const role of [LinkType.Syntax,LinkType.SourceToken,LinkType.Field,LinkType.Trivia])assert.ok(restored.projectedLinks().some(link=>link.metadata().linkType===role));
  for(const token of restored.links().filter(link=>link.metadata().linkType===LinkType.SourceToken&&!link.metadata().flags.isMissing)){const{term,span}=token.metadata();assert.ok(span);assert.equal(term,bytes.subarray(span.byteRange.start,span.byteRange.end).toString('utf8'));}
  const before=restored.toLino();for(const link of restored.projectedLinks(NetworkProjection.AbstractSyntax)){assert.ok(![LinkType.SourceToken,LinkType.Trivia].includes(link.metadata().linkType));for(const reference of link.references())assert.ok(restored.link(reference));}
  assert.equal(restored.toLino(),before);assert.deepEqual(restored.projectedLinks().map(link=>link.id().asU64()),ids);
 }
 assert.deepEqual(encodeLinkNetworkBinary(decodeLinkNetworkBinary(binary)),binary);
 return {language:'JavaScript',rootTerm:root.metadata().term,rows,sourceBytes:bytes.length};
}
