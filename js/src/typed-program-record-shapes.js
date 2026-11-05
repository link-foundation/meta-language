// Layout adapters preserve actual owning IDs and ordered child/reference edges.
// Shape declarations are ordinary inspectable Links, never hidden tables.
import {readRuntimeProjectData} from './project-data-loader.js';
import {LinkMetadata,LinkType} from './primitives.js';
import {LinkNetwork} from './network.js';
import {readTypedProgramRecordShapeError,readTypedProgramRecordShapeSelectionError,readTypedProgramRecordRoleTextError} from './translation/typed-record-shape-rules.js';
const profile=Object.freeze(readRuntimeProjectData(new URL('./data/typed-program-record-shapes.lino',import.meta.url)));
const key=id=>id.asU64();
const fail=message=>{throw new TypeError('typed program record layout: '+message);};
export function readTypedProgramRecordShapeProfile(){
 if(profile.schemaVersion!==1||profile.model!=='ordered typed program record fields 1'||profile.recordOccurrences!==false||profile.rootField!=='record layout'||profile.profileTerm!=='typed program record layout'||profile.shapeTerm!=='typed program record field shape')fail('unknown qualified layout profile');
 return profile;
}
function metadata(language,term,definition){let value=LinkMetadata.new().withLinkType(LinkType.Semantic).withLanguage(language).withTerm(term);if(definition!==undefined)value=value.withDefinition(definition);return value;}
function qualifiedField(point){const value=point?.metadata();return !!point&&value.linkType===LinkType.Field&&value.named===true&&value.language===undefined&&value.definition==='A field label names a relation between links.'&&point.references().length===1&&key(point.references()[0])===key(point.id());}
export function selectTypedProgramRecordShapes(network,root,language){
 const relations=network.fieldRelations(),selected=relations.filter(relation=>key(relation.parent)===key(root)&&relation.label===profile.rootField);
 if(!selected.length)return false;
 readTypedProgramRecordShapeProfile();
 const relation=selected[0],label=network.link(relation.link.references()[1]),edge=relation.link.metadata(),link=network.link(relation.child),value=link?.metadata();
 const qualified=qualifiedField(label)&&!relations.some(relation=>key(relation.parent)===key(label.id()))&&edge.language===undefined&&edge.term===undefined&&edge.definition===undefined&&edge.named===false&&!!link&&value.linkType===LinkType.Semantic&&value.named===false&&value.language===language&&value.term===profile.profileTerm&&value.definition===profile.model;
 const error=readTypedProgramRecordShapeSelectionError(selected.length,qualified,link?.references().length??-1,!!link&&relations.some(relation=>key(relation.parent)===key(link.id())));if(error)fail(error);
 return true;
}
export function readTypedProgramRecordShapeFields(network,record,language,selected,fields){
 const references=record.references(),shape=network.link(references[0]),roles=shape?.references()??[];
 if(references.length>100001||roles.length>100000)fail('ordered program record roles must match every actual value coordinate');
 const points=roles.map(id=>network.link(id)),value=shape?.metadata();
 let units=0;for(const point of points){const size=(point?.metadata().term??'').length;units+=size;const error=readTypedProgramRecordRoleTextError(units,size>1000000);if(error)fail(error);}
 const error=readTypedProgramRecordShapeError(selected,references.length,fields.has(key(record.id())),!!shape&&fields.has(key(shape.id())),value?.linkType===LinkType.Semantic&&value?.named===false,value?.language??'',value?.term??'',value?.definition??'',language,profile.shapeTerm,
  points.map(point=>point?.metadata().term??''),points.map(qualifiedField),points.map((point,index)=>!!point&&point.references().length===1&&key(point.references()[0])===key(roles[index])),points.map(point=>!!point&&fields.has(key(point.id()))));
 if(error)fail(error);
 return roles.map((role,index)=>({parent:record.id(),label:points[index].metadata().term,child:references[index+1]}));
}
/** Only the fresh, private encoder graph is compacted. Any reference to an
 * owned field relation from elsewhere refuses compaction before mutation.
 * Original record/binder identities and all value edges remain unchanged. */
export function compactTypedProgramRecordShapes(network,root,language,recordTerm){
 readTypedProgramRecordShapeProfile();
 if(selectTypedProgramRecordShapes(network,root,language))fail('a graph already selected its record layout');
 const links=network.links(),records=new Map(links.filter(link=>link.metadata().language===language&&link.metadata().term===recordTerm).map(link=>[key(link.id()),link]));
 const grouped=new Map(),removed=new Set();
 for(const relation of network.fieldRelations())if(records.has(key(relation.parent))){if(!grouped.has(key(relation.parent)))grouped.set(key(relation.parent),[]);grouped.get(key(relation.parent)).push(relation);removed.add(key(relation.link.id()));}
 for(const link of links){if(removed.has(key(link.id())))continue;if(link.references().some(id=>removed.has(key(id))))fail('a field relation has external identity references');}
 for(const record of records.values())if(record.references().length)fail('legacy records must have complete named fields before compaction');
 const shapes=new Map();
 for(const [identity,record]of records){
  const fields=grouped.get(identity)??[],roles=fields.map(relation=>relation.link.references()[1]),shapeKey=roles.map(key).join(',');
  let shape=shapes.get(shapeKey);if(shape===undefined){shape=network.insertLink(roles,metadata(language,profile.shapeTerm));shapes.set(shapeKey,shape);}
  record.setReferences([shape,...fields.map(relation=>relation.child)]);
 }
 for(const id of removed)network.deleteLink(id);
 network.insertFieldRelation(root,profile.rootField,network.insertLink([],metadata(language,profile.profileTerm,profile.model)));
 return {network,root};
}

/** Lossless legacy view for algorithms which explicitly traverse named field
 * relations. Original value, declaration and binder IDs are retained. Only
 * reachable shape declarations with no remaining users can be removed. */
export function expandTypedProgramRecordShapes(original,root,language,recordTerm){
 if(!selectTypedProgramRecordShapes(original,root,language))return {network:original,root};
 const network=LinkNetwork.fromLino(original.toLino()),fields=new Map();
 for(const relation of network.fieldRelations()){const parent=key(relation.parent);if(!fields.has(parent))fields.set(parent,[]);fields.get(parent).push(relation);}
 const pending=[root],seen=new Set(),records=[],shapes=new Set();
 for(let index=0;index<pending.length;index+=1){
  const id=pending[index],identity=key(id);if(seen.has(identity))continue;
  if(seen.size>=100000||pending.length>1000000)fail('record expansion exceeds its structural resource budget');seen.add(identity);
  const link=network.link(id);if(!link)fail('a record layout contains a dangling reference');
  pending.push(...link.references(),...(fields.get(identity)??[]).map(relation=>relation.child));
  if(link.metadata().language===language&&link.metadata().term===recordTerm&&link.references().length){records.push([link,readTypedProgramRecordShapeFields(network,link,language,true,fields)]);shapes.add(key(link.references()[0]));}
 }
 for(const [record,values]of records){record.setReferences([]);for(const field of values)network.insertFieldRelation(record.id(),field.label,field.child);}
 for(const relation of fields.get(key(root))??[])if(relation.label===profile.rootField){network.deleteLink(relation.link.id());shapes.add(key(relation.child));}
 const referenced=new Set(network.links().flatMap(link=>link.references().map(key)));
 for(const id of shapes)if(!referenced.has(id))network.deleteLink(id);
 return {network,root};
}
