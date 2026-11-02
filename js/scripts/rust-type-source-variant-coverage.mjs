// Structural inventory assertion over the ACTUAL current enum CST and the
// independently declared scenario inputs. It supplies no behavioral oracle.
import assert from 'node:assert/strict';
import {readProgramConcreteSyntaxFacts} from '../src/program-representation.js';
import {tokenize} from '../src/translation/lexer.js';
export function assertActualTypeVariantCoverage(network,source,capture,inputs){
 const items=capture.items.filter(row=>row.name==='Type'&&['enum_item','enumeration_item'].includes(row.kind));assert.equal(items.length,1);const owner=items[0],facts=readProgramConcreteSyntaxFacts(network,source),byId=new Map(facts.map(row=>[row.linkId,row])),relations=network.fieldRelations(),actual=[];
 for(const variant of facts.filter(row=>['enum_variant','enumeration_variant'].includes(row.term)&&row.start>=owner.bodyRange.start&&row.end<=owner.bodyRange.end)){
  const names=relations.filter(edge=>edge.parent.asU64()===variant.linkId&&edge.label==='name');assert.equal(names.length,1);const name=byId.get(names[0].child.asU64());assert.ok(name&&name.start>=variant.start&&name.end<=variant.end);const raw=source.slice(name.start,name.end);actual.push(raw.startsWith('r#')?raw.slice(2):raw);
 }
 assert.ok(actual.length>0,'the actual enum must expose its complete current variant inventory');assert.equal(new Set(actual).size,actual.length);
 const declared=[];for(const input of inputs){const tokens=tokenize(input,'Rust').tokens;assert.equal(tokens[0].value,'Type');assert.equal(tokens[1].value,'::');assert.equal(tokens[2].kind,'identifier');declared.push(tokens[2].value);}
 assert.deepEqual([...new Set(declared)].sort(),actual.sort(),'every actual Type variant needs an independently declared scenario and obsolete variants must be reconciled');
 return {actualVariants:actual,declaredVariants:declared};
}
