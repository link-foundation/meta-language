import {readFileSync,writeFileSync} from 'node:fs';
import {sourceTextByteRulesSource} from '../../experiments/generate-source-text-byte-rules.mjs';
import {DecoratorSet} from '../src/decorators.js';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {emitRust} from '../src/translation/emit-rust.js';
import {formatProductRuleSource} from './product-rule-style-corrections.mjs';
import {readSourceTextRustBlock,readSourceTextByteLibraryStyles} from './source-text-byte-library-style.mjs';
import {readProjectData} from './project-data.mjs';
const javascript=new URL('../src/translation/source-text-byte-rules.js',import.meta.url);
const native=new URL('../../rust/src/translation/source_text_byte_rules.rs',import.meta.url);
const style=new URL('../../parity/self-translation/source-text-byte-rules-decorators.lino',import.meta.url);
const originalPolicy=new URL('../../experiments/source-text-byte-model.lino',import.meta.url);
const maximum=readProjectData(originalPolicy).constants.maximumUnits;
const source=sourceTextByteRulesSource()+`\n/** @param {number} count @returns {boolean} */\nexport function acceptSourceTextUnitCount(count){return Number.isInteger(count)&&count>=0&&count<=${maximum};}\n`;
if(process.argv.includes('--check')){if(readFileSync(javascript,'utf8')!==source)throw new Error('source-text JavaScript numeric rules are stale');}
else writeFileSync(javascript,source);
const emitted=emitRust(checkProgram(parseJavaScript(source)));
const raw=formatProductRuleSource(readSourceTextRustBlock(emitted.text,'mod','ml_array')+'\n\n'+readSourceTextRustBlock(emitted.text,'fn','is_integer')+'\n\n'+emitted.definitions.join('\n\n'));
const provider=readFileSync(new URL('../../rust/src/translation/frontend_rules.rs',import.meta.url),'utf8');
const expectedStyle=new DecoratorSet([{id:'source-text-byte-library-style',level:'emitter',order:0,when:[['format','Rust']],actions:readSourceTextByteLibraryStyles(raw,provider)}]).toLino();
if(process.argv.includes('--update-styles'))writeFileSync(style,expectedStyle);
else if(readFileSync(style,'utf8')!==expectedStyle)throw new Error('source text byte library styles are stale; regenerate the recorded styles');
const decorated=DecoratorSet.fromLino(readFileSync(style,'utf8')).decorate('emitter',{format:'Rust',source:raw}).source;
const output='// Generated automatically from the linked source-text byte JavaScript kernel. Do not edit.\n'+formatProductRuleSource(decorated);
if(process.argv.includes('--check')){if(readFileSync(native,'utf8')!==output)throw new Error('source-text native numeric rules are stale');}
else writeFileSync(native,output);
for(const file of [new URL('../src/translation/data/source-text-byte-model.lino',import.meta.url),new URL('../../rust/src/data/source-text-byte-model.lino',import.meta.url)]){
 if(process.argv.includes('--check')){if(!readFileSync(file).equals(readFileSync(originalPolicy)))throw new Error('source-text shared model is stale');}
 else writeFileSync(file,readFileSync(originalPolicy));
}
console.log('Source-text byte rules: seven shared numeric algorithms automatically translated');
