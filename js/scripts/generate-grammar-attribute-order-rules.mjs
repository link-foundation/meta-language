// Translate the complete closed JavaScript kernel; hosts keep native map storage.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {emitRust} from '../src/translation/emit-rust.js';
import {checkProjectData,writeProjectData} from './project-data.mjs';
import {writeRecordRuleLibrary} from './record-rule-library.mjs';
import {generateGrammarAttributeOrderFixtures} from './generate-grammar-attribute-order-fixtures.mjs';

const root=new URL('../../',import.meta.url);
export function generateGrammarAttributeOrderRules({check=false,native=false}={}) {
  const path='js/src/translation/grammar-attribute-order-rules.js';
  const source=readFileSync(new URL(path,root),'utf8');
  const program=checkProgram(parseJavaScript(source,{referenceSemantics:false}));
  const emitted=emitRust(program,{arrayConsumption:true});
  assert.equal(emitted.definitions.length,program.declarations.size,
    'every checked source declaration and generated continuation is retained');
  const policy={schemaVersion:1,model:'ordinary grammar attribute key order 1',
    source:{path,bytes:Buffer.byteLength(source),sha256:createHash('sha256').update(source).digest('hex'),text:source},
    owners:[...program.declarations.values()].map(item=>({name:item.name,kind:item.k,fullName:item.fullName,modulePath:item.modulePath})),
    inputs:'Hosts supply exact UTF-16 code units. At most eleven leading units suffice: more than ten units always classify as a non-index key.',
    ordering:'Canonical decimal array-index names from 0 through 4294967294 come first in numeric order. Other names retain insertion order. Overwriting a key retains its position.',
    equality:'The existing grammar attribute equality ignores dictionary entry order. Lexical and selection metadata keep their distinct existing ordered equality.',
    nativeStorage:{crate:'indexmap',version:'2.14.2',role:'Store every attribute entry and its observable position. Shared translated decisions select the index prefix; native map operations supply storage only.'},
    boundary:'This complete-source rule translation does not establish native execution, full CST outcome transport or whole-language coverage.'};
  for(const runtime of native?['js','rust']:['js']){
    const file=new URL(`${runtime}/src/data/grammar-attribute-order-rules.lino`,root);
    if(check)checkProjectData(file,policy);else writeProjectData(file,policy);
  }
  if(native)writeRecordRuleLibrary(emitted,'grammar_attribute_order_rules',{check});
  generateGrammarAttributeOrderFixtures({check});
  return policy;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  const check=process.argv.includes('--check');
  generateGrammarAttributeOrderRules({check,native:!process.argv.includes('--javascript-only')&&(check||process.argv.includes('--native'))});
  console.log('Complete grammar attribute order rules and continuation owners match their shared source.');
}
