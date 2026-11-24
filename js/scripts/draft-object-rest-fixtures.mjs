// Complete future source controls and immutable predecessor assertions only.
import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const cases=[];
function add(name,body,value,prefix=''){const entry='objectRestCase'+cases.length;cases.push({name,source:prefix+'export function compute(){'+body+'}',entry,value,abort:null});}
add('retained exact former rest refusal','return f();','3','function f(){const {...rest}={value:3};return rest.value;}');
add('rest alone copies all actual fields','const {...rest}={beta:3,alpha:4};return rest.beta+rest.alpha;','7');
add('one bound property is excluded','const {left,...rest}={left:3,right:4};return String(left)+":"+String(rest.right);','3:4');
add('empty source creates an empty own field record','const {...rest}={};return Object.keys(rest).join(",");','');
add('excluding the complete source creates an empty record','const {value,...rest}={value:3};return String(value)+":"+Object.keys(rest).join(",");','3:');
add('missing initialized property does not add an own field','const {missing=3,...rest}={value:4};return String(missing)+":"+Object.keys(rest).join(",");','3:value');
add('aliased exclusions use the property name','const {left:renamed,...rest}={left:3,renamed:4};return String(renamed)+":"+String(rest.renamed);','3:4');
add('repeated selected properties exclude one own field','const {left:first,left:second,...rest}={left:3,right:4};return String(first+second)+":"+Object.keys(rest).join(",");','6:right');
add('numeric fields retain numeric order and text fields retain insertion','const {selected,...rest}={zeta:1,10:2,2:3,selected:4,alpha:5};return Object.keys(rest).join(",");','2,10,zeta,alpha');
add('record alternatives retain their actual value order','const choose=flag=>flag?{left:1,right:2}:{right:2,left:1};const {...rest}=choose(false);return Object.keys(rest).join(",");','right,left');
add('ordered spreads retain overwrite positions','const first={zeta:1,alpha:2};const {selected,...rest}={...first,selected:3,zeta:4,beta:5};return Object.keys(rest).join(",")+":"+String(rest.zeta);','zeta,alpha,beta:4');
add('heterogeneous field types retain their separate identities','const {selected,...rest}={selected:0,text:"value",number:3,flag:true};return rest.text+":"+String(rest.number)+":"+String(rest.flag);','value:3:true');
add('ordinary own prototype names remain own properties','const {...rest}={constructor:3,toString:4};return Object.keys(rest).join(",")+":"+String(rest.constructor+rest.toString);','constructor,toString:7');
add('mutable lexical rest binding retains source writes','let {left,...rest}={left:3,right:4};rest={right:left+rest.right};return rest.right;','7');
add('a copied primitive field remains independent of source updates','const source={value:3};const {...rest}=source;source.value=7;return String(rest.value)+":"+String(source.value);','3:7');
add('a rest record has a fresh actual reference identity','const source={value:3};const {...rest}=source;return String(rest===source);','false');
add('shallow copies preserve nested shared field identity','const child={value:3};const source={child};const {...rest}=source;child.value=7;return String(rest.child===child)+":"+String(rest.child.value);','true:7');
add('the right hand side is evaluated once before copying','let calls=0;const source=()=>{calls+=1;return {value:3};};const {...rest}=source();return String(calls)+":"+String(rest.value);','1:3');
add('defaults run before copying the remaining fields','let events="";const mark=()=>{events+="d";return 3;};const source=()=>{events+="s";return {right:4};};const {left=mark(),...rest}=source();return events+":"+String(left+rest.right);','sd:7');
add('rest bindings retain their outer lexical capture','const source={value:3};const read=()=>{const {...rest}=source;return rest.value;};return read();','3');
const refusals=[
 {name:'a rest property is the final actual binding',source:'function f(){const {...rest,value}={value:3};return value;}',kind:'unsupported',reason:'rest property: name every field'},
 {name:'rest has no initializer',source:'function f(){const {...rest={}}={value:3};return rest;}',kind:'unsupported',reason:'rest property: name every field'},
 {name:'rest does not permit a trailing comma',source:'function f(){const {...rest,}={value:3};return rest;}',kind:'unsupported',reason:'rest property: name every field'},
 {name:'rest duplicates remain actual BoundNames errors',source:'function f(){let {left:value,...value}={left:3};return value;}',kind:'syntax',reason:'duplicate lexical binding value'},
 {name:'primitive ToObject rest domains retain their complete obligations',source:'function f(){const {...rest}="abc";return rest;}',kind:'unsupported',reason:'object rest receiver: every actual enumerable own field and its source order must be represented'},
];
const revision='62160f11';
const history=['parity/fixtures/object-binding-declarations.lino','js/scripts/draft-object-binding-declaration-fixtures.mjs','js/tests/translation-object-binding-declarations.test.js','rust/tests/unit/translation_javascript_object_binding_declarations.rs','js/src/translation/javascript-record-parameter-rules.js'].map(path=>{const bytes=execFileSync('git',['show',revision+':'+path],{maxBuffer:4*1024*1024});return {path,revision,source:bytes.toString('utf8'),bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex')};});
writeProjectData(new URL('../../parity/fixtures/object-rest-bindings.lino',import.meta.url),{schemaVersion:1,model:'actual closed own-field lexical object rest 1',sourceLanguage:'JavaScript',cases,refusals,history,versions:{JavaScript:'ECMAScript 2025; Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024'},source:cases.map(row=>row.source.replace('function compute(','function '+row.entry+'(')).join('\n'),assertions:['actualRestBoundNameAndFinalSyntax','exactOriginalPropertyExclusions','completeHeterogeneousOwnFieldTypes','sourceRuntimePropertyOrderAfterExclusion','freshOuterRecordAndShallowSharedFields','rightHandSideOnceThenOrderedDefaultsThenCopy','retainedFormerRestInputIndependentValue','sourceDefinitionsTwoCodecsFourEmissionsNativeProducts'],boundary:'Future controls only; none executed. Closed ordinary enumerable String-key records use the actual source value order and selected fields. Complete symbol/getter/proxy/inherited/primitive ToObject behavior, nested/computed/string-key binding patterns and formal rest retain their implementation obligations.'});
