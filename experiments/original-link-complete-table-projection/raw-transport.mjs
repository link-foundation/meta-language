// Primitive snapshot only. No fieldRelations(), ordinary object decode, source
// owner decisions or old role transport may run before this collector.
import {originalLinkIdentityIndex,originalLinkIdentityWords} from '../original-link-identity/policy.mjs';
import {checkOriginalLinkRawShape} from './raw-rules.mjs';
import {originalLinkRawPolicy as policy} from './raw-policy.mjs';
import {decodeRustTextCertificateFacet} from '../rust-text-certificate-adapter/raw-identity.mjs';
import {projectOriginalLinkCompleteTables} from './transport.mjs';
const kind=value=>value===undefined?0:typeof value==='string'?1:value===null?2:typeof value==='boolean'?3:typeof value==='number'?4:typeof value==='bigint'?5:typeof value==='symbol'?6:typeof value==='function'?7:Array.isArray(value)?9:8;
export function readOriginalLinkRawTransport(network,root){
 if(network.len()>policy.maximumNodes)throw new RangeError('raw primitive node bound');
 const snapshot=network.links(),index=originalLinkIdentityIndex(snapshot.map(link=>link.id())),anchors=index.anchors.map(row=>row.original),byId=new Map(snapshot.map(link=>[link.id().asU64(),link])),links=anchors.map(id=>byId.get(id.asU64())),rootWords=originalLinkIdentityWords(root);
 const parentHigh=[],parentLow=[],high=[],low=[],orderHigh=[],orderLow=[],textKinds=[],starts=[],lengths=[],units=[],payloads=[],named=[],flags=[],spans=[],metadataAnchors=[];
 for(const node of snapshot){const parent=originalLinkIdentityWords(node.id());orderHigh.push(parent[0]);orderLow.push(parent[1]);for(const child of node.references()){if(parentHigh.length>=policy.maximumReferences)throw new RangeError('raw primitive reference bound');const words=originalLinkIdentityWords(child);parentHigh.push(parent[0]);parentLow.push(parent[1]);high.push(words[0]);low.push(words[1]);}}
 for(const node of links){
  const metadata=node.metadata();metadataAnchors.push(metadata);
  for(const value of[metadata.linkType,metadata.term,metadata.language,metadata.definition]){textKinds.push(kind(value));starts.push(units.length);lengths.push(typeof value==='string'?value.length:-1);payloads.push(typeof value==='number'?value:typeof value==='boolean'?Number(value):0);if(typeof value==='string'){if(units.length+value.length>policy.maximumUnits)throw new RangeError('raw primitive metadata unit bound');for(let at=0;at<value.length;at++)units.push(value.charCodeAt(at));}}
  named.push(kind(metadata.named),typeof metadata.named==='boolean'?Number(metadata.named):0);for(const name of['isError','hasError','isMissing','isExtra']){const value=metadata.flags?.[name];flags.push(kind(value),typeof value==='boolean'?Number(value):0);}spans.push(metadata.span===undefined?0:metadata.span===null?1:2);
 }
 return{anchors,metadataAnchors,root,columns:[index.high,index.low,parentHigh,parentLow,high,low,textKinds,starts,lengths,units,payloads,named,flags,spans,orderHigh,orderLow],arguments:[rootWords[0],rootWords[1],[index.high,index.low,parentHigh,parentLow,high,low,textKinds,starts,lengths,units,payloads,named,flags,spans,orderHigh,orderLow]]};
}
export function checkOriginalLinkRawSelectedGraph(network,root){
 const transport=readOriginalLinkRawTransport(network,root),decision=checkOriginalLinkRawShape(...transport.arguments);if(decision[0]===0)return{transport,decision,checked:null};if(decision[0]!==1)throw new TypeError('Original raw shape component refused at '+decision.join(','));
 // This remains necessary: raw shape alone does not establish source meaning.
 const checked=decodeRustTextCertificateFacet(network,root);if(!checked)throw new TypeError('selected raw facet lost its independent interpretation');const tables=projectOriginalLinkCompleteTables(network,root);return{transport,decision,checked,tables};
}
