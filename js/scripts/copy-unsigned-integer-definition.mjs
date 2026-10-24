// Mechanical source declaration copy; no represented arithmetic is executed.
import {readFileSync,writeFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {tokenize} from '../src/translation/lexer.js';
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const path='js/src/translation/number-bitwise-rules.js';
const source=readFileSync(new URL('../../'+path,import.meta.url),'utf8');
const digest=value=>createHash('sha256').update(value).digest('hex');
const sourceSha256='a9c16524d4da2e55092f3dc3b0f3ae6b9a08de44792966782d14bfdce1598f56';
if(digest(source)!==sourceSha256)throw new TypeError('unreviewed unsigned normalization source context');
const {tokens,comments}=tokenize(source,'JavaScript');
const at=tokens.findIndex((token,index)=>token.kind==='identifier'&&token.value==='function'&&tokens[index+1]?.value==='normalizeJavaScriptUnsignedInteger');
if(at<1||tokens[at-1].value!=='export')throw new TypeError('the actual exported unsigned normalization declaration is required');
const exported=tokens[at-1],documentation=comments.filter(comment=>comment.end<=exported.start).at(-1);
if(!documentation||source.slice(documentation.end,exported.start).trim()!=='')throw new TypeError('the actual full documentation context is required');
let open=at+1;while(tokens[open]?.value!=='{')open+=1;
let depth=0,close=open;
for(;close<tokens.length;close+=1){const token=tokens[close];if(token.kind!=='punct')continue;if(token.value==='{')depth+=1;if(token.value==='}'){depth-=1;if(depth===0)break;}}
if(close>=tokens.length)throw new TypeError('unclosed actual source declaration');
const declaration=source.slice(documentation.start,tokens[close].end);
const target='js/src/translation/unsigned-integer-rules.js';
const text='// Mechanically copied complete source declaration; do not edit.\n// Qualified copy pending complete equivalence admission; regenerate with copy-unsigned-integer-definition.mjs.\n'+declaration+'\n';
writeFileSync(new URL('../../'+target,import.meta.url),text);
writeProjectData(new URL('../../parity/self-translation/unsigned-integer-definition-source.lino',import.meta.url),{schemaVersion:1,model:'qualified complete unsigned normalization source copy 1',source:{path,source,sourceSha256,bytes:Buffer.byteLength(source)},declaration:{start:documentation.start,end:tokens[close].end,text:declaration},target:{path:target,source:text,sourceSha256:digest(text),bytes:Buffer.byteLength(text)},boundary:'Complete source construction only. Original and prepared definitions retain qualified identities until whole meaning/effect/context equivalence is admitted. No arithmetic, fixture, oracle, tests or observations.'});
