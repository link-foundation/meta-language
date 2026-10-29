// Actual CST field relationships transport source facts to shared definitions.
// Every result remains a link in the complete original concrete network.
import {LinkNetwork} from './network.js';
import {LinkMetadata,LinkType} from './primitives.js';
import {tokenize} from './translation/lexer.js';
import {requireJavaScriptStringUnitTransport} from './translation/javascript-string-unit-rules.js';
import {acceptSourceModuleRequestTerm,acceptSourceModuleRequestTypeScope,acceptSourceModuleRequestLiteral,acceptSourceModuleRequestImportCallee,readSourceModuleRequestKind,readSourceModuleRequestCode,readSourceModuleRequestGrammar} from './translation/source-module-request-rules.js';
const fail=message=>{throw new TypeError('source module requests: '+message);};
export function analyzeSourceModuleRequests(source,language='JavaScript',options={}){
 const byteLimit=options.byteLimit??4*1024*1024,workLimit=options.workLimit??1000000;
 if(typeof source!=='string'||!source.isWellFormed()||!['JavaScript','TypeScript'].includes(language))fail('expected scalar JavaScript or TypeScript source');
 if(!Number.isSafeInteger(byteLimit)||byteLimit<1||byteLimit>128*1024*1024||!Number.isSafeInteger(workLimit)||workLimit<1||workLimit>10000000)fail('invalid resource limit');
 const bytes=new TextEncoder().encode(source);if(bytes.length>byteLimit)fail('source byte limit exceeded');
 if(options.tsx!==undefined&&typeof options.tsx!=='boolean'||options.tsx===true&&language!=='TypeScript')fail('invalid syntax profile');
 const network=LinkNetwork.parse(source,readSourceModuleRequestGrammar(language,options.tsx===true));if(!network.verifyFullMatch().isClean())fail('source does not parse cleanly');
 const links=network.links();if(links.length>workLimit)fail('request work limit exceeded');let work=links.length;
 const decoder=new TextDecoder('utf-8',{fatal:true}),text=span=>decoder.decode(bytes.subarray(span.start,span.end));
 const facts=links.filter(link=>link.metadata().linkType===LinkType.Syntax&&link.metadata().span).map(link=>({link,term:link.metadata().term,start:link.metadata().span.byteRange.start,end:link.metadata().span.byteRange.end}));
 const nodes=facts.filter(node=>acceptSourceModuleRequestTerm(node.term)).sort((left,right)=>left.start-right.start||left.end-right.end||left.link.id().asU64()-right.link.id().asU64());
 const statements=nodes.filter(node=>node.term!=='call_expression').sort((left,right)=>left.start-right.start||right.end-left.end);
 const typeSpans=facts.filter(node=>acceptSourceModuleRequestTypeScope(node.term)).map(node=>[node.start,node.end]).sort((left,right)=>left[0]-right[0]);let maximum=0;for(const span of typeSpans){maximum=Math.max(maximum,span[1]);span[1]=maximum;}
 const inType=node=>{let low=0,high=typeSpans.length;while(low<high){const middle=Math.floor((low+high)/2);if(typeSpans[middle][0]<=node.start)low=middle+1;else high=middle;}return language==='TypeScript'&&low>0&&typeSpans[low-1][1]>=node.end;};
 const fields=network.fieldRelations(),sources=new Map(),functions=new Map(),arguments_=new Map();
 for(const field of fields){if(++work>workLimit)fail('request work limit exceeded');const parent=network.link(field.parent),child=network.link(field.child);if(!parent||!child)fail('invalid concrete field relationship');const id=String(field.parent.asU64());
  if(field.label==='function')functions.set(id,child);if(field.label==='arguments')arguments_.set(id,child);
  if(field.label!=='source'||!acceptSourceModuleRequestLiteral(child.metadata().term)||!parent.metadata().span)continue;
  const span=parent.metadata().span.byteRange;let low=0,high=statements.length;while(low<high){const middle=Math.floor((low+high)/2);if(statements[middle].start<=span.start)low=middle+1;else high=middle;}
  const owner=statements[low-1];if(owner&&owner.end>=span.end)sources.set(String(owner.link.id().asU64()),child);
 }
 const requests=[],requestLinks=[];
 for(const node of nodes){if(++work>workLimit)fail('request work limit exceeded');const id=String(node.link.id().asU64());let literal=null,second='',third='';
  if(node.term==='call_expression'){
   if(!acceptSourceModuleRequestImportCallee(functions.get(id)?.metadata().term??''))continue;
   const args=arguments_.get(id);if(!args)fail('missing concrete import arguments');
   work+=args.references().length;if(work>workLimit)fail('request work limit exceeded');
   const first=args.references().map(reference=>network.link(reference)).find(link=>link&&link.metadata().linkType===LinkType.Syntax&&link.metadata().named&&!link.metadata().flags.isExtra);
   if(first&&acceptSourceModuleRequestLiteral(first.metadata().term))literal=first;
  }else{
   literal=sources.get(id);if(!literal)continue;
   const start=literal.metadata().span?.byteRange.start;if(start===undefined)fail('missing concrete specifier span');
   work+=start-node.start;if(work>workLimit)fail('request work limit exceeded');
   const prefix=tokenize(text({start:node.start,end:start}),'JavaScript').tokens;second=prefix[1]?.value??'';third=prefix[2]?.value??'';
  }
  let specifier=null,specifierUnits=null;
  if(literal){const span=literal.metadata().span?.byteRange;if(!span)fail('missing concrete specifier span');work+=span.end-span.start;if(work>workLimit)fail('request work limit exceeded');const tokens=tokenize(text(span),'JavaScript').tokens;if(tokens.length!==2||tokens[0].kind!=='string')fail('invalid concrete specifier literal');const value=tokens[0].value;specifierUnits=tokens[0].stringUnits??Array.from({length:value.length},(_,index)=>value.charCodeAt(index));if(!requireJavaScriptStringUnitTransport(specifierUnits))specifier=value;}
  work+=3+(specifierUnits?.length??0);if(work>workLimit)fail('request work limit exceeded');
  const kind=readSourceModuleRequestKind(readSourceModuleRequestCode(language,node.term,second,third,inType(node)));
  const kindLink=network.insertTypedPoint(LinkType.Concept,kind,'A source module dependency request kind.');
  const value=specifierUnits===null?network.insertTypedPoint(LinkType.Concept,'computed module specifier','A module specifier requiring a source interpretation.'):network.insertLink(specifierUnits.map(code=>network.insertTypedPoint(LinkType.Semantic,'UTF-16 code unit '+code,String(code))),LinkMetadata.new().withLinkType(LinkType.Semantic).withTerm('UTF-16 code unit sequence'));
  const link=network.insertLink([kindLink,node.link.id(),value],node.link.metadata().clone({linkType:LinkType.Semantic,term:'source module request'}));requestLinks.push(link);
  requests.push({kind,specifier,specifierUnits,start:node.start,end:node.end,linkId:String(link.asU64()),syntaxLinkId:id});
 }
 const root=network.insertLink(requestLinks,LinkMetadata.new().withLinkType(LinkType.Semantic).withTerm('source module requests'));
 return{network,root,requests,coordinateUnit:'UTF-8 bytes'};
}
