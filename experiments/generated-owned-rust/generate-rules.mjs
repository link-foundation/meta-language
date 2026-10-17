// A private linked interpretation for complete actual generated owned programs.
import {readFileSync,writeFileSync,mkdirSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {readProjectData,writeProjectData,checkProjectData} from '../../js/scripts/project-data.mjs';
import {DecoratorSet} from '../../js/src/decorators.js';
import {formatProductRuleSource} from '../../js/scripts/product-rule-style-corrections.mjs';
import {stripRustSourceHeaderStyleParentheses} from '../../js/scripts/rust-source-header-library-style.mjs';
const runtime=readProjectData(new URL('../../js/src/translation/data/rust-runtime-shell.lino',import.meta.url));
const owned=readProjectData(new URL('../../js/src/translation/data/rust-owned-local-state.lino',import.meta.url));
export const generatedOwnedRustPolicy={
 version:1,model:'closed generated owned Rust module 1',language:'Rust',observationModel:'closed generated owned String value and successful Unit runtime observation 1',
 runtimeModel:runtime.model,ownedCopyModel:owned.models.copies,
 syntax:{attribute:runtime.syntax.attribute,worker:runtime.syntax.workerFunction,entry:runtime.syntax.entry},runtimeAssumptions:runtime.assumptions,
 domains:{String:{kind:'string'},i32:{kind:'fixed',bits:32,signed:true},bool:{kind:'bool'}},
 operations:['owned generated value helper','owned standard String clone','owned standard String literal construction','owned lexical value','owned ordered product','owned product field','owned immutable local','owned scalar equality','owned conditional value','owned declared product call'],
 clone:{owner:['std','string','String'],method:'clone',arity:0,receiverKinds:['name','tupleIndex'],observation:'The actual unshadowed standard String clone preserves its Unicode scalar text; allocation failures, counts, pointer identity and destruction timing remain outside this value observation.'},
 generatedBindingPattern:'^ml_owned_local_value[0-9]+$',reservedOwners:['std','core','alloc','String','i32','bool','main','ml_main','ml_ownedTextInput'],
 literalQuoting:{minimumOrdinaryCodePoint:32,controlCodePoints:[9,10,13],scope:'Valid Unicode scalar literals admit the ordinary range plus tab, LF and CR. Other control characters need an independent edited-literal Rust quoting adapter; source interface inputs still admit every valid Rust String scalar.'},
 equality:{operation:'eq',operator:'==',nestedComparison:'Each nested comparison operand must retain its actual parentheses; unparenthesized comparison chains have no Rust source interpretation.'},
 trivia:{whitespaceRule:'acceptRustSourceWhitespace',initialByteOrderMarkRule:'acceptRustSourceByteOrderMark',lineComment:'//',blockCommentOpen:'/*',blockCommentClose:'*/',docMarkerCodeUnits:{exclamation:33,asterisk:42,slash:47},scope:'Only the shared exact Rust whitespace and initial byte-order-mark rules, ordinary line comments and correctly nested ordinary block comments are trivia. Outermost doc comments are attributes and require a separate source interpretation. Doc-looking nested block contents remain inside their actual ordinary outer comment.',reference:'https://doc.rust-lang.org/reference/comments.html'},
 inputAssumption:owned.targetInputAssumption,
 valueAssumption:{id:'generated-owned-string-value-success',operations:['owned standard String clone','owned standard String literal construction'],meaning:'Whenever this actual owned standard String operation is reached by a well-typed helper invocation, its allocation succeeds and preserves its Unicode scalar text. Allocation failures and counts, pointer identity, destruction timing and failure equivalence remain outside this value observation.'},
 fields:{proof:['model','language','observationModel','span','tokens','gaps','operations','assumptions','valueAssumptions','worker','clones'],token:['kind','value','raw','span','suffix','floating','declaration','binding','literal'],gap:['span','units'],operation:['kind','op','parts','span'],assumption:['id','meaning','operationNode'],clone:['operationNode','receiver','owner','method']},
 boundaries:['Only flat direct String/i32/Boolean headers, immutable local values, ordered String/i32 products and direct owned helper calls are interpreted.','Every String variable or String field read must be an actual standard zero-argument clone on its actual typed owner.','No references, dereferences, imports, attributes beyond the shared inert lint attribute, custom methods, recursive helpers, arithmetic or hidden effects are admitted.','The whole original token stream and every trivia code unit are represented; comments never grant semantic authority.','The experimental codec stores the inspectable source interpretation as root provenance with actual links to core declarations and binders. The generic codec alone does not validate this experimental interpretation.','The four runtime assumptions remain operation-bound successful-runtime conditions, not failure equivalence.','No native pairing, production selector or ledger evidence is claimed.'],
};
export function generatedOwnedRustRules(){return '// Generated from the private linked whole owned Rust policy. Do not edit.\n'
 +'/** @param {string} model @returns {boolean} */\nexport function acceptGeneratedOwnedRustModel(model){return model === '+JSON.stringify(generatedOwnedRustPolicy.model)+';}\n'
 +'/** @param {boolean} standard @param {boolean} string @param {boolean} receiver @param {number} arity @returns {boolean} */\nexport function acceptGeneratedOwnedRustClone(standard,string,receiver,arity){return standard&&string&&receiver&&arity === '+generatedOwnedRustPolicy.clone.arity+';}\n'
 +'/** @param {number} value @param {boolean} negativeZero @returns {boolean} */\nexport function acceptGeneratedOwnedRustCodeUnit(value,negativeZero){return !negativeZero&&value >= 0&&value <= 65535&&value % 1 === 0;}\n'
 +'/** @param {number} value @returns {boolean} */\nexport function acceptGeneratedOwnedRustLiteralCodePoint(value){return value >= '+generatedOwnedRustPolicy.literalQuoting.minimumOrdinaryCodePoint+' || '+generatedOwnedRustPolicy.literalQuoting.controlCodePoints.map(value=>'value === '+value).join(' || ')+';}\n'
 +'/** @param {boolean} comparison @param {boolean} parenthesized @returns {boolean} */\nexport function acceptGeneratedOwnedRustEqualityGrouping(comparison,parenthesized){return !comparison||parenthesized;}\n'
 +'/** @param {boolean} line @param {number} third @param {number} fourth @returns {boolean} */\nexport function acceptGeneratedOwnedRustOrdinaryComment(line,third,fourth){return third !== '+generatedOwnedRustPolicy.trivia.docMarkerCodeUnits.exclamation+' && (line ? third !== '+generatedOwnedRustPolicy.trivia.docMarkerCodeUnits.slash+' || fourth === '+generatedOwnedRustPolicy.trivia.docMarkerCodeUnits.slash+' : third !== '+generatedOwnedRustPolicy.trivia.docMarkerCodeUnits.asterisk+' || fourth === '+generatedOwnedRustPolicy.trivia.docMarkerCodeUnits.asterisk+' || fourth === '+generatedOwnedRustPolicy.trivia.docMarkerCodeUnits.slash+');}\n';}
async function nativeDecisions(check){
 const [{parseJavaScript},{checkProgram},{emitRust}]=await Promise.all([import('../../js/src/translation/javascript.js'),import('../../js/src/translation/check.js'),import('../../js/src/translation/emit-rust.js')]);
 const raw=emitRust(checkProgram(parseJavaScript(generatedOwnedRustRules()))).definitions.join('\n\n');
 const actions=[];let current=raw;const replace=(from,to)=>{if(!current.includes(from))throw new Error('whole owned rule style target disappeared');actions.push({op:'replace',field:'source',from,to});current=current.split(from).join(to);};replace('model: String','model: &str');
 for(const match of [...current.matchAll(/pub fn accept_generated_owned_rust_[\s\S]*?\n\}/gu)]){const d=match[0],at=d.indexOf('{'),body=stripRustSourceHeaderStyleParentheses(d.slice(at+1,-1));replace(d,'/// Experimental generated whole owned source decision.\n#[must_use]\n'+d.slice(0,at+1)+'\n    '+body+'\n}');}
 const style=new DecoratorSet([{id:'generated-owned-rust-library-style',level:'emitter',order:0,when:[['format','Rust']],actions}]).toLino();
 const decorated='// Experimental automatic translation; not a native runtime integration.\n'+formatProductRuleSource(DecoratorSet.fromLino(style).decorate('emitter',{format:'Rust',source:raw}).source);
 const file=new URL('./data/rules.rs',import.meta.url),styles=new URL('./data/library-styles.lino',import.meta.url);
 if(check){if(readFileSync(file,'utf8')!==decorated||readFileSync(styles,'utf8')!==style)throw new Error('experimental translated rules or styles are stale');}else{writeFileSync(file,decorated);writeFileSync(styles,style);}
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
 mkdirSync(new URL('./data/',import.meta.url),{recursive:true});const check=process.argv.includes('--check'),file=new URL('./data/policy.lino',import.meta.url),rules=new URL('./rules.mjs',import.meta.url);
 if(check){checkProjectData(file,generatedOwnedRustPolicy);if(readFileSync(rules,'utf8')!==generatedOwnedRustRules())throw new Error('whole owned Rust rules are stale');}else{writeProjectData(file,generatedOwnedRustPolicy);writeFileSync(rules,generatedOwnedRustRules());}
 await nativeDecisions(check);
}
