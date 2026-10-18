// JavaScript-first prototype. No catalog defaults or runtime exports change.
import {Link, LinkMetadata, LinkType, idKey} from '../../js/src/primitives.js';
import {StructuredDataNetwork, encodeStructuredDataLinks} from '../../js/src/structured-data-encode.js';
import {decodeStructuredDataLinks} from '../../js/src/structured-data-graph.js';
import {limitsOf} from '../../js/src/structured-data-model.js';
import {checkGrammarSelectionMappedTree} from '../../js/src/grammar-runtime/mapped-tree-validation.js';

const language = 'meta-language concrete grammar outcome';
const term = 'owned concrete grammar outcome';
const fail = detail => { throw new TypeError('owned grammar outcome: '+detail); };

function sourceBytes(network, tokens) {
  const chunks=[];
  let cursor=0;
  const seen=new Set();
  for(const token of tokens){
    if(seen.has(idKey(token)))fail('duplicate physical source token');
    seen.add(idKey(token));
    const data=network.link(token)?.metadata();
    if(data?.linkType!==LinkType.SourceToken||!data.span||typeof data.term!=='string')fail('owned physical source tokens are required');
    const range=data.span.byteRange;
    if(!Number.isSafeInteger(range.start)||range.start<0||range.start!==cursor||!Number.isSafeInteger(range.end)||range.end<range.start)fail('physical token intervals must be contiguous');
    const bytes=Buffer.from(data.term,'utf8');
    if(data.flags.isMissing){if(range.end!==range.start)fail('a missing token must be empty');}
    else {if(bytes.length!==range.end-range.start)fail('physical token bytes differ from their interval');chunks.push(bytes);}
    cursor=range.end;
  }
  return Buffer.concat(chunks);
}

function ownedPhysicalTokens(network,owner,limit){
  const pending=[owner],seen=new Set(),tokens=new Set();
  while(pending.length){
    const id=pending.pop(),key=idKey(id);
    if(seen.has(key))continue;
    if(seen.size>=limit)fail('concrete ownership node resource limit exceeded');
    seen.add(key);
    const link=network.link(id);
    if(!link)fail('a concrete containment member is absent');
    const type=link.metadata().linkType;
    if(type===LinkType.SourceToken)tokens.add(key);
    else if(type===LinkType.Syntax||type===LinkType.Document){
      for(const reference of link.references()){
        if(pending.length>=limit)fail('concrete ownership node resource limit exceeded');
        pending.push(reference);
      }
    }
    else fail('concrete containment must reach only syntax or physical tokens');
  }
  return tokens;
}

function physicalBytes(tree){
  if(!tree||typeof tree!=='object')fail('a complete concrete root is required');
  if(tree.type==='missing')return Buffer.alloc(0);
  if(tree.type==='token'||tree.type==='error'){
    if(typeof tree.text==='string')return Buffer.from(tree.text,'utf8');
    if(tree.text===null&&typeof tree.hex==='string'&&/^(?:[0-9a-f]{2})*$/u.test(tree.hex))return Buffer.from(tree.hex,'hex');
    fail('physical leaf bytes are required');
  }
  if(tree.type==='node'&&Array.isArray(tree.children))return Buffer.concat(tree.children.map(physicalBytes));
  if(tree.type==='embed')return tree.sourceChildren===undefined?physicalBytes(tree.root):Buffer.concat(tree.sourceChildren.map(physicalBytes));
  fail('unsupported concrete node');
}

function validate(network,owner,tokens,outcome,options){
  if(typeof outcome?.ok!=='boolean'||!outcome.tree||!Array.isArray(outcome.ambiguities)
    ||(outcome.ok?outcome.rejection!==null:!outcome.rejection||typeof outcome.rejection.reason!=='string'))
    fail('this finite prototype requires a complete grammar outcome with a concrete tree');
  const owned=ownedPhysicalTokens(network,owner,limitsOf(options.codec??{}).nodeLimit);
  if(owned.size!==tokens.length||tokens.some(token=>!owned.has(idKey(token))))
    fail('physical source tokens must belong to the concrete owner');
  // The mapping verifier checks derivation/ownership, not grammar acceptance.
  // A caller must still obtain this outcome from its actual grammar parser.
  checkGrammarSelectionMappedTree({tree:outcome.tree,sourcePlanes:outcome.sourcePlanes},options.mapping??{});
  if(!physicalBytes(outcome.tree).equals(sourceBytes(network,tokens)))fail('outcome physical bytes differ from owned source tokens');
}

export function insertOwnedGrammarOutcome(network,owner,tokens,outcome,options={}){
  const host=network.link(owner);
  if(!host||![LinkType.Syntax,LinkType.Document].includes(host.metadata().linkType))fail('an existing concrete owner is required');
  // Descriptor validation and a checked plain snapshot happen before any
  // host getter is read or any target-network link is inserted.
  const encoded=encodeStructuredDataLinks(outcome,options.codec??{});
  const snapshot=decodeStructuredDataLinks(encoded.network,encoded.root,options.codec??{});
  validate(network,owner,tokens,snapshot,options);
  const translations=new Map();
  for(const link of encoded.network.links())translations.set(idKey(link.id()),network.insertLink([],link.metadata().clone()));
  for(const link of encoded.network.links())network.link(translations.get(idKey(link.id()))).setReferences(link.references().map(reference=>translations.get(idKey(reference))));
  const source=network.insertLink(tokens,LinkMetadata.new().withLinkType(LinkType.Syntax).withLanguage(language).withTerm('owned physical source tokens').withNamed(true));
  const root=translations.get(idKey(encoded.root));
  return network.insertLink([owner,source,root,...translations.values()],LinkMetadata.new().withLinkType(LinkType.Syntax).withLanguage(language).withTerm(term).withNamed(true));
}

export function readOwnedGrammarOutcome(network,association,options={}){
  const link=network.link(association),metadata=link?.metadata();
  if(metadata?.linkType!==LinkType.Syntax||metadata.language!==language||metadata.term!==term)fail('an owned concrete outcome association is required');
  const [owner,source,root,...members]=link.references();
  if(!network.link(owner)||![LinkType.Syntax,LinkType.Document].includes(network.link(owner).metadata().linkType))fail('the concrete owner is absent');
  const physical=network.link(source),data=physical?.metadata();
  if(data?.linkType!==LinkType.Syntax||data.language!==language||data.term!=='owned physical source tokens')fail('the physical source manifest is absent');
  const isolated=new StructuredDataNetwork();
  for(const id of members){
    if(isolated.link(id))fail('duplicate owned payload identity');
    const stored=network.link(id);
    if(!stored)fail('an owned payload link is absent');
    isolated._links.set(idKey(id),new Link(id,stored.references(),stored.metadata().clone()));
  }
  const outcome=decodeStructuredDataLinks(isolated,root,options.codec??{});
  validate(network,owner,physical.references(),outcome,options);
  return {owner,tokens:physical.references(),outcome,members};
}

export function concreteLinksWithOwnedGrammarOutcomes(network,options={}){
  // Prototype projection only: integration must move this decision into the
  // shared runtime policy before changing the public default projection.
  const owned=new Set();
  for(const link of network.links())if(link.metadata().language===language&&link.metadata().term===term){
    for(const member of readOwnedGrammarOutcome(network,link.id(),options).members)owned.add(idKey(member));
  }
  return network.links().filter(link=>link.metadata().linkType!==LinkType.Semantic||owned.has(idKey(link.id())));
}
