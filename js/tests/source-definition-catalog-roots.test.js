// Future author/complete-reader compatibility; no source author imports this.
import assert from 'node:assert/strict';
import test from 'node:test';
import {readAuthoredSourceDefinitionRoot} from '../scripts/source-definition-catalog-roots.mjs';
import {LinkNetwork} from '../src/network.js';
import {LinkMetadata,LinkType} from '../src/primitives.js';
test('streamed source root preserves the full canonical reader including cycles and disconnected definitions',()=>{
 const network=new LinkNetwork(),body=network.insertLink([],LinkMetadata.new().withLinkType(LinkType.Semantic).withTerm('program record').withLanguage('meta-language typed program'));
 network.link(body).setReferences([body]);network.insertLink([],LinkMetadata.new().withDefinition('unrelated 🙂 metadata'));
 const root=network.insertLink([body],LinkMetadata.new().withLinkType(LinkType.Semantic).withNamed(true).withTerm('typed program').withLanguage('meta-language typed program').withDefinition('1'));
 const source=network.toLino();assert.equal(readAuthoredSourceDefinitionRoot(source),String(root.asU64()));assert.equal(readAuthoredSourceDefinitionRoot(source,{complete:true}),String(root.asU64()));
 assert.throws(()=>readAuthoredSourceDefinitionRoot(source+source),/duplicate/u);
 network.link(body).setReferences([999999]);assert.throws(()=>readAuthoredSourceDefinitionRoot(network.toLino()),/dangling/u);
});
