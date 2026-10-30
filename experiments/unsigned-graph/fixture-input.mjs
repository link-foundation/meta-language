// Future input construction only; never import during source authoring.
import assert from 'node:assert/strict';
import {UnsignedLinkNetwork,encodeUnsignedLinkNetworkBinary} from '../../js/src/unsigned-link-network.js';
import {createUnsignedTypedProgramLinks} from '../../js/src/unsigned-typed-program-links.js';
import {TYPED_PROGRAM_LINKS_SCHEMA} from '../../js/src/typed-program-links.js';
export function createUnsignedGraphFixtureInput(row,fixture){
 let network,root,second;
 if(row.raw!==undefined)network=UnsignedLinkNetwork.fromLino(row.raw);
 else{
  const packet=createUnsignedTypedProgramLinks(fixture.typedSource,'Rust',{}, {firstIdentity:fixture.firstIdentity});root=packet.root;network=packet.network.clone();
  if(row.edit!==undefined){const values=network.links().filter(link=>link.metadata().language===TYPED_PROGRAM_LINKS_SCHEMA.language&&link.metadata().term===TYPED_PROGRAM_LINKS_SCHEMA.scalars.string&&link.metadata().definition==='7');assert.equal(values.length,1,'the original program has one exact scalar spelling to edit');const id=values[0].id();assert.equal(network.setLinkMetadata(id,values[0].metadata().clone().withDefinition(row.edit)),true);assert.equal(packet.network.link(id).metadata().definition,'7','editing the clone preserves the original source graph');}
  if(row.multiple){const link=network.link(root);second=network.insertLink(link.references(),link.metadata().clone());}
 }
 if(row.history){network.deleteLink(network.insertLinkWithOptionalId('18446744073709551614'));network._conceptSyntax.set('meaning\0Rust','selected syntax');}
 const args=row.args.map(arg=>arg==='$root'?String(root):arg==='$second'?String(second):arg);
 return {network,root,second,args,text:network.toLino(),bytes:encodeUnsignedLinkNetworkBinary(network)};
}
