import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {Grammar} from '../src/grammar.js';
import {renderGrammarLinks} from '../src/grammar-links.js';
import {grammarAttributeOrderCases,grammarAttributeIntegerNameCases} from '../tests/support/grammar-attribute-order-cases.js';
import {checkProjectData,writeProjectData} from './project-data.mjs';

const root=new URL('../../',import.meta.url);
export function generateGrammarAttributeOrderFixtures({check=false}={}) {
  const source='js/src/translation/grammar-attribute-order-rules.js';
  const cases=grammarAttributeOrderCases().map(item=>{
    const action=item.keys.map((attribute,value)=>({operation:'setAttribute',attribute,value:{operation:'integer',value}}));
    const grammar=new Grammar('document',[['document',{expression:{kind:'literal',value:'x'},action}]]);
    return {...item,source:'x',grammar:renderGrammarLinks(grammar),
      entries:item.order.map(key=>[key,item.keys.lastIndexOf(key)])};
  });
  const fixture={schemaVersion:1,source,sourceSha256:createHash('sha256').update(readFileSync(new URL(source,root))).digest('hex'),
    boundary:'Exact ordinary own-key ordering, overwrite positions and compact UTF-16 attribute rendering on authored grammar actions. This does not establish complete CST graph transport or native execution.',
    nativeOrdering:{id:'grammar-attribute-map-order',subject:'indexmap',comparison:{kind:'release',precision:3},
      expectations:[{id:'indexmap-2.14.2-insertion-order',minimum:'2.14.2',maximum:'2.14.2',includeMinimum:true,includeMaximum:true,
        expected:'preserve insertion order when collecting attribute entries',
        evidence:'IndexMap 2.14.2 documents insertion-ordered iteration. The shared grammar cases independently declare observable entry order; native execution remains a CI check. https://docs.rs/indexmap/2.14.2/indexmap/map/struct.IndexMap.html'}]},
    cases,integerNames:grammarAttributeIntegerNameCases()};
  const output=new URL('parity/fixtures/grammar-attribute-order.lino',root);
  if(check)checkProjectData(output,fixture);else writeProjectData(output,fixture);
  return fixture;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  generateGrammarAttributeOrderFixtures({check:process.argv.includes('--check')});
  console.log('Grammar attribute order inputs and independent expectations encoded as Links.');
}
