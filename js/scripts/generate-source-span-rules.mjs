import {readFileSync, writeFileSync} from 'node:fs';
import {spawnSync} from 'node:child_process';
import {readProjectData} from './project-data.mjs';
import {DecoratorSet} from '../src/decorators.js';
import {stripRustSourceHeaderStyleParentheses} from './rust-source-header-library-style.mjs';

const policyFile=new URL('../src/translation/data/source-span.lino',import.meta.url);
const policy=readProjectData(policyFile);
const javascript=new URL('../src/source-span-rules.js',import.meta.url);
const native=new URL('../../rust/src/typed_program_links/source_span_rules.rs',import.meta.url);
const copy=new URL('../../rust/src/data/source-span.lino',import.meta.url);
const style=new URL('../../parity/self-translation/source-span-rules-decorators.lino',import.meta.url);
const source=`// Generated from the shared source-span offset policy.\n/** @param {number} value @param {boolean} negativeZero @returns {boolean} */\nexport function acceptSourceSpanOffset(value, negativeZero) { return !negativeZero && value >= 0 && value <= ${policy.offsetMaximum} && value % 1 === 0; }\n/** @param {number} start @param {number} end @returns {boolean} */\nexport function acceptSourceSpanInterval(start, end) { return start <= end; }\n`;
if(process.argv.includes('--check')) {
  if(readFileSync(javascript,'utf8')!==source)throw new Error('source-span JavaScript rules are stale');
} else writeFileSync(javascript,source);
if(!process.argv.includes('--javascript-only')) {
  const {parseJavaScript}=await import('../src/translation/javascript.js');
  const {checkProgram}=await import('../src/translation/check.js');
  const {emitRust}=await import('../src/translation/emit-rust.js');
  const raw=emitRust(checkProgram(parseJavaScript(source))).definitions.join('\n\n');
  if(process.argv.includes('--update-styles')) {
    const actions=[];let current=raw;
    const replace=(from,to)=>{if(!current.includes(from))throw new Error(`source-span style target is absent: ${from}`);actions.push({op:'replace',field:'source',from,to});current=current.split(from).join(to);};
    for(const definition of raw.matchAll(/pub fn [\s\S]*?\n\}/gu)) {
      const original=definition[0],opening=original.indexOf('{');
      let body=stripRustSourceHeaderStyleParentheses(original.slice(opening+1,-1));
      if(original.startsWith('pub fn accept_source_span_offset(')) {
        // This predicate is a conjunction of pure scalar comparisons. Removing
        // their grouping preserves precedence and short-circuit evaluation.
        body=body.replace(/[()]/gu,'');
        body=body.replace('value >= 0f64 && value <= 9007199254740991f64','(0.0_f64..=9_007_199_254_740_991_f64).contains(&value)');
      }
      const header=original.slice(0,opening+1).replace('pub fn accept_source_span_interval(','pub const fn accept_source_span_interval(');
      replace(original,`/// Apply the shared source-span offset policy.\n#[must_use]\n${header}\n    ${body}\n}`);
    }
    writeFileSync(style,new DecoratorSet([{id:'source-span-library-style',level:'emitter',order:0,when:[['format','Rust']],actions}]).toLino());
  }
  const decorated=DecoratorSet.fromLino(readFileSync(style,'utf8')).decorate('emitter',{format:'Rust',source:raw}).source;
  const formatted=spawnSync('rustfmt',['--edition','2024','--emit','stdout'],{input:decorated,encoding:'utf8'});
  if(formatted.error||formatted.status!==0)throw formatted.error??new Error(formatted.stderr);
  const generated='// Generated through the checked JavaScript translator. Do not edit.\n'+formatted.stdout;
  if(process.argv.includes('--check')) {
    if(readFileSync(native,'utf8')!==generated||!readFileSync(copy).equals(readFileSync(policyFile)))throw new Error('native source-span rules or policy are stale');
  } else {writeFileSync(native,generated);writeFileSync(copy,readFileSync(policyFile));}
}
console.log('source spans: shared offset and interval rules generated');
