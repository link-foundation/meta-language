// Shared graph/CLI decisions over actual exact-ID metadata columns.
import {acceptUnsignedSourceCommandIdentity,acceptUnsignedSourceTarget} from './unsigned-source-command-rules.js';
/** @param {string} value @returns {string} */
function normalizeUnsignedGraphIdentity(value){let at=0;while(at+1<value.length&&value.charAt(at)==='0')at+=1;return value.slice(at);}
/** @param {string} first @param {string} second @returns {number} */
function compareUnsignedGraphIdentities(first,second){if(first.length!==second.length)return first.length<second.length?-1:1;for(let at=0;at<first.length;at+=1){const a=first.charCodeAt(at),b=second.charCodeAt(at);if(a!==b)return a<b?-1:1;}return 0;}
/** @param {string} type @param {string} language @param {string} term @param {string} definition @returns {boolean} */
function acceptUnsignedGraphRootMarker(type,language,term,definition){return type==='Semantic'&&language==='meta-language typed program'&&term==='typed program'&&definition==='1';}
/** @param {string} source @param {number} at @returns {number} */
function skipUnsignedGraphWhitespace(source,at){while(at<source.length){const code=source.charCodeAt(at);if(code!==9&&code!==10&&code!==13&&code!==32)break;at+=1;}return at;}
/** @param {string} source @param {number} at @returns {number} */
function readUnsignedGraphIdentityEnd(source,at){let end=at;while(end<source.length){const code=source.charCodeAt(end);if(code<48||code>57)break;end+=1;}return acceptUnsignedSourceCommandIdentity(source.slice(at,end))?end:-1;}
/** Canonical metadata uses percent-encoded scalar tokens, not quoted source.
 * @param {string} source @param {number} at @returns {number} */
function readUnsignedGraphMetadataEnd(source,at){
 if(source.slice(at,at+6)!=='(meta:')return -1;
 let depth=0;
 for(let index=at;index<source.length;index+=1){const code=source.charCodeAt(index);
  if(code===40){depth+=1;if(depth>64)return -1;}
  else if(code===41){depth-=1;if(depth===0)return index+1;}
  else if(!(code===9||code===10||code===13||code===32||code===37||code===45||code===46||code===58||code===95||code>=48&&code<=57||code>=65&&code<=90||code>=97&&code<=122))return -1;
 }
 return -1;
}
/** @param {string} source @param {number} at @returns {number} */
function readUnsignedGraphLinkEnd(source,at){
 if(source.charAt(at)!=='(')return -1;
 const end=readUnsignedGraphIdentityEnd(source,skipUnsignedGraphWhitespace(source,at+1));if(end<0)return -1;
 let cursor=skipUnsignedGraphWhitespace(source,end);if(source.charAt(cursor)!==':')return -1;cursor+=1;
 while(cursor<source.length){cursor=skipUnsignedGraphWhitespace(source,cursor);const token=source.charAt(cursor);
  if(token===')')return cursor+1;
  if(token==='('){const meta=readUnsignedGraphMetadataEnd(source,cursor);if(meta<0)return -1;const close=skipUnsignedGraphWhitespace(source,meta);return source.charAt(close)===')'?close+1:-1;}
  const next=readUnsignedGraphIdentityEnd(source,cursor);if(next<0)return -1;cursor=next;
 }
 return -1;
}
/** Validate only the identified snapshot surface; metadata meaning remains the
 * existing canonical decoder's responsibility. Source/LiNo DSL parsing is separate.
 * @param {string} source @returns {string} */
export function readUnsignedGraphTextSurfaceError(source){
 let at=skipUnsignedGraphWhitespace(source,0);
 while(at<source.length){const end=readUnsignedGraphLinkEnd(source,at);if(end<0)return 'saved graph text needs identified flat unsigned links with canonical metadata';at=skipUnsignedGraphWhitespace(source,end);}
 return '';
}
/** @param {number} bytes @param {number} records @param {number} byteLimit @param {number} recordLimit @returns {string} */
export function readUnsignedGraphReadBoundsError(bytes,records,byteLimit,recordLimit){
 if(!Number.isSafeInteger(byteLimit)||byteLimit<33||!Number.isSafeInteger(recordLimit)||recordLimit<1)return 'invalid saved graph resource bounds';
 if(!Number.isSafeInteger(bytes)||bytes<0||!Number.isSafeInteger(records)||records<0)return 'saved graph input needs exact nonnegative byte and record counts';
 return bytes>byteLimit?'saved graph input exceeds its byte limit':records>recordLimit?'saved graph input exceeds its record limit':'';
}
/** @param {number} records @param {number} limit @returns {string} */
export function readUnsignedTypedRootBoundsError(records,limit){return !Number.isSafeInteger(limit)||limit<1||!Number.isSafeInteger(records)||records<0||records>limit?'typed root facts exceed their record limit':'';}
/** Only stored actual schema roots qualify. An ambiguous graph needs a choice.
 * @param {string[]} ids @param {string[]} types @param {string[]} languages @param {string[]} terms @param {string[]} definitions @param {string} requested @param {number} limit @returns {string[]} */
export function planUnsignedTypedGraphRoot(ids,types,languages,terms,definitions,requested,limit){
 const bounds=readUnsignedTypedRootBoundsError(ids.length,limit);if(bounds!=='')return ['error',bounds];
 if(types.length!==ids.length||languages.length!==ids.length||terms.length!==ids.length||definitions.length!==ids.length)return ['error','typed root facts need complete aligned metadata columns'];
 if(requested!==''&&!acceptUnsignedSourceCommandIdentity(requested))return ['error','a typed graph root needs an exact positive unsigned identity'];
 const selected=requested===''?'':normalizeUnsignedGraphIdentity(requested);let found='',count=0;
 for(let at=0;at<ids.length;at+=1){const id=ids[at]??'';
  if(!acceptUnsignedSourceCommandIdentity(id)||normalizeUnsignedGraphIdentity(id)!==id||at>0&&compareUnsignedGraphIdentities(ids[at-1]??'',id)>=0)return ['error','typed root facts need canonical ordered unsigned identities'];
  if(!acceptUnsignedGraphRootMarker(types[at]??'',languages[at]??'',terms[at]??'',definitions[at]??''))continue;
  if(selected!==''&&id===selected)found=id;
  if(selected===''){found=id;count+=1;}
 }
 if(selected!=='')return found===''?['error','the selected link is not an actual declared typed program root']:['root',found];
 return count===1?['root',found]:count===0?['error','a graph needs an actual declared schema-1 typed program root']:['error','a graph with multiple typed program roots needs an explicit root'];
}
/** @returns {string} */
export function readUnsignedGraphCommandUsage(){return 'usage: meta-language exact-graph <file> [--input-binary] [--binary | --to <JavaScript|Rust|Lean|Rocq>] [--root <decimal>] [--rebase <decimal>]\nReads exact Links snapshots without reparsing source; emission checks the current selected typed graph.\n';}
/** @param {string} file @param {string} root @param {string} first @param {string} target @param {boolean} input @param {boolean} binary @returns {string[]} */
function finishUnsignedGraphCommand(file,root,first,target,input,binary){
 if(file===''||root!==''&&target===''||binary&&target!=='')return ['error'];
 if(target!==''&&!acceptUnsignedSourceTarget(target))return ['error'];
 return [file,input?'binary':'text',binary?'binary':'text',target,root,first];
}
/** [file, input encoding, output encoding, target, requested root, optional new first].
 * @param {string[]} args @returns {string[]} */
export function planUnsignedGraphCommand(args){
 let file='',root='',first='',target='',input=false,binary=false,ended=false;
 for(let at=0;at<args.length;at+=1){const arg=args[at]??'';
  if(!ended&&(arg==='--help'||arg==='-h'||arg==='help'))return ['help'];
  if(!ended&&arg==='--'){ended=true;continue;}
  if(!ended&&arg==='--input-binary'){if(input)return ['error'];input=true;continue;}
  if(!ended&&arg==='--binary'){if(binary)return ['error'];binary=true;continue;}
  if(!ended&&(arg==='--root'||arg==='--rebase'||arg==='--to')){
   at+=1;const value=args[at]??'';if(value===''||value.startsWith('-'))return ['error'];
   if(arg==='--to'){if(target!=='')return ['error'];target=value;}
   else{if(!acceptUnsignedSourceCommandIdentity(value))return ['error'];if(arg==='--root'){if(root!=='')return ['error'];root=value;}else{if(first!=='')return ['error'];first=value;}}
   continue;
  }
  if(file!==''||arg===''||!ended&&arg.startsWith('-'))return ['error'];file=arg;
 }
 return finishUnsignedGraphCommand(file,root,first,target,input,binary);
}
